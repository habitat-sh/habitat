use crate::{error::{Error,
                    Result},
            fs::find_command};
use std::{path::PathBuf,
          process::Command};

const DOCKER_CMD: &str = "docker";

pub fn command_path() -> Result<PathBuf> {
    find_command(DOCKER_CMD).ok_or_else(|| Error::DockerCommandNotFound(DOCKER_CMD))
}

/// Makes a best attempt to retrieve the appropriate image tag based on
/// https://hub.docker.com/_/microsoft-windows-servercore
/// Note that changes here should be mirrored in .buildkite/scripts/build_docker_image.ps1
pub fn default_base_tag_for_host() -> Result<&'static str> {
    if cfg!(windows) {
        let mut cmd = Command::new(command_path()?);
        cmd.arg("info").arg("--format='{{.Isolation}}'");
        let result = cmd.output().expect("Docker command failed to spawn");
        if String::from_utf8(result.stdout)?.trim() == "'hyperv'" {
            // hyperv isolation can build any version so we will default to 2016
            Ok("ltsc2016")
        } else {
            match os_info::get().version().version() {
                os_info::VersionType::Semantic(10, 0, 14393) => Ok("ltsc2016"),
                os_info::VersionType::Semantic(10, 0, 17134) => Ok("1803"),
                os_info::VersionType::Semantic(10, 0, 17763) => Ok("ltsc2019"),
                os_info::VersionType::Semantic(10, 0, 18362) => Ok("1903"),
                unsupported_version => {
                    Err(Error::UnsupportedDockerHostKernel(unsupported_version.to_string()))
                }
            }
        }
    } else {
        Ok("latest")
    }
}
