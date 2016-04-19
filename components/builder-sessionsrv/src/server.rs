// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use dbcache::{DataStore, Model};
use hnet::{Supervisor, Supervisable};
use protobuf::{parse_from_bytes, Message};
use protocol::net::{self, ErrCode};
use protocol::sessionsrv::{self, SessionGet, GitHubAuth};
use zmq;

use config::Config;
use data_model::{Account, Session};
use error::{Error, Result};
use oauth::github;

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

pub struct Worker {
    config: Arc<Mutex<Config>>,
    sock: zmq::Socket,
    datastore: DataStore,
}

impl Worker {
    fn dispatch(&mut self, msg: &zmq::Message) -> Result<()> {
        // JW TOOD: refactor the dispatch loop into handlers
        match msg.as_str() {
            Some("SessionCreate") => {
                let request = try!(self.sock.recv_msg(0));
                let req: GitHubAuth = parse_from_bytes(&request).unwrap();
                match github::authenticate(&req.get_code()) {
                    Ok(token) => {
                        // JW TODO: refactor this mess into a find and create routine
                        let account: Account = match Session::get(&self.datastore, &token) {
                            Ok(Session {owner_id: owner, ..}) => {
                                // JW TODO: handle error. This should not ever happen since session
                                // and account create will be transactional
                                self.datastore.find(&owner).unwrap()
                            }
                            _ => {
                                match github::user(&token) {
                                    Ok(user) => {
                                        // JW TODO: wrap session & account creation into a
                                        // transaction and handle errors
                                        let mut account: Account = user.into();
                                        try!(account.save(&self.datastore));
                                        let session = Session::new(token.clone(),
                                                                   account.id.clone());
                                        try!(session.create(&self.datastore));
                                        account
                                    }
                                    Err(e @ Error::JsonDecode(_)) => {
                                        debug!("github user get, err={:?}", e);
                                        let reply = net::err(ErrCode::BAD_REMOTE_REPLY,
                                                             "ss:auth:2");
                                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                                        self.sock
                                            .send(&reply.write_to_bytes().unwrap(), 0)
                                            .unwrap();
                                        return Ok(());
                                    }
                                    Err(e) => {
                                        error!("github user get, err={:?}", e);
                                        let reply = net::err(ErrCode::BUG, "ss:auth:3");
                                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                                        self.sock
                                            .send(&reply.write_to_bytes().unwrap(), 0)
                                            .unwrap();
                                        return Ok(());
                                    }
                                }
                            }
                        };
                        let mut reply: sessionsrv::Session = account.into();
                        reply.set_token(token);
                        self.sock.send_str("Session", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(Error::Auth(e)) => {
                        debug!("github authentication, err={:?}", e);
                        let reply = net::err(ErrCode::REMOTE_REJECTED, e.error);
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(e @ Error::JsonDecode(_)) => {
                        debug!("github authentication, err={:?}", e);
                        let reply = net::err(ErrCode::BAD_REMOTE_REPLY, "ss:auth:1");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(e) => {
                        error!("github authentication, err={:?}", e);
                        let reply = net::err(ErrCode::BUG, "ss:auth:0");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                }
            }
            Some("SessionGet") => {
                let request = try!(self.sock.recv_msg(0));
                let req: SessionGet = parse_from_bytes(&request).unwrap();
                match Session::get(&self.datastore, &req.get_token()) {
                    Ok(Session {owner_id: owner, ..}) => {
                        // JW TODO: handle error. This should not ever happen since session
                        // and account create will be transactional
                        let account: Account = self.datastore.find(&owner).unwrap();
                        let mut reply: sessionsrv::Session = account.into();
                        reply.set_token(req.get_token().to_string());
                        self.sock.send_str("Session", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(Error::EntityNotFound) => {
                        let reply = net::err(ErrCode::ENTITY_NOT_FOUND, "ss:auth:4");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
                    Err(e) => {
                        error!("database error, err={:?}", e);
                        let reply = net::err(ErrCode::INTERNAL, "ss:auth:5");
                        self.sock.send_str("NetError", zmq::SNDMORE).unwrap();
                        self.sock.send(&reply.write_to_bytes().unwrap(), 0).unwrap();
                    }
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
            datastore: DataStore::new(),
        }
    }

    fn init(&mut self) -> Result<()> {
        loop {
            let result = {
                let cfg = self.config.lock().unwrap();
                self.datastore.open(cfg.deref())
            };
            match result {
                Ok(()) => break,
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
