use crate::manager::{event::{self,
                             ServiceMetadata as ServiceEventMetadata},
                     service::{hook_runner,
                               hooks::HealthCheckHook,
                               supervisor::Supervisor},
                     GatewayState};
use futures::{future::{self,
                       lazy,
                       Either,
                       Future,
                       Loop},
              IntoFuture};
use habitat_common::templating::package::Pkg;
use habitat_core::service::{HealthCheckInterval,
                            ServiceGroup};
use std::{fmt,
          ops::Deref,
          sync::{Arc,
                 Mutex,
                 RwLock},
          time::{Duration,
                 Instant}};

/// The possible results of running a health check hook.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum HealthCheckResult {
    Ok,
    Warning,
    Critical,
    Unknown,
}

impl Default for HealthCheckResult {
    fn default() -> HealthCheckResult { HealthCheckResult::Unknown }
}

/// Convert health check hook exit codes into `HealthCheckResult` statuses.
impl From<i8> for HealthCheckResult {
    fn from(value: i8) -> HealthCheckResult {
        match value {
            0 => HealthCheckResult::Ok,
            1 => HealthCheckResult::Warning,
            2 => HealthCheckResult::Critical,
            3 => HealthCheckResult::Unknown,
            _ => HealthCheckResult::Unknown,
        }
    }
}

impl fmt::Display for HealthCheckResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            HealthCheckResult::Ok => "OK",
            HealthCheckResult::Warning => "WARNING",
            HealthCheckResult::Critical => "CRITICAL",
            HealthCheckResult::Unknown => "UNKNOWN",
        };
        write!(f, "{}", msg)
    }
}

/// All state needed for checking the health of a service over time.
#[derive(Clone)]
pub struct State {
    // All hooks currently need these
    hook:                   Option<Arc<HealthCheckHook>>,
    service_group:          ServiceGroup,
    package:                Pkg,
    svc_encrypted_password: Option<String>,

    service_event_metadata: ServiceEventMetadata,

    /// A reference to the process supervisor for the service. This is
    /// used to create a "proxy health check" for services that do not
    /// provide their own health check hook.
    supervisor: Arc<Mutex<Supervisor>>,

    /// The configured interval at which to run health checks for this
    /// service. The interval actually used may differ based on the
    /// previous health check result.
    nominal_interval: HealthCheckInterval,

    /// A reference to the service's current health check status. We
    /// store the result of the health check here.
    service_health_result: Arc<Mutex<HealthCheckResult>>,

    /// A reference to the Supervisor's gateway state. We also store
    /// the status in here for making it available via the HTTP
    /// gateway.
    gateway_state: Arc<RwLock<GatewayState>>,
}

impl State {
    #[allow(clippy::too_many_arguments)]
    pub fn new(hook: Option<Arc<HealthCheckHook>>,
               service_group: ServiceGroup,
               package: Pkg,
               svc_encrypted_password: Option<String>,
               service_event_metadata: ServiceEventMetadata,
               supervisor: Arc<Mutex<Supervisor>>,
               nominal_interval: HealthCheckInterval,
               service_health_result: Arc<Mutex<HealthCheckResult>>,
               gateway_state: Arc<RwLock<GatewayState>>)
               -> Self {
        State { hook,
                service_group,
                package,
                svc_encrypted_password,
                service_event_metadata,
                supervisor,
                nominal_interval,
                service_health_result,
                gateway_state }
    }

    /// Creates a future that runs the health check and then waits for
    /// a suitable interval. Multiple such iterations will then be
    /// chained together for an unending stream of health checks.
    fn single_iteration(self) -> impl Future<Item = (), Error = ()> {
        let State { hook,
                    service_group,
                    package,
                    svc_encrypted_password,
                    service_event_metadata,
                    supervisor,
                    nominal_interval,
                    service_health_result,
                    gateway_state, } = self;

        // Use an Arc to avoid having to have full clones everywhere. :/
        let service_group = Arc::new(service_group);
        let service_group_ref = Arc::clone(&service_group);

        if let Some(hook) = hook {
            let hr = hook_runner::HookRunner::new(hook,
                                                  service_group.deref().clone(),
                                                  package,
                                                  svc_encrypted_password);
            Either::A(hr.into_future()
                        .map(|(result, duration)| (result, Some(duration))))
        } else {
            let status = match supervisor.lock()
                                         .expect("couldn't unlock supervisor")
                                         .status()
            {
                (true, _) => HealthCheckResult::Ok,
                (false, _) => HealthCheckResult::Critical,
            };
            // no hook means no execution time!
            Either::B(lazy(move || Ok((status, None::<Duration>))))
        }.map_err(move |e| {
             error!("Error running health check hook for {}: {:?}",
                    service_group_ref, e)
         })
         .and_then(move |(check_result, duration)| {
             event::health_check(service_event_metadata, check_result, duration);
             debug!("Caching HealthCheckResult = '{}' for '{}'",
                    check_result, service_group);

             *service_health_result.lock()
                                   .expect("Could not unlock service_health_result") = check_result;
             gateway_state.write()
                          .expect("GatewayState lock is poisoned")
                          .health_check_data
                          .insert(service_group.deref().clone(), check_result);

             let interval = if check_result == HealthCheckResult::Ok {
                 // routine health check
                 nominal_interval
             } else {
                 // special health check interval
                 HealthCheckInterval::default()
             };

             trace!("Next health check for {} in {}", service_group, interval);

             let instant =
                 Instant::now().checked_add(interval.into())
                               .expect("This should never happen with normal health check \
                                        interval sizes");
             tokio_timer::timer::Handle::current().delay(instant)
                                                  .map_err(move |timer_error| {
                                                      if timer_error.is_shutdown() {
                                                          warn!("Timer for {} health check shut \
                                                                 down!",
                                                                service_group);
                                                      }
                                                      if timer_error.is_at_capacity() {
                                                          warn!("Timer for {} health check is at \
                                                                 capacity!",
                                                                service_group);
                                                      }
                                                  })
         })
    }

    /// Repeatedly runs a health check, followed by an appropriate
    /// delay, forever.
    pub fn check_repeatedly(self) -> impl Future<Item = (), Error = ()> {
        future::loop_fn(self, move |state| {
            // TODO (CM): If we wanted to keep track of how many times
            // a health check has failed in the past X executions, or
            // do similar historical tracking, here's where we'd do
            // it.
            let service_group = state.service_group.clone();
            state.clone().single_iteration().then(move |res| {
                                                if res.is_ok() {
                                                    trace!("Health check future for {} \
                                                            succeeded; continuing loop",
                                                           service_group);
                                                    Ok(Loop::Continue(state))
                                                } else {
                                                    trace!("Health check future for {} failed \
                                                            failed; continuing loop.",
                                                           service_group);
                                                    Ok(Loop::Continue(state))
                                                }
                                            })
        })
    }
}
