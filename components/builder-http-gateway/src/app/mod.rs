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

//! # Example HttpGateway
//!
//! ```rust,no_run
//! extern crate habitat_builder_protocol as protocol;
//! extern crate builder_http_gateway as http_gateway;
//! #[macro_use]
//! extern crate log;
//! #[macro_use]
//! extern crate router;
//!
//! use std::process;
//!
//! use http_gateway::app::prelude::*;
//!
//! pub mod config {
//!     use http_gateway::config::prelude::*;
//!
//!     #[derive(Default)]
//!     pub struct SrvConfig {
//!         pub http: HttpCfg,
//!         pub routers: Vec<RouterAddr>,
//!     }
//!
//!     impl GatewayCfg for SrvConfig {
//!         fn listen_addr(&self) -> &IpAddr {
//!             &self.http.listen
//!         }
//!
//!         fn listen_port(&self) -> u16 {
//!             self.http.port
//!         }
//!
//!         fn route_addrs(&self) -> &[RouterAddr] {
//!             self.routers.as_slice()
//!         }
//!     }
//!
//!     pub struct HttpCfg {
//!         pub listen: IpAddr,
//!         pub port: u16,
//!     }
//!
//!     impl Default for HttpCfg {
//!         fn default() -> Self {
//!             HttpCfg {
//!                 listen: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
//!                 port: 1234,
//!             }
//!         }
//!     }
//! }
//!
//! mod handlers {
//!     use http_gateway::http::controller::*;
//!
//!     pub fn status(_req: &mut Request) -> IronResult<Response> {
//!         Ok(Response::with(status::Ok))
//!     }
//! }
//!
//! use config::SrvConfig;
//!
//! struct MyGatewaySrv;
//! impl HttpGateway for MyGatewaySrv {
//!     const APP_NAME: &'static str = "my-gateway";
//!
//!     type Config = SrvConfig;
//!
//!     fn router(config: Arc<Self::Config>) -> Router {
//!         router!(
//!             status: get "/status" => handlers::status,
//!         )
//!     }
//! }
//!
//! fn main() {
//!     let config = SrvConfig::default();
//!     if let Err(err) = http_gateway::start::<MyGatewaySrv>(config) {
//!         error!("{}", err);
//!         process::exit(1);
//!     }
//! }
//! ```

pub mod error;
pub mod prelude;

use std::sync::Arc;
use std::thread;

use hab_net;
use iron;
use iron::prelude::*;
use mount::Mount;
use router::Router;

use self::error::AppResult;
use config::GatewayCfg;
use conn::RouteBroker;
use http::middleware::{Cors, XRouteClient};

/// Apply to a networked application which will act as a Gateway connecting to a RouteSrv.
pub trait HttpGateway {
    const APP_NAME: &'static str;

    type Config: GatewayCfg;

    /// Callback for adding or removing middleware to the `iron::Chain` before server start.
    fn add_middleware(Arc<Self::Config>, &mut iron::Chain) {
        ()
    }

    /// Callback for mounting additional Iron Routers before server start.
    fn mount(Arc<Self::Config>, chain: iron::Chain) -> Mount {
        let mut mount = Mount::new();
        mount.mount("/", chain);
        mount
    }

    /// Returns the Iron Router used when starting the server.
    fn router(Arc<Self::Config>) -> Router;
}

/// Runs the main server and starts and manages all supporting threads. This function will
/// block the calling thread.
///
/// # Errors
///
/// * HTTP server could not start
pub fn start<T>(cfg: T::Config) -> AppResult<()>
where
    T: HttpGateway,
{
    let cfg = Arc::new(cfg);
    let mut chain = Chain::new(T::router(cfg.clone()));
    T::add_middleware(cfg.clone(), &mut chain);
    chain.link_before(XRouteClient);
    chain.link_after(Cors);
    let mount = T::mount(cfg.clone(), chain);
    let mut server = Iron::new(mount);
    server.threads = cfg.handler_count();
    let http_listen_addr = (cfg.listen_addr().clone(), cfg.listen_port());
    thread::Builder::new()
        .name("http-handler".to_string())
        .spawn(move || server.http(http_listen_addr))
        .unwrap();
    info!(
        "HTTP Gateway listening on {}:{}",
        cfg.listen_addr(),
        cfg.listen_port()
    );
    info!("{} is ready to go.", T::APP_NAME);
    RouteBroker::start(hab_net::socket::srv_ident(), cfg.route_addrs())?;
    Ok(())
}
