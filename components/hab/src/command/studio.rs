// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
use std::ffi::OsString;

use common::ui::UI;
use hcore::crypto::CACHE_KEY_PATH_ENV_VAR;
use hcore::env as henv;
use hcore::fs::CACHE_KEY_PATH;
use hcore::os::users;

use config;
use error::Result;

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
    try!(inner::rerun_with_sudo_if_needed(ui));

    // If the `$HAB_ORIGIN` environment variable is not present, then see if a default is set in
    // the CLI config. If so, set it as the `$HAB_ORIGIN` environment variable for the `hab-studio`
    // or `docker` execv call.
    if henv::var("HAB_ORIGIN").is_err() {
        let config = try!(config::load_with_sudo_user());
        if let Some(default_origin) = config.origin {
            debug!("Setting default origin {} via CLI config", &default_origin);
            env::set_var("HAB_ORIGIN", default_origin);
        }
    }

    // If the `$HAB_CACHE_KEY_PATH` environment variable is not present, check if we are running
    // under a `sudo` invocation. If so, determine the non-root user that issued the command in
    // order to set their key cache location in the environment variable. This is done so that the
    // `hab-studio` command will find the correct key cache or so that the correct directory will
    // be volume mounted when used with Docker.
    if henv::var(CACHE_KEY_PATH_ENV_VAR).is_err() {
        if let Some(sudo_user) = henv::sudo_user() {
            if let Some(home) = users::get_home_for_user(&sudo_user) {
                let cache_key_path = home.join(format!(".{}", CACHE_KEY_PATH));
                debug!("Setting cache_key_path for SUDO_USER={} to: {}",
                       &sudo_user,
                       cache_key_path.display());
                env::set_var(CACHE_KEY_PATH_ENV_VAR, cache_key_path);
                // Prevent any inner `hab` invocations from triggering similar logic: we will be
                // operating in the context `hab-studio` which is running with rootlike privileges.
                env::remove_var("SUDO_USER");
            }
        }
    }

    inner::start(ui, args)
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

    const STUDIO_CMD: &'static str = "hab-studio";
    const STUDIO_CMD_ENVVAR: &'static str = "HAB_STUDIO_BINARY";
    const STUDIO_PACKAGE_IDENT: &'static str = "core/hab-studio";
    const SUDO_CMD: &'static str = "sudo";

    pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let command = match henv::var(STUDIO_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let ident = try!(PackageIdent::from_str(STUDIO_PACKAGE_IDENT));
                try!(exec::command_from_pkg(ui,
                                            STUDIO_CMD,
                                            &ident,
                                            &default_cache_key_path(None),
                                            0))
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            try!(process::become_command(cmd, args));
        } else {
            return Err(Error::ExecCommandNotFound(command.to_string_lossy().into_owned()));
        }
        Ok(())
    }

    pub fn rerun_with_sudo_if_needed(ui: &mut UI) -> Result<()> {
        // If I have root permissions, early return, we are done.
        if am_i_root() {
            return Ok(());
        }

        // Otherwise we will try to re-run this program using `sudo`
        match find_command(SUDO_CMD) {
            Some(sudo_prog) => {
                let mut args: Vec<OsString> =
                    vec!["-p".into(), "[sudo hab-studio] password for %u: ".into(), "-E".into()];
                args.append(&mut env::args_os().collect());
                Ok(try!(process::become_command(sudo_prog, args)))
            }
            None => {
                try!(ui.warn(format!("Could not find the `{}' command, is it in your PATH?",
                                     SUDO_CMD)));
                try!(ui.warn("Running Habitat Studio requires root or administrator privileges. \
                              Please retry this command as a super user or use a \
                              privilege-granting facility such as sudo."));
                try!(ui.br());
                Err(Error::RootRequired)
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use std::env;
    use std::ffi::OsString;
    use std::process::{Command, Stdio};

    use common::ui::UI;
    use hcore::crypto::default_cache_key_path;
    use hcore::env as henv;
    use hcore::fs::{CACHE_KEY_PATH, find_command};
    use hcore::os::process;

    use error::{Error, Result};
    use VERSION;

    const DOCKER_CMD: &'static str = "docker";
    const DOCKER_CMD_ENVVAR: &'static str = "HAB_DOCKER_BINARY";
    const DOCKER_IMAGE: &'static str = "habitat-docker-registry.bintray.io/studio";
    const DOCKER_IMAGE_ENVVAR: &'static str = "HAB_DOCKER_STUDIO_IMAGE";
    const DOCKER_OPTS: &'static str = "HAB_DOCKER_OPTS";

    pub fn start(_ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let docker = henv::var(DOCKER_CMD_ENVVAR).unwrap_or(DOCKER_CMD.to_string());

        let cmd = match find_command(&docker) {
            Some(cmd) => cmd,
            None => return Err(Error::ExecCommandNotFound(docker.to_string())),
        };

        let output = Command::new(&cmd)
            .arg("images")
            .arg(&image_identifier())
            .arg("-q")
            .output()
            .expect("docker failed to start");

        let stdout = String::from_utf8(output.stdout).unwrap();
        if stdout.is_empty() {
            debug!("Failed to find studio image locally.");

            let child = Command::new(&cmd)
                .arg("pull")
                .arg(&image_identifier())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("docker failed to start");

            let output = child.wait_with_output()
                .expect("failed to wait on child");

            if output.status.success() {
                debug!("Docker image is reachable. Proceeding with launching docker.");
            } else {
                debug!("Docker image is unreachable. Exit code = {:?}",
                       output.status);

                let err_output = String::from_utf8(output.stderr).unwrap();

                if err_output.contains("image") && err_output.contains("not found") {
                    return Err(Error::DockerImageNotFound(image_identifier().to_string()));
                } else if err_output.contains("Cannot connect to the Docker daemon") {
                    return Err(Error::DockerDaemonDown);
                } else {
                    return Err(Error::DockerNetworkDown(image_identifier().to_string()));
                }
            }
        } else {
            debug!("Found studio image locally.");
        }

        let mut cmd_args: Vec<OsString> = vec!["run".into(),
                                               "--rm".into(),
                                               "--tty".into(),
                                               "--interactive".into(),
                                               "--privileged".into()];

        // All the args already placed in `cmd_args` are things that we don't want to insert again.
        // Later args such as `--env` will overwrite any options (potentially) set mistakenly here.
        if let Ok(opts) = henv::var(DOCKER_OPTS) {
            let opts = opts.split(" ")
                .map(|v| v.into())
                // Ensure we're not passing something like `--tty` again here.
                .filter(|v| !cmd_args.contains(v))
                .collect::<Vec<_>>();
            debug!("Docker opts originating from DOCKER_OPTS = {:?}", opts);
            cmd_args.extend_from_slice(opts.as_slice());
        }

        let env_vars = vec!["HAB_DEPOT_URL", "HAB_ORIGIN", "http_proxy", "https_proxy"];
        for var in env_vars {
            if let Ok(val) = henv::var(var) {
                debug!("Propagating environment variable into container: {}={}",
                       var,
                       val);
                cmd_args.push("--env".into());
                cmd_args.push(format!("{}={}", var, val).into());
            }
        }

        cmd_args.push("--volume".into());
        cmd_args.push("/var/run/docker.sock:/var/run/docker.sock".into());
        cmd_args.push("--volume".into());
        cmd_args.push(format!("{}:/{}",
                              default_cache_key_path(None).to_string_lossy(),
                              CACHE_KEY_PATH)
            .into());
        cmd_args.push("--volume".into());
        cmd_args.push(format!("{}:/src", env::current_dir().unwrap().to_string_lossy()).into());

        cmd_args.push(image_identifier().into());
        cmd_args.extend_from_slice(args.as_slice());

        for var in vec!["http_proxy", "https_proxy"] {
            if let Ok(_) = henv::var(var) {
                debug!("Unsetting proxy environment variable '{}' before calling `{}'",
                       var,
                       docker);
                env::remove_var(var);
            }
        }

        debug!("Docker arguments = {:?}", cmd_args);
        Ok(try!(process::become_command(cmd, cmd_args)))
    }

    pub fn rerun_with_sudo_if_needed(_ui: &mut UI) -> Result<()> {
        // No sudo calls necessary here--we are calling `docker` commands instead
        Ok(())
    }

    /// Returns the Docker Studio image with tag for the desired version which corresponds to the
    /// same version (minus release) as this program.
    fn image_identifier() -> String {
        let version: Vec<&str> = VERSION.split("/").collect();
        henv::var(DOCKER_IMAGE_ENVVAR).unwrap_or(format!("{}:{}", DOCKER_IMAGE, version[0]))
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
