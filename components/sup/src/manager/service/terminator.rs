use super::spawned_future::SpawnedFuture;
use crate::{manager::ShutdownConfig,
            sys::{service,
                  ShutdownMethod}};
use futures::sync::oneshot;
use habitat_common::outputln;
use habitat_core::os::process::Pid;
use std::thread;

static LOGKEY: &str = "ST"; // "Service Terminator"

/// Shut a service process down.
///
/// This is performed in a separate thread in order to prevent
/// blocking the rest of the Supervisor.
pub fn terminate_service(pid: Pid,
                         service_group: String,
                         shutdown_config: ShutdownConfig)
                         -> SpawnedFuture<ShutdownMethod> {
    let (tx, rx) = oneshot::channel();

    let handle_result = thread::Builder::new()
        .name(format!("{}-{}", LOGKEY, pid))
        .spawn(move || {
            outputln!(preamble service_group, "Terminating service (PID: {})", pid);
            let shutdown = service::kill(pid, shutdown_config);
            outputln!(preamble service_group, "{} (PID: {})", shutdown, pid);
            tx.send(shutdown)
                .expect("Couldn't send oneshot signal from terminate_service: receiver went away");
        });

    match handle_result {
        Ok(_handle) => rx.into(),
        Err(io_err) => io_err.into(),
    }
}
