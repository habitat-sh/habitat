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

use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs};
use std::ops::{Deref, DerefMut};
use std::option;
use std::result;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;

use actix;
use actix_web::{
    http::StatusCode, pred::Predicate, server, App, FromRequest, HttpRequest, HttpResponse, Path,
    Request,
};
use hcore::service::ServiceGroup;
use protocol::socket_addr_env_or_default;
use serde_json::{self, Value as Json};

use error::{Error, Result, SupError};
use manager;
use manager::service::hooks::{self, HealthCheckHook};
use manager::service::HealthCheck;

use feat;

static LOGKEY: &'static str = "HG";
const APIDOCS: &'static str = include_str!(concat!(env!("OUT_DIR"), "/api.html"));

/// Default listening port for the HTTPGateway listener.
pub const DEFAULT_PORT: u16 = 9631;

/// Default environment variable override for HTTPGateway listener address.
pub const DEFAULT_ADDRESS_ENVVAR: &'static str = "HAB_LISTEN_HTTP";

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListenAddr(SocketAddr);

impl ListenAddr {
    pub fn new(ip: IpAddr, port: u16) -> ListenAddr {
        ListenAddr(SocketAddr::new(ip, port))
    }
}

impl Default for ListenAddr {
    fn default() -> ListenAddr {
        ListenAddr(socket_addr_env_or_default(
            DEFAULT_ADDRESS_ENVVAR,
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), DEFAULT_PORT)),
        ))
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
            Err(_) => match IpAddr::from_str(val) {
                Ok(ip) => {
                    let mut addr = ListenAddr::default();
                    addr.set_ip(ip);
                    Ok(addr)
                }
                Err(_) => Err(sup_error!(Error::IPFailed)),
            },
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

#[derive(Default, Serialize)]
struct HealthCheckBody {
    status: String,
    stdout: String,
    stderr: String,
}

impl Into<StatusCode> for HealthCheck {
    fn into(self) -> StatusCode {
        match self {
            HealthCheck::Ok | HealthCheck::Warning => StatusCode::OK,
            HealthCheck::Critical => StatusCode::SERVICE_UNAVAILABLE,
            HealthCheck::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

struct AppState {
    fs_cfg: Arc<manager::FsCfg>,
    gateway_state: Arc<RwLock<manager::GatewayState>>,
}

impl AppState {
    fn new(fs_cfg: Arc<manager::FsCfg>, gs: Arc<RwLock<manager::GatewayState>>) -> Self {
        AppState {
            fs_cfg: fs_cfg,
            gateway_state: gs,
        }
    }
}

pub struct Server;

impl Server {
    pub fn run(
        listen_addr: ListenAddr,
        fs_cfg: Arc<manager::FsCfg>,
        gateway_state: Arc<RwLock<manager::GatewayState>>,
    ) {
        thread::spawn(move || {
            let sys = actix::System::new("sup-http-gateway");

            server::new(move || {
                let app_state = AppState::new(fs_cfg.clone(), gateway_state.clone());
                App::with_state(app_state).configure(routes)
            }).bind(listen_addr.to_string())
            .unwrap()
            .start();

            let _ = sys.run();
        });
    }
}

struct RedactHTTP;

impl<S: 'static> Predicate<S> for RedactHTTP {
    fn check(&self, _req: &Request, _state: &S) -> bool {
        !feat::is_enabled(feat::RedactHTTP)
    }
}

fn routes(app: App<AppState>) -> App<AppState> {
    app.resource("/", |r| r.get().f(doc))
        .resource("/services", |r| r.get().f(services))
        .resource("/services/{svc}/{group}", |r| {
            r.get().f(service_without_org)
        }).resource("/services/{svc}/{group}/config", |r| {
            r.get().f(config_without_org)
        }).resource("/services/{svc}/{group}/health", |r| {
            r.get().f(health_without_org)
        }).resource("/services/{svc}/{group}/{org}", |r| {
            r.get().f(service_with_org)
        }).resource("/services/{svc}/{group}/{org}/config", |r| {
            r.get().f(config_with_org)
        }).resource("/services/{svc}/{group}/{org}/health", |r| {
            r.get().f(health_with_org)
        }).resource("/butterfly", |r| r.get().filter(RedactHTTP).f(butterfly))
        .resource("/census", |r| r.get().filter(RedactHTTP).f(census))
}

// Begin route handlers
fn butterfly(req: &HttpRequest<AppState>) -> HttpResponse {
    let data = &req
        .state()
        .gateway_state
        .read()
        .expect("Gateway State lock is poisoned")
        .butterfly_data;
    HttpResponse::Ok().body(data)
}

fn census(req: &HttpRequest<AppState>) -> HttpResponse {
    let data = &req
        .state()
        .gateway_state
        .read()
        .expect("Gateway State lock is poisoned")
        .census_data;
    HttpResponse::Ok().body(data)
}

fn services(req: &HttpRequest<AppState>) -> HttpResponse {
    let data = &req
        .state()
        .gateway_state
        .read()
        .expect("Gateway State lock is poisoned")
        .services_data;
    HttpResponse::Ok().body(data)
}

// Honestly, this doesn't feel great, but it's the pattern builder-api uses, and at the
// moment, I don't have a better way of doing it.
fn config_with_org(req: &HttpRequest<AppState>) -> HttpResponse {
    let (svc, group, org) = Path::<(String, String, String)>::extract(&req)
        .unwrap()
        .into_inner();
    config(req, svc, group, Some(&org))
}

fn config_without_org(req: &HttpRequest<AppState>) -> HttpResponse {
    let (svc, group) = Path::<(String, String)>::extract(&req)
        .unwrap()
        .into_inner();
    config(req, svc, group, None)
}

fn config(
    req: &HttpRequest<AppState>,
    svc: String,
    group: String,
    org: Option<&str>,
) -> HttpResponse {
    let data = &req
        .state()
        .gateway_state
        .read()
        .expect("Gateway State lock is poisoned")
        .services_data;
    let service_group = match ServiceGroup::new(None, svc, group, org) {
        Ok(sg) => sg,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match service_from_services(&service_group, &data) {
        Some(mut s) => HttpResponse::Ok().json(s["cfg"].take()),
        None => HttpResponse::NotFound().finish(),
    }
}

fn health_with_org(req: &HttpRequest<AppState>) -> HttpResponse {
    let (svc, group, org) = Path::<(String, String, String)>::extract(&req)
        .unwrap()
        .into_inner();
    health(req, svc, group, Some(&org))
}

fn health_without_org(req: &HttpRequest<AppState>) -> HttpResponse {
    let (svc, group) = Path::<(String, String)>::extract(&req)
        .unwrap()
        .into_inner();
    health(req, svc, group, None)
}

fn health(
    req: &HttpRequest<AppState>,
    svc: String,
    group: String,
    org: Option<&str>,
) -> HttpResponse {
    let state = &req.state();
    let service_group = match ServiceGroup::new(None, svc, group, org) {
        Ok(sg) => sg,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let health_file = &state.fs_cfg.health_check_cache(&service_group);
    let stdout_path = hooks::stdout_log_path::<HealthCheckHook>(&service_group);
    let stderr_path = hooks::stderr_log_path::<HealthCheckHook>(&service_group);

    match File::open(&health_file) {
        Ok(mut file) => {
            let mut buf = String::new();
            let mut body = HealthCheckBody::default();
            file.read_to_string(&mut buf).unwrap();
            let code = i8::from_str(buf.trim()).unwrap();
            let health_check = HealthCheck::from(code);
            let http_status: StatusCode = HealthCheck::from(code).into();

            body.status = health_check.to_string();
            if let Ok(mut file) = File::open(&stdout_path) {
                let _ = file.read_to_string(&mut body.stdout);
            }
            if let Ok(mut file) = File::open(&stderr_path) {
                let _ = file.read_to_string(&mut body.stderr);
            }

            HttpResponse::build(http_status).json(&body)
        }
        Err(e) => {
            error!("Error opening health file. e = {:?}", e);
            HttpResponse::NotFound().finish()
        }
    }
}

fn service_with_org(req: &HttpRequest<AppState>) -> HttpResponse {
    let (svc, group, org) = Path::<(String, String, String)>::extract(&req)
        .unwrap()
        .into_inner();
    service(req, svc, group, Some(&org))
}

fn service_without_org(req: &HttpRequest<AppState>) -> HttpResponse {
    let (svc, group) = Path::<(String, String)>::extract(&req)
        .unwrap()
        .into_inner();
    service(req, svc, group, None)
}

fn service(
    req: &HttpRequest<AppState>,
    svc: String,
    group: String,
    org: Option<&str>,
) -> HttpResponse {
    let data = &req
        .state()
        .gateway_state
        .read()
        .expect("Gateway State lock is poisoned")
        .services_data;
    let service_group = match ServiceGroup::new(None, svc, group, org) {
        Ok(sg) => sg,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match service_from_services(&service_group, &data) {
        Some(s) => HttpResponse::Ok().json(s),
        None => HttpResponse::NotFound().finish(),
    }
}

fn doc(_req: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(APIDOCS)
}
// End route handlers

fn service_from_services(service_group: &ServiceGroup, services_json: &str) -> Option<Json> {
    match serde_json::from_str(services_json) {
        Ok(Json::Array(services)) => services
            .into_iter()
            .find(|s| s["service_group"] == service_group.as_ref()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::Read,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT},
    };

    use butterfly::{
        member::Member,
        server::{Server, ServerProxy, Suitability},
        trace::Trace,
    };
    use hcore::service::ServiceGroup;
    use serde_json;

    use test_helpers::*;

    fn validate_sample_file_against_schema(name: &str, schema: &str) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join("http-gateway")
            .join(name);

        let mut f = File::open(path).expect(&format!("could not open {}", &name));
        let mut json = String::new();
        f.read_to_string(&mut json)
            .expect(&format!("could not read {}", &name));

        assert_valid(&json, schema);
    }

    #[test]
    fn sample_census_file_is_valid() {
        validate_sample_file_against_schema(
            "sample-census-output.json",
            "http_gateway_census_schema.json",
        );
    }

    #[test]
    fn trivial_census_failure() {
        let failure = validate_string(
            r#"{"census_groups": {}, "changed": false, "last_election_counter": "narf"}"#,
            "http_gateway_census_schema.json",
        );
        assert!(
            !failure.is_valid(),
            "Expected schema validation to fail, but it succeeded"
        );
    }

    #[test]
    fn sample_butterfly_file_is_valid() {
        validate_sample_file_against_schema(
            "sample-butterfly-output.json",
            "http_gateway_butterfly_schema.json",
        );
    }

    #[test]
    fn trivial_butterfly_failure() {
        let failure = validate_string(r#"{"departure": {}, "election": {}, "member": {}, "service": false, "service_file": []}"#, "http_gateway_butterfly_schema.json");
        assert!(
            !failure.is_valid(),
            "Expected schema validation to fail, but it succeeded"
        );
    }

    #[test]
    fn butterfly_server_proxy_is_valid() {
        static SWIM_PORT: AtomicUsize = ATOMIC_USIZE_INIT;
        static GOSSIP_PORT: AtomicUsize = ATOMIC_USIZE_INIT;

        #[derive(Debug)]
        struct ZeroSuitability;
        impl Suitability for ZeroSuitability {
            fn get(&self, _service_group: &ServiceGroup) -> u64 {
                0
            }
        }

        fn start_server() -> Server {
            SWIM_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
            GOSSIP_PORT.compare_and_swap(0, 7777, Ordering::Relaxed);
            let swim_port = SWIM_PORT.fetch_add(1, Ordering::Relaxed);
            let swim_listen = format!("127.0.0.1:{}", swim_port);
            let gossip_port = GOSSIP_PORT.fetch_add(1, Ordering::Relaxed);
            let gossip_listen = format!("127.0.0.1:{}", gossip_port);
            let mut member = Member::default();
            member.swim_port = swim_port as i32;
            member.gossip_port = gossip_port as i32;
            Server::new(
                &swim_listen[..],
                &gossip_listen[..],
                member,
                Trace::default(),
                None,
                None,
                None::<PathBuf>,
                Box::new(ZeroSuitability),
            ).unwrap()
        }

        let server = start_server();
        let proxy = ServerProxy::new(&server);
        let json = serde_json::to_string(&proxy).unwrap();
        assert_valid(&json, "http_gateway_butterfly_schema.json");
    }

    #[test]
    fn sample_services_with_cfg_file_is_valid() {
        validate_sample_file_against_schema(
            "sample-services-with-cfg-output.json",
            "http_gateway_services_schema.json",
        );
    }

    #[test]
    fn sample_services_without_cfg_file_is_valid() {
        validate_sample_file_against_schema(
            "sample-services-without-cfg-output.json",
            "http_gateway_services_schema.json",
        );
    }

    #[test]
    fn trivial_services_failure() {
        let failure = validate_string(r#"[{"lulz": true}]"#, "http_gateway_services_schema.json");
        assert!(
            !failure.is_valid(),
            "Expected schema validation to fail, but it succeeded"
        );
    }
}
