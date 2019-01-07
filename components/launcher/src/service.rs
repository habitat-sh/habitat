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

use std::fmt;
use std::io::{self, BufRead, BufReader, Read, Write};
#[cfg(unix)]
use std::process::{ChildStderr, ChildStdout, ExitStatus};
use std::thread;

#[cfg(windows)]
use core::os::process::windows_child::{ChildStderr, ChildStdout, ExitStatus};
use crate::protocol;

pub use crate::sys::service::*;

pub struct Service {
    args: protocol::Spawn,
    process: Process,
}

impl Service {
    pub fn new(
        spawn: protocol::Spawn,
        process: Process,
        stdout: Option<ChildStdout>,
        stderr: Option<ChildStderr>,
    ) -> Self {
        if let Some(stdout) = stdout {
            let id = spawn.get_id().to_string();
            thread::Builder::new()
                .name(format!("{}-out", spawn.get_id()))
                .spawn(move || pipe_stdout(stdout, id))
                .ok();
        }
        if let Some(stderr) = stderr {
            let id = spawn.get_id().to_string();
            thread::Builder::new()
                .name(format!("{}-err", spawn.get_id()))
                .spawn(move || pipe_stderr(stderr, id))
                .ok();
        }
        Service {
            args: spawn,
            process: process,
        }
    }

    pub fn args(&self) -> &protocol::Spawn {
        &self.args
    }

    pub fn id(&self) -> u32 {
        self.process.id()
    }

    /// Attempt to gracefully terminate a proccess and then forcefully kill it after
    /// 8 seconds if it has not terminated.
    pub fn kill(&mut self) -> protocol::ShutdownMethod {
        self.process.kill()
    }

    pub fn name(&self) -> &str {
        self.args.get_id()
    }

    pub fn take_args(self) -> protocol::Spawn {
        self.args
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.process.try_wait()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.process.wait()
    }
}

impl fmt::Debug for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Service {{ pid: {:?} }}", self.process.id())
    }
}

/// Consume output from a child process until EOF, then finish
fn pipe_stdout<T>(out: T, id: String)
where
    T: Read,
{
    let mut reader = BufReader::new(out);
    let mut buffer = String::new();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        let line = output_format!(preamble &id, logkey "O", buffer);
        writeln!(&mut io::stdout(), "{}", line).expect("unable to write to stdout");
        buffer.clear();
    }
}

/// Consume standard error from a child process until EOF, then finish
fn pipe_stderr<T>(err: T, id: String)
where
    T: Read,
{
    let mut reader = BufReader::new(err);
    let mut buffer = String::new();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        let line = output_format!(preamble &id, logkey "E", buffer);
        writeln!(&mut io::stderr(), "{}", line).expect("unable to write to stderr");
        buffer.clear();
    }
}
