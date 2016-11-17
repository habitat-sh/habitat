// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use std::fmt;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Child;
use std::result;
use std::thread;

use hcore;
use hcore::os::process;
use hcore::package::PackageIdent;
use hcore::service::ServiceGroup;
use libc::c_int;
use rustc_serialize::{Encodable, Encoder};
use time::{Duration, SteadyTime};

use error::{Result, Error};
use util;
use manager::signals;

const PIDFILE_NAME: &'static str = "PID";
static LOGKEY: &'static str = "SV";

/// A simple compatability type for external functions
#[allow(non_camel_case_types)]
pub type idtype_t = c_int;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;

// Process flags
pub const WCONTINUED: c_int = 8;
pub const WNOHANG: c_int = 1;
pub const WUNTRACED: c_int = 2;
pub const WEXITED: c_int = 4;
pub const WNOWAIT: c_int = 16777216;
pub const WSTOPPED: c_int = 2;

/// Get the exit status from waitpid's errno
#[allow(non_snake_case)]
pub fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/// Get the exit status from waitpid's errno
#[allow(non_snake_case)]
pub fn WIFCONTINUED(status: c_int) -> bool {
    status == 0xffff
}

#[allow(non_snake_case)]
pub fn WIFEXITED(status: c_int) -> bool {
    WTERMSIG(status) == 0
}

/// Has a value if our child was signaled
#[allow(non_snake_case)]
pub fn WIFSIGNALED(status: c_int) -> bool {
    ((((status) & 0x7f) + 1) as i8 >> 1) > 0
}

#[allow(non_snake_case)]
pub fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

#[allow(non_snake_case)]
pub fn WSTOPSIG(status: c_int) -> c_int {
    WEXITSTATUS(status)
}

#[allow(non_snake_case)]
pub fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

pub type Pid = u32;

#[derive(Debug, RustcEncodable)]
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


/// Additional params used to start the Supervisor.
/// These params are outside the scope of what is in
/// Supervisor.package_ident, and aren't runtime params that are stored
/// in the top-level Supervisor struct (such as PID etc)
#[derive(Debug, RustcEncodable)]
pub struct RuntimeConfig {
    pub svc_user: String,
    pub svc_group: String,
}

impl RuntimeConfig {
    pub fn new(svc_user: String, svc_group: String) -> RuntimeConfig {
        RuntimeConfig {
            svc_user: svc_user,
            svc_group: svc_group,
        }
    }
}

#[derive(Debug)]
pub struct Supervisor {
    pub pid: Option<Pid>,
    pub package_ident: PackageIdent,
    pub preamble: String,
    pub state: ProcessState,
    pub state_entered: SteadyTime,
    pub has_started: bool,
    pub runtime_config: RuntimeConfig,
}

impl Supervisor {
    pub fn new(package_ident: PackageIdent,
               service_group: ServiceGroup,
               runtime_config: RuntimeConfig)
               -> Supervisor {
        Supervisor {
            pid: None,
            package_ident: package_ident,
            preamble: format!("{}", service_group),
            state: ProcessState::Down,
            state_entered: SteadyTime::now(),
            has_started: false,
            runtime_config: runtime_config,
        }
    }

    fn enter_state(&mut self, state: ProcessState) {
        self.state = state;
        self.state_entered = SteadyTime::now();
    }

    pub fn status(&self) -> (bool, String) {
        let status = format!("{}: {} for {}",
                             self.preamble,
                             self.state,
                             SteadyTime::now() - self.state_entered);
        let healthy = match self.state {
            ProcessState::Up | ProcessState::Start | ProcessState::Restart => true,
            ProcessState::Down => false,
        };
        (healthy, status)
    }

    pub fn start(&mut self) -> Result<()> {
        if self.pid.is_none() {
            outputln!(preamble & self.preamble, "Starting");
            self.enter_state(ProcessState::Start);
            let mut child = try!(util::create_command(self.run_cmd(),
                                                      &self.runtime_config.svc_user,
                                                      &self.runtime_config.svc_group)
                .spawn());

            self.pid = Some(child.id());
            try!(self.create_pidfile());
            let package_name = self.preamble.clone();
            try!(thread::Builder::new()
                .name(String::from("sup-service-read"))
                .spawn(move || -> Result<()> { child_reader(&mut child, package_name) }));
            self.enter_state(ProcessState::Up);
            self.has_started = true;
        } else {
            outputln!(preamble & self.preamble, "Already started");
        }
        Ok(())
    }

    /// Send a SIGTERM to a process, wait 8 seconds, then send SIGKILL
    pub fn stop(&mut self) -> Result<()> {
        let wait = match self.pid {
            Some(ref pid) => {
                outputln!(preamble & self.preamble, "Stopping");
                try!(signals::send_signal(*pid, signals::Signal::SIGTERM as u32));
                true
            }
            None => false,
        };
        if wait {
            let stop_time = SteadyTime::now() + Duration::seconds(8);
            loop {
                try!(self.check_process());
                if SteadyTime::now() > stop_time {
                    outputln!(preamble & self.preamble,
                              "Process failed to stop with SIGTERM; sending SIGKILL");
                    if let Some(pid) = self.pid {
                        try!(signals::send_signal(pid, signals::Signal::SIGKILL as u32));
                    }
                    break;
                }
                if self.pid.is_none() {
                    break;
                } else {
                    continue;
                }
            }
        }
        Ok(())
    }

    pub fn is_up(&self) -> bool {
        if let ProcessState::Up = self.state {
            true
        } else {
            false
        }
    }

    pub fn is_down(&self) -> bool {
        if let ProcessState::Down = self.state {
            true
        } else {
            false
        }
    }

    pub fn down(&mut self) -> Result<()> {
        self.enter_state(ProcessState::Down);
        try!(self.stop());
        self.cleanup_pidfile();
        Ok(())
    }

    pub fn restart(&mut self) -> Result<()> {
        self.enter_state(ProcessState::Restart);
        try!(self.stop());
        try!(self.start());
        Ok(())
    }

    /// Pass through a Unix signal to a process
    pub fn send_unix_signal(&self, sig: signals::Signal) -> Result<()> {
        if let Some(pid) = self.pid {
            try!(signals::send_signal(pid, sig as u32));
        }
        Ok(())
    }

    /// if the child process exists, check it's status via waitpid().
    pub fn check_process(&mut self) -> Result<()> {
        if self.pid.is_none() {
            return Ok(());
        }

        let mut status: c_int = 0;
        let cpid = self.pid.unwrap();
        match process::wait_for_exit(cpid, &mut status) {
            0 => {} // Nothing returned,
            pid if pid == cpid => {
                if WIFEXITED(status) {
                    let exit_code = WEXITSTATUS(status);
                    outputln!("{} - process {} died with exit code {}",
                              self.preamble,
                              pid,
                              exit_code);
                } else if WIFSIGNALED(status) {
                    let exit_signal = WTERMSIG(status);
                    outputln!("{} - process {} died with signal {}",
                              self.preamble,
                              pid,
                              exit_signal);
                } else {
                    outputln!("{} - process {} died, but I don't know how.",
                              self.preamble,
                              pid);
                }
                match self.state {
                    ProcessState::Up | ProcessState::Start | ProcessState::Restart => {
                        outputln!("{} - Service exited", self.preamble);
                        self.pid = None;
                    }
                    ProcessState::Down => {
                        self.enter_state(ProcessState::Down);
                        self.pid = None;
                    }
                }
            }
            // ZOMBIES! Bad zombies! We listen for zombies. ZOMBOCOM!
            pid => {
                if WIFEXITED(status) {
                    let exit_code = WEXITSTATUS(status);
                    debug!("Process {} died with exit code {}", pid, exit_code);
                } else if WIFSIGNALED(status) {
                    let exit_signal = WTERMSIG(status);
                    debug!("Process {} terminated with signal {}", pid, exit_signal);
                } else {
                    debug!("Process {} died, but I don't know how.", pid);
                }
            }
        }
        Ok(())
    }

    pub fn run_cmd(&self) -> PathBuf {
        self.service_dir().join("run")
    }

    pub fn service_dir(&self) -> PathBuf {
        hcore::fs::svc_path(&self.package_ident.name)
    }

    pub fn pid_file(&self) -> PathBuf {
        self.service_dir().join(PIDFILE_NAME)
    }

    /// Create a pid file for a package
    /// The existence of this file does not guarantee that a
    /// process exists at the PID contained within.
    pub fn create_pidfile(&self) -> Result<()> {
        match self.pid {
            Some(ref pid) => {
                let pid_file = self.pid_file();
                debug!("Creating PID file for child {} -> {:?}",
                       pid_file.display(),
                       pid);
                let mut f = try!(File::create(pid_file));
                try!(write!(f, "{}", pid));
                Ok(())
            }
            None => Ok(()),
        }
    }

    /// Remove a pidfile for this package if it exists.
    /// Do NOT fail if there is an error removing the PIDFILE
    pub fn cleanup_pidfile(&self) {
        let pid_file = self.pid_file();
        debug!("Attempting to clean up pid file {}", &pid_file.display());
        match fs::remove_file(pid_file) {
            Ok(_) => {
                debug!("Removed pid file");
            }
            Err(e) => {
                debug!("Error removing pidfile: {}, continuing", e);
            }
        };
    }

    /// attempt to read the pidfile for this package.
    /// If the pidfile does not exist, then return None,
    /// otherwise, return Some(pid, uptime_seconds).
    pub fn read_pidfile(&self) -> Result<Option<Pid>> {
        let pid_file = self.pid_file();
        debug!("Reading pidfile {}", &pid_file.display());

        let mut f = try!(File::open(pid_file));
        let mut contents = String::new();
        try!(f.read_to_string(&mut contents));
        debug!("pidfile contents = {}", contents);
        let pid = match contents.parse::<u32>() {
            Ok(pid) => pid,
            Err(e) => {
                debug!("Error reading pidfile: {}", e);
                return Err(sup_error!(Error::InvalidPidFile));
            }
        };
        Ok(Some(pid))
    }
}

impl Encodable for Supervisor {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("supervisor", 7, |s| {
            try!(s.emit_struct_field("pid", 0, |s| self.pid.encode(s)));
            try!(s.emit_struct_field("package_ident", 1, |s| self.package_ident.encode(s)));
            try!(s.emit_struct_field("preamble", 2, |s| self.preamble.encode(s)));
            try!(s.emit_struct_field("state", 3, |s| self.state.encode(s)));
            try!(s.emit_struct_field("state_entered",
                                     4,
                                     |s| self.state_entered.to_string().encode(s)));
            try!(s.emit_struct_field("has_started", 5, |s| self.has_started.encode(s)));
            try!(s.emit_struct_field("runtime_config", 6, |s| self.runtime_config.encode(s)));
            Ok(())
        }));
        Ok(())
    }
}

impl Drop for Supervisor {
    fn drop(&mut self) {
        let _ = self.cleanup_pidfile();
    }
}

/// Consume output from a child process until EOF, then finish
fn child_reader(child: &mut Child, package_name: String) -> Result<()> {
    let c_stdout = match child.stdout {
        Some(ref mut s) => s,
        None => return Err(sup_error!(Error::UnpackFailed)),
    };

    let mut reader = BufReader::new(c_stdout);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() > 0 {
        let mut line = output_format!(preamble &package_name, logkey "O");
        line.push_str(&buffer);
        print!("{}", line);
        buffer.clear();
    }
    debug!("child_reader exiting");
    Ok(())
}
