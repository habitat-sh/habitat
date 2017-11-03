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

use std::ffi::OsStr;
use std::path::Path;
use std::process::{self, Command};

use unshare::{self, Namespace};

use FsRoot;
use namespace;
use user;
use util;

use {Error, Result};

pub fn run(
    fs_root: FsRoot,
    cmd: &OsStr,
    args: Vec<&OsStr>,
    namespaces: Option<(&Path, &Path)>,
) -> Result<()> {
    check_required_packages()?;
    util::check_user_group_membership(&user::my_username()?)?;
    if let Some((userns, netns)) = namespaces {
        join_network_namespaces(userns, netns)?;
    }
    let new_userns = namespaces == None;
    let mut command = unshare_command(fs_root.as_ref(), cmd, args, new_userns)?;
    debug!("running, command={:?}", command);
    let exit_status = command.spawn()?.wait()?;
    fs_root.finish()?;
    process::exit(exit_status.code().unwrap_or(127));
}

fn check_required_packages() -> Result<()> {
    for ident in vec!["core/hab", "core/busybox-static"].iter() {
        debug!("checking for package, ident={}", ident);
        let mut command = Command::new("hab");
        command.args(&["pkg", "path", ident]);
        debug!("running, command={:?}", &command);
        let output = command.output()?;
        if !output.status.success() {
            return Err(Error::PackageNotFound(String::from(*ident)));
        }
    }
    Ok(())
}

fn join_network_namespaces(userns: &Path, netns: &Path) -> Result<()> {
    namespace::setns_user(userns)?;
    namespace::setns_network(netns)?;
    Ok(())
}

fn unshare_command(
    rootfs: &Path,
    cmd: &OsStr,
    args: Vec<&OsStr>,
    new_userns: bool,
) -> Result<unshare::Command> {
    let program = util::proc_exe()?;
    let mut namespaces = vec![
        Namespace::Mount,
        Namespace::Uts,
        Namespace::Ipc,
        Namespace::Pid,
    ];
    if new_userns {
        namespaces.push(Namespace::User);
    }

    let mut command = unshare::Command::new(program);
    command.arg("nsrun");
    command.arg(rootfs);
    command.arg(cmd);
    command.args(&args);
    command.unshare(namespaces.iter().cloned());
    if new_userns {
        command.set_id_maps(
            namespace::uid_maps(&user::my_username()?)?,
            namespace::gid_maps(&user::my_groupname()?)?,
        );
        command.set_id_map_commands(
            util::find_command("newuidmap")?,
            util::find_command("newgidmap")?,
        );
    }
    command.uid(0);
    command.gid(0);

    Ok(command)
}
