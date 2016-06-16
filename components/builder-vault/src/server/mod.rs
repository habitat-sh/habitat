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

use protocol::net;
use zmq;

use hab_net::{Application, Dispatcher, Supervisor};
use hab_net::server::{Envelope, NetIdent, RouteConn, Service, ZMQ_CONTEXT};

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
    type State = ServerState;
    type Config = Config;
    type Error = Error;

    fn message_queue() -> &'static str {
        BE_LISTEN_ADDR
    }

    fn dispatch(message: &mut Envelope,
                sock: &mut zmq::Socket,
                state: &mut ServerState)
                -> Result<()> {
        match message.message_id() {
            "AccountInvitationListRequest" => {
                handlers::account_invitation_list(message, sock, state)
            }
            "CheckOriginAccessRequest" => handlers::origin_check_access(message, sock, state),
            "OriginCreate" => handlers::origin_create(message, sock, state),
            "OriginGet" => handlers::origin_get(message, sock, state),
            "OriginInvitationAcceptRequest" => {
                handlers::origin_invitation_accept(message, sock, state)
            }
            "OriginInvitationCreate" => handlers::origin_invitation_create(message, sock, state),
            "OriginInvitationListRequest" => handlers::origin_invitation_list(message, sock, state),
            "OriginList" => handlers::origin_list(message, sock, state),
            "OriginMemberListRequest" => handlers::origin_member_list(message, sock, state),
            "AccountOriginListRequest" => handlers::account_origin_list(message, sock, state),
            "OriginSecretKeyCreate" => handlers::origin_secret_key_create(message, sock, state),
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
        // JW break; how do we pass a mutable context from static ref?
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
        let cfg = self.config.clone();
        let sup: Supervisor<Worker> = Supervisor::new(cfg);
        try!(sup.start());
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
