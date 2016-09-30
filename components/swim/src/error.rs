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

use protobuf;

use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadMessage(String),
    CannotBind(io::Error),
    ProtobufError(protobuf::ProtobufError),
    SocketSetReadTimeout(io::Error),
    SocketSetWriteTimeout(io::Error),
    ServerCloneError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadMessage(ref err) => format!("Bad Message: {:?}", err),
            Error::CannotBind(ref err) => format!("Cannot bind to port: {:?}", err),
            Error::ProtobufError(ref err) => format!("ProtoBuf Error: {}", err),
            Error::SocketSetReadTimeout(ref err) => {
                format!("Cannot set UDP socket read timeout: {}", err)
            }
            Error::SocketSetWriteTimeout(ref err) => {
                format!("Cannot set UDP socket write timeout: {}", err)
            },
            Error::ServerCloneError => format!("Cannot clone the underlying UDP socket"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadMessage(ref _err) => "Bad Protobuf Message; should be Ping/Ack/PingReq",
            Error::CannotBind(ref _err) => "Cannot bind to port",
            Error::ProtobufError(ref err) => err.description(),
            Error::SocketSetReadTimeout(ref _err) => "Cannot set UDP socket read timeout",
            Error::SocketSetWriteTimeout(ref _err) => "Cannot set UDP socket write timeout",
            Error::ServerCloneError => "Cannot clone the underlying UDP socket",
        }
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::ProtobufError(err)
    }
}
