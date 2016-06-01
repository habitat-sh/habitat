// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

use dbcache::{self, IndexSet, InstaSet};
use hab_net::server::{Application, Envelope, NetIdent, RouteConn, Service, Supervisor,
                      Supervisable};
use protocol::net::{self, ErrCode};
use protocol::vault as proto;
use zmq;

use config::Config;
use data_store::DataStore;
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

pub struct Worker {
    config: Arc<RwLock<Config>>,
    sock: zmq::Socket,
    datastore: Option<DataStore>,
}

impl Worker {
    fn datastore(&self) -> &DataStore {
        self.datastore.as_ref().unwrap()
    }

    fn dispatch(&mut self, req: &mut Envelope) -> Result<()> {
        match req.message_id() {
            "OriginCreate" => {
                let msg: proto::OriginCreate = try!(req.parse_msg());
                let mut origin = proto::Origin::new();
                origin.set_name(msg.get_name().to_string());
                origin.set_owner_id(msg.get_owner_id());
                // JW TODO: handle db errors
                try!(self.datastore().origins.write(&mut origin));
                try!(req.reply_complete(&mut self.sock, &origin));
            }
            "OriginGet" => {
                let mut msg: proto::OriginGet = try!(req.parse_msg());
                match self.datastore().origins.name_idx.find(&msg.take_name()) {
                    Ok(origin_id) => {
                        let origin = self.datastore().origins.find(&origin_id).unwrap();
                        try!(req.reply_complete(&mut self.sock, &origin));
                    }
                    Err(dbcache::Error::EntityNotFound) => {
                        let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-get:1");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                    Err(e) => {
                        error!("OriginGet, err={:?}", e);
                        let err = net::err(ErrCode::BUG, "vt:origin-get:0");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                }
            }
            "OriginList" => {
                let origin1 = proto::Origin::new();
                let origin2 = proto::Origin::new();
                let origins = vec![origin1, origin2];
                for (i, origin) in origins.iter().enumerate() {
                    if i == origins.len() {
                        try!(req.reply_complete(&mut self.sock, origin));
                    } else {
                        try!(req.reply(&mut self.sock, origin));
                    }
                }
            }
            _ => panic!("unexpected message: {}", req.message_id()),
        }
        Ok(())
    }
}

impl Supervisable for Worker {
    type Config = Config;
    type Error = Error;

    fn new(context: &mut zmq::Context, config: Arc<RwLock<Config>>) -> Self {
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
                let cfg = self.config.read().unwrap();
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

    fn on_message(&mut self, req: &mut Envelope) -> Result<()> {
        self.dispatch(req)
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
    config: Arc<RwLock<Config>>,
    ctx: Arc<RwLock<zmq::Context>>,
    router: RouteConn,
    be_sock: zmq::Socket,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let mut ctx = zmq::Context::new();
        let router = try!(RouteConn::new(Self::net_ident(), &mut ctx));
        let be = try!(ctx.socket(zmq::DEALER));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
            ctx: Arc::new(RwLock::new(ctx)),
            router: router,
            be_sock: be,
        })
    }

    pub fn reconfigure(&self, config: Config) -> Result<()> {
        {
            let mut cfg = self.config.write().unwrap();
            *cfg = config;
        }
        // obtain lock and replace our config
        // notify datastore to refresh it's connection if it needs to
        // notify sockets to reconnect if changes
        Ok(())
    }
}

impl Application for Server {
    type Error = Error;

    fn run(&mut self) -> Result<()> {
        try!(self.be_sock.bind(BE_LISTEN_ADDR));
        let ctx = self.ctx.clone();
        let cfg = self.config.clone();
        let sup: Supervisor<Worker> = Supervisor::new(ctx, cfg);
        {
            let cfg = self.config.read().unwrap();
            try!(sup.start(BE_LISTEN_ADDR, cfg.worker_threads));
        }
        try!(self.connect());
        try!(zmq::proxy(&mut self.router.socket, &mut self.be_sock));
        Ok(())
    }
}

impl Service for Server {
    type Application = Self;
    type Config = Config;
    type Error = Error;

    fn protocol() -> net::Protocol {
        net::Protocol::VaultSrv
    }

    fn config(&self) -> &Arc<RwLock<Self::Config>> {
        &self.config
    }

    fn conn(&self) -> &RouteConn {
        &self.router
    }

    fn conn_mut(&mut self) -> &mut RouteConn {
        &mut self.router
    }
}

impl NetIdent for Server {}

pub fn run(config: Config) -> Result<()> {
    try!(Server::new(config)).run()
}
