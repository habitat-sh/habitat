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
use std::io;
use std::result;

use hab_core;
use hab_net;
use depot;
use hyper;
use protobuf;
use zmq;

#[derive(Debug)]
pub enum Error {
    BadPort(String),
    Depot(depot::Error),
    HabitatCore(hab_core::Error),
    HyperError(hyper::error::Error),
    HTTP(hyper::status::StatusCode),
    IO(io::Error),
    NetError(hab_net::NetError),
    Protobuf(protobuf::ProtobufError),
    UnknownGitHubEvent(String),
    Zmq(zmq::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::Depot(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HyperError(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::UnknownGitHubEvent(ref e) => {
                format!("Unknown or unsupported GitHub event, {}", e)
            }
            Error::Zmq(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::Depot(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::IO(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
            Error::UnknownGitHubEvent(_) => {
                "Unknown or unsupported GitHub event received in request"
            }
            Error::Zmq(ref err) => err.description(),
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<hab_net::NetError> for Error {
    fn from(err: hab_net::NetError) -> Self {
        Error::NetError(err)
    }
}

impl From<depot::Error> for Error {
    fn from(err: depot::Error) -> Error {
        Error::Depot(err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Self {
        Error::HyperError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::Protobuf(err)
    }
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Error {
        Error::Zmq(err)
    }
}
