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

use std::error;
use std::io;
use std::fmt;
use std::net;
use std::result;

use hcore;

#[derive(Debug)]
pub enum Error {
    AddrParseError(net::AddrParseError),
    DirectorError(String),
    HabitatCore(hcore::Error),
    IO(io::Error),
    NoServices,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::AddrParseError(ref e) => format!("Can't parse IP address {}", e),
            Error::DirectorError(ref e) => format!("Director error: {}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::NoServices => "No services specified in configuration".to_string(),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AddrParseError(_) => "Can't parse IP address",
            Error::DirectorError(_) => "Director Error",
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::NoServices => "No services specified in configuration",
        }
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error {
        Error::HabitatCore(err)
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::AddrParseError(err)
    }
}
