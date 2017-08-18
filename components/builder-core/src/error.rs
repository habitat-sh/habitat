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

use std::error;
use std::fmt;
use std::result;

use protocol::net;

use data_structures;

#[derive(Debug)]
pub enum Error {
    GroupNotComplete,
    NetError(net::NetError),
    OriginAccessDenied,
    OriginNotFound(String),
    PartialJobGroupPromote(data_structures::PartialJobGroupPromote),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::GroupNotComplete => format!("This group is not complete"),
            Error::NetError(ref e) => format!("{}", e),
            Error::OriginAccessDenied => format!("You don't have access to this origin"),
            Error::OriginNotFound(ref e) => format!("Origin {} was not found", e),
            Error::PartialJobGroupPromote(_) => {
                format!("Some packages failed to promote to the specified channel")
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::GroupNotComplete => "Group not complete",
            Error::NetError(ref err) => err.description(),
            Error::OriginAccessDenied => "Origin access denied",
            Error::OriginNotFound(_) => "Origin not found",
            Error::PartialJobGroupPromote(_) => "Some packages failed to promote",
        }
    }
}
