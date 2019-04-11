//! Defines types for sending information about "actions" from one
//! part of the Supervisor to another.

use super::service::ServiceSpec;
#[cfg(unix)]
use habitat_core::os::process::ShutdownSignal;
use habitat_core::os::process::ShutdownTimeout;
use std::sync::mpsc;

/// Defines the parameters by which a service process is to be shut
/// down cleanly.
#[cfg(windows)]
#[derive(Clone, Debug, Default)]
pub struct ShutdownSpec {
    /// How long to wait after sending a process a Ctrl-C to shutdown
    /// until we forcibly terminate it.
    pub timeout: ShutdownTimeout,
}

/// Defines the parameters by which a service process is to be shut
/// down cleanly.
#[cfg(unix)]
#[derive(Clone, Debug, Default)]
pub struct ShutdownSpec {
    /// The signal to send a process to make it shut down cleanly.
    pub signal: ShutdownSignal,
    /// How long to wait for a process to end, after sending it a
    /// shutdown signal, until we forcibly terminate it.
    pub timeout: ShutdownTimeout,
}

/// Describe actions initiated by user interaction in terms that the
/// Supervisor itself can understand and operate on.
// TODO (CM): More actions will be added to this with future
// refactorings
#[derive(Clone, Debug)]
pub enum SupervisorAction {
    StopService {
        service_spec:  ServiceSpec,
        shutdown_spec: ShutdownSpec,
    },
    UnloadService {
        service_spec:  ServiceSpec,
        shutdown_spec: ShutdownSpec,
    },
}

pub type ActionSender = mpsc::Sender<SupervisorAction>;

#[cfg(unix)]
impl Into<ShutdownSpec> for habitat_sup_protocol::ctl::SvcUnload {
    fn into(self) -> ShutdownSpec {
        let timeout = self.timeout.map(Into::into).unwrap_or_default();
        let signal = self.signal
                         .map(|s| s.parse().unwrap_or_default())
                         .unwrap_or_default();
        ShutdownSpec { signal, timeout }
    }
}

#[cfg(windows)]
impl Into<ShutdownSpec> for habitat_sup_protocol::ctl::SvcUnload {
    fn into(self) -> ShutdownSpec {
        let timeout = self.timeout.map(Into::into).unwrap_or_default();
        ShutdownSpec { timeout }
    }
}

#[cfg(unix)]
impl Into<ShutdownSpec> for habitat_sup_protocol::ctl::SvcStop {
    fn into(self) -> ShutdownSpec {
        let timeout = self.timeout.map(Into::into).unwrap_or_default();
        let signal = self.signal
                         .map(|s| s.parse().unwrap_or_default())
                         .unwrap_or_default();
        ShutdownSpec { signal, timeout }
    }
}

#[cfg(windows)]
impl Into<ShutdownSpec> for habitat_sup_protocol::ctl::SvcStop {
    fn into(self) -> ShutdownSpec {
        let timeout = self.timeout.map(Into::into).unwrap_or_default();
        ShutdownSpec { timeout }
    }
}
