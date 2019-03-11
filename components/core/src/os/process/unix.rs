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

use crate::error::{Error,
                   Result};
use libc::{self,
           pid_t};
use std::{ffi::OsString,
          fmt,
          io,
          os::unix::process::CommandExt,
          path::PathBuf,
          process::Command,
          result,
          str::FromStr};

pub type Pid = libc::pid_t;
pub(crate) type SignalCode = libc::c_int;

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Signal {
    INT,
    ILL,
    ABRT,
    FPE,
    KILL,
    SEGV,
    TERM,
    HUP,
    QUIT,
    ALRM,
    USR1,
    USR2,
    CHLD,
}

pub fn become_command(command: PathBuf, args: &[OsString]) -> Result<()> {
    become_exec_command(command, args)
}

/// Get process identifier of calling process.
pub fn current_pid() -> Pid { unsafe { libc::getpid() as pid_t } }

/// Determines if a process is running with the given process identifier.
pub fn is_alive(pid: Pid) -> bool {
    match unsafe { libc::kill(pid as pid_t, 0) } {
        0 => true,
        _ => {
            match io::Error::last_os_error().raw_os_error() {
                Some(libc::EPERM) => true,
                Some(libc::ESRCH) => false,
                _ => false,
            }
        }
    }
}

pub fn signal(pid: Pid, signal: Signal) -> Result<()> {
    unsafe {
        match libc::kill(pid as pid_t, signal.into()) {
            0 => Ok(()),
            e => Err(Error::SignalFailed(e, io::Error::last_os_error())),
        }
    }
}

impl From<Signal> for SignalCode {
    fn from(value: Signal) -> SignalCode {
        match value {
            Signal::INT => libc::SIGINT,
            Signal::ILL => libc::SIGILL,
            Signal::ABRT => libc::SIGABRT,
            Signal::FPE => libc::SIGFPE,
            Signal::KILL => libc::SIGKILL,
            Signal::SEGV => libc::SIGSEGV,
            Signal::TERM => libc::SIGTERM,
            Signal::HUP => libc::SIGHUP,
            Signal::QUIT => libc::SIGQUIT,
            Signal::ALRM => libc::SIGALRM,
            Signal::USR1 => libc::SIGUSR1,
            Signal::USR2 => libc::SIGUSR2,
            Signal::CHLD => libc::SIGCHLD,
        }
    }
}
/// Makes an `execvp(3)` system call to become a new program.
///
/// Note that if successful, this function will not return.
///
/// # Failures
///
/// * If the system call fails the error will be returned, otherwise this function does not return
fn become_exec_command(command: PathBuf, args: &[OsString]) -> Result<()> {
    debug!("Calling execvp(): ({:?}) {:?}", command.display(), &args);
    let error_if_failed = Command::new(command).args(args).exec();
    // The only possible return for the above function is an `Error` so return it, meaning that we
    // failed to exec to our target program
    Err(error_if_failed.into())
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let signal = match s {
            "HUP" => Signal::HUP,
            "INT" => Signal::INT,
            "QUIT" => Signal::QUIT,
            "ILL" => Signal::ILL,
            "ABRT" => Signal::ABRT,
            "FPE" => Signal::FPE,
            "KILL" => Signal::KILL,
            "USR1" => Signal::USR1,
            "SEGV" => Signal::SEGV,
            "USR2" => Signal::USR2,
            "ALRM" => Signal::ALRM,
            "TERM" => Signal::TERM,
            "CHLD" => Signal::CHLD,
            _ => return Err(Error::ParseSignalError(s.to_string())),
        };
        Ok(signal)
    }
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Signal::HUP => "HUP",
            Signal::INT => "INT",
            Signal::QUIT => "QUIT",
            Signal::ILL => "ILL",
            Signal::ABRT => "ABRT",
            Signal::FPE => "FPE",
            Signal::KILL => "KILL",
            Signal::USR1 => "USR1",
            Signal::SEGV => "SEGV",
            Signal::USR2 => "USR2",
            Signal::ALRM => "ALRM",
            Signal::TERM => "TERM",
            Signal::CHLD => "CHLD",
        };
        write!(f, "{}", s)
    }
}

/// Encapsulates logic for defining the default shutdown signal we
/// send services, and handles translation from external types at the
/// edges of our system.
#[derive(Debug, Clone)]
pub struct ShutdownSignal(Signal);

impl Default for ShutdownSignal {
    /// Unless otherwise specified, the Supervisor will shut down
    /// services by sending the `TERM` signal.
    fn default() -> Self { Signal::TERM.into() }
}

impl FromStr for ShutdownSignal {
    type Err = Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> { Ok(ShutdownSignal(s.parse()?)) }
}

impl fmt::Display for ShutdownSignal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

impl From<Signal> for ShutdownSignal {
    fn from(signal: Signal) -> Self { ShutdownSignal(signal) }
}

impl From<ShutdownSignal> for Signal {
    fn from(shutdown_signal: ShutdownSignal) -> Self { shutdown_signal.0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn signal_names_are_only_accepted_as_uppercase() {
        assert_eq!(Signal::HUP, "HUP".parse().unwrap());
        assert!("hup".parse::<Signal>().is_err());
    }

    #[test]
    fn signals_can_render_as_strings() {
        assert_eq!("HUP", Signal::HUP.to_string());
    }

    #[test]
    fn signals_can_round_trip_through_parsing() {
        for signal in &[Signal::HUP,
                        Signal::INT,
                        Signal::QUIT,
                        Signal::ABRT,
                        Signal::FPE,
                        Signal::KILL,
                        Signal::USR1,
                        Signal::SEGV,
                        Signal::USR2,
                        Signal::ALRM,
                        Signal::TERM,
                        Signal::CHLD]
        {
            assert_eq!(*signal,
                       signal.to_string()
                             .parse::<Signal>()
                             .expect("Couldn't parse back into a Signal!"));
        }
    }
}
