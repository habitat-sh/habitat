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
use VERSION;

pub const DOCKER_IMAGE: &'static str = "habitat-docker-registry.bintray.io/studio";
const DOCKER_CMD: &'static str = "docker";
const DOCKER_CMD_ENVVAR: &'static str = "HAB_DOCKER_BINARY";
const DOCKER_IMAGE_ENVVAR: &'static str = "HAB_DOCKER_STUDIO_IMAGE";

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
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

/// Retrieves the
pub fn image_identifier() -> String {
    let version: Vec<&str> = VERSION.split("/").collect();
    henv::var(DOCKER_IMAGE_ENVVAR).unwrap_or(format!("{}:{}", DOCKER_IMAGE, version[0]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use VERSION;

    #[test]
    fn retrieve_image_identifier() {
        assert_eq!(image_identifier(), format!("{}:{}", DOCKER_IMAGE, VERSION));
    }
}

#[cfg(target_os = "linux")]
mod inner {
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::str::FromStr;

    use common::ui::UI;
    use hcore::crypto::{init, default_cache_key_path};
    use hcore::env as henv;
    use hcore::fs::find_command;
    use hcore::package::PackageIdent;

    use error::{Error, Result};
    use exec;

    const STUDIO_CMD: &'static str = "hab-studio";
    const STUDIO_CMD_ENVVAR: &'static str = "HAB_STUDIO_BINARY";
    const STUDIO_PACKAGE_IDENT: &'static str = "core/hab-studio";

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
            try!(exec::exec_command(cmd, args));
        } else {
            return Err(Error::ExecCommandNotFound(command.to_string_lossy().into_owned()));
        }
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use std::env;
    use std::ffi::OsString;

    use common::ui::UI;
    use hcore::crypto::default_cache_key_path;
    use hcore::env as henv;
    use hcore::fs::{CACHE_KEY_PATH, find_command};

    use error::{Error, Result};
    use exec;
    use super::{DOCKER_CMD, DOCKER_CMD_ENVVAR, image_identifier};

    pub fn start(_ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        let docker = henv::var(DOCKER_CMD_ENVVAR).unwrap_or(DOCKER_CMD.to_string());

        let cmd = match find_command(&docker) {
            Some(cmd) => cmd,
            None => return Err(Error::ExecCommandNotFound(docker.to_string())),
        };

        let mut cmd_args: Vec<OsString> = vec!["run".into(),
                                               "--rm".into(),
                                               "--tty".into(),
                                               "--interactive".into(),
                                               "--privileged".into()];
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

        exec::exec_command(cmd, cmd_args)
    }
}
