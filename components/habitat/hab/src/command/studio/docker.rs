// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use common::ui::UI;
use hcore::crypto::default_cache_key_path;
use hcore::env as henv;
use hcore::fs::{CACHE_ARTIFACT_PATH, CACHE_KEY_PATH, find_command};
use hcore::os::process;
use command::studio::enter::ARTIFACT_PATH_ENVVAR;

use error::{Error, Result};
use VERSION;

const DOCKER_CMD: &'static str = "docker";
const DOCKER_CMD_ENVVAR: &'static str = "HAB_DOCKER_BINARY";
const DOCKER_IMAGE: &'static str = "habitat-docker-registry.bintray.io/studio";
const DOCKER_WINDOWS_IMAGE: &'static str = "habitat-docker-registry.bintray.io/win-studio";
const DOCKER_IMAGE_ENVVAR: &'static str = "HAB_DOCKER_STUDIO_IMAGE";
const DOCKER_OPTS_ENVVAR: &'static str = "HAB_DOCKER_OPTS";
const DOCKER_SOCKET: &'static str = "/var/run/docker.sock";

pub fn start_docker_studio(_ui: &mut UI, mut args: Vec<OsString>) -> Result<()> {
    let docker_cmd = find_docker_cmd()?;

    if is_image_present(&docker_cmd) {
        debug!("Found Studio Docker image locally.");
    } else {
        debug!("Failed to find Studio Docker image locally.");
        pull_image(&docker_cmd)?;
    }

    let mnt_prefix = match is_serving_windows_containers(&docker_cmd) {
        true => "c:",
        false => "",
    };
    let mut volumes = vec![
        format!(
            "{}:{}{}",
            env::current_dir().unwrap().to_string_lossy(),
            mnt_prefix,
            "/src"
        ),
        format!(
            "{}:{}/{}",
            default_cache_key_path(None).to_string_lossy(),
            mnt_prefix,
            CACHE_KEY_PATH
        ),
    ];
    if let Ok(cache_artifact_path) = henv::var(ARTIFACT_PATH_ENVVAR) {
        volumes.push(format!(
            "{}:{}/{}",
            cache_artifact_path,
            mnt_prefix,
            CACHE_ARTIFACT_PATH
        ));
    }
    if !is_serving_windows_containers(&docker_cmd) &&
        (Path::new(DOCKER_SOCKET).exists() || cfg!(target_os = "windows"))
    {
        volumes.push(format!("{}:{}", DOCKER_SOCKET, DOCKER_SOCKET));
    }

    let env_vars = vec![
        "DEBUG",
        "HAB_AUTH_TOKEN",
        "HAB_BLDR_URL",
        "HAB_BLDR_CHANNEL",
        "HAB_ORIGIN",
        "HAB_ORIGIN_KEYS",
        "HAB_STUDIO_BACKLINE_PKG",
        "HAB_STUDIO_NOSTUDIORC",
        "HAB_STUDIO_SUP",
        "HAB_UPDATE_STRATEGY_FREQUENCY_MS",
        "http_proxy",
        "https_proxy",
        "RUST_LOG",
    ];
    // We need to strip out the -D if it exists to avoid
    // it getting passed to the sup on entering the studio
    let to_cull = OsString::from("-D");
    if let Some(index) = args.iter().position(|x| *x == to_cull) {
        args.remove(index);
    }

    check_mounts(&docker_cmd, volumes.iter())?;
    run_container(docker_cmd, args, volumes.iter(), env_vars.iter())
}


fn find_docker_cmd() -> Result<PathBuf> {
    let docker_cmd = henv::var(DOCKER_CMD_ENVVAR).unwrap_or(DOCKER_CMD.to_string());

    match find_command(&docker_cmd) {
        Some(docker_abs_path) => Ok(docker_abs_path),
        None => Err(Error::ExecCommandNotFound(docker_cmd.into())),
    }
}

fn is_image_present(docker_cmd: &Path) -> bool {
    let mut cmd = Command::new(docker_cmd);
    cmd.arg("images").arg(&image_identifier(docker_cmd)).arg(
        "-q",
    );
    debug!("Running command: {:?}", cmd);
    let result = cmd.output().expect("Docker command failed to spawn");

    !String::from_utf8_lossy(&result.stdout).as_ref().is_empty()
}

fn is_serving_windows_containers(docker_cmd: &Path) -> bool {
    let mut cmd = Command::new(docker_cmd);
    cmd.arg("version").arg("--format='{{.Server.Os}}'");
    debug!("Running command: {:?}", cmd);
    let result = cmd.output().expect("Docker command failed to spawn");
    String::from_utf8_lossy(&result.stdout).contains("windows")
}

fn pull_image(docker_cmd: &Path) -> Result<()> {
    let image = image_identifier(docker_cmd);
    let mut cmd = Command::new(docker_cmd);
    cmd.arg("pull")
        .arg(&image)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    debug!("Running command: {:?}", cmd);
    let result = cmd.spawn()
        .expect("Docker command failed to spawn")
        .wait_with_output()
        .expect("Failed to wait on child process");

    if result.status.success() {
        debug!("Docker image '{}' is present locally.", &image);
    } else {
        debug!(
            "Pulling Docker image '{}' failed with exit code: {:?}",
            &image,
            result.status
        );

        let err_output = String::from_utf8_lossy(&result.stderr);

        if err_output.contains("image") && err_output.contains("not found") {
            return Err(Error::DockerImageNotFound(
                image_identifier(docker_cmd).to_string(),
            ));
        } else if err_output.contains("Cannot connect to the Docker daemon") {
            return Err(Error::DockerDaemonDown);
        } else {
            return Err(Error::DockerNetworkDown(
                image_identifier(docker_cmd).to_string(),
            ));
        }
    }

    Ok(())
}

/// Checks whether or not the volume mounts are working.
///
/// We need to ensure that filesystem sharing has been enabled, otherwise the user will be
/// greeted with a horrible error message that's difficult to make sense of. To mitigate this,
/// we check the studio version. This will cause Docker to go through the mounting steps, so we
/// can watch stderr for failure, but has the advantage of not requiring a TTY.
fn check_mounts<I, S>(docker_cmd: &Path, volumes: I) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd_args: Vec<OsString> = vec!["run".into(), "--rm".into()];
    for vol in volumes {
        cmd_args.push("--volume".into());
        cmd_args.push(vol.as_ref().into());
    }

    if !is_serving_windows_containers(docker_cmd) {
        cmd_args.push("--privileged".into());
    }
    cmd_args.push(image_identifier(docker_cmd).into());
    cmd_args.push("-V".into());
    let version_output = Command::new(docker_cmd).args(&cmd_args).output().expect(
        "docker failed to start",
    );

    let stderr = String::from_utf8(version_output.stderr).unwrap();
    if !stderr.is_empty() &&
        (stderr.as_str().contains("Mounts denied") ||
             stderr.as_str().contains("drive is not shared"))
    {
        return Err(Error::DockerFileSharingNotEnabled);
    }
    Ok(())
}

fn run_container<I, J, S, T>(
    docker_cmd: PathBuf,
    args: Vec<OsString>,
    volumes: I,
    env_vars: J,
) -> Result<()>
where
    I: IntoIterator<Item = S>,
    J: IntoIterator<Item = T>,
    S: AsRef<OsStr>,
    T: AsRef<str>,
{
    let mut cmd_args: Vec<OsString> = vec!["run".into(), "--rm".into()];
    if !is_serving_windows_containers(&docker_cmd) {
        cmd_args.push("--privileged".into());
    }
    match args.first().map(|f| f.to_str().unwrap_or_default()) {
        Some("build") => {}
        _ => {
            cmd_args.push("--tty".into());
            cmd_args.push("--interactive".into());
        }
    }
    if let Ok(opts) = henv::var(DOCKER_OPTS_ENVVAR) {
        let opts = opts.split(" ")
                .map(|v| v.into())
                // Ensure we're not passing something like `--tty` again here.
                .filter(|v| !cmd_args.contains(v))
                .collect::<Vec<_>>();
        if !opts.is_empty() {
            debug!(
                "Adding extra Docker options from {} = {:?}",
                DOCKER_OPTS_ENVVAR,
                opts
            );
            cmd_args.extend_from_slice(opts.as_slice());
        }
    }
    for var in env_vars {
        if let Ok(val) = henv::var(var.as_ref()) {
            debug!("Setting container env var: {:?}='{}'", var.as_ref(), val);
            cmd_args.push("--env".into());
            cmd_args.push(format!("{}={}", var.as_ref(), val).into());
        }
    }
    for vol in volumes {
        cmd_args.push("--volume".into());
        cmd_args.push(vol.as_ref().into());
    }
    cmd_args.push(image_identifier(&docker_cmd).into());
    cmd_args.extend_from_slice(args.as_slice());
    if is_serving_windows_containers(&docker_cmd) {
        cmd_args.push("-w".into());
        cmd_args.push("-n".into());
        cmd_args.push("-o".into());
        cmd_args.push("c:/".into());
    }
    unset_proxy_env_vars();
    Ok(process::become_command(docker_cmd, cmd_args)?)
}

fn unset_proxy_env_vars() {
    for var in vec!["http_proxy", "https_proxy"] {
        if let Ok(_) = henv::var(var) {
            debug!("Unsetting process environment variable '{}'", var);
            env::remove_var(var);
        }
    }
}

/// Returns the Docker Studio image with tag for the desired version which corresponds to the
/// same version (minus release) as this program.
fn image_identifier(docker_cmd: &Path) -> String {
    let version: Vec<&str> = VERSION.split("/").collect();
    let img = match is_serving_windows_containers(docker_cmd) {
        true => DOCKER_WINDOWS_IMAGE,
        false => DOCKER_IMAGE,
    };
    henv::var(DOCKER_IMAGE_ENVVAR).unwrap_or(format!("{}:{}", img, version[0]))
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::{find_docker_cmd, image_identifier, DOCKER_IMAGE};
    use VERSION;

    #[test]
    fn retrieve_image_identifier() {
        let docker_cmd = find_docker_cmd().unwrap();
        assert_eq!(
            image_identifier(&docker_cmd),
            format!("{}:{}", DOCKER_IMAGE, VERSION)
        );
    }
}
