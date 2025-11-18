use crate::{error::Error,
            manager::service::{ProcessOutput,
                               ProcessState,
                               hook_runner,
                               hooks::HealthCheckHook,
                               supervisor::Supervisor}};
use habitat_common::{outputln,
                     templating::package::Pkg};
use habitat_core::service::{HealthCheckInterval,
                            ServiceGroup};
use log::{debug,
          error,
          trace};
use rand::Rng;
use serde::Serialize;
use std::{cmp,
          convert::TryFrom,
          fmt,
          sync::{Arc,
                 Mutex},
          time::Duration};
use tokio::{sync::mpsc::{self,
                         UnboundedReceiver},
            time};

static LOGKEY: &str = "HK";

/// The possible service health result from the status of running the health check.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum HealthCheckResult {
    Ok,
    Warning,
    Critical,
    Unknown,
}

/// Convert health check hook exit codes into `HealthCheckResult`.
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

/// The possible statuses from running a health check hook.
pub enum HealthCheckHookStatus {
    Ran(ProcessOutput, Duration),
    FailedToRun(Duration),
    FailedToStart,
    NoHook,
}

impl HealthCheckHookStatus {
    pub fn maybe_duration(&self) -> Option<Duration> {
        if let Self::Ran(_, duration) | Self::FailedToRun(duration) = self {
            Some(*duration)
        } else {
            None
        }
    }

    pub fn maybe_process_output(self) -> Option<ProcessOutput> {
        if let Self::Ran(output, _) = self {
            Some(output)
        } else {
            None
        }
    }
}

/// The complete set of information from running a health check
///
/// `status` is the raw result from running the health check hook.
/// `result` is a computed value from `status` and other conditions (eg supervisor status if there
/// is not a health check hook)
/// `interval` the computed interval to wait until running the next health check
pub struct HealthCheckBundle {
    pub status:   HealthCheckHookStatus,
    pub result:   HealthCheckResult,
    pub interval: HealthCheckInterval,
}

/// Run the health check hook and get the hook status and result.
async fn check(supervisor: Arc<Mutex<Supervisor>>,
               hook: Option<Arc<HealthCheckHook>>,
               service_group: ServiceGroup,
               package: Pkg,
               password: Option<String>)
               -> (HealthCheckHookStatus, HealthCheckResult) {
    let status = if let Some(hook) = hook {
        let result = hook_runner::HookRunner::new(hook,
                                                  service_group.clone(),
                                                  package.clone(),
                                                  password).into_future()
                                                           .await;
        match result {
            Ok((output, duration)) => HealthCheckHookStatus::Ran(output, duration),
            Err(Error::WithDuration(e, duration)) => {
                error!("Error running health check hook for {}: {:?}",
                       service_group, e);
                HealthCheckHookStatus::FailedToRun(duration)
            }
            Err(e) => {
                error!("Error starting health check hook for {}: {:?}",
                       service_group, e);
                HealthCheckHookStatus::FailedToStart
            }
        }
    } else {
        HealthCheckHookStatus::NoHook
    };

    let result = match &status {
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

    (status, result)
}

/// Start a task to repeatedly check the service health, followed by an appropriate delay, forever.
/// The function returns the receiving end of a channel that acts as a stream of
/// `HealthCheckBundle`s. When this receiving end is dropped or closed health checking will be
/// stopped.
pub fn check_repeatedly(supervisor: Arc<Mutex<Supervisor>>,
                        hook: Option<Arc<HealthCheckHook>>,
                        nominal_interval: HealthCheckInterval,
                        service_group: ServiceGroup,
                        package: Pkg,
                        password: Option<String>)
                        -> UnboundedReceiver<HealthCheckBundle> {
    // TODO (CM): If we wanted to keep track of how many times
    // a health check has failed in the past X executions, or
    // do similar historical tracking, here's where we'd do
    // it.

    let service_group_clone = service_group.clone();
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        let mut first_ok_health_check_recorded = false;
        loop {
            let (status, result) = check(Arc::clone(&supervisor),
                                         hook.as_ref().map(Arc::clone),
                                         service_group.clone(),
                                         package.clone(),
                                         password.clone()).await;

            let interval = if result == HealthCheckResult::Ok {
                if !first_ok_health_check_recorded {
                    // If this was the first successful check, splay future health check runs across
                    // the nominal interval
                    let splay = rand::rng().random_range(0..u64::from(nominal_interval));
                    let splay = Duration::from_secs(splay);
                    debug!("Following `{}`'s first `ok` health-check, delaying a randomly chosen \
                            {}s to introduce health-check splay",
                           service_group,
                           splay.as_secs());
                    first_ok_health_check_recorded = true;
                    splay.into()
                } else {
                    // routine health check
                    nominal_interval
                }
            } else {
                // TODO (DM): Implment exponential backoff
                // https://github.com/habitat-sh/habitat/issues/7265
                // Until exponential backoff is implmented never wait longer than the default
                // interval following a failing health check. If the configured interval is less
                // than the default interval use it instead.
                cmp::min(nominal_interval, HealthCheckInterval::default())
            };

            // This can only fail if the receiving end is closed or dropped indicating to stop
            // executing health checks.
            if tx.send(HealthCheckBundle { status,
                                           result,
                                           interval })
                 .is_err()
            {
                break;
            }

            trace!("`{}` health-check was `{}` next check in {}",
                   service_group, result, interval);
            time::sleep(interval.into()).await;
        }
        outputln!(preamble service_group_clone, "Health checking has been stopped");
    });

    rx
}
