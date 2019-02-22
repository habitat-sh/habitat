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

use crate::protocol;
use ipc_channel;
use std::{error,
          fmt,
          io,
          result};

use crate::{SUP_CMD,
            SUP_PACKAGE_IDENT};

#[derive(Debug)]
pub enum Error {
    AcceptConn,
    Connect(io::Error),
    ExecWait(io::Error),
    GroupNotFound(String),
    OpenPipe(io::Error),
    Protocol(protocol::Error),
    Send(ipc_channel::Error),
    Spawn(io::Error),
    SupBinaryVersion,
    SupBinaryNotFound,
    SupPackageNotFound,
    SupShutdown,
    SupSpawn(io::Error),
    UserNotFound(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::AcceptConn => "Unable to accept connection from Supervisor".to_string(),
            Error::Connect(ref e) => {
                format!("Unable to connect to Supervisor's comm channel, {}", e)
            }
            Error::ExecWait(ref e) => format!("Error waiting on PID, {}", e),
            Error::GroupNotFound(ref e) => format!("No GID for group '{}' could be found", e),
            Error::OpenPipe(ref e) => format!("Unable to open Launcher's comm channel, {}", e),
            Error::Protocol(ref e) => format!("{}", e),
            Error::Send(ref e) => format!("Unable to send to Launcher's comm channel, {}", e),
            Error::Spawn(ref e) => format!("Unable to spawn process, {}", e),
            Error::SupBinaryVersion => "Unsupported Supervisor binary version".to_string(),
            Error::SupBinaryNotFound => {
                format!("Supervisor package didn't contain '{}' binary", SUP_CMD)
            }
            Error::SupPackageNotFound => {
                format!("Unable to locate Supervisor package, {}", SUP_PACKAGE_IDENT)
            }
            Error::SupShutdown => "Error waiting for Supervisor to shutdown".to_string(),
            Error::SupSpawn(ref e) => format!("Unable to spawn Supervisor, {}", e),
            Error::UserNotFound(ref e) => format!("No UID for user '{}' could be found", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AcceptConn => "Unable to accept connection from Supervisor",
            Error::Connect(_) => "Unable to connect to Supervisor's pipe",
            Error::GroupNotFound(_) => "No matching GID for group found",
            Error::ExecWait(_) => "OS Error while waiting on PID",
            Error::OpenPipe(_) => "Unable to open Launcher's pipe",
            Error::Protocol(_) => "Error with the Supervisor protocol",
            Error::Send(_) => "Unable to send to Launcher's pipe",
            Error::Spawn(_) => "Unable to spawn process",
            Error::SupBinaryVersion => "Unsupported Supervisor binary version",
            Error::SupBinaryNotFound => "Unable to locate Supervisor binary in package",
            Error::SupPackageNotFound => "Unable to locate Supervisor package on disk",
            Error::SupShutdown => "Error waiting for Supervisor to shutdown",
            Error::SupSpawn(_) => "Unable to spawn Supervisor",
            Error::UserNotFound(_) => "No matching UID for user found",
        }
    }
}

impl From<Error> for protocol::ErrCode {
    fn from(err: Error) -> protocol::ErrCode {
        match err {
            Error::ExecWait(_) => protocol::ErrCode::ExecWait,
            Error::GroupNotFound(_) => protocol::ErrCode::GroupNotFound,
            Error::UserNotFound(_) => protocol::ErrCode::UserNotFound,
            _ => protocol::ErrCode::Unknown,
        }
    }
}

impl From<protocol::Error> for Error {
    fn from(err: protocol::Error) -> Error { Error::Protocol(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Spawn(err) }
}
