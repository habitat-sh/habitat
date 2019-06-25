use crate::{error::Error,
            manager::ShutdownConfig,
            sys::{service,
                  ShutdownMethod}};
use futures::future::{self,
                      Future};
use habitat_common::outputln;
use habitat_core::os::process::Pid;

static LOGKEY: &str = "ST"; // "Service Terminator"

/// Shut a service process down.
///
/// This is performed in a separate thread in order to prevent
/// blocking the rest of the Supervisor.
pub fn terminate_service(pid: Pid,
                         service_group: String,
                         shutdown_config: ShutdownConfig)
                         -> impl Future<Item = ShutdownMethod, Error = Error> {
    future::lazy(move || {
        outputln!(preamble service_group, "Terminating service (PID: {})", pid);
        let shutdown = service::kill(pid, &shutdown_config);
        outputln!(preamble service_group, "{} (PID: {})", shutdown, pid);
        future::ok(shutdown)
    })
}
