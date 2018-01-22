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

use hab_core;
use protocol;

use conn::ConnErr;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Connection(ConnErr),
    HabitatCore(hab_core::Error),
    Protocol(protocol::ProtocolError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Connection(ref e) => write!(f, "{}", e),
            Error::HabitatCore(ref e) => write!(f, "{}", e),
            Error::Protocol(ref e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Connection(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::Protocol(ref err) => err.description(),
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<ConnErr> for Error {
    fn from(err: ConnErr) -> Error {
        Error::Connection(err)
    }
}

impl From<protocol::ProtocolError> for Error {
    fn from(err: protocol::ProtocolError) -> Error {
        Error::Protocol(err)
    }
}
