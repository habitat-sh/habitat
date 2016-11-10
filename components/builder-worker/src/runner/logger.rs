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

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process;

use super::workspace::Workspace;

const EOL_MARKER: &'static str = "\n";

pub struct Logger {
    stdout: File,
    stderr: File,
}

impl Logger {
    pub fn init(workspace: &Workspace) -> Self {
        let stdout = workspace.root().join("stdout.log");
        let stderr = workspace.root().join("stderr.log");
        Logger {
            stdout: File::create(stdout).expect("Failed to initialize stdout log file"),
            stderr: File::create(stderr).expect("Failed to initialize stderr log file"),
        }
    }

    /// Stream stdout and stderr of the given child process into the appropriate log files
    pub fn pipe(&mut self, process: &mut process::Child) {
        if let Some(ref mut stdout) = process.stdout {
            for line in BufReader::new(stdout).lines() {
                let mut l: String = line.unwrap();
                l = l + EOL_MARKER;
                self.log_stdout(l.as_bytes());
            }
        }
        if let Some(ref mut stderr) = process.stderr {
            for line in BufReader::new(stderr).lines() {
                let mut l: String = line.unwrap();
                l = l + EOL_MARKER;
                self.log_stderr(l.as_bytes());
            }
        }
    }

    /// Log message to stdout logfile
    pub fn log_stdout(&mut self, msg: &[u8]) {
        self.stdout.write_all(msg).expect(&format!("Logger unable to write to {:?}", self.stdout));
    }

    /// Log message to stderr logfile
    pub fn log_stderr(&mut self, msg: &[u8]) {
        self.stderr.write_all(msg).expect(&format!("Logger unable to write to {:?}", self.stderr))
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.stdout.sync_all().expect("Unable to sync stdout log file");
        self.stderr.sync_all().expect("Unable to sync stderr log file");
    }
}
