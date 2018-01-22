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
use std::fs as stdfs;
use std::ffi::OsString;
use std::path::PathBuf;

use common::ui::UI;
use hcore::crypto::CACHE_KEY_PATH_ENV_VAR;
use hcore::env as henv;
use hcore::fs;

use config;
use error::Result;

pub const ARTIFACT_PATH_ENVVAR: &'static str = "ARTIFACT_PATH";

const ORIGIN_ENVVAR: &'static str = "HAB_ORIGIN";
const STUDIO_CMD: &'static str = "hab-studio";
const STUDIO_CMD_ENVVAR: &'static str = "HAB_STUDIO_BINARY";
const STUDIO_PACKAGE_IDENT: &'static str = "core/hab-studio";

pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
    if henv::var(ORIGIN_ENVVAR).is_err() {
        let config = config::load()?;
        if let Some(default_origin) = config.origin {
            debug!("Setting default origin {} via CLI config", &default_origin);
            env::set_var("HAB_ORIGIN", default_origin);
        }
    }

    if henv::var(CACHE_KEY_PATH_ENV_VAR).is_err() {
        let path = fs::cache_key_path(None::<&str>);
        debug!("Setting {}={}", CACHE_KEY_PATH_ENV_VAR, path.display());
        env::set_var(CACHE_KEY_PATH_ENV_VAR, &path);
    };

    let artifact_path = match henv::var(ARTIFACT_PATH_ENVVAR) {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            let path = fs::cache_artifact_path(None::<&str>);
            debug!("Setting {}={}", ARTIFACT_PATH_ENVVAR, path.display());
            env::set_var(ARTIFACT_PATH_ENVVAR, &path);
            path
        }
    };
    if !artifact_path.is_dir() {
        debug!("Creating artifact_path at: {}", artifact_path.display());
        stdfs::create_dir_all(&artifact_path)?;
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
    use VERSION;

    use command::studio::docker;

    const SUDO_CMD: &'static str = "sudo";

    pub fn start(ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        rerun_with_sudo_if_needed(ui)?;
        if is_docker_studio(&args) {
            docker::start_docker_studio(ui, args)
        } else {
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
    }

    fn is_docker_studio(args: &Vec<OsString>) -> bool {
        if cfg!(not(target_os = "linux")) {
            return false;
        }

        for arg in args.iter() {
            let str_arg = arg.to_string_lossy();
            if str_arg == String::from("-D") {
                return true;
            }
        }

        return false;
    }

    fn rerun_with_sudo_if_needed(ui: &mut UI) -> Result<()> {
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
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::str::FromStr;

    use common::ui::UI;
    use hcore::crypto::{init, default_cache_key_path};
    use hcore::env as henv;
    use hcore::fs::find_command;
    use hcore::os::process;
    use hcore::package::PackageIdent;

    use error::{Error, Result};
    use exec;
    use VERSION;
    use command::studio::docker;



    pub fn start(_ui: &mut UI, args: Vec<OsString>) -> Result<()> {
        if is_windows_studio(&args) {
            start_windows_studio(_ui, args)
        } else {
            docker::start_docker_studio(_ui, args)
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
}
