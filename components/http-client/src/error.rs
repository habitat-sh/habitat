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

use hab_core;
use hyper;
use openssl::ssl;

#[derive(Debug)]
pub enum Error {
    HabitatCore(hab_core::Error),
    HyperError(hyper::error::Error),
    IO(io::Error),
    SslError(ssl::error::SslError),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HyperError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::SslError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HabitatCore(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::SslError(ref err) => err.description(),
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
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

impl From<ssl::error::SslError> for Error {
    fn from(err: ssl::error::SslError) -> Error {
        Error::SslError(err)
    }
}
