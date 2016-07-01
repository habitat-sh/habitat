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
pub mod worker_manager;

use std::ops::Deref;
use std::sync::{Arc, RwLock};

use dbcache::InstaSet;
use dbcache::data_store::Pool;
use hab_net::dispatcher::prelude::*;
use hab_net::{Application, Supervisor};
use hab_net::server::{Envelope, NetIdent, RouteConn, Service, ZMQ_CONTEXT};
use protocol::net;
use zmq;

use self::worker_manager::{WorkerMgr, WorkerMgrClient};
use config::Config;
use data_store::DataStore;
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

#[derive(Clone)]
pub struct InitServerState {
    datastore: Arc<Box<DataStore>>,
}

impl InitServerState {
    pub fn new(datastore: DataStore) -> Self {
        InitServerState { datastore: Arc::new(Box::new(datastore)) }
    }
}

impl Into<ServerState> for InitServerState {
    fn into(self) -> ServerState {
        let mut state = ServerState::default();
        state.datastore = Some(self.datastore);
        state
    }
}

#[derive(Default)]
pub struct ServerState {
    datastore: Option<Arc<Box<DataStore>>>,
    worker_mgr: Option<WorkerMgrClient>,
}

impl ServerState {
    fn datastore(&self) -> &DataStore {
        self.datastore.as_ref().unwrap()
    }

    fn worker_mgr(&mut self) -> &mut WorkerMgrClient {
        self.worker_mgr.as_mut().unwrap()
    }
}

impl DispatcherState for ServerState {
    fn is_initialized(&self) -> bool {
        self.datastore.is_some() && self.worker_mgr.is_some()
    }
}

pub struct Worker {
    #[allow(dead_code)]
    config: Arc<RwLock<Config>>,
}

impl Dispatcher for Worker {
    type Config = Config;
    type Error = Error;
    type InitState = InitServerState;
    type State = ServerState;

    fn message_queue() -> &'static str {
        BE_LISTEN_ADDR
    }

    fn dispatch(message: &mut Envelope,
                sock: &mut zmq::Socket,
                state: &mut Self::State)
                -> Result<()> {
        match message.message_id() {
            "JobSpec" => handlers::job_create(message, sock, state),
            "JobGet" => handlers::job_get(message, sock, state),
            _ => panic!("unexpected message: {:?}", message.message_id()),
        }
    }

    fn context(&mut self) -> &mut zmq::Context {
        (**ZMQ_CONTEXT).as_mut()
    }

    fn new(config: Arc<RwLock<Config>>) -> Self {
        Worker { config: config }
    }

    fn init(&mut self, init_state: Self::InitState) -> Result<Self::State> {
        let mut worker_mgr = WorkerMgrClient::default();
        try!(worker_mgr.connect());
        let mut state: ServerState = init_state.into();
        state.worker_mgr = Some(worker_mgr);
        Ok(state)
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
        let datastore = {
            let cfg = self.config.read().unwrap();
            DataStore::start(cfg.deref())
        };
        let cfg = self.config.clone();
        let cfg2 = self.config.clone();
        let init_state = InitServerState::new(datastore);
        let ds2 = init_state.datastore.clone();
        let sup: Supervisor<Worker> = Supervisor::new(cfg, init_state);
        let worker_mgr = try!(WorkerMgr::start(cfg2, ds2));
        try!(sup.start());
        try!(self.connect());
        try!(zmq::proxy(&mut self.router.socket, &mut self.be_sock));
        worker_mgr.join().unwrap();
        Ok(())
    }
}

impl Service for Server {
    type Application = Self;
    type Config = Config;
    type Error = Error;

    fn protocol() -> net::Protocol {
        net::Protocol::JobSrv
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
