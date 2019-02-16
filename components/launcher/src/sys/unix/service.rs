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

use std::{
    io,
    ops::Neg,
    os::unix::process::CommandExt,
    process::{Child, Command, ExitStatus, Stdio},
    result,
};

use crate::{
    core::os::{
        self,
        process::{signal, Signal},
    },
    protocol::{self, ShutdownMethod},
};
use libc;
use time::{Duration, SteadyTime};

use crate::{
    error::{Error, Result},
    service::Service,
};

pub struct Process(Child);

impl Process {
    pub fn id(&self) -> u32 {
        self.0.id()
    }

    /// Attempt to gracefully terminate a process and then forcefully kill it after
    /// 8 seconds if it has not terminated.
    pub fn kill(&mut self) -> ShutdownMethod {
        let mut pid_to_kill = self.0.id() as i32;
        // check the group of the process being killed
        // if it is the root process of the process group
        // we send our signals to the entire process group
        // to prevent orphaned processes.
        let pgid = unsafe { libc::getpgid(pid_to_kill) };
        if pid_to_kill == pgid {
            debug!(
                "pid to kill {} is the process group root. Sending signal to process group.",
                pid_to_kill
            );
            // sending a signal to the negative pid sends it to the
            // entire process group instead just the single pid
            pid_to_kill = pid_to_kill.neg();
        }

        // JW TODO: Determine if the error represents a case where the process was already
        // exited before we return out and assume so.
        if signal(pid_to_kill, Signal::TERM).is_err() {
            return ShutdownMethod::AlreadyExited;
        }
        let stop_time = SteadyTime::now() + Duration::seconds(8);
        loop {
            if let Ok(Some(_status)) = self.try_wait() {
                return ShutdownMethod::GracefulTermination;
            }
            if SteadyTime::now() < stop_time {
                continue;
            }
            // JW TODO: Determine if the error represents a case where the process was already
            // exited before we return out and assume so.
            if signal(pid_to_kill, Signal::KILL).is_err() {
                return ShutdownMethod::GracefulTermination;
            }
            return ShutdownMethod::Killed;
        }
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.0.try_wait()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.0.wait()
    }
}

pub fn run(msg: protocol::Spawn) -> Result<Service> {
    debug!("launcher is spawning {}", msg.binary);
    let mut cmd = Command::new(&msg.binary);

    // Favor explicitly set UID/GID over names when present
    let uid = if let Some(suid) = msg.svc_user_id {
        suid
    } else if let Some(suser) = &msg.svc_user {
        os::users::get_uid_by_name(&suser).ok_or_else(|| Error::UserNotFound(suser.to_string()))?
    } else {
        return Err(Error::UserNotFound(String::from("")));
    };

    let gid = if let Some(sgid) = msg.svc_group_id {
        sgid
    } else if let Some(sgroup) = &msg.svc_group {
        os::users::get_gid_by_name(&sgroup)
            .ok_or_else(|| Error::GroupNotFound(sgroup.to_string()))?
    } else {
        return Err(Error::GroupNotFound(String::from("")));
    };

    cmd.before_exec(owned_pgid);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .uid(uid)
        .gid(gid);
    for (key, val) in msg.env.iter() {
        cmd.env(key, val);
    }
    let mut child = cmd.spawn().map_err(Error::Spawn)?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let process = Process(child);
    Ok(Service::new(msg, process, stdout, stderr))
}

// we want the command to spawn processes in their own process group
// and not the same group as the Launcher. Otherwise if a child process
// sends SIGTERM to the group, the Launcher could be terminated.
fn owned_pgid() -> result::Result<(), io::Error> {
    unsafe {
        libc::setpgid(0, 0);
    }
    Ok(())
}
