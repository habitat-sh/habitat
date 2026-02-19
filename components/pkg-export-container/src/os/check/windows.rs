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
pub(crate) enum DockerOS {
    /// Docker daemon is managing Linux containers
    Linux,
    /// Docker daemon is managing Windows containers
    Windows,
    /// Generic fall-through for error handling and extra paranoia
    /// Contains either the unexpected output or error message
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
        let docker_path = match docker::command_path() {
            Ok(path) => path,
            Err(e) => return DockerOS::Unknown(format!("Unable to locate docker: {}", e)),
        };

        let mut cmd = Command::new(docker_path);
        cmd.arg("version").arg("--format={{.Server.Os}}");
        debug!("Running command: {:?}", cmd);

        let output = match cmd.output() {
            Ok(output) => output,
            Err(e) => return DockerOS::Unknown(format!("Docker command failed to execute: {}", e)),
        };

        // Check if the command succeeded
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return DockerOS::Unknown(format!("Docker command failed with exit code {}: stderr: \
                                              {}, stdout: {}",
                                             output.status.code().unwrap_or(-1),
                                             stderr.trim(),
                                             stdout.trim()));
        }

        let result = String::from_utf8_lossy(&output.stdout);
        if result.contains("windows") {
            DockerOS::Windows
        } else if result.contains("linux") {
            DockerOS::Linux
        } else {
            // We really shouldn't get down here, but we *are* parsing
            // strings from other software that might change in the
            // future.
            DockerOS::Unknown(format!("Unexpected docker version output: {}", result.trim()))
        }
    }
}
