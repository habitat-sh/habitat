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

use std::ffi;
use std::fmt;
use std::io;
use std::result;

use unshare;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    CreateMaster(String),
    FileEntryNotFound(String, String),
    FileNotFound(String),
    Grantpt(String),
    GroupnameNotFound,
    IO(io::Error),
    Mount(String),
    NulError(ffi::NulError),
    PivotRoot(String),
    ProgramNotFound(String),
    Ptsname(String),
    Unlockpt(String),
    Unshare(unshare::Error),
    UsernameNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::CreateMaster(ref e) => format!("Error creating pseudoterminal master, {}", e),
            Error::FileEntryNotFound(ref e, ref f) => {
                format!("Could not find file entry {} in {}", e, f)
            }
            Error::FileNotFound(ref f) => format!("Could not find file {}", f),
            Error::Grantpt(ref e) => format!("Error calling grantpt, {}", e),
            Error::GroupnameNotFound => String::from("Could not determine groupname of process"),
            Error::IO(ref e) => format!("{}", e),
            Error::Mount(ref e) => format!("Error calling mount, {}", e),
            Error::NulError(ref e) => format!("Error encoding c string, {}", e),
            Error::PivotRoot(ref e) => format!("Error calling pivot_root, {}", e),
            Error::ProgramNotFound(ref p) => format!("Could not find program {}", p),
            Error::Ptsname(ref e) => format!("Error calling ptsname, {}", e),
            Error::Unlockpt(ref e) => format!("Error calling unlockpt, {}", e),
            Error::Unshare(ref e) => format!("Unshare error: {}", e),
            Error::UsernameNotFound => String::from("Could not determine username of process"),
        };
        write!(f, "{}", msg)
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

impl From<unshare::Error> for Error {
    fn from(err: unshare::Error) -> Error {
        Error::Unshare(err)
    }
}
