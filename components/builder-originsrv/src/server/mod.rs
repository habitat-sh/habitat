// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use protocol::net;
use zmq;

use hab_net::config::RouterCfg;
use hab_net::routing::Broker;
use hab_net::{Application, Supervisor};
use hab_net::dispatcher::prelude::*;
use hab_net::server::{Envelope, NetIdent, RouteConn, Service, ZMQ_CONTEXT};

use config::Config;
use data_store::DataStore;
use error::{Error, Result};

const BE_LISTEN_ADDR: &'static str = "inproc://backend";

#[derive(Clone)]
pub struct ServerState {
    datastore: DataStore,
}

impl ServerState {
    pub fn new(datastore: DataStore) -> Self {
        ServerState { datastore: datastore }
    }
}

impl DispatcherState for ServerState {
    fn is_initialized(&self) -> bool {
        true
    }
}

pub struct Worker {
    #[allow(dead_code)]
    config: Arc<RwLock<Config>>,
}

impl Dispatcher for Worker {
    type Config = Config;
    type Error = Error;
    type InitState = ServerState;
    type State = ServerState;

    fn message_queue() -> &'static str {
        BE_LISTEN_ADDR
    }

    fn dispatch(
        message: &mut Envelope,
        sock: &mut zmq::Socket,
        state: &mut ServerState,
    ) -> Result<()> {
        match message.message_id() {
            "CheckOriginAccessRequest" => handlers::origin_check_access(message, sock, state),
            "OriginCreate" => handlers::origin_create(message, sock, state),
            "OriginGet" => handlers::origin_get(message, sock, state),
            "OriginInvitationAcceptRequest" => {
                handlers::origin_invitation_accept(message, sock, state)
            }
            "OriginInvitationCreate" => handlers::origin_invitation_create(message, sock, state),
            "OriginInvitationListRequest" => handlers::origin_invitation_list(message, sock, state),
            "OriginMemberListRequest" => handlers::origin_member_list(message, sock, state),
            "OriginSecretKeyCreate" => handlers::origin_secret_key_create(message, sock, state),
            "OriginSecretKeyGet" => handlers::origin_secret_key_get(message, sock, state),
            "OriginPublicKeyCreate" => handlers::origin_public_key_create(message, sock, state),
            "OriginPublicKeyGet" => handlers::origin_public_key_get(message, sock, state),
            "OriginPublicKeyLatestGet" => {
                handlers::origin_public_key_latest_get(message, sock, state)
            }
            "OriginPublicKeyListRequest" => handlers::origin_public_key_list(message, sock, state),
            "OriginProjectCreate" => handlers::project_create(message, sock, state),
            "OriginProjectDelete" => handlers::project_delete(message, sock, state),
            "OriginProjectGet" => handlers::project_get(message, sock, state),
            "OriginProjectUpdate" => handlers::project_update(message, sock, state),
            "OriginPackageCreate" => handlers::origin_package_create(message, sock, state),
            "OriginPackageGet" => handlers::origin_package_get(message, sock, state),
            "OriginPackageLatestGet" => handlers::origin_package_latest_get(message, sock, state),
            "OriginPackageListRequest" => handlers::origin_package_list(message, sock, state),
            "OriginPackageChannelListRequest" => {
                handlers::origin_package_channel_list(message, sock, state)
            }
            "OriginPackageVersionListRequest" => {
                handlers::origin_package_version_list(message, sock, state)
            }
            "OriginPackageDemote" => handlers::origin_package_demote(message, sock, state),
            "OriginPackageGroupPromote" => {
                handlers::origin_package_group_promote(message, sock, state)
            }
            "OriginPackagePromote" => handlers::origin_package_promote(message, sock, state),
            "OriginPackageUniqueListRequest" => {
                handlers::origin_package_unique_list(message, sock, state)
            }
            "OriginPackageSearchRequest" => handlers::origin_package_search(message, sock, state),
            "OriginChannelCreate" => handlers::origin_channel_create(message, sock, state),
            "OriginChannelDelete" => handlers::origin_channel_delete(message, sock, state),
            "OriginChannelGet" => handlers::origin_channel_get(message, sock, state),
            "OriginChannelListRequest" => handlers::origin_channel_list(message, sock, state),
            "OriginChannelPackageGet" => handlers::origin_channel_package_get(message, sock, state),
            "OriginChannelPackageLatestGet" => {
                handlers::origin_channel_package_latest_get(message, sock, state)
            }
            "OriginChannelPackageListRequest" => {
                handlers::origin_channel_package_list(message, sock, state)
            }
            _ => {
                debug!("dispatch: unhandled message: {}", message.message_id());
                Ok(())
            }
        }
    }

    fn new(config: Arc<RwLock<Config>>) -> Self {
        Worker { config: config }
    }

    fn context(&mut self) -> &mut zmq::Context {
        (**ZMQ_CONTEXT).as_mut()
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
        let router = RouteConn::new(Self::net_ident(), (**ZMQ_CONTEXT).as_mut())?;
        let be = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER)?;
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
        self.be_sock.bind(BE_LISTEN_ADDR)?;
        let broker = {
            let cfg = self.config.read().unwrap();
            Broker::run(Self::net_ident(), cfg.route_addrs())
        };
        let datastore = {
            let cfg = self.config.read().unwrap();
            DataStore::new(cfg.deref())?
        };
        datastore.setup()?;
        datastore.start_async();
        let cfg = self.config.clone();
        let init_state = ServerState::new(datastore);
        let sup: Supervisor<Worker> = Supervisor::new(cfg, init_state);
        sup.start()?;
        self.connect()?;
        info!("builder-originsrv is ready to go.");
        zmq::proxy(&mut self.router.socket, &mut self.be_sock)?;
        broker.join().unwrap();
        Ok(())
    }
}

impl Service for Server {
    type Application = Self;
    type Config = Config;
    type Error = Error;

    fn protocol() -> net::Protocol {
        net::Protocol::OriginSrv
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
    Server::new(config)?.run()
}
