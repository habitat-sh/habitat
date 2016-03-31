// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::error;
use std::ffi;
use std::fmt;
use std::result;

use hcore::{self, package};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ExecCommandNotFound(String),
    FFINulError(ffi::NulError),
    HabitatCore(hcore::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ExecCommandNotFound(ref c) => {
                format!("`{}' was not found on the filesystem or in PATH", c)
            }
            Error::FFINulError(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ExecCommandNotFound(_) => "Exec command was not found on filesystem or in PATH",
            Error::FFINulError(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error {
        Error::FFINulError(err)
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error {
        Error::HabitatCore(err)
    }
}
