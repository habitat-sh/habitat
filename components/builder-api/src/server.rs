// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Contains core functionality for the Application's main server.

use std::sync::{Arc, Mutex};

use hab_net::config::RouteAddrs;
use hab_net::routing::Broker;
use hab_net::server::NetIdent;
use zmq;

use config::Config;
use error::Result;
use http;

/// The main server for the Builder-API application. This should be run on the main thread.
pub struct Server {
    pub config: Arc<Config>,
    context: Arc<Mutex<zmq::Context>>,
}

impl Server {
    /// Create a new `Server`
    pub fn new(config: Config) -> Self {
        let ctx = zmq::Context::new();
        Server {
            config: Arc::new(config),
            context: Arc::new(Mutex::new(ctx)),
        }
    }

    /// Runs the main server and starts and manages all supporting threads. This function will
    /// block the calling thread.
    ///
    /// # Errors
    ///
    /// * HTTP server could not start
    pub fn run(&mut self) -> Result<()> {
        let cfg1 = self.config.clone();
        let ctx1 = self.context.clone();
        let ctx2 = self.context.clone();
        let broker = Broker::run(Self::net_ident(), ctx1, self.config.route_addrs());
        let http = try!(http::run(cfg1, ctx2));

        println!("Builder API listening on {}", &self.config.http_addr);
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
