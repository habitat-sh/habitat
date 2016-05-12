// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::error;
use std::io;
use std::fmt;
use std::result;

use hyper;
use url;

use hcore::{self, package};

#[derive(Debug)]
pub enum Error {
    HabitatCore(hcore::Error),
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
            Error::HTTP(ref e) => format!("{}", e),
            Error::HyperError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::NoFilePart => {
                format!("An invalid path was passed - we needed a filename, and this path does \
                         not have one")
            }
            Error::NoXFilename => format!("Invalid download from a Depot - missing X-Filename header"),
            Error::RemoteOriginKeyNotFound(ref e) => format!("{}", e),
            Error::RemotePackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package in any sources: {}", pkg)
                } else {
                    format!("Cannot find a release of package in any sources: {}", pkg)
                }
            }
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::WriteSyncFailed => format!("Could not write to destination; perhaps the disk is full?"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HabitatCore(ref err) => err.description(),
            Error::HTTP(_) => "Received an HTTP error",
            Error::HyperError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::NoFilePart => "An invalid path was passed - we needed a filename, and this path does not have one",
            Error::NoXFilename => "Invalid download from a Depot - missing X-Filename header",
            Error::RemoteOriginKeyNotFound(_) => "Remote origin key not found",
            Error::RemotePackageNotFound(_) => "Cannot find a package in any sources",
            Error::UrlParseError(ref err) => err.description(),
            Error::WriteSyncFailed => "Could not write to destination; bytes written was 0 on a non-0 buffer",
        }
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error {
        Error::HabitatCore(err)
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
