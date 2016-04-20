// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use dbcache::{self, IndexTable, RecordTable};
use hnet::{Supervisor, Supervisable};
use protobuf::{parse_from_bytes, Message};
use protocol::net::{self, ErrCode};
use protocol::vault as proto;
use zmq;

use config::Config;
use data_store::{DataStore, Origin};
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

pub struct Worker {
    config: Arc<Mutex<Config>>,
    sock: zmq::Socket,
    datastore: Option<DataStore>,
}

impl Worker {
    fn datastore(&self) -> &DataStore {
        self.datastore.as_ref().unwrap()
    }

    fn dispatch(&mut self, msg: &zmq::Message) -> Result<()> {
        match msg.as_str() {
            Some("OriginCreate") => {
                let request = try!(self.sock.recv_msg(0));
                let req: proto::OriginCreate = parse_from_bytes(&request).unwrap();
                let mut origin = Origin::from(req);
                // JW TODO: handle db errors
                try!(self.datastore().origins.write(&mut origin));
                let reply: proto::Origin = origin.into();
                self.sock.send_str("Origin", zmq::SNDMORE).unwrap();
                self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
            }
            Some("OriginGet") => {
                let request = try!(self.sock.recv_msg(0));
                let req: proto::OriginGet = parse_from_bytes(&request).unwrap();
                match self.datastore().origins.name_idx.find(req.get_name()) {
                    Ok(origin_id) => {
                        let origin = self.datastore().origins.find(&origin_id).unwrap();
                        let reply: proto::Origin = origin.into();
                        self.sock.send_str("Origin", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(dbcache::Error::EntityNotFound) => {
                        let reply = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-get:1");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(e) => {
                        error!("OriginGet, err={:?}", e);
                        let reply = net::err(ErrCode::BUG, "vt:origin-get:0");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                }
            }
            Some("OriginList") => {
                let origin1 = proto::Origin::new();
                let origin2 = proto::Origin::new();
                let origins = vec![origin1, origin2];
                for origin in origins {
                    self.sock.send_str("Origin", zmq::SNDMORE).unwrap();
                    self.sock.send(&origin.write_to_bytes().unwrap(), 0).unwrap();
                }
            }
            _ => panic!("unexpected message: {:?}", msg.as_str()),
        }
        Ok(())
    }
}

impl Supervisable for Worker {
    type Config = Config;
    type Error = Error;

    fn new(context: &mut zmq::Context, config: Arc<Mutex<Config>>) -> Self {
        let sock = context.socket(zmq::DEALER).unwrap();
        Worker {
            config: config,
            sock: sock,
            datastore: None,
        }
    }

    fn init(&mut self) -> Result<()> {
        loop {
            let result = {
                let cfg = self.config.lock().unwrap();
                DataStore::open(cfg.deref())
            };
            match result {
                Ok(datastore) => {
                    self.datastore = Some(datastore);
                    break;
                }
                Err(e) => {
                    error!("{}", e);
                    thread::sleep(Duration::from_millis(5000));
                }
            }
        }
        Ok(())
    }

    fn on_message(&mut self, ident: zmq::Message) -> Result<()> {
        // JW TODO: abstract this out to be more developer friendly
        // pop lq ident
        let ident2 = try!(self.sock.recv_msg(0));
        let mut msg = zmq::Message::new().unwrap();
        // pop request frame
        try!(self.sock.recv(&mut msg, 0));
        // pop message-id
        try!(self.sock.recv(&mut msg, 0));
        // send reply
        //  -> client ident
        //  -> lq ident
        //  -> empty rep frame
        //  -> actual message
        self.sock.send_msg(ident, zmq::SNDMORE).unwrap();
        self.sock.send_msg(ident2, zmq::SNDMORE).unwrap();
        self.sock.send(&[], zmq::SNDMORE).unwrap();
        self.dispatch(&msg)
    }

    fn socket(&mut self) -> &mut zmq::Socket {
        &mut self.sock
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.sock.close().unwrap();
    }
}

pub struct Server {
    config: Arc<Mutex<Config>>,
    ctx: Arc<Mutex<zmq::Context>>,
    fe_sock: zmq::Socket,
    be_sock: zmq::Socket,
}

impl Server {
    pub fn new(config: Config) -> Self {
        let mut ctx = zmq::Context::new();
        let fe = ctx.socket(zmq::ROUTER).unwrap();
        let be = ctx.socket(zmq::DEALER).unwrap();
        Server {
            config: Arc::new(Mutex::new(config)),
            ctx: Arc::new(Mutex::new(ctx)),
            fe_sock: fe,
            be_sock: be,
        }
    }

    pub fn reconfigure(&self, config: Config) -> Result<()> {
        {
            let mut cfg = self.config.lock().unwrap();
            *cfg = config;
        }
        // obtain lock and replace our config
        // notify datastore to refresh it's connection if it needs to
        // notify sockets to reconnect if changes
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        {
            let cfg = self.config.lock().unwrap();
            try!(self.fe_sock.bind(&cfg.fe_addr()));
            try!(self.be_sock.bind(BE_LISTEN_ADDR));
            println!("Listening on ({})", cfg.fe_addr());
        }

        let ctx = self.ctx.clone();
        let cfg = self.config.clone();
        let sup: Supervisor<Worker> = Supervisor::new(ctx, cfg);
        // JW TODO: use config to determine worker count? I don't know if that's a good idea.
        try!(sup.start(BE_LISTEN_ADDR, 8));
        try!(zmq::proxy(&mut self.fe_sock, &mut self.be_sock));
        Ok(())
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.fe_sock.close().unwrap();
        self.be_sock.close().unwrap();
    }
}

pub fn run(config: Config) -> Result<()> {
    Server::new(config).run()
}
