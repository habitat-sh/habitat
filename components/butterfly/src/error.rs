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

use habitat_core;

use protobuf;
use zmq;

use std::io;
use std::string::FromUtf8Error;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadMessage(String),
    CannotBind(io::Error),
    HabitatCore(habitat_core::error::Error),
    NonExistentRumor(String, String),
    ProtobufError(protobuf::ProtobufError),
    ServiceConfigNotUtf8(FromUtf8Error),
    SocketSetReadTimeout(io::Error),
    SocketSetWriteTimeout(io::Error),
    SocketCloneError,
    ZmqConnectError(zmq::Error),
    ZmqSendError(zmq::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadMessage(ref err) => format!("Bad Message: {:?}", err),
            Error::CannotBind(ref err) => format!("Cannot bind to port: {:?}", err),
            Error::HabitatCore(ref err) => format!("{}", err),
            Error::NonExistentRumor(ref member_id, ref rumor_id) => {
                format!("Non existent rumor asked to be written to bytes: {} {}",
                        member_id,
                        rumor_id)
            }
            Error::ProtobufError(ref err) => format!("ProtoBuf Error: {}", err),
            Error::ServiceConfigNotUtf8(ref err) => {
                format!("Cannot decode service configuration; it is not UTF-8: {}",
                        err)
            }
            Error::SocketSetReadTimeout(ref err) => {
                format!("Cannot set UDP socket read timeout: {}", err)
            }
            Error::SocketSetWriteTimeout(ref err) => {
                format!("Cannot set UDP socket write timeout: {}", err)
            }
            Error::SocketCloneError => format!("Cannot clone the underlying UDP socket"),
            Error::ZmqConnectError(ref err) => format!("Cannot connect ZMQ socket: {}", err),
            Error::ZmqSendError(ref err) => {
                format!("Cannot send message through ZMQ socket: {}", err)
            }

        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadMessage(ref _err) => "Bad Protobuf Message; should be Ping/Ack/PingReq",
            Error::CannotBind(ref _err) => "Cannot bind to port",
            Error::HabitatCore(ref _err) => "Habitat core error",
            Error::NonExistentRumor(ref _member_id, ref _rumor_id) => {
                "Cannot write rumor to bytes because it does not exist"
            }
            Error::ProtobufError(ref err) => err.description(),
            Error::ServiceConfigNotUtf8(ref _err) => "Cannot convert a service config to UTF-8",
            Error::SocketSetReadTimeout(ref _err) => "Cannot set UDP socket read timeout",
            Error::SocketSetWriteTimeout(ref _err) => "Cannot set UDP socket write timeout",
            Error::SocketCloneError => "Cannot clone the underlying UDP socket",
            Error::ZmqConnectError(ref _err) => "Cannot connect ZMQ socket",
            Error::ZmqSendError(ref _err) => "Cannot send message through ZMQ socket",
        }
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::ProtobufError(err)
    }
}

impl From<habitat_core::error::Error> for Error {
    fn from(err: habitat_core::error::Error) -> Error {
        Error::HabitatCore(err)
    }
}
