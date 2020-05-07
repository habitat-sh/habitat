use crate::{error::Error,
            manager::{event::{self,
                              ServiceMetadata as ServiceEventMetadata},
                      service::{hook_runner,
                                hooks::HealthCheckHook,
                                supervisor::Supervisor,
                                ProcessOutput,
                                ProcessState},
                      sync::GatewayState}};
use habitat_common::{outputln,
                     templating::package::Pkg};
use habitat_core::service::{HealthCheckInterval,
                            ServiceGroup};
use std::{convert::TryFrom,
          fmt,
          ops::Deref,
          sync::{Arc,
                 Mutex},
          time::Duration};
use tokio::time;

static LOGKEY: &str = "HK";

/// The possible results of running a health check hook.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum HealthCheckResult {
    Ok,
    Warning,
    Critical,
    Unknown,
}

/// Convert health check hook exit codes into `HealthCheckResult` statuses.
impl TryFrom<i32> for HealthCheckResult {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HealthCheckResult::Ok),
            1 => Ok(HealthCheckResult::Warning),
            2 => Ok(HealthCheckResult::Critical),
            3 => Ok(HealthCheckResult::Unknown),
            v => Err(Error::InvalidHealthCheckResult(v)),
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

pub enum HealthCheckHookStatus {
    Ran(ProcessOutput, Duration),
    FailedToRun(Duration),
    FailedToStart,
    NoHook,
}

impl HealthCheckHookStatus {
    pub fn maybe_duration(&self) -> Option<Duration> {
        match &self {
            Self::Ran(_, duration) => Some(*duration),
            Self::FailedToRun(duration) => Some(*duration),
            Self::FailedToStart | Self::NoHook => None,
        }
    }

    pub fn maybe_process_output(self) -> Option<ProcessOutput> {
        match self {
            Self::Ran(process_output, _) => Some(process_output),
            Self::FailedToRun(_) | Self::FailedToStart | Self::NoHook => None,
        }
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
    gateway_state: Arc<GatewayState>,
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
               gateway_state: Arc<GatewayState>)
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

    // Initialize the gateway_state for this health check to Unknown.
    //
    // # Locking (see locking.md)
    // # `GatewayState::inner` (write)
    pub fn init_gateway_state_gsw(self) {
        self.gateway_state
            .lock_gsw()
            .set_health_of(self.service_group, HealthCheckResult::Unknown);
    }

    /// Creates a future that runs the health check and then waits for
    /// a suitable interval. Multiple such iterations will then be
    /// chained together for an unending stream of health checks.
    /// # Locking for the returned Future (see locking.md)
    /// * `GatewayState::inner` (write)
    async fn single_iteration_gsw(self) {
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

        let health_check_hook_status = if let Some(hook) = hook {
            let result = hook_runner::HookRunner::new(hook,
                                                      service_group.deref().clone(),
                                                      package.clone(),
                                                      svc_encrypted_password).into_future()
                                                                             .await;
            match result {
                Ok((output, duration)) => HealthCheckHookStatus::Ran(output, duration),
                Err(Error::WithDuration(e, duration)) => {
                    error!("Error running health check hook for {}: {:?}",
                           service_group, e);
                    HealthCheckHookStatus::FailedToRun(duration)
                }
                Err(_) => {
                    error!("Error starting health check hook for {}: {:?}",
                           service_group, e);
                    HealthCheckHookStatus::FailedToStart
                }
            }
        } else {
            HealthCheckHookStatus::NoHook
        };
        let health_check_result = match &health_check_hook_status {
            HealthCheckHookStatus::Ran(output, _) => {
                // The hook ran. Try and convert its exit status to a `HealthCheckResult`.
                output.exit_status()
                      .code()
                      .and_then(|code| {
                          let result = HealthCheckResult::try_from(code);
                          if let Err(e) = &result {
                              let pkg_name = &package.name;
                              outputln!(preamble pkg_name,
                                             "Health check exited with an unknown status code, {}",
                                             e);
                          }
                          result.ok()
                      })
                      .unwrap_or(HealthCheckResult::Unknown)
            }
            HealthCheckHookStatus::FailedToRun(_) | HealthCheckHookStatus::FailedToStart => {
                // There was a hook but it did not successfully run. The health check result is
                // unknown.
                HealthCheckResult::Unknown
            }
            HealthCheckHookStatus::NoHook => {
                //  There was no hook to run. Use the supervisor status as a healthcheck.
                match supervisor.lock()
                                .expect("couldn't unlock supervisor")
                                .status()
                {
                    ProcessState::Up => HealthCheckResult::Ok,
                    ProcessState::Down => HealthCheckResult::Critical,
                }
            }
        };

        let interval = if health_check_result == HealthCheckResult::Ok {
            // routine health check
            nominal_interval
        } else {
            // special health check interval
            HealthCheckInterval::default()
        };

        event::health_check(service_event_metadata,
                            health_check_result,
                            health_check_hook_status,
                            interval);

        debug!("Caching HealthCheckResult = '{}' for '{}'",
               health_check_result, service_group);
        *service_health_result.lock()
                              .expect("Could not unlock service_health_result") =
            health_check_result;
        gateway_state.lock_gsw()
                     .set_health_of(service_group.deref().clone(), health_check_result);

        trace!("Next health check for {} in {}", service_group, interval);

        time::delay_for(interval.into()).await;
    }

    /// Repeatedly runs a health check, followed by an appropriate
    /// delay, forever.
    /// # Locking for the returned Future (see locking.md)
    /// * `GatewayState::inner` (write)
    pub async fn check_repeatedly_gsw(self) {
        // TODO (CM): If we wanted to keep track of how many times
        // a health check has failed in the past X executions, or
        // do similar historical tracking, here's where we'd do
        // it.
        loop {
            self.clone().single_iteration_gsw().await;
        }
    }
}
