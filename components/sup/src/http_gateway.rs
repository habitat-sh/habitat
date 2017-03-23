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
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::net::{IpAddr, Ipv4Addr, ToSocketAddrs, SocketAddr, SocketAddrV4};
use std::ops::{Deref, DerefMut};
use std::option;
use std::result;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use hcore::service::ServiceGroup;
use iron::prelude::*;
use iron::{headers, status, typemap};
use iron::modifiers::Header;
use persistent;
use prometheus::{self, CounterVec, HistogramVec, TextEncoder, Encoder};
use router::Router;
use serde_json;

use error::{Result, Error, SupError};
use fs;
use manager;
use manager::service::HealthCheck;
use manager::service::hooks::{self, HealthCheckHook};

static LOGKEY: &'static str = "HG";

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

#[derive(PartialEq, Eq, Debug, Clone)]
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

impl fmt::Display for ListenAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

struct ManagerFs;

impl typemap::Key for ManagerFs {
    type Value = manager::FsCfg;
}

pub struct Server(Iron<Chain>, ListenAddr);

impl Server {
    pub fn new(manager_state: Arc<manager::FsCfg>, listen_addr: ListenAddr) -> Self {
        let router = router!(
            butterfly: get "/butterfly" => with_metrics!(butterfly, "butterfly"),
            census: get "/census" => with_metrics!(census, "census"),
            metrics: get "/metrics" => with_metrics!(metrics, "metrics"),
            services: get "/services" => with_metrics!(services, "services"),
            service_config: get "/services/:svc/:group/config" => {
                with_metrics!(config, "config")
            },
            service_health: get "/services/:svc/:group/health" => with_metrics!(health, "health"),
            service_config_org: get "/services/:svc/:group/:org/config" => {
                with_metrics!(config, "config")
            },
            service_health_org: get "/services/:svc/:group/:org/health" => {
                with_metrics!(health, "config")
            }
        );
        let mut chain = Chain::new(router);
        chain.link(persistent::Read::<ManagerFs>::both(manager_state));
        Server(Iron::new(chain), listen_addr)
    }

    pub fn start(self) -> Result<JoinHandle<()>> {
        let handle = try!(thread::Builder::new()
            .name("http-gateway".to_string())
            .spawn(move || {
                self.0.http(*self.1).expect("unable to start http-gateway thread");
            }));
        Ok(handle)
    }
}

#[derive(Default, Serialize)]
struct HealthCheckBody {
    stdout: String,
    stderr: String,
}

fn butterfly(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    match File::open(&state.butterfly_data_path) {
        Ok(file) => Ok(Response::with((status::Ok, Header(headers::ContentType::json()), file))),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn census(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    match File::open(&state.census_data_path) {
        Ok(file) => Ok(Response::with((status::Ok, Header(headers::ContentType::json()), file))),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn config(req: &mut Request) -> IronResult<Response> {
    // JW TODO: We don't really care about the other parts of the service group. This is because
    // we're maybe doing the wrong thing by placing all services in /hab/svc without including
    // any information about the group name or organization perhaps? Either way - this isn't
    // harmful for now - we'll either include that or change the URI to this endpoint to only
    // require service name.
    let config_file = match build_service_group(req) {
        Ok(sg) => fs::svc_config_file(sg.service()),
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    match File::open(&config_file) {
        Ok(file) => Ok(Response::with((status::Ok, file))),
        Err(_) => Ok(Response::with(status::NotFound)),
    }
}

fn health(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    let (health_file, stdout_path, stderr_path) = match build_service_group(req) {
        Ok(sg) => {
            (state.health_check_cache(&sg),
             hooks::stdout_log_path::<HealthCheckHook>(&sg),
             hooks::stderr_log_path::<HealthCheckHook>(&sg))
        }
        Err(_) => return Ok(Response::with(status::BadRequest)),
    };
    match File::open(&health_file) {
        Ok(mut file) => {
            let mut buf = String::new();
            let mut body = HealthCheckBody::default();
            file.read_to_string(&mut buf).unwrap();
            let code = i8::from_str(buf.trim()).unwrap();
            let status: status::Status = HealthCheck::from(code).into();
            if let Ok(mut file) = File::open(&stdout_path) {
                let _ = file.read_to_string(&mut body.stdout);
            }
            if let Ok(mut file) = File::open(&stderr_path) {
                let _ = file.read_to_string(&mut body.stderr);
            }
            Ok(Response::with((status,
                               Header(headers::ContentType::json()),
                               serde_json::to_string(&body).unwrap())))
        }
        Err(_) => Ok(Response::with(status::NotFound)),
    }
}

fn services(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerFs>>().unwrap();
    match File::open(&state.services_data_path) {
        Ok(file) => Ok(Response::with((status::Ok, Header(headers::ContentType::json()), file))),
        Err(_) => Ok(Response::with(status::ServiceUnavailable)),
    }
}

fn metrics(_req: &mut Request) -> IronResult<Response> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_familys = prometheus::gather();
    encoder.encode(&metric_familys, &mut buffer).unwrap();

    Ok(Response::with((status::Ok, String::from_utf8(buffer).unwrap())))
}

impl Into<Response> for HealthCheck {
    fn into(self) -> Response {
        let status: status::Status = self.into();
        Response::with(status)
    }
}

impl Into<status::Status> for HealthCheck {
    fn into(self) -> status::Status {
        match self {
            HealthCheck::Ok | HealthCheck::Warning => status::Ok,
            HealthCheck::Critical => status::ServiceUnavailable,
            HealthCheck::Unknown => status::InternalServerError,
        }
    }
}

fn build_service_group(req: &mut Request) -> Result<ServiceGroup> {
    let sg = ServiceGroup::new(req.extensions
                                   .get::<Router>()
                                   .unwrap()
                                   .find("svc")
                                   .unwrap_or(""),
                               req.extensions
                                   .get::<Router>()
                                   .unwrap()
                                   .find("group")
                                   .unwrap_or(""),
                               req.extensions
                                   .get::<Router>()
                                   .unwrap()
                                   .find("org"))?;
    Ok(sg)
}
