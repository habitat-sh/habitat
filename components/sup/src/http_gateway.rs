use crate::manager::{self,
                     service::{HealthCheckHook,
                               HealthCheckResult}};
use actix_web::{dev::{Body,
                      Service,
                      ServiceRequest,
                      ServiceResponse},
                http::{self,
                       StatusCode},
                web::{self,
                      Data,
                      Path},
                App,
                Error,
                HttpResponse,
                HttpServer,
                Scope};
use futures::future::{ok,
                      Either,
                      Future};
use habitat_common::{self,
                     templating::hooks,
                     types::HttpListenAddr,
                     FeatureFlag};
use habitat_core::{crypto,
                   env as henv,
                   service::ServiceGroup};
use manager::sync::GatewayState;

use prometheus::{self,
                 CounterVec,
                 Encoder,
                 HistogramTimer,
                 HistogramVec,
                 TextEncoder};
use rustls::ServerConfig;
use serde_json::{self,
                 Value as Json};
use std::{self,
          cell::Cell,
          fs::File,
          io::Read,
          sync::{Arc,
                 Condvar,
                 Mutex},
          thread};

const APIDOCS: &str = include_str!(concat!(env!("OUT_DIR"), "/api.html"));
pub const HTTP_THREADS_ENVVAR: &str = "HAB_SUP_HTTP_THREADS";
pub const HTTP_THREAD_COUNT: usize = 2;

/// Default listening port for the HTTPGateway listener.
pub const DEFAULT_PORT: u16 = 9631;

lazy_static! {
    static ref HTTP_GATEWAY_REQUESTS: CounterVec =
        register_counter_vec!("hab_sup_http_gateway_requests_total",
                              "Total number of HTTP gateway requests",
                              &["path"]).unwrap();
    static ref HTTP_GATEWAY_REQUEST_DURATION: HistogramVec =
        register_histogram_vec!("hab_sup_http_gateway_request_duration_seconds",
                                "The latency for HTTP gateway requests",
                                &["path"]).unwrap();
}

habitat_core::env_config!(
    /// This represents an environment variable that holds an authentication token for the supervisor's
    /// HTTP gateway. If the environment variable is present, then its value is the auth token and all
    /// of the HTTP endpoints will require its presence. If it's not present, then everything continues
    /// to work unauthenticated.
    #[derive(Clone, Debug)]
    pub GatewayAuthenticationToken,
    Option<String>,
    HAB_SUP_GATEWAY_AUTH_TOKEN,
    None,
    std::string::ParseError,
    s,
    Ok(GatewayAuthenticationToken(Some(String::from(s)))));

#[derive(Default, Serialize)]
struct HealthCheckBody {
    status: String,
    stdout: String,
    stderr: String,
}

impl Into<StatusCode> for HealthCheckResult {
    fn into(self) -> StatusCode {
        match self {
            HealthCheckResult::Ok | HealthCheckResult::Warning => StatusCode::OK,
            HealthCheckResult::Critical => StatusCode::SERVICE_UNAVAILABLE,
            HealthCheckResult::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

struct AppState {
    gateway_state:        Arc<GatewayState>,
    authentication_token: Option<String>,
    timer:                Cell<Option<HistogramTimer>>,
    feature_flags:        FeatureFlag,
}

impl AppState {
    fn new(gs: Arc<GatewayState>,
           authentication_token: GatewayAuthenticationToken,
           feature_flags: FeatureFlag)
           -> Self {
        AppState { gateway_state: gs,
                   // We'll unwrap to the inner type, since the
                   // GatewayAuthenticationToken type has done its job by this point.
                   authentication_token: authentication_token.0,
                   timer: Cell::new(None),
                   feature_flags }
    }
}

// Begin middleware

fn authentication_middleware<S>(req: ServiceRequest,
                                srv: &mut S)
                                -> impl Future<Item = ServiceResponse<Body>, Error = Error>
    where S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error>
{
    let current_token = &req.app_data::<AppState>()
                            .expect("app data")
                            .authentication_token;
    let current_token = if let Some(t) = current_token {
        t
    } else {
        debug!("No authentication token present. HTTP gateway starting in unauthenticated mode.");
        return Either::A(srv.call(req));
    };

    // From this point forward, we know that we have an
    // authentication token in the state. Therefore, anything
    // short of a fully formed Authorization header (yes,
    // Authorization; HTTP is fun, kids!) containing a Bearer
    // token that matches the value we have in our state, results
    // in an Unauthorized response.
    let hdr = match req.headers()
                       .get(http::header::AUTHORIZATION)
                       .ok_or("header missing")
                       .and_then(|hv| hv.to_str().or(Err("can't convert to str")))
    {
        Ok(h) => h,
        Err(e) => {
            debug!("Error reading required Authorization header: {:?}.", e);
            return Either::B(ok(req.into_response(HttpResponse::Unauthorized().finish())));
        }
    };

    let hdr_components: Vec<&str> = hdr.split_whitespace().collect();

    match hdr_components.as_slice() {
        ["Bearer", incoming_token] if crypto::secure_eq(current_token, incoming_token) => {
            Either::A(srv.call(req))
        }
        _ => Either::B(ok(req.into_response(HttpResponse::Unauthorized().finish()))),
    }
}

fn metrics_middleware<S>(req: ServiceRequest,
                         srv: &mut S)
                         -> impl Future<Item = ServiceResponse<Body>, Error = Error>
    where S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error>
{
    let label_values = &[req.path()];

    HTTP_GATEWAY_REQUESTS.with_label_values(label_values).inc();
    let timer = HTTP_GATEWAY_REQUEST_DURATION.with_label_values(label_values)
                                             .start_timer();
    req.app_data::<AppState>()
       .expect("app data")
       .timer
       .set(Some(timer));

    srv.call(req).and_then(|res| {
                     if let Some(timer) = res.request()
                                             .app_data::<AppState>()
                                             .expect("app data")
                                             .timer
                                             .replace(None)
                     {
                         timer.observe_duration();
                     }
                     Ok(res)
                 })
}

fn redact_http_middleware<S>(req: ServiceRequest,
                             srv: &mut S)
                             -> impl Future<Item = ServiceResponse<Body>, Error = Error>
    where S: Service<Request = ServiceRequest, Response = ServiceResponse<Body>, Error = Error>
{
    if req.app_data::<AppState>()
          .expect("app data")
          .feature_flags
          .contains(FeatureFlag::REDACT_HTTP)
    {
        Either::B(ok(req.into_response(HttpResponse::NotFound().finish())))
    } else {
        Either::A(srv.call(req))
    }
}

// End middleware

#[derive(Debug, PartialEq, Eq)]
pub enum ServerStartup {
    NotStarted,
    Started,
    BindFailed,
}

pub struct Server;

impl Server {
    pub fn run(listen_addr: HttpListenAddr,
               tls_config: Option<ServerConfig>,
               gateway_state: Arc<GatewayState>,
               authentication_token: GatewayAuthenticationToken,
               feature_flags: FeatureFlag,
               control: Arc<(Mutex<ServerStartup>, Condvar)>) {
        thread::spawn(move || {
            let &(ref lock, ref cvar) = &*control;
            let thread_count = match henv::var(HTTP_THREADS_ENVVAR) {
                Ok(val) => {
                    match val.parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => HTTP_THREAD_COUNT,
                    }
                }
                Err(_) => HTTP_THREAD_COUNT,
            };

            let mut server = HttpServer::new(move || {
                                 let app_state = AppState::new(gateway_state.clone(),
                                                               authentication_token.clone(),
                                                               feature_flags);
                                 App::new().data(app_state)
                                           .wrap_fn(authentication_middleware)
                                           .wrap_fn(metrics_middleware)
                                           .service(routes())
                             }).workers(thread_count);

            // On Windows the default actix signal handler will create a ctrl+c handler for the
            // process which will disable default windows ctrl+c behavior and allow us to
            // handle via check_for_signal in the supervisor service loop. However, if the
            // supervisor is in a long running non-run hook, that loop will not get to
            // check_for_signal in a reasonable amount of time and the supervisor will not
            // respond to ctrl+c. On Windows, we let the launcher catch ctrl+c and gracefully
            // shut down services. ctrl+c should simply halt the supervisor. The IgnoreSignals
            // feature is always enabled in the Habitat Windows Service which relies on ctrl+c
            // signals to stop the supervisor.
            if feature_flags.contains(FeatureFlag::IGNORE_SIGNALS) {
                server = server.disable_signals();
            }

            let bind = match tls_config {
                Some(c) => server.bind_rustls(listen_addr.to_string(), c),
                None => server.bind(listen_addr.to_string()),
            };

            *lock.lock().expect("Control mutex is poisoned") = match bind {
                Ok(_) => ServerStartup::Started,
                Err(ref e) => {
                    error!("HTTP gateway failed to bind: {}", e);
                    ServerStartup::BindFailed
                }
            };

            cvar.notify_one();

            if let Ok(b) = bind {
                b.run().expect("to start http server");
            }
        });
    }
}

fn services_routes() -> Scope {
    web::scope("/services").route("", web::get().to(services_gsr))
                           .route("/{svc}/{group}", web::get().to(service_without_org_gsr))
                           .route("/{svc}/{group}/config",
                                  web::get().to(config_without_org_gsr))
                           .route("/{svc}/{group}/health",
                                  web::get().to(health_without_org_gsr))
                           .route("/{svc}/{group}/{org}", web::get().to(service_with_org_gsr))
                           .route("/{svc}/{group}/{org}/config",
                                  web::get().to(config_with_org_gsr))
                           .route("/{svc}/{group}/{org}/health",
                                  web::get().to(health_with_org_gsr))
}

fn routes() -> Scope {
    web::scope("/").route("", web::get().to(doc))
                   .service(services_routes())
                   .service(web::resource("/butterfly").route(web::get().to(butterfly_gsr))
                                                       .wrap_fn(redact_http_middleware))
                   .service(web::resource("/census").route(web::get().to(census_gsr))
                                                    .wrap_fn(redact_http_middleware))
                   .route("/metrics", web::get().to(metrics))
}

fn json_response(data: String) -> HttpResponse {
    HttpResponse::Ok().content_type("application/json")
                      .body(data)
}

// Begin route handlers

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn butterfly_gsr(state: Data<AppState>) -> HttpResponse {
    let data = state.gateway_state.lock_gsr().butterfly_data().to_string();
    json_response(data)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn census_gsr(state: Data<AppState>) -> HttpResponse {
    let data = state.gateway_state.lock_gsr().census_data().to_string();
    json_response(data)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn services_gsr(state: Data<AppState>) -> HttpResponse {
    let data = state.gateway_state.lock_gsr().services_data().to_string();
    json_response(data)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
// Honestly, this doesn't feel great, but it's the pattern builder-api uses, and at the
// moment, I don't have a better way of doing it.
#[allow(clippy::needless_pass_by_value)]
fn config_with_org_gsr(path: Path<(String, String, String)>,
                       state: Data<AppState>)
                       -> HttpResponse {
    let (svc, group, org) = path.into_inner();
    config_gsr(svc, group, Some(&org), &state)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn config_without_org_gsr(path: Path<(String, String)>, state: Data<AppState>) -> HttpResponse {
    let (svc, group) = path.into_inner();
    config_gsr(svc, group, None, &state)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
fn config_gsr(svc: String, group: String, org: Option<&str>, state: &AppState) -> HttpResponse {
    let service_group = match ServiceGroup::new(None, svc, group, org) {
        Ok(sg) => sg,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match service_from_services(&service_group,
                                state.gateway_state.lock_gsr().services_data())
    {
        Some(mut s) => HttpResponse::Ok().json(s["cfg"].take()),
        None => HttpResponse::NotFound().finish(),
    }
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn health_with_org_gsr(path: Path<(String, String, String)>,
                       state: Data<AppState>)
                       -> HttpResponse {
    let (svc, group, org) = path.into_inner();
    health_gsr(svc, group, Some(&org), &state)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn health_without_org_gsr(path: Path<(String, String)>, state: Data<AppState>) -> HttpResponse {
    let (svc, group) = path.into_inner();
    health_gsr(svc, group, None, &state)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
fn health_gsr(svc: String, group: String, org: Option<&str>, state: &AppState) -> HttpResponse {
    let service_group = match ServiceGroup::new(None, svc, group, org) {
        Ok(sg) => sg,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if let Some(health_check) = state.gateway_state.lock_gsr().health_of(&service_group) {
        let mut body = HealthCheckBody::default();
        let stdout_path = hooks::stdout_log_path::<HealthCheckHook>(&service_group);
        let stderr_path = hooks::stderr_log_path::<HealthCheckHook>(&service_group);
        let http_status: StatusCode = health_check.into();

        body.status = health_check.to_string();
        if let Ok(mut file) = File::open(&stdout_path) {
            let _ = file.read_to_string(&mut body.stdout);
        }
        if let Ok(mut file) = File::open(&stderr_path) {
            let _ = file.read_to_string(&mut body.stderr);
        }

        HttpResponse::build(http_status).json(&body)
    } else {
        debug!("Didn't find any health data for service group {:?}",
               &service_group);
        HttpResponse::NotFound().finish()
    }
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn service_with_org_gsr(path: Path<(String, String, String)>,
                        state: Data<AppState>)
                        -> HttpResponse {
    let (svc, group, org) = path.into_inner();
    service_gsr(svc, group, Some(&org), &state)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
#[allow(clippy::needless_pass_by_value)]
fn service_without_org_gsr(path: Path<(String, String)>, state: Data<AppState>) -> HttpResponse {
    let (svc, group) = path.into_inner();
    service_gsr(svc, group, None, &state)
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
fn service_gsr(svc: String, group: String, org: Option<&str>, state: &AppState) -> HttpResponse {
    let service_group = match ServiceGroup::new(None, svc, group, org) {
        Ok(sg) => sg,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match service_from_services(&service_group,
                                state.gateway_state.lock_gsr().services_data())
    {
        Some(s) => HttpResponse::Ok().json(s),
        None => HttpResponse::NotFound().finish(),
    }
}

fn metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];

    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        error!("Error encoding metrics: {:?}", e);
    }

    let resp = match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(e) => {
            error!("Error constructing string from metrics buffer: {:?}", e);
            String::from("")
        }
    };

    HttpResponse::Ok().content_type(encoder.format_type())
                      .body(resp)
}

fn doc() -> HttpResponse { HttpResponse::Ok().content_type("text/html").body(APIDOCS) }
// End route handlers

fn service_from_services(service_group: &ServiceGroup, services_json: &str) -> Option<Json> {
    match serde_json::from_str(services_json) {
        Ok(Json::Array(services)) => {
            services.into_iter()
                    .find(|s| s["service_group"] == service_group.as_ref())
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;
    use habitat_butterfly::{member::Member,
                            server::{Server,
                                     ServerProxy,
                                     Suitability}};
    use serde_json;
    use std::{fs::File,
              io::Read,
              net::{IpAddr,
                    Ipv4Addr,
                    SocketAddr},
              path::PathBuf,
              sync::Mutex};

    fn validate_sample_file_against_schema(name: &str, schema: &str) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                            .join("fixtures")
                                                            .join("http-gateway")
                                                            .join(name);

        let mut f = File::open(path).unwrap_or_else(|_| panic!("could not open {}", &name));
        let mut json = String::new();
        f.read_to_string(&mut json)
         .unwrap_or_else(|_| panic!("could not read {}", &name));

        assert_valid(&json, schema);
    }

    #[test]
    fn sample_census_file_is_valid() {
        validate_sample_file_against_schema("sample-census-output.json",
                                            "http_gateway_census_schema.json");
    }

    #[test]
    fn trivial_census_failure() {
        let failure = validate_string(
            r#"{"census_groups": {}, "changed": false, "last_election_counter": "narf"}"#,
            "http_gateway_census_schema.json",
        );
        assert!(!failure.is_valid(),
                "Expected schema validation to fail, but it succeeded");
    }

    #[test]
    fn sample_butterfly_file_is_valid() {
        validate_sample_file_against_schema("sample-butterfly-output.json",
                                            "http_gateway_butterfly_schema.json");
    }

    #[test]
    fn trivial_butterfly_failure() {
        let failure = validate_string(r#"{"departure": {}, "election": {}, "member": {}, "service": false, "service_file": []}"#, "http_gateway_butterfly_schema.json");
        assert!(!failure.is_valid(),
                "Expected schema validation to fail, but it succeeded");
    }

    #[test]
    fn butterfly_server_proxy_is_valid() {
        lazy_static! {
            static ref SWIM_PORT: Mutex<u16> = Mutex::new(6666);
            static ref GOSSIP_PORT: Mutex<u16> = Mutex::new(7777);
        }

        #[derive(Debug)]
        struct ZeroSuitability;
        impl Suitability for ZeroSuitability {
            fn suitability_for_msr(&self, _service_group: &str) -> u64 { 0 }
        }

        fn start_server() -> Server {
            let swim_port;
            {
                let mut swim_port_guard = SWIM_PORT.lock().expect("SWIM_PORT poisoned");
                swim_port = *swim_port_guard;
                *swim_port_guard += 1;
            }
            let swim_listen = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), swim_port);
            let gossip_port;
            {
                let mut gossip_port_guard = GOSSIP_PORT.lock().expect("GOSSIP_PORT poisoned");
                gossip_port = *gossip_port_guard;
                *gossip_port_guard += 1;
            }
            let gossip_listen =
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), gossip_port);
            let mut member = Member::default();
            member.swim_port = swim_port;
            member.gossip_port = gossip_port;
            Server::new(swim_listen,
                        gossip_listen,
                        member,
                        None,
                        None,
                        None,
                        std::sync::Arc::new(ZeroSuitability)).unwrap()
        }

        let server = start_server();
        let proxy = ServerProxy::new(&server);
        let json = serde_json::to_string(&proxy).unwrap();
        assert_valid(&json, "http_gateway_butterfly_schema.json");
    }

    #[test]
    fn sample_services_with_cfg_file_is_valid() {
        validate_sample_file_against_schema("sample-services-with-cfg-output.json",
                                            "http_gateway_services_schema.json");
    }

    #[test]
    fn sample_services_without_cfg_file_is_valid() {
        validate_sample_file_against_schema("sample-services-without-cfg-output.json",
                                            "http_gateway_services_schema.json");
    }

    #[test]
    fn trivial_services_failure() {
        let failure = validate_string(r#"[{"lulz": true}]"#, "http_gateway_services_schema.json");
        assert!(!failure.is_valid(),
                "Expected schema validation to fail, but it succeeded");
    }
}
