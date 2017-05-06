// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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
use std::process::{Command, Stdio};

use super::Pkg;
use error::{Error, Result};

static LOGKEY: &'static str = "EX";

pub fn run_cmd<S: AsRef<OsStr>>(path: S, pkg: &Pkg) -> Result<Command> {
    exec(path, pkg)
}

#[cfg(any(target_os="linux", target_os="macos"))]
fn exec<S: AsRef<OsStr>>(path: S, pkg: &Pkg) -> Result<Command> {
    let mut cmd = Command::new(path);
    use hcore::os;
    use libc;
    use std::os::unix::process::CommandExt;
    let uid = os::users::get_uid_by_name(&pkg.svc_user)
        .ok_or(sup_error!(Error::Permissions(format!("No uid for user '{}' could be found",
                                                     &pkg.svc_user))))?;
    let gid = os::users::get_gid_by_name(&pkg.svc_group)
        .ok_or(sup_error!(Error::Permissions(format!("No gid for group '{}' could be found",
                                                     &pkg.svc_group))))?;
    // we want the command to spawn processes in their own process group
    // and not the same group as the supervisor. Otherwise if a child process
    // sends SIGTERM to the group, the supervisor could be terminated.
    cmd.before_exec(|| {
                        unsafe {
                            libc::setpgid(0, 0);
                        }
                        Ok(())
                    });
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .uid(uid)
        .gid(gid);
    for (key, val) in pkg.env.iter() {
        cmd.env(key, val);
    }
    Ok(cmd)
}

#[cfg(target_os = "windows")]
fn exec<S: AsRef<OsStr>>(path: S, pkg: &Pkg) -> Result<Command> {
    let mut cmd = Command::new("powershell.exe");
    let ps_command = format!("iex $(gc {} | out-string)", path.as_ref().to_string_lossy());
    cmd.arg("-command")
        .arg(ps_command)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, val) in pkg.env.iter() {
        cmd.env(key, val);
    }
    Ok(cmd)
}
