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
use std::fs;
use std::ffi::OsString;
use std::path::Path;

use common::ui::UI;
use hcore::crypto::CACHE_KEY_PATH_ENV_VAR;
use hcore::env as henv;
use hcore::fs::{CACHE_ARTIFACT_PATH, CACHE_KEY_PATH};
use hcore::os::{filesystem, users};

use config;
use error::Result;

const ARTIFACT_PATH_ENVVAR: &'static str = "ARTIFACT_PATH";
const STUDIO_CMD: &'static str = "hab-studio";
const STUDIO_CMD_ENVVAR: &'static str = "HAB_STUDIO_BINARY";
const STUDIO_PACKAGE_IDENT: &'static str = "core/hab-studio";

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
    inner::rerun_with_sudo_if_needed(ui)?;

    // If the `$HAB_ORIGIN` environment variable is not present, then see if a default is set in
    // the CLI config. If so, set it as the `$HAB_ORIGIN` environment variable for the `hab-studio`
    // or `docker` execv call.
    if henv::var("HAB_ORIGIN").is_err() {
        let config = config::load_with_sudo_user()?;
        if let Some(default_origin) = config.origin {
            debug!("Setting default origin {} via CLI config", &default_origin);
            env::set_var("HAB_ORIGIN", default_origin);
        }
    }

    // Check if we are running under a `sudo` invocation. If so, determine the non-root user that
    // issued the command in order to set some Studio-related environment variables. This is done
    // so that the `hab-studio` command will find the correct key cache, artifact cache, etc. and
    // so that the correct directores will be volume mounted when used with Docker.
    if let Some(sudo_user) = henv::sudo_user() {
        if let Some(home) = users::get_home_for_user(&sudo_user) {
            // If the `$HAB_CACHE_KEY_PATH` environment variable is not present, set it to the
            // non-root user's key cache
            if henv::var(CACHE_KEY_PATH_ENV_VAR).is_err() {
                let cache_key_path = home.join(format!(".{}", CACHE_KEY_PATH));
                debug!(
                    "Setting cache_key_path for SUDO_USER={} to: {}",
                    &sudo_user,
                    cache_key_path.display()
                );
                env::set_var(CACHE_KEY_PATH_ENV_VAR, cache_key_path);
            }
            // If the `$ARTIFACT_PATH` environment variable is not present, set it to the non-root
            // user's key cache
            if henv::var(ARTIFACT_PATH_ENVVAR).is_err() {
                let cache_artifact_path = home.join(format!(".{}", CACHE_ARTIFACT_PATH));
                try!(create_cache_artifact_path(
                    &cache_artifact_path,
                    Some(&sudo_user),
                ));
                debug!(
                    "Setting cache_artifact_path for SUDO_USER={} to: {}",
                    &sudo_user,
                    cache_artifact_path.display()
                );
                env::set_var(ARTIFACT_PATH_ENVVAR, cache_artifact_path);
            }
            // Prevent any inner `hab` invocations from triggering similar logic: we will be
            // operating in the context `hab-studio` which is running with root like privileges.
            env::remove_var("SUDO_USER");
        }
    } else {
        if let Some(user) = users::get_current_username() {
            if let Some(home) = users::get_home_for_user(&user) {
                if henv::var(ARTIFACT_PATH_ENVVAR).is_err() {
                    let cache_artifact_path = home.join(format!(".{}", CACHE_ARTIFACT_PATH));
                    try!(create_cache_artifact_path(&cache_artifact_path, None));
                    debug!(
                        "Setting cache_artifact_path at: {}",
                        cache_artifact_path.display()
                    );
                    env::set_var(ARTIFACT_PATH_ENVVAR, cache_artifact_path);
                }
            }
        }
    }

    inner::start(ui, args)
}

fn create_cache_artifact_path(path: &Path, sudo_user: Option<&str>) -> Result<()> {
    if path.is_dir() {
        Ok(())
    } else {
        match sudo_user {
            Some(sudo_user) => {
                debug!(
                    "Creating cache_artifact_path for SUDO_USER={} at: {}",
                    &sudo_user,
                    path.display()
                )
            }
            None => debug!("Creating cache_artifact_path at: {}", path.display()),
        };
        try!(fs::create_dir_all(&path));
        if let Some(sudo_user) = sudo_user {
            if let (Some(uid), Some(gid)) =
                (
                    users::get_uid_by_name(sudo_user),
                    users::get_primary_gid_for_user(sudo_user),
                )
            {
                debug!(
                    "Setting permissions of {} for SUDO_USER={} to: {}:{}",
                    path.display(),
                    &sudo_user,
                    uid,
                    gid
                );
                try!(filesystem::chown(path.to_string_lossy().as_ref(), uid, gid));
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod inner {
    use std::env;
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::str::FromStr;

    use common::ui::UI;
    use hcore::crypto::{init, default_cache_key_path};
    use hcore::env as henv;
    use hcore::fs::{am_i_root, find_command};
    use hcore::os::process;
    use hcore::package::PackageIdent;

    use error::{Error, Result};
    use exec;
    use VERSION;

    const SUDO_CMD: &'static str = "sudo";

    pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let version: Vec<&str> = VERSION.split("/").collect();
                let ident = PackageIdent::from_str(
                    &format!("{}/{}", super::STUDIO_PACKAGE_IDENT, version[0]),
                )?;
                exec::command_from_min_pkg(
                    ui,
                    super::STUDIO_CMD,
                    &ident,
                    &default_cache_key_path(None),
                    0,
                )?
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            Ok(process::become_command(cmd, args)?)
        } else {
            Err(Error::ExecCommandNotFound(command))
        }
    }

    pub fn rerun_with_sudo_if_needed(ui: &mut UI) -> Result<()> {
        // If I have root permissions, early return, we are done.
        if am_i_root() {
            return Ok(());
        }

        // Otherwise we will try to re-run this program using `sudo`
        match find_command(SUDO_CMD) {
            Some(sudo_prog) => {
                let mut args: Vec<OsString> = vec![
                    "-p".into(),
                    "[sudo hab-studio] password for %u: ".into(),
                    "-E".into(),
                ];
                args.append(&mut env::args_os().collect());
                Ok(process::become_command(sudo_prog, args)?)
            }
            None => {
                ui.warn(format!(
                    "Could not find the `{}' command, is it in your PATH?",
                    SUDO_CMD
                ))?;
                ui.warn(
                    "Running Habitat Studio requires root or administrator privileges. \
                              Please retry this command as a super user or use a \
                              privilege-granting facility such as sudo.",
                )?;
                ui.br()?;
                Err(Error::RootRequired)
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use std::env;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use std::str::FromStr;

    use common::ui::UI;
    use hcore::crypto::{init, default_cache_key_path};
    use hcore::env as henv;
    use hcore::fs::{CACHE_ARTIFACT_PATH, CACHE_KEY_PATH, find_command};
    use hcore::os::process;
    use hcore::package::PackageIdent;

    use error::{Error, Result};
    use exec;
    use VERSION;

    const DOCKER_CMD: &'static str = "docker";
    const DOCKER_CMD_ENVVAR: &'static str = "HAB_DOCKER_BINARY";
    const DOCKER_IMAGE: &'static str = "habitat-docker-registry.bintray.io/studio";
    const DOCKER_IMAGE_ENVVAR: &'static str = "HAB_DOCKER_STUDIO_IMAGE";
    const DOCKER_OPTS_ENVVAR: &'static str = "HAB_DOCKER_OPTS";
    const DOCKER_SOCKET: &'static str = "/var/run/docker.sock";

    pub fn start(_ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        if is_windows_studio(&args) {
            start_windows_studio(_ui, args)
        } else {
            start_docker_studio(_ui, args)
        }
    }

    pub fn start_windows_studio(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let version: Vec<&str> = VERSION.split("/").collect();
                let ident = PackageIdent::from_str(
                    &format!("{}/{}", super::STUDIO_PACKAGE_IDENT, version[0]),
                )?;
                exec::command_from_min_pkg(
                    ui,
                    super::STUDIO_CMD,
                    &ident,
                    &default_cache_key_path(None),
                    0,
                )?
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            process::become_command(cmd, args)?;
        } else {
            return Err(Error::ExecCommandNotFound(command));
        }
        Ok(())
    }

    pub fn start_docker_studio(_ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let docker_cmd = find_docker_cmd()?;

        if is_image_present(&docker_cmd) {
            debug!("Found Studio Docker image locally.");
        } else {
            debug!("Failed to find Studio Docker image locally.");
            pull_image(&docker_cmd)?;
        }

        let mut volumes = vec![
            format!("{}:/src", env::current_dir().unwrap().to_string_lossy()),
            format!(
                "{}:/{}",
                default_cache_key_path(None).to_string_lossy(),
                CACHE_KEY_PATH
            ),
        ];
        if let Ok(cache_artifact_path) = henv::var(super::ARTIFACT_PATH_ENVVAR) {
            volumes.push(format!("{}:/{}", cache_artifact_path, CACHE_ARTIFACT_PATH));
        }
        if Path::new(DOCKER_SOCKET).exists() {
            volumes.push(format!("{}:{}", DOCKER_SOCKET, DOCKER_SOCKET));
        }

        let env_vars = vec![
            "HAB_DEPOT_URL",
            "HAB_DEPOT_CHANNEL",
            "HAB_ORIGIN",
            "HAB_STUDIO_SUP",
            "HAB_UPDATE_STRATEGY_FREQUENCY_MS",
            "http_proxy",
            "https_proxy",
        ];

        check_mounts(&docker_cmd, volumes.iter())?;
        run_container(docker_cmd, args, volumes.iter(), env_vars.iter())
    }

    pub fn rerun_with_sudo_if_needed(_ui: &mut UI) -> Result<()> {
        // No sudo calls necessary here--we are calling `docker` commands instead
        Ok(())
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
        cmd.arg("images").arg(&image_identifier()).arg("-q");
        debug!("Running command: {:?}", cmd);
        let result = cmd.output().expect("Docker command failed to spawn");

        !String::from_utf8_lossy(&result.stdout).as_ref().is_empty()
    }

    fn pull_image(docker_cmd: &Path) -> Result<()> {
        let image = image_identifier();
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
                return Err(Error::DockerImageNotFound(image_identifier().to_string()));
            } else if err_output.contains("Cannot connect to the Docker daemon") {
                return Err(Error::DockerDaemonDown);
            } else {
                return Err(Error::DockerNetworkDown(image_identifier().to_string()));
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
        let mut volume_args: Vec<OsString> = Vec::new();
        for vol in volumes {
            volume_args.push("--volume".into());
            volume_args.push(vol.as_ref().into());
        }

        let version_output = Command::new(docker_cmd)
            .arg("run")
            .arg("--rm")
            .arg("--privileged")
            .args(volume_args)
            .arg(image_identifier())
            .arg("-V")
            .output()
            .expect("docker failed to start");

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
        let mut cmd_args: Vec<OsString> = vec![
            "run".into(),
            "--rm".into(),
            "--tty".into(),
            "--interactive".into(),
            "--privileged".into(),
        ];
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
        cmd_args.push(image_identifier().into());
        cmd_args.extend_from_slice(args.as_slice());

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
    fn image_identifier() -> String {
        let version: Vec<&str> = VERSION.split("/").collect();
        henv::var(DOCKER_IMAGE_ENVVAR).unwrap_or(format!("{}:{}", DOCKER_IMAGE, version[0]))
    }

    fn is_windows_studio(args: &Vec<OsString>) -> bool {
        if cfg!(not(target_os = "windows")) {
            return false;
        }

        for arg in args.iter() {
            let str_arg = arg.to_string_lossy().to_lowercase();
            if str_arg == String::from("--windows") || str_arg == String::from("-w") {
                return true;
            }
        }

        return false;
    }

    #[cfg(test)]
    mod tests {
        use super::{image_identifier, DOCKER_IMAGE};
        use VERSION;

        #[test]
        fn retrieve_image_identifier() {
            assert_eq!(image_identifier(), format!("{}:{}", DOCKER_IMAGE, VERSION));
        }
    }
}
