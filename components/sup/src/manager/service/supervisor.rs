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

/// Supervise a service.
///
/// The supervisor is responsible for running any services we are asked to start. It handles
/// spawning the new process, watching for failure, and ensuring the service is either up or down.
/// If the process dies, the supervisor will restart it.

use std;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::{ChildStderr, ChildStdout};
use std::result;
use std::thread;

use ansi_term::Colour;
use hcore::os::process::{HabChild, ExitStatusExt};
use hcore::util::perm::set_owner;
use hcore::service::ServiceGroup;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use time::{self, Timespec};

use super::exec;
use error::{Result, Error};
use manager::service::Pkg;

static LOGKEY: &'static str = "SV";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ProcessState {
    Down,
    Up,
    Start,
    Restart,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self {
            &ProcessState::Down => "down",
            &ProcessState::Up => "up",
            &ProcessState::Start => "start",
            &ProcessState::Restart => "restart",
        };
        write!(f, "{}", state)
    }
}

#[derive(Debug)]
pub struct Supervisor {
    pub child: Option<HabChild>,
    pub preamble: String,
    pub state: ProcessState,
    pub state_entered: Timespec,
    pub has_started: bool,
    pid: Option<PathBuf>,
}

impl Supervisor {
    pub fn new(service_group: &ServiceGroup) -> Supervisor {
        Supervisor {
            child: None,
            preamble: format!("{}", service_group),
            state: ProcessState::Down,
            state_entered: time::get_time(),
            has_started: false,
            pid: None,
        }
    }

    fn enter_state(&mut self, state: ProcessState) {
        self.state = state;
        self.state_entered = time::get_time();
    }

    pub fn status(&self) -> (bool, String) {
        let status = format!("{}: {} for {}",
                             self.preamble,
                             self.state,
                             time::get_time() - self.state_entered);
        let healthy = match self.state {
            ProcessState::Up | ProcessState::Start | ProcessState::Restart => true,
            ProcessState::Down => false,
        };
        (healthy, status)
    }

    pub fn start(&mut self, pkg: &Pkg) -> Result<()> {
        if self.child.is_some() {
            outputln!(preamble & self.preamble, "Already started");
            return Ok(());
        }
        debug!("Setting PATH for {} to PATH='{}'",
               &self.preamble,
               pkg.env
                   .get("PATH")
                   .map(|v| &**v)
                   .unwrap_or("<unknown>"));
        outputln!(preamble self.preamble,
                  "Starting process as user={}, group={}",
                  &pkg.svc_user,
                  &pkg.svc_group);
        self.enter_state(ProcessState::Start);
        let mut child = exec::run_cmd(&pkg.svc_run, &pkg)?.spawn()?;
        self.child = Some(HabChild::from(&mut child)?);
        let c_stdout = child.stdout;
        let c_stderr = child.stderr;
        self.create_pidfile(pkg)?;
        let out_package_name = self.preamble.clone();
        let err_package_name = self.preamble.clone();
        thread::Builder::new()
            .name(String::from("sup-service-read-out"))
            .spawn(move || -> Result<()> { child_out_reader(c_stdout, out_package_name) })?;
        thread::Builder::new()
            .name(String::from("sup-service-read-err"))
            .spawn(move || -> Result<()> { child_err_reader(c_stderr, err_package_name) })?;
        self.enter_state(ProcessState::Up);
        self.has_started = true;
        Ok(())
    }

    /// Send a SIGTERM to a process, wait 8 seconds, then send SIGKILL
    pub fn stop(&mut self) -> Result<()> {
        match self.child {
            Some(ref mut child) => {
                outputln!(preamble & self.preamble, "Stopping...");
                let shutdown = try!(child.kill());
                outputln!("{} - Shutdown method: {}", self.preamble, shutdown);
            }
            None => {}
        };
        self.check_process();
        Ok(())
    }

    pub fn down(&mut self) -> Result<()> {
        self.enter_state(ProcessState::Down);
        try!(self.stop());
        self.cleanup_pidfile();
        Ok(())
    }

    pub fn restart(&mut self, pkg: &Pkg) -> Result<()> {
        self.enter_state(ProcessState::Restart);
        try!(self.stop());
        try!(self.start(pkg));
        Ok(())
    }

    /// if the child process exists, check it's status via waitpid().
    pub fn check_process(&mut self) {
        let changed = match self.child {
            None => false,
            Some(ref mut child) => {
                match child.status() {
                    Ok(ref status) if status.no_status() => false,
                    Ok(ref status) => {
                        if status.code().is_some() {
                            outputln!("{} - process {} died with exit code {}",
                                      self.preamble,
                                      child.id(),
                                      status.code().unwrap());
                        } else if status.signal().is_some() {
                            outputln!("{} - process {} died with signal {}",
                                      self.preamble,
                                      child.id(),
                                      status.signal().unwrap());
                        }
                        true
                    }
                    Err(e) => {
                        debug!("Error checking process status: {}, continuing", e);
                        false
                    }
                }
            }
        };
        if changed {
            match self.state {
                ProcessState::Up | ProcessState::Start | ProcessState::Restart => {
                    outputln!("{} - Service exited", self.preamble);
                    self.child = None;
                }
                ProcessState::Down => {
                    self.enter_state(ProcessState::Down);
                    self.child = None;
                }
            }
        }
    }

    /// Create a pid file for a package
    /// The existence of this file does not guarantee that a
    /// process exists at the PID contained within.
    pub fn create_pidfile(&mut self, pkg: &Pkg) -> Result<()> {
        match self.child {
            Some(ref child) => {
                let ref pid = child.id();
                debug!("Creating PID file for child {} -> {:?}",
                       pkg.svc_pid_file.display(),
                       pid);
                let mut f = try!(File::create(&pkg.svc_pid_file));
                set_owner(&pkg.svc_pid_file, &pkg.svc_user, &pkg.svc_group)?;
                write!(f, "{}", pid)?;
                self.pid = Some(pkg.svc_pid_file.clone());
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Remove a pidfile for this package if it exists.
    /// Do NOT fail if there is an error removing the PIDFILE
    pub fn cleanup_pidfile(&mut self) {
        if let Some(ref pid_file) = self.pid {
            debug!("Attempting to clean up pid file {}", &pid_file.display());
            match std::fs::remove_file(pid_file) {
                Ok(_) => debug!("Removed pid file"),
                Err(e) => debug!("Error removing pidfile: {}, continuing", e),
            }
        }
        self.pid = None;
    }
}

impl Serialize for Supervisor {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let pid = match self.child {
            Some(ref child) => Some(child.id()),
            None => None,
        };
        let mut strukt = try!(serializer.serialize_struct("supervisor", 5));
        try!(strukt.serialize_field("pid", &pid));
        try!(strukt.serialize_field("preamble", &self.preamble));
        try!(strukt.serialize_field("state", &self.state));
        try!(strukt.serialize_field("state_entered", &self.state_entered.sec));
        try!(strukt.serialize_field("started", &self.has_started));
        strukt.end()
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        let _ = self.cleanup_pidfile();
    }
}

/// Consume output from a child process until EOF, then finish
fn child_out_reader(c_stdout: Option<ChildStdout>, package_name: String) -> Result<()> {
    let out = match c_stdout {
        Some(s) => s,
        None => return Err(sup_error!(Error::UnpackFailed)),
    };
    let mut reader = BufReader::new(out);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() > 0 {
        let mut line = output_format!(preamble &package_name, logkey "O");
        line.push_str(&buffer);
        print!("{}", line);
        buffer.clear();
    }
    debug!("child_out_reader exiting");
    Ok(())
}

/// Consume standard error from a child process until EOF, then finish
fn child_err_reader(c_stderr: Option<ChildStderr>, package_name: String) -> Result<()> {
    let err = match c_stderr {
        Some(s) => s,
        None => return Err(sup_error!(Error::UnpackFailed)),
    };
    let mut reader = BufReader::new(err);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() > 0 {
        let mut line = output_format!(preamble &package_name, logkey "E");
        let c = format!("{}", Colour::Red.bold().paint(buffer.clone()));
        line.push_str(c.as_str());
        let _ = write!(&mut std::io::stderr(), "{}", line);
        buffer.clear();
    }
    debug!("child_err_reader exiting");
    Ok(())
}
