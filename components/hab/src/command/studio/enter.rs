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

use crate::{common::{cli::env_var,
                     ui::UI},
            config,
            error::Result,
            hcore::{crypto::CACHE_KEY_PATH_ENV_VAR,
                    env as henv,
                    fs}};
use std::{env,
          ffi::OsString,
          fs as stdfs,
          path::{Path,
                 PathBuf}};

const STUDIO_CMD: &str = "hab-studio";
const STUDIO_CMD_ENVVAR: &str = "HAB_STUDIO_BINARY";
const STUDIO_PACKAGE_IDENT: &str = "core/hab-studio";

pub fn start(ui: &mut UI, cache_key_path: &Path, args: &[OsString]) -> Result<()> {
    if henv::var(env_var::ORIGIN).is_err() {
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

    let artifact_path = match henv::var(env_var::ARTIFACT_PATH) {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            let path = fs::cache_artifact_path(None::<&str>);
            debug!("Setting {}={}", env_var::ARTIFACT_PATH, path.display());
            env::set_var(env_var::ARTIFACT_PATH, &path);
            path
        }
    };
    if !artifact_path.is_dir() {
        debug!("Creating artifact_path at: {}", artifact_path.display());
        stdfs::create_dir_all(&artifact_path)?;
    }

    inner::start(ui, cache_key_path, args)
}

#[cfg(target_os = "linux")]
mod inner {
    use crate::{command::studio::docker,
                common::ui::{UIWriter,
                             UI},
                error::{Error,
                        Result},
                exec,
                hcore::{crypto::init,
                        env as henv,
                        fs::{am_i_root,
                             find_command},
                        os::process,
                        package::PackageIdent,
                        users::linux as group},
                VERSION};
    use std::{env,
              ffi::OsString,
              path::{Path,
                     PathBuf},
              str::FromStr};

    const SUDO_CMD: &str = "sudo";

    pub fn start(ui: &mut UI, cache_key_path: &Path, args: &[OsString]) -> Result<()> {
        rerun_with_sudo_if_needed(ui, &args)?;
        if is_docker_studio(&args) {
            docker::start_docker_studio(cache_key_path, args)
        } else {
            let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
                Ok(command) => PathBuf::from(command),
                Err(_) => {
                    init();
                    let version: Vec<&str> = VERSION.split('/').collect();
                    let ident = PackageIdent::from_str(&format!("{}/{}",
                                                                super::STUDIO_PACKAGE_IDENT,
                                                                version[0]))?;
                    exec::command_from_min_pkg(ui, super::STUDIO_CMD, &ident)?
                }
            };

            if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
                process::become_command(cmd, args)?;
                Ok(())
            } else {
                Err(Error::ExecCommandNotFound(command))
            }
        }
    }

    fn is_docker_studio(args: &[OsString]) -> bool {
        if cfg!(not(target_os = "linux")) {
            return false;
        }

        for arg in args {
            let str_arg = arg.to_string_lossy();
            if str_arg == "-D" {
                return true;
            }
        }

        false
    }

    fn has_docker_group() -> bool {
        let current_user = group::get_current_username().unwrap();
        let docker_members = group::get_members_by_groupname("docker");
        docker_members.map_or(false, |d| d.contains(&current_user))
    }

    fn rerun_with_sudo_if_needed(ui: &mut UI, args: &[OsString]) -> Result<()> {
        // If I have root permissions or if I am executing a docker studio
        // and have the appropriate group - early return, we are done.
        if am_i_root() || (is_docker_studio(args) && has_docker_group()) {
            return Ok(());
        }

        // Otherwise we will try to re-run this program using `sudo`
        match find_command(SUDO_CMD) {
            Some(sudo_prog) => {
                let mut args: Vec<OsString> = vec!["-p".into(),
                                                   "[sudo hab-studio] password for %u: ".into(),
                                                   "-E".into(),];
                args.append(&mut env::args_os().collect());
                process::become_command(sudo_prog, &args)?;
                Ok(())
            }
            None => {
                ui.warn(format!("Could not find the `{}' command, is it in your PATH?",
                                SUDO_CMD))?;
                ui.warn("Running Habitat Studio requires root or administrator privileges. \
                         Please retry this command as a super user or use a privilege-granting \
                         facility such as sudo.")?;
                ui.br()?;
                Err(Error::RootRequired)
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod inner {
    use crate::{command::studio::docker,
                common::ui::UI,
                error::{Error,
                        Result},
                exec,
                hcore::{crypto::init,
                        env as henv,
                        fs::find_command,
                        os::process,
                        package::PackageIdent},
                VERSION};
    use std::{ffi::OsString,
              path::{Path,
                     PathBuf},
              str::FromStr};

    pub fn start(_ui: &mut UI, cache_key_path: &Path, args: &[OsString]) -> Result<()> {
        if is_windows_studio(&args) {
            start_windows_studio(_ui, args)
        } else {
            docker::start_docker_studio(cache_key_path, args)
        }
    }

    pub fn start_windows_studio(ui: &mut UI, args: &[OsString]) -> Result<()> {
        let command = match henv::var(super::STUDIO_CMD_ENVVAR) {
            Ok(command) => PathBuf::from(command),
            Err(_) => {
                init();
                let version: Vec<&str> = VERSION.split('/').collect();
                let ident = PackageIdent::from_str(&format!("{}/{}",
                                                            super::STUDIO_PACKAGE_IDENT,
                                                            version[0]))?;
                exec::command_from_min_pkg(ui, super::STUDIO_CMD, &ident)?
            }
        };

        if let Some(cmd) = find_command(command.to_string_lossy().as_ref()) {
            process::become_command(cmd, args)?;
        } else {
            return Err(Error::ExecCommandNotFound(command));
        }
        Ok(())
    }

    fn is_windows_studio(args: &[OsString]) -> bool {
        if cfg!(not(target_os = "windows")) {
            return false;
        }

        for arg in args {
            let str_arg = arg.to_string_lossy();
            if str_arg == "-D" {
                return false;
            }
        }

        true
    }
}
