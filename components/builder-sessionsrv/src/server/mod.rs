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

pub mod handlers;

use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

use dbcache::{ExpiringSet, InstaSet, IndexSet};
use hab_net::{Application, Dispatcher, Supervisor};
use hab_net::server::{Envelope, NetIdent, RouteConn, Service, ZMQ_CONTEXT};
use protocol::net;
use zmq;

use config::Config;
use data_store::DataStore;
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

pub struct ServerState {
    datastore: DataStore,
}

pub struct Worker {
    config: Arc<RwLock<Config>>,
    state: Option<ServerState>,
}

impl Dispatcher for Worker {
    type Config = Config;
    type Error = Error;
    type State = ServerState;

    fn message_queue() -> &'static str {
        BE_LISTEN_ADDR
    }

    fn dispatch(message: &mut Envelope,
                sock: &mut zmq::Socket,
                state: &mut ServerState)
                -> Result<()> {
        match message.message_id() {
            "AccountGet" => handlers::account_get(message, sock, state),
            "SessionCreate" => handlers::session_create(message, sock, state),
            "SessionGet" => handlers::session_get(message, sock, state),
            _ => panic!("unhandled message"),
        }
    }

    fn new(config: Arc<RwLock<Config>>) -> Self {
        Worker {
            config: config,
            state: None,
        }
    }

    fn context(&mut self) -> &mut zmq::Context {
        (**ZMQ_CONTEXT).as_mut()
    }

    fn init(&mut self) -> Result<()> {
        loop {
            let result = {
                let cfg = self.config.read().unwrap();
                DataStore::open(cfg.deref())
            };
            match result {
                Ok(datastore) => {
                    self.state = Some(ServerState { datastore: datastore });
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

    fn state(&mut self) -> &mut ServerState {
        self.state.as_mut().unwrap()
    }
}

pub struct Server {
    config: Arc<RwLock<Config>>,
    router: RouteConn,
    be_sock: zmq::Socket,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let router = try!(RouteConn::new(Self::net_ident(), (**ZMQ_CONTEXT).as_mut()));
        let be = try!((**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER));
        Ok(Server {
            config: Arc::new(RwLock::new(config)),
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
        let cfg = self.config.clone();
        let sup: Supervisor<Worker> = Supervisor::new(cfg);
        {
            let cfg = self.config.read().unwrap();
            try!(sup.start(cfg.worker_threads));
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
