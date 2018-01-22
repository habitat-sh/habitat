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
use std::string;

use base64;
use chrono;
use hab_core;

#[derive(Debug)]
pub enum Error {
    Base64Error(base64::DecodeError),
    ChronoError(chrono::format::ParseError),
    DecryptError(String),
    EncryptError(String),
    FromUtf8Error(string::FromUtf8Error),
    HabitatCore(hab_core::Error),
    TokenExpired,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Base64Error(ref e) => format!("{}", e),
            Error::ChronoError(ref e) => format!("{}", e),
            Error::DecryptError(ref e) => format!("{}", e),
            Error::EncryptError(ref e) => format!("{}", e),
            Error::FromUtf8Error(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::TokenExpired => format!("Token is expired"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Base64Error(ref e) => e.description(),
            Error::ChronoError(ref e) => e.description(),
            Error::DecryptError(_) => "Error decrypting integration",
            Error::EncryptError(_) => "Error encrypting integration",
            Error::FromUtf8Error(ref e) => e.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::TokenExpired => "Token is expired",
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}
