//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::process::Command;
use std::fmt::{self, Display, Formatter};

use error::{BldrResult, BldrError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Ok,
    Warning,
    Critical,
    Unknown
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    pub status: Status,
    pub output: String
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

pub fn run(check: &str) -> BldrResult<CheckResult> {
    let result = try!(Command::new(check).output());
    let stdout = try!(String::from_utf8(result.stdout));
    let stderr = try!(String::from_utf8(result.stderr));
    let output = format!("{}\n{}", stdout, stderr);

    match result.status.code() {
        Some(0) => {
            Ok(CheckResult{ status: Status::Ok, output: output })
        },
        Some(1) => {
            Ok(CheckResult{ status: Status::Warning, output: output })
        },
        Some(2) => {
            Ok(CheckResult{ status: Status::Critical, output: output })
        }
        Some(3) => {
            Ok(CheckResult{ status: Status::Unknown, output: output })
        },
        Some(x) => {
            Err(BldrError::HealthCheck(format!("exited {}: {}", x, output)))
        },
        None => {
            Err(BldrError::HealthCheck(format!("exited from a signal: {}", output)))
        }
    }
}
