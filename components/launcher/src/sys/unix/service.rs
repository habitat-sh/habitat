use crate::{error::ServiceRunError,
            protocol::{self,
                       ShutdownMethod},
            service::Service};
use anyhow::Result;
use habitat_core::os::{self,
                       process::{Signal,
                                 exec,
                                 signal}};
use log::debug;
use nix::unistd::{Gid,
                  Uid};
use std::{io,
          ops::Neg,
          process::{Child,
                    ExitStatus},
          time::{Duration,
                 Instant}};

pub struct Process(Child);

impl Process {
    pub fn id(&self) -> u32 { self.0.id() }

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
            debug!("pid to kill {} is the process group root. Sending signal to process group.",
                   pid_to_kill);
            // sending a signal to the negative pid sends it to the
            // entire process group instead just the single pid
            pid_to_kill = pid_to_kill.neg();
        }

        // JW TODO: Determine if the error represents a case where the process was already
        // exited before we return out and assume so.
        #[allow(clippy::question_mark)]
        if signal(pid_to_kill, Signal::TERM).is_err() {
            return ShutdownMethod::AlreadyExited;
        }
        let shutdown_timeout = Duration::from_secs(8);
        let start_time = Instant::now();
        loop {
            if let Ok(Some(_status)) = self.try_wait() {
                return ShutdownMethod::GracefulTermination;
            }
            if start_time.elapsed() < shutdown_timeout {
                continue;
            }
            // JW TODO: Determine if the error represents a case where the process was already
            // exited before we return out and assume so.
            #[allow(clippy::question_mark)]
            if signal(pid_to_kill, Signal::KILL).is_err() {
                return ShutdownMethod::GracefulTermination;
            }
            return ShutdownMethod::Killed;
        }
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> { self.0.try_wait() }

    pub fn wait(&mut self) -> io::Result<ExitStatus> { self.0.wait() }
}

pub fn run(msg: protocol::Spawn) -> Result<Service, ServiceRunError> {
    debug!("launcher is spawning {}", msg.binary);

    // Favor explicitly set UID/GID over names when present
    let user_id = if let Some(suid) = msg.svc_user_id {
        suid
    } else if let Some(suser) = &msg.svc_user {
        os::users::get_uid_by_name(suser).map_err(|err| {
                                             ServiceRunError::GetUid(suser.to_string(), err)
                                         })?
                                         .ok_or_else(|| {
                                             ServiceRunError::UserNotFound(suser.to_string())
                                         })?
    } else {
        return Err(ServiceRunError::UserNotFound(String::from("")));
    };
    let uid = Uid::from_raw(user_id);

    let group_id = if let Some(sgid) = msg.svc_group_id {
        sgid
    } else if let Some(sgroup) = &msg.svc_group {
        os::users::get_gid_by_name(sgroup).map_err(|err| {
                                              ServiceRunError::GetGid(sgroup.to_string(), err)
                                          })?
                                          .ok_or_else(|| {
                                              ServiceRunError::GroupNotFound(sgroup.to_string())
                                          })?
    } else {
        return Err(ServiceRunError::GroupNotFound(String::from("")));
    };
    let gid = Gid::from_raw(group_id);

    let mut cmd = exec::unix::hook_command(&msg.binary, &msg.env, Some((uid, gid)));

    let mut child = cmd.spawn().map_err(ServiceRunError::Spawn)?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let process = Process(child);
    debug!(target: "pidfile_tracing", "Launcher spawned {} with PID = {}", msg.binary, process.id());
    Ok(Service::new(msg, process, stdout, stderr))
}
