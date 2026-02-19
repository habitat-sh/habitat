use habitat_core::{error::{Error,
                           Result},
                   util::docker};
use log::debug;
use std::process::Command;

/// Currently when exporting containers on Windows, the Docker daemon
/// *must* be in Windows mode (i.e., only Windows containers can be
/// exported on Windows machines).
///
/// If the daemon is in Linux mode, we return an error and should stop
/// the export process.
pub(crate) fn ensure_proper_docker_platform() -> Result<()> {
    match DockerOS::current() {
        Ok(DockerOS::Windows) => Ok(()),
        Ok(other) => {
            if let DockerOS::Unknown(ref s) = other {
                debug!("Unknown Docker OS: {}", s);
            }
            Err(Error::UnsupportedDockerHostKernel(format!("Only Windows \
                                                            container export \
                                                            is supported; \
                                                            please set \
                                                            your Docker \
                                                            daemon to Windows \
                                                            container mode. \
                                                            The Docker \
                                                            daemon is currently \
                                                            set for: {:?}",
                                                           other)))
        }
        Err(e) => {
            debug!("Failed to determine Docker OS: {}", e);
            Err(e)
        }
    }
}

/// Describes the OS of the containers the Docker daemon is currently
/// configured to manage.
#[derive(Clone, Debug)]
pub(crate) enum DockerOS {
    /// Docker daemon is managing Linux containers
    Linux,
    /// Docker daemon is managing Windows containers
    Windows,
    /// Docker daemon reports an unrecognized OS string
    /// This is only for genuinely unknown OS types, not errors
    Unknown(String),
}

impl DockerOS {
    /// Returns the OS for which the locally-running Docker daemon is
    /// managing containers.
    ///
    /// Daemons running on Linux would report "Linux", while a Windows
    /// daemon may report "Windows" or "Linux", depending on what mode
    /// it is currently running in.
    fn current() -> Result<DockerOS> {
        let docker_path = docker::command_path()?;

        let mut cmd = Command::new(docker_path);
        cmd.arg("version").arg("--format=\"{{.Server.Os}}\"");
        debug!("Running command: {:?}", cmd);

        let output = cmd.output().map_err(Error::DockerVersionCommandFailed)?;

        // Check if the command succeeded
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            return Err(Error::DockerVersionCommandExitFailure { exit_code: output.status
                                                                                 .code()
                                                                                 .unwrap_or(-1),
                                                                stderr:    stderr.trim()
                                                                                 .to_string(),
                                                                stdout:    stdout.trim()
                                                                                 .to_string(), });
        }

        let result = String::from_utf8_lossy(&output.stdout);
        if result.contains("windows") {
            Ok(DockerOS::Windows)
        } else if result.contains("linux") {
            Ok(DockerOS::Linux)
        } else {
            // We really shouldn't get down here, but we *are* parsing
            // strings from other software that might change in the
            // future.
            Ok(DockerOS::Unknown(result.trim().to_string()))
        }
    }
}
