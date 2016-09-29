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
use std::io::Write;

use super::workspace::Workspace;

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new(workspace: &Workspace) -> Self {
        Logger { file: File::create(workspace.log()).expect("Failed to initialize logger") }
    }

    pub fn log(&mut self, msg: &[u8]) -> () {
        self.file.write_all(msg).expect("unable to write to log file");
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.file.sync_all().expect("unable to sync log file");
    }
}
