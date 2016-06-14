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
use std::result;

use hyper;
use url;

use hab_core::{self, package};
use hab_core::package::Identifiable;
use hab_http;

#[derive(Debug)]
pub enum Error {
    HabitatCore(hab_core::Error),
    HabitatHttpClient(hab_http::Error),
    HTTP(hyper::status::StatusCode),
    HyperError(hyper::error::Error),
    IO(io::Error),
    NoFilePart,
    NoXFilename,
    RemoteOriginKeyNotFound(String),
    RemotePackageNotFound(package::PackageIdent),
    UrlParseError(url::ParseError),
    WriteSyncFailed,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HabitatHttpClient(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::HyperError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::NoFilePart => {
                format!("An invalid path was passed - we needed a filename, and this path does \
                         not have one")
            }
            Error::NoXFilename => {
                format!("Invalid download from a Depot - missing X-Filename header")
            }
            Error::RemoteOriginKeyNotFound(ref e) => format!("{}", e),
            Error::RemotePackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package in any sources: {}", pkg)
                } else {
                    format!("Cannot find a release of package in any sources: {}", pkg)
                }
            }
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::WriteSyncFailed => {
                format!("Could not write to destination; perhaps the disk is full?")
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HabitatCore(ref err) => err.description(),
            Error::HabitatHttpClient(ref err) => err.description(),
            Error::HTTP(_) => "Received an HTTP error",
            Error::HyperError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::NoFilePart => {
                "An invalid path was passed - we needed a filename, and this path does not have one"
            }
            Error::NoXFilename => "Invalid download from a Depot - missing X-Filename header",
            Error::RemoteOriginKeyNotFound(_) => "Remote origin key not found",
            Error::RemotePackageNotFound(_) => "Cannot find a package in any sources",
            Error::UrlParseError(ref err) => err.description(),
            Error::WriteSyncFailed => {
                "Could not write to destination; bytes written was 0 on a non-0 buffer"
            }
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<hab_http::Error> for Error {
    fn from(err: hab_http::Error) -> Error {
        Error::HabitatHttpClient(err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::HyperError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}
