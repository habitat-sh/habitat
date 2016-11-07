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

use std::collections::HashMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs, SocketAddr, SocketAddrV4};
use std::ops::{Deref, DerefMut};
use std::option;
use std::str::FromStr;
use std::thread::{self, JoinHandle};

use hcore::service::ServiceGroup;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;
use router::Router;
use serde_json;
use prometheus::{CounterVec, HistogramVec, TextEncoder, Encoder};
use prometheus;

use config::gconfig;
use error::{Result, Error, SupError};
use health_check;
use manager;

static LOGKEY: &'static str = "HG";


lazy_static! {
    static ref HTTP_COUNTER: CounterVec = register_counter_vec!(
        opts!(
            "http_requests_total",
            "Total number of HTTP requests made."),
        &["handler"]).unwrap();

    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        histogram_opts!(
            "http_request_duration_seconds",
            "HTTP request latencies in seconds."),
        &["handler"]).unwrap();
}

#[derive(PartialEq, Eq, Debug)]
pub struct ListenAddr(SocketAddr);

impl Default for ListenAddr {
    fn default() -> ListenAddr {
        ListenAddr(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9631)))
    }
}

impl Deref for ListenAddr {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}

impl DerefMut for ListenAddr {
    fn deref_mut(&mut self) -> &mut SocketAddr {
        &mut self.0
    }
}

impl FromStr for ListenAddr {
    type Err = SupError;

    fn from_str(val: &str) -> Result<Self> {
        match SocketAddr::from_str(val) {
            Ok(addr) => Ok(ListenAddr(addr)),
            Err(_) => {
                match IpAddr::from_str(val) {
                    Ok(ip) => {
                        let mut addr = ListenAddr::default();
                        addr.set_ip(ip);
                        Ok(addr)
                    }
                    Err(_) => Err(sup_error!(Error::IPFailed)),
                }
            }
        }
    }
}

impl ToSocketAddrs for ListenAddr {
    type Iter = option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        self.0.to_socket_addrs()
    }
}

struct ManagerState;

impl typemap::Key for ManagerState {
    type Value = manager::State;
}

pub struct Server(Iron<Chain>);


// Simple macro to encapsulate the HTTP metrics for each endpoint
macro_rules! with_metrics {
    ($method:expr, $name:expr) => {{
        let mut labels = HashMap::new();
        labels.insert("handler", $name);
        
        HTTP_COUNTER.with(&labels.clone()).inc();
        let timer = HTTP_REQ_HISTOGRAM.with(&labels).start_timer();
        let result = $method;
        timer.observe_duration();
        result
    }}
}

impl Server {
    pub fn new(manager_state: manager::State) -> Self {
        let router = router!(
            butterfly: get "/butterfly" => with_metrics!(butterfly, "butterfly"),
            census: get "/census" => with_metrics!(census, "census"),
            metrics: get "/metrics" => with_metrics!(metrics, "metrics"),
            services: get "/services" => with_metrics!(services, "services"),
            service_config: get "/services/:svc/:group/config" => with_metrics!(config, "config"),
            service_health: get "/services/:svc/:group/health" => with_metrics!(health, "health"),
            service_config_org: get "/services/:svc/:group/:org/config" => with_metrics!(config, "config"),
            service_health_org: get "/services/:svc/:group/:org/health" => with_metrics!(health, "config"),
        );
        let mut chain = Chain::new(router);
        chain.link(persistent::Read::<ManagerState>::both(manager_state));
        Server(Iron::new(chain))
    }

    pub fn start(self) -> Result<JoinHandle<()>> {
        let handle = try!(thread::Builder::new()
            .name("http-gateway".to_string())
            .spawn(move || {
                self.0
                    .http(*gconfig().http_listen_addr())
                    .expect("unable to start http-gateway thread");
            }));
        Ok(handle)
    }
}


fn butterfly(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string(&state.butterfly).unwrap())))
}

fn census(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let data = state.census_list.read().unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string(&*data).unwrap())))
}

fn config(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let service_group = match build_service_group(req) {
        Ok(sg) => sg,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    let services = state.services.read().unwrap();
    match services.iter().find(|s| s.service_group == service_group) {
        Some(service) => {
            match service.package.last_config() {
                Ok(config) => Ok(Response::with((status::Ok, config))),
                Err(err) => {
                    error!("Couldn't retrieve last config, err={:?}", err);
                    Ok(Response::with(status::ServiceUnavailable))
                }
            }
        }
        None => Ok(Response::with(status::NotFound)),
    }
}

fn health(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let service_group = match build_service_group(req) {
        Ok(sg) => sg,
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    let services = state.services.read().unwrap();
    match services.iter().find(|s| s.service_group == service_group) {
        Some(service) => {
            match service.health_check() {
                Ok(result) => Ok(result.into()),
                Err(err) => {
                    error!("Health Check failed, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
        None => Ok(Response::with(status::NotFound)),
    }
}

fn services(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let data = state.services.read().unwrap();
    Ok(Response::with((status::Ok, serde_json::to_string(&*data).unwrap())))
}

fn metrics(_req: &mut Request) -> IronResult<Response> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_familys = prometheus::gather();
    encoder.encode(&metric_familys, &mut buffer).unwrap();

    Ok(Response::with((status::Ok, String::from_utf8(buffer).unwrap())))
}

impl Into<Response> for health_check::CheckResult {
    fn into(self) -> Response {
        let status: status::Status = self.into();
        Response::with(status)
    }
}

impl Into<status::Status> for health_check::CheckResult {
    fn into(self) -> status::Status {
        match self {
            health_check::CheckResult::Ok |
            health_check::CheckResult::Warning => status::Ok,
            health_check::CheckResult::Critical => status::ServiceUnavailable,
            health_check::CheckResult::Unknown => status::InternalServerError,
        }
    }
}

fn build_service_group(req: &mut Request) -> Result<ServiceGroup> {
    let sg =
        ServiceGroup::new(req.extensions.get::<Router>().unwrap().find("svc").unwrap_or(""),
                          req.extensions.get::<Router>().unwrap().find("group").unwrap_or(""),
                          req.extensions.get::<Router>().unwrap().find("org"))?;
    Ok(sg)
}
