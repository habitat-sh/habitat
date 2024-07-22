use anyhow::Result;
use habitat_core::util::docker;
use log::debug;
use std::process::Command;
use thiserror::Error;

/// Currently when exporting containers on Windows, the Docker daemon
/// *must* be in Windows mode (i.e., only Windows containers can be
/// exported on Windows machines).
///
/// If the daemon is in Linux mode, we return an error and should stop
/// the export process.
pub(crate) fn ensure_proper_docker_platform() -> Result<(), Error> {
    match DockerOS::current() {
        DockerOS::Windows => Ok(()),
        other => {
            if let DockerOS::Unknown(ref s) = other {
                debug!("Unknown Docker OS: {}", s);
            }
            Err(Error::DockerNotInWindowsMode(other))
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Only Windows container export is supported; please set your Docker daemon to \
             Windows container mode.\n\nThe Docker daemon is currently set for: {0:?}")]
    DockerNotInWindowsMode(DockerOS),
}

/// Describes the OS of the containers the Docker daemon is currently
/// configured to manage.
#[derive(Clone, Debug)]
enum DockerOS {
    /// Docker daemon is managing Linux containers
    Linux,
    /// Docker daemon is managing Windows containers
    Windows,
    /// Generic fall-through for error handling and extra paranoia
    Unknown(String),
}

impl DockerOS {
    /// Returns the OS for which the locally-running Docker daemon is
    /// managing containers.
    ///
    /// Daemons running on Linux would report "Linux", while a Windows
    /// daemon may report "Windows" or "Linux", depending on what mode
    /// it is currently running in.
    fn current() -> DockerOS {
        let mut cmd = Command::new(docker::command_path().expect("Unable to locate docker"));
        cmd.arg("version").arg("--format={{.Server.Os}}");
        debug!("Running command: {:?}", cmd);
        let result = cmd.output().expect("Docker command failed to spawn");
        let result = String::from_utf8_lossy(&result.stdout);
        if result.contains("windows") {
            DockerOS::Windows
        } else if result.contains("linux") {
            DockerOS::Linux
        } else {
            // We really shouldn't get down here, but we *are* parsing
            // strings from other software that might change in the
            // future.
            DockerOS::Unknown(result.to_string())
        }
    }
}
