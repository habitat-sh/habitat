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

//! Contains core functionality for the Application's main server.

use std::sync::Arc;

use hab_net::config::RouteAddrs;
use hab_net::routing::Broker;
use hab_net::server::NetIdent;

use config::Config;
use error::Result;
use http;

/// The main server for the Builder-API application. This should be run on the main thread.
pub struct Server {
    pub config: Arc<Config>,
}

impl Server {
    /// Create a new `Server`
    pub fn new(config: Config) -> Self {
        Server { config: Arc::new(config) }
    }

    /// Runs the main server and starts and manages all supporting threads. This function will
    /// block the calling thread.
    ///
    /// # Errors
    ///
    /// * HTTP server could not start
    pub fn run(&mut self) -> Result<()> {
        let cfg1 = self.config.clone();
        let broker = Broker::run(Self::net_ident(), self.config.route_addrs());
        let http = try!(http::run(cfg1));

        println!("Builder Admin listening on {}", &self.config.http_addr);
        http.join().unwrap();
        broker.join().unwrap();
        Ok(())
    }
}

impl NetIdent for Server {}

/// Helper function for creating a new Server and running it. This function will block the calling
/// thread.
pub fn run(config: Config) -> Result<()> {
    Server::new(config).run()
}
