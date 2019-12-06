use crate::{manager::ShutdownConfig,
            sys::{service,
                  ShutdownMethod}};
use habitat_common::outputln;
use habitat_core::{os::process::Pid,
                   service::ServiceGroup};
use tokio::task::{self,
                  JoinError};

static LOGKEY: &str = "ST"; // "Service Terminator"

/// Shut a service process down.
///
/// This is performed in a separate thread in order to prevent
/// blocking the rest of the Supervisor.
pub async fn terminate_service(pid: Pid,
                               service_group: ServiceGroup,
                               shutdown_config: ShutdownConfig)
                               -> Result<ShutdownMethod, JoinError> {
    task::spawn_blocking(move || {
        outputln!(preamble service_group, "Terminating service (PID: {})", pid);
        let shutdown = service::kill(pid, &shutdown_config);
        outputln!(preamble service_group, "{} (PID: {})", shutdown, pid);
        shutdown
    }).await
}
