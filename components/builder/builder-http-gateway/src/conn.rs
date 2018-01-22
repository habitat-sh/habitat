// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use hab_net::app::config::RouterAddr;
use hab_net::conn::{RECV_TIMEOUT_MS, SEND_TIMEOUT_MS, ConnErr, RouteClient};
use hab_net::socket::{DEFAULT_CONTEXT, ToAddrString};
use iron::typemap;
use zmq;

/// A messaging RouteBroker for proxying messages from clients to one or more `RouteSrv` and vice
/// versa.
pub struct RouteBroker {
    client_sock: zmq::Socket,
    router_sock: zmq::Socket,
}

impl RouteBroker {
    // RouteBroker IPC listening address for the application's RouteBroker's queue.
    const IPC_ADDR: &'static str = "inproc://http-gateway.broker";

    /// Helper function for creating a new `RouteClient`.
    ///
    /// # Errors
    ///
    /// * Could not connect to `RouteBroker`
    /// * Could not create socket
    ///
    /// # Panics
    ///
    /// * Could not read `zmq::Context` due to deadlock or poisoning
    pub fn connect() -> Result<RouteClient, ConnErr> {
        let conn = RouteClient::new()?;
        conn.connect(Self::IPC_ADDR)?;
        Ok(conn)
    }

    pub fn start(net_ident: String, routers: &[RouterAddr]) -> Result<(), ConnErr> {
        let mut broker = Self::new(net_ident)?;
        broker.run(routers)
    }

    /// Create a new `RouteBroker`
    ///
    /// # Errors
    ///
    /// * A socket cannot be created within the given `zmq::Context`
    /// * A socket cannot be configured
    ///
    /// # Panics
    ///
    /// * Could not read `zmq::Context` due to deadlock or poisoning
    fn new(net_ident: String) -> Result<Self, ConnErr> {
        let client_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::ROUTER)?;
        let router_sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;
        router_sock.set_identity(net_ident.as_bytes())?;
        router_sock.set_rcvtimeo(RECV_TIMEOUT_MS)?;
        router_sock.set_sndtimeo(SEND_TIMEOUT_MS)?;
        router_sock.set_immediate(true)?;
        Ok(RouteBroker {
            client_sock: client_sock,
            router_sock: router_sock,
        })
    }

    pub fn run(&mut self, routers: &[RouterAddr]) -> Result<(), ConnErr> {
        self.client_sock.bind(Self::IPC_ADDR)?;
        for addr in routers {
            self.router_sock.connect(&addr.to_addr_string())?;
        }
        zmq::proxy(&mut self.client_sock, &mut self.router_sock).map_err(ConnErr::Socket)
    }
}

impl typemap::Key for RouteBroker {
    type Value = Self;
}
