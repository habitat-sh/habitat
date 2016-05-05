// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

use dbcache::{Model, RecordTable};
use hab_net::server::{Application, Envelope, NetIdent, RouteConn, Service, Supervisor,
                      Supervisable};
use protocol::net::{self, ErrCode};
use protocol::sessionsrv::{self, SessionGet, SessionCreate};
use zmq;

use config::Config;
use data_store::{DataStore, Account, Session};
use error::{Error, Result};
use oauth::github;

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
            "SessionCreate" => {
                let msg: SessionCreate = try!(req.parse_msg());
                match github::authenticate(&msg.get_code()) {
                    Ok(token) => {
                        // JW TODO: refactor this mess into a find and create routine
                        let account: Account = match Session::get(&self.datastore().sessions,
                                                                  &token) {
                            Ok(Session { owner_id: owner, .. }) => {
                                // JW TODO: handle error. This should not ever happen since session
                                // and account create will be transactional
                                self.datastore().accounts.find(owner).unwrap()
                            }
                            _ => {
                                match github::user(&token) {
                                    Ok(user) => {
                                        // JW TODO: wrap session & account creation into a
                                        // transaction and handle errors
                                        let mut account: Account = user.into();
                                        try!(self.datastore().accounts.write(&mut account));
                                        let session = Session::new(token.clone(),
                                                                   account.id.clone());
                                        try!(session.create(&self.datastore().sessions));
                                        account
                                    }
                                    Err(e @ Error::JsonDecode(_)) => {
                                        debug!("github user get, err={:?}", e);
                                        let err = net::err(ErrCode::BAD_REMOTE_REPLY, "ss:auth:2");
                                        try!(req.reply_complete(&mut self.sock, &err));
                                        return Ok(());
                                    }
                                    Err(e) => {
                                        error!("github user get, err={:?}", e);
                                        let err = net::err(ErrCode::BUG, "ss:auth:3");
                                        try!(req.reply_complete(&mut self.sock, &err));
                                        return Ok(());
                                    }
                                }
                            }
                        };
                        let mut reply: sessionsrv::Session = account.into();
                        reply.set_token(token);
                        try!(req.reply_complete(&mut self.sock, &reply));
                    }
                    Err(Error::Auth(e)) => {
                        debug!("github authentication, err={:?}", e);
                        let err = net::err(ErrCode::REMOTE_REJECTED, e.error);
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                    Err(e @ Error::JsonDecode(_)) => {
                        debug!("github authentication, err={:?}", e);
                        let err = net::err(ErrCode::BAD_REMOTE_REPLY, "ss:auth:1");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                    Err(e) => {
                        error!("github authentication, err={:?}", e);
                        let err = net::err(ErrCode::BUG, "ss:auth:0");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                }
            }
            "SessionGet" => {
                let msg: SessionGet = try!(req.parse_msg());
                match Session::get(&self.datastore().sessions, &msg.get_token()) {
                    Ok(Session { owner_id: owner, .. }) => {
                        // JW TODO: handle error. This should not ever happen since session
                        // and account create will be transactional
                        let account: Account = self.datastore().accounts.find(owner).unwrap();
                        let mut reply: sessionsrv::Session = account.into();
                        reply.set_token(msg.get_token().to_string());
                        try!(req.reply_complete(&mut self.sock, &reply));
                    }
                    Err(Error::EntityNotFound) => {
                        let err = net::err(ErrCode::ENTITY_NOT_FOUND, "ss:auth:4");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                    Err(e) => {
                        error!("datastore error, err={:?}", e);
                        let err = net::err(ErrCode::INTERNAL, "ss:auth:5");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                }
            }
            _ => panic!("unexpected message: {:?}", req.message_id()),
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
    #[allow(dead_code)]
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
        // * disconnect from removed routers
        // * notify remaining routers of any shard hosting changes
        // * connect to new shard servers
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
        net::Protocol::SessionSrv
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
