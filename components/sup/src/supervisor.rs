// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
use std::process::{Command, Stdio, Child};
use std::thread;

use hcore;
use hcore::package::PackageIdent;
use libc::{pid_t, c_int};
use time::{Duration, SteadyTime};

use error::{Result, Error};
use util::signals;

const PIDFILE_NAME: &'static str = "PID";
static LOGKEY: &'static str = "SV";

// Functions from POSIX libc.
extern "C" {
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
}

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

#[derive(Debug)]
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
    pub pid: Option<Pid>,
    pub package_ident: PackageIdent,
    pub state: ProcessState,
    pub state_entered: SteadyTime,
    pub has_started: bool,
}

impl Supervisor {
    pub fn new(package_ident: PackageIdent) -> Supervisor {
        Supervisor {
            pid: None,
            package_ident: package_ident,
            state: ProcessState::Down,
            state_entered: SteadyTime::now(),
            has_started: false,
        }
    }

    fn enter_state(&mut self, state: ProcessState) {
        self.state = state;
        self.state_entered = SteadyTime::now();
    }

    pub fn status(&self) -> (bool, String) {
        let status = format!("{}: {} for {}",
                             self.package_ident,
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
            outputln!(preamble & self.package_ident.name, "Starting");
            self.enter_state(ProcessState::Start);
            let mut child = try!(Command::new(self.run_cmd())
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn());
            self.pid = Some(child.id());
            try!(self.create_pidfile());
            let package_name = self.package_ident.name.clone();
            try!(thread::Builder::new()
                .name(String::from("sup-service-read"))
                .spawn(move || -> Result<()> { child_reader(&mut child, package_name) }));
            self.enter_state(ProcessState::Up);
            self.has_started = true;
        } else {
            outputln!(preamble & self.package_ident.name, "Already started");
        }
        Ok(())
    }

    /// Send a SIGTERM to a process, wait 8 seconds, then send SIGKILL
    pub fn stop(&mut self) -> Result<()> {
        let wait = match self.pid {
            Some(ref pid) => {
                outputln!(preamble & self.package_ident.name, "Stopping");
                try!(signals::send_signal_to_pid(*pid, signals::Signal::SIGTERM));
                true
            }
            None => {
                outputln!(preamble & self.package_ident.name, "Already stopped");
                false
            }
        };
        if wait {
            let stop_time = SteadyTime::now() + Duration::seconds(8);
            loop {
                try!(self.check_process());
                if SteadyTime::now() > stop_time {
                    outputln!(preamble & self.package_ident.name,
                              "Process failed to stop with SIGTERM; sending SIGKILL");
                    if let Some(pid) = self.pid {
                        try!(signals::send_signal_to_pid(pid, signals::Signal::SIGKILL));
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
            try!(signals::send_signal_to_pid(pid, sig));
        }
        Ok(())
    }

    /// if the child process exists, check it's status via waitpid().
    ///
    /// Returns true if the process is still running, false if it has died.
    pub fn check_process(&mut self) -> Result<()> {
        if self.pid.is_none() {
            return Ok(());
        }

        unsafe {
            let mut status: c_int = 0;
            let cpid = self.pid.unwrap() as pid_t;
            match waitpid(cpid, &mut status, 1 as c_int) {
                0 => {} // Nothing returned,
                pid if pid == cpid => {
                    if WIFEXITED(status) {
                        let exit_code = WEXITSTATUS(status);
                        outputln!("{} - process {} died with exit code {}",
                                  self.package_ident.name,
                                  pid,
                                  exit_code);
                    } else if WIFSIGNALED(status) {
                        let exit_signal = WTERMSIG(status);
                        outputln!("{} - process {} died with signal {}",
                                  self.package_ident.name,
                                  pid,
                                  exit_signal);
                    } else {
                        outputln!("{} - process {} died, but I don't know how.",
                                  self.package_ident.name,
                                  pid);
                    }
                    match self.state {
                        ProcessState::Up | ProcessState::Start | ProcessState::Restart => {
                            outputln!("{} - Service exited", self.package_ident.name);
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
