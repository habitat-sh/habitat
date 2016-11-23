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

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable)]
pub enum Status {
    Ok,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable)]
pub struct CheckResult {
    pub status: Status,
    pub output: String,
}

impl CheckResult {
    pub fn ok(output: String) -> Self {
        Self::new(Status::Ok, output)
    }

    pub fn warning(output: String) -> Self {
        Self::new(Status::Warning, output)
    }

    pub fn critical(output: String) -> Self {
        Self::new(Status::Critical, output)
    }

    pub fn unknown(output: String) -> Self {
        Self::new(Status::Unknown, output)
    }

    fn new(status: Status, output: String) -> Self {
        CheckResult {
            status: status,
            output: output,
        }
    }
}

impl Display for CheckResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let status_code = match self.status {
            Status::Ok => "OK",
            Status::Warning => "WARNING",
            Status::Critical => "CRITICAL",
            Status::Unknown => "UNKNOWN",
        };
        write!(f, "{} - {}", status_code, self.output)
    }
}
