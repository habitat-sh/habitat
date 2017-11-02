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
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{self, Command};

use FsRoot;
use namespace;
use unshare::{self, Namespace};
use users;
use users::os::unix::GroupExt;

use {Error, Result};

const MIN_SUB_RANGE: u32 = 65536;

pub fn run(
    fs_root: FsRoot,
    cmd: &OsStr,
    args: Vec<&OsStr>,
    namespaces: Option<(&Path, &Path)>,
) -> Result<()> {
    check_required_packages()?;
    check_user_group_membership()?;
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

fn check_user_group_membership() -> Result<()> {
    let user = username()?;
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
    let program = proc_exe()?;
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
        command.set_id_maps(uid_maps()?, gid_maps()?);
        command.set_id_map_commands(find_command("newuidmap")?, find_command("newgidmap")?);
    }
    command.uid(0);
    command.gid(0);

    Ok(command)
}

fn uid_maps() -> Result<Vec<unshare::UidMap>> {
    let (start_uid, range) = sub_range(&username()?, Path::new("/etc/subuid"))?;
    if range < MIN_SUB_RANGE {
        return Err(Error::SubUidRangeTooSmall(range, MIN_SUB_RANGE));
    }

    Ok(vec![
        // Maps the outside user to the root user
        unshare::UidMap {
            inside_uid: 0,
            outside_uid: uid(),
            count: 1,
        },
        // Maps the remaining 1000 uids to externally unmappable uids
        unshare::UidMap {
            inside_uid: 1,
            outside_uid: start_uid + 1,
            count: 999,
        },
        // Maps the nobody user to an externally unmappable uid
        unshare::UidMap {
            inside_uid: 65534,
            outside_uid: start_uid + 1000,
            count: 1,
        },
    ])
}

fn gid_maps() -> Result<Vec<unshare::GidMap>> {
    let (start_gid, range) = sub_range(&groupname()?, Path::new("/etc/subgid"))?;
    if range < MIN_SUB_RANGE {
        return Err(Error::SubGidRangeTooSmall(range, MIN_SUB_RANGE));
    }

    Ok(vec![
        // Maps the outside group to the root group
        unshare::GidMap {
            inside_gid: 0,
            outside_gid: gid(),
            count: 1,
        },
        // Maps the remaining 1000 gids to externally unmappable gids
        unshare::GidMap {
            inside_gid: 1,
            outside_gid: start_gid + 1,
            count: 999,
        },
        // Maps the nogroup user to an externally unmappable gid
        unshare::GidMap {
            inside_gid: 65534,
            outside_gid: start_gid + 1000,
            count: 1,
        },
    ])
}
fn proc_exe() -> Result<PathBuf> {
    Ok(Path::new("/proc/self/exe").canonicalize()?)
}

fn username() -> Result<String> {
    users::get_effective_username().ok_or(Error::UsernameNotFound)
}

fn uid() -> u32 {
    users::get_effective_uid()
}

fn groupname() -> Result<String> {
    users::get_effective_groupname().ok_or(Error::GroupnameNotFound)
}

fn gid() -> u32 {
    users::get_effective_gid()
}

fn find_command<P: AsRef<Path>>(command: P) -> Result<PathBuf> {
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

fn sub_range(entry: &str, path: &Path) -> Result<(u32, u32)> {
    if !path.exists() {
        return Err(Error::FileNotFound(path.to_string_lossy().into()));
    }
    let line = {
        let file = File::open(path)?;
        let file = BufReader::new(file);
        match file.lines().map(|l| l.unwrap()).find(|ref line| {
            line.split(":").next().unwrap_or("") == entry
        }) {
            Some(line) => line,
            None => {
                return Err(Error::FileEntryNotFound(
                    String::from(entry),
                    path.to_string_lossy().into(),
                ))
            }
        }
    };
    let start_id = line.split(":")
        .nth(1)
        .ok_or(Error::FileEntryNotFound(
            String::from(entry),
            path.to_string_lossy().into(),
        ))?
        .parse()
        .map_err(|_err| {
            Error::FileEntryNotFound(String::from(entry), path.to_string_lossy().into())
        })?;
    let range = line.split(":")
        .nth(2)
        .ok_or(Error::FileEntryNotFound(
            String::from(entry),
            path.to_string_lossy().into(),
        ))?
        .parse()
        .map_err(|_err| {
            Error::FileEntryNotFound(String::from(entry), path.to_string_lossy().into())
        })?;

    Ok((start_id, range))
}
