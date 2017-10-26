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
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process;

use unshare::{self, Namespace};
use users;

use {Error, Result};

pub fn run(cmd: &str, args: Vec<OsString>) -> Result<()> {
    let program = proc_exe()?;
    let mut command = unshare_command(cmd, args)?;
    debug!("Running: {:?}", command);
    let exit_status = command.spawn()?.wait()?;
    // TODO fn: cleanup
    process::exit(exit_status.code().unwrap_or(127));
    Ok(())
}

fn unshare_command(cmd: &str, args: Vec<OsString>) -> Result<unshare::Command> {
    let program = proc_exe()?;
    let namespaces = vec![
        Namespace::User,
        Namespace::Mount,
        Namespace::Uts,
        Namespace::Ipc,
        Namespace::Pid,
    ];

    let mut command = unshare::Command::new(program);
    command.arg("invoke");
    command.arg(cmd);
    command.args(&args);
    command.unshare(namespaces.iter().cloned());
    command.set_id_maps(uid_maps()?, gid_maps()?);
    command.set_id_map_commands(find_command("newuidmap")?, find_command("newgidmap")?);
    command.uid(0);
    command.gid(0);

    Ok(command)
}

fn uid_maps() -> Result<Vec<unshare::UidMap>> {
    let (start_uid, range) = sub_range(Path::new("/etc/subuid"))?;

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
    let (start_gid, range) = sub_range(Path::new("/etc/subgid"))?;

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

fn sub_range(path: &Path) -> Result<(u32, u32)> {
    if !path.exists() {
        return Err(Error::FileNotFound(path.to_string_lossy().into()));
    }
    let username = username()?;
    let line = {
        let file = File::open(path)?;
        let file = BufReader::new(file);
        match file.lines().map(|l| l.unwrap()).find(|ref line| {
            line.split(":").next().unwrap_or("") == username
        }) {
            Some(line) => line,
            None => {
                return Err(Error::FileEntryNotFound(
                    username,
                    path.to_string_lossy().into(),
                ))
            }
        }
    };
    let start_id = line.split(":")
        .nth(1)
        .ok_or(Error::FileEntryNotFound(
            username.clone(),
            path.to_string_lossy().into(),
        ))?
        .parse()
        .map_err(|_err| {
            Error::FileEntryNotFound(username.clone(), path.to_string_lossy().into())
        })?;
    let range = line.split(":")
        .nth(2)
        .ok_or(Error::FileEntryNotFound(
            username.clone(),
            path.to_string_lossy().into(),
        ))?
        .parse()
        .map_err(|_err| {
            Error::FileEntryNotFound(username.clone(), path.to_string_lossy().into())
        })?;

    Ok((start_id, range))
}
