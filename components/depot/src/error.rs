// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::error;
use std::ffi;
use std::io;
use std::fmt;
use std::result;

use dbcache;
use hab_core;
use hab_core::package::{self, Identifiable};
use hab_net;
use hyper;
use redis;

#[derive(Debug)]
pub enum Error {
    BadPort(String),
    DataStore(dbcache::Error),
    HabitatCore(hab_core::Error),
    HabitatNet(hab_net::Error),
    HTTP(hyper::status::StatusCode),
    InvalidPackageIdent(String),
    IO(io::Error),
    NoXFilename,
    NoFilePart,
    NulError(ffi::NulError),
    RemotePackageNotFound(package::PackageIdent),
    WriteSyncFailed,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::DataStore(ref e) => format!("DataStore error, {}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HabitatNet(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: acme/redis)",
                        e)
            }
            Error::IO(ref e) => format!("{}", e),
            Error::NoXFilename => {
                format!("Invalid download from a Depot - missing X-Filename header")
            }
            Error::NoFilePart => {
                format!("An invalid path was passed - we needed a filename, and this path does \
                         not have one")
            }
            Error::NulError(ref e) => format!("{}", e),
            Error::RemotePackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package in any sources: {}", pkg)
                } else {
                    format!("Cannot find a release of package in any sources: {}", pkg)
                }
            }
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
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::DataStore(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::HabitatNet(ref err) => err.description(),
            Error::HTTP(_) => "Received an HTTP error",
            Error::InvalidPackageIdent(_) => {
                "Package identifiers must be in origin/name format (example: acme/redis)"
            }
            Error::IO(ref err) => err.description(),
            Error::NulError(_) => {
                "An attempt was made to build a CString with a null byte inside it"
            }
            Error::RemotePackageNotFound(_) => "Cannot find a package in any sources",
            Error::NoXFilename => "Invalid download from a Depot - missing X-Filename header",
            Error::NoFilePart => {
                "An invalid path was passed - we needed a filename, and this path does not have one"
            }
            Error::WriteSyncFailed => {
                "Could not write to destination; bytes written was 0 on a non-0 buffer"
            }
        }
    }
}

impl From<dbcache::Error> for Error {
    fn from(err: dbcache::Error) -> Error {
        Error::DataStore(err)
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error {
        Error::NulError(err)
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        let e = dbcache::Error::from(err);
        Error::DataStore(e)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Error {
        Error::HabitatNet(err)
    }
}
