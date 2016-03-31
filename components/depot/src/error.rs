// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::error;
use std::ffi;
use std::io;
use std::fmt;
use std::result;

use hcore::{self, package};
use hyper;

use data_store;

#[derive(Debug)]
pub enum Error {
    BadPort(String),
    DbInvalidPath,
    HabitatCore(hcore::Error),
    HTTP(hyper::status::StatusCode),
    InvalidPackageIdent(String),
    IO(io::Error),
    MdbError(data_store::MdbError),
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
            Error::DbInvalidPath => format!("Invalid filepath to internal datastore"),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: chef/redis)",
                        e)
            }
            Error::IO(ref e) => format!("{}", e),
            Error::MdbError(ref err) => format!("{}", err),
            Error::NoXFilename => format!("Invalid download from a Depot - missing X-Filename header"),
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
            Error::WriteSyncFailed => format!("Could not write to destination; perhaps the disk is full?"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::DbInvalidPath => "A bad filepath was provided for an internal datastore",
            Error::HabitatCore(ref err) => err.description(),
            Error::HTTP(_) => "Received an HTTP error",
            Error::InvalidPackageIdent(_) => "Package identifiers must be in origin/name format (example: chef/redis)",
            Error::IO(ref err) => err.description(),
            Error::MdbError(_) => "Database error",
            Error::NulError(_) => "An attempt was made to build a CString with a null byte inside it",
            Error::RemotePackageNotFound(_) => "Cannot find a package in any sources",
            Error::NoXFilename => "Invalid download from a Depot - missing X-Filename header",
            Error::NoFilePart => "An invalid path was passed - we needed a filename, and this path does not have one",
            Error::WriteSyncFailed => "Could not write to destination; bytes written was 0 on a non-0 buffer",
        }
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<data_store::MdbError> for Error {
    fn from(err: data_store::MdbError) -> Error {
        Error::MdbError(err)
    }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error {
        Error::NulError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}
