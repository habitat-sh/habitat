// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

use dbcache::{self, ExpiringSet, InstaSet, IndexSet};
use hab_net::server::{Application, Envelope, NetIdent, RouteConn, Service, Supervisor,
                      Supervisable};
use protocol::net::{self, ErrCode};
use protocol::sessionsrv::{Account, AccountGet, Session, SessionGet, SessionCreate, SessionToken};
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
            "AccountGet" => {
                let msg: AccountGet = try!(req.parse_msg());
                match self.datastore().accounts.find_by_username(&msg.get_name().to_string()) {
                    Ok(account) => {
                        try!(req.reply_complete(&mut self.sock, &account));
                    }
                    Err(dbcache::Error::EntityNotFound) => {
                        let err = net::err(ErrCode::ENTITY_NOT_FOUND, "ss:account_get:0");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                    Err(e) => {
                        error!("datastore error, err={:?}", e);
                        let err = net::err(ErrCode::INTERNAL, "ss:account_get:1");
                        try!(req.reply_complete(&mut self.sock, &err));
                    }
                }
            }
            "SessionCreate" => {
                let mut msg: SessionCreate = try!(req.parse_msg());
                let mut account: Account = match self.datastore()
                    .sessions
                    .find(&msg.get_token().to_string()) {
                    Ok(session) => self.datastore().accounts.find(&session.get_owner_id()).unwrap(),
                    _ => try!(self.datastore().accounts.find_or_create(&msg)),
                };
                let mut session_token = SessionToken::new();
                session_token.set_owner_id(account.get_id());
                session_token.set_token(msg.take_token());
                try!(self.datastore().sessions.write(&mut session_token));
                let mut session = Session::new();
                session.set_token(session_token.take_token());
                session.set_id(session_token.get_owner_id());
                session.set_email(account.take_email());
                session.set_name(account.take_name());
                try!(req.reply_complete(&mut self.sock, &session));
            }
            "SessionGet" => {
                let msg: SessionGet = try!(req.parse_msg());
                match self.datastore().sessions.find(&msg.get_token().to_string()) {
                    Ok(mut token) => {
                        let account: Account =
                            self.datastore().accounts.find(&token.get_owner_id()).unwrap();
                        let mut session: Session = account.into();
                        session.set_token(token.take_token());
                        try!(req.reply_complete(&mut self.sock, &session));
                    }
                    Err(dbcache::Error::EntityNotFound) => {
                        let err = net::err(ErrCode::SESSION_EXPIRED, "ss:auth:4");
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
