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
use std::path::{Path, PathBuf};
use std::process::Command;

use users;
use users::os::unix::GroupExt;

use {Error, Result};

pub const IP_PKG: &'static str = "core/iproute2";
pub const DEBUG_ENVVAR: &'static str = "RUST_LOG";

pub fn find_command<P: AsRef<Path>>(command: P) -> Result<PathBuf> {
    match env::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = PathBuf::from(&path).join(command.as_ref());
                if candidate.is_file() {
                    return Ok(candidate);
                }
            }
            Err(Error::ProgramNotFound(
                command.as_ref().to_string_lossy().into(),
            ))
        }
        None => Err(Error::ProgramNotFound(
            command.as_ref().to_string_lossy().into(),
        )),
    }
}

pub fn proc_exe() -> Result<PathBuf> {
    Ok(Path::new("/proc/self/exe").canonicalize()?)
}

pub fn run_cmd(mut command: Command) -> Result<()> {
    debug!("running, command={:?}", command);
    let exit_status = command.spawn()?.wait()?;
    if exit_status.success() {
        Ok(())
    } else {
        Err(Error::Command(exit_status))
    }
}

pub fn hab_cmd() -> Result<Command> {
    let mut command = Command::new(find_command("hab")?);
    command.env_remove(DEBUG_ENVVAR);

    Ok(command)
}

pub fn ip_cmd() -> Result<Command> {
    let mut command = hab_cmd()?;
    command.arg("pkg");
    command.arg("exec");
    command.arg(IP_PKG);
    command.arg("ip");

    Ok(command)
}

pub fn check_required_packages(pkgs: &[&str]) -> Result<()> {
    for ident in pkgs.iter() {
        debug!("checking for package, ident={}", ident);
        let mut command = hab_cmd()?;
        command.arg("pkg");
        command.arg("path");
        command.arg(ident);

        debug!("running, command={:?}", &command);
        let output = command.output()?;
        if !output.status.success() {
            return Err(Error::PackageNotFound(String::from(*ident)));
        }
    }

    Ok(())
}
pub fn check_user_group_membership(username: &str) -> Result<()> {
    let user = String::from(username);
    for grp in vec!["tty"].iter() {
        let user_group = users::get_group_by_name(grp).ok_or(Error::GroupNotFound(
            String::from(*grp),
        ))?;
        if !user_group.members().contains(&user) {
            return Err(Error::UserNotInGroup(user, String::from(*grp)));
        }
    }
    Ok(())
}
