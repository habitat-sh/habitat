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

use std::error;
use std::fmt;
use std::io;
use std::result;

use crate::protocol;
use ipc_channel;
use protobuf;

#[derive(Debug)]
pub enum Error {
    AcceptConn,
    BadPipe(io::Error),
    Connect(io::Error),
    Deserialize(protobuf::ProtobufError),
    IPCBincode(String),
    IPCIO(io::ErrorKind),
    Protocol(protocol::NetErr),
    Send(ipc_channel::Error),
    Serialize(protobuf::ProtobufError),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::AcceptConn => "Unable to accept connection from Launcher".to_string(),
            Error::BadPipe(ref e) => format!("Unable to open pipe to Launcher, {}", e),
            Error::Connect(ref e) => format!("Unable to connect to Launcher's pipe, {}", e),
            Error::Deserialize(ref e) => {
                format!("Unable to deserialize message from Launcher, {}", e)
            }
            Error::IPCBincode(ref e) => {
                format!("Unable to read message frame from Launcher, {}", e)
            }
            Error::IPCIO(ref e) => format!("Unable to receive message from Launcher, {:?}", e),
            Error::Protocol(ref e) => format!("{}", e),
            Error::Send(ref e) => format!("Unable to send to Launcher's pipe, {}", e),
            Error::Serialize(ref e) => format!("Unable to serialize message to Launcher, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AcceptConn => "Unable to accept connection from Launcher",
            Error::BadPipe(_) => "Unable to open pipe to Launcher",
            Error::Connect(_) => "Unable to connect to Launcher's pipe",
            Error::Deserialize(_) => "Unable to deserialize message from Launcher",
            Error::IPCBincode(_) => "Unable to encode/decode message framing to/from Launcher",
            Error::IPCIO(_) => "Unable to receive message from Launcher",
            Error::Protocol(_) => "Received an error from Launcher",
            Error::Send(_) => "Unable to send to Launcher's pipe",
            Error::Serialize(_) => "Unable to serialize message to Launcher",
        }
    }
}

impl From<ipc_channel::ErrorKind> for Error {
    fn from(err: ipc_channel::ErrorKind) -> Error {
        match err {
            ipc_channel::ErrorKind::Io(io) => Error::IPCIO(io.kind()),
            _ => Error::IPCBincode(err.to_string()),
        }
    }
}
