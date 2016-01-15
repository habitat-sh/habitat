// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Ok,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
