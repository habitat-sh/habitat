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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};

use errno;
use libc;
use unshare;

use {Error, Result};
use user;

const MIN_SUB_RANGE: u32 = 65536;

pub fn setns_network<P: AsRef<Path>>(path: P) -> Result<()> {
    setns(path, libc::CLONE_NEWNET)
}

pub fn setns_user<P: AsRef<Path>>(path: P) -> Result<()> {
    setns(path, libc::CLONE_NEWUSER)
}

pub fn ns_pid_file<P: AsRef<Path>>(ns_dir: P) -> PathBuf {
    ns_dir.as_ref().join(".ns.pid")
}

pub fn ns_created_file<P: AsRef<Path>>(ns_dir: P) -> PathBuf {
    ns_dir.as_ref().join(".ns.created")
}

pub fn userns_file<P: AsRef<Path>>(ns_dir: P) -> PathBuf {
    ns_dir.as_ref().join("userns")
}

pub fn netns_file<P: AsRef<Path>>(ns_dir: P) -> PathBuf {
    ns_dir.as_ref().join("netns")
}

pub fn uid_maps(username: &str) -> Result<Vec<unshare::UidMap>> {
    let (start_uid, range) = sub_range(username, Path::new("/etc/subuid"))?;
    if range < MIN_SUB_RANGE {
        return Err(Error::SubUidRangeTooSmall(range, MIN_SUB_RANGE));
    }
    let uid = user::uid_for_username(username)?;

    Ok(vec![
        // Maps the outside user to the root user
        unshare::UidMap {
            inside_uid: 0,
            outside_uid: uid,
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

pub fn gid_maps(groupname: &str) -> Result<Vec<unshare::GidMap>> {
    let (start_gid, range) = sub_range(groupname, Path::new("/etc/subgid"))?;
    if range < MIN_SUB_RANGE {
        return Err(Error::SubGidRangeTooSmall(range, MIN_SUB_RANGE));
    }
    let gid = user::gid_for_groupname(groupname)?;

    Ok(vec![
        // Maps the outside group to the root group
        unshare::GidMap {
            inside_gid: 0,
            outside_gid: gid,
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

fn setns<P: AsRef<Path>>(path: P, nstype: libc::c_int) -> Result<()> {
    let file = File::open(path.as_ref())?;
    let fd = file.as_raw_fd();
    debug!(
        "calling setns(), path={}, fd={}, nstype={}",
        path.as_ref().display(),
        fd,
        nstype,
    );
    match unsafe { libc::setns(fd, nstype) } {
        rc if rc < 0 => {
            Err(Error::Setns(format!(
                "setns({}, {}) returned: {} ({})",
                path.as_ref().display(),
                nstype,
                rc,
                errno::errno(),
            )))
        }
        _ => Ok(()),
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
