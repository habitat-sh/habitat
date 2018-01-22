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
use std::path::PathBuf;
use std::process;
use std::result;

use unshare;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    Command(process::ExitStatus),
    CreateMaster(String),
    EffectiveGroupnameNotFound,
    EffectiveUsernameNotFound,
    FileEntryNotFound(String, String),
    FileNotFound(String),
    FsRoot(PathBuf, io::Error),
    Grantpt(String),
    GidNotFound(u32),
    GroupnameNotFound(String),
    GroupNotFound(String),
    HomeDirectoryNotFound,
    InterfaceNotFound(String),
    IpAddressNotFound(String),
    IO(io::Error),
    Mount(String),
    NulError(ffi::NulError),
    PackageNotFound(String),
    PivotRoot(String),
    ProgramNotFound(String),
    Ptsname(String),
    RootUserRequired,
    Setns(String),
    SubGidRangeTooSmall(u32, u32),
    SubUidRangeTooSmall(u32, u32),
    Unlockpt(String),
    Unshare(unshare::Error),
    UserNotInGroup(String, String),
    UsernameNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Command(ref e) => format!("Error calling command, exited {}", e),
            Error::CreateMaster(ref e) => format!("Error creating pseudoterminal master, {}", e),
            Error::EffectiveGroupnameNotFound => String::from(
                "Could not determine groupname of process",
            ),
            Error::EffectiveUsernameNotFound => String::from(
                "Could not determine username of process",
            ),
            Error::FileEntryNotFound(ref e, ref f) => {
                format!("Could not find file entry {} in {}", e, f)
            }
            Error::FileNotFound(ref f) => format!("Could not find file {}", f),
            Error::FsRoot(ref p, ref e) => format!("FsRoot error for {}, {}", p.display(), e),
            Error::Grantpt(ref e) => format!("Error calling grantpt, {}", e),
            Error::GidNotFound(ref g) => format!("Could not find group for gid: '{}'", g),
            Error::GroupnameNotFound(ref g) => format!("Could not find local groupname: '{}'", g),
            Error::GroupNotFound(ref g) => format!("Could not find {} unix group", g),
            Error::HomeDirectoryNotFound => String::from(
                "Could not determine user's home directory",
            ),
            Error::InterfaceNotFound(ref i) => format!("Could not find network interface {}", i),
            Error::IpAddressNotFound(ref i) => {
                format!("Could not find IP address for network interface {}", i)
            }
            Error::IO(ref e) => format!("{}", e),
            Error::Mount(ref e) => format!("Error calling mount, {}", e),
            Error::NulError(ref e) => format!("Error encoding c string, {}", e),
            Error::PackageNotFound(ref p) => format!("Could not find package {}", p),
            Error::PivotRoot(ref e) => format!("Error calling pivot_root, {}", e),
            Error::ProgramNotFound(ref p) => format!("Could not find program {}", p),
            Error::Ptsname(ref e) => format!("Error calling ptsname, {}", e),
            Error::RootUserRequired => String::from("This command must be run as the root user"),
            Error::Setns(ref e) => format!("Error calling setns, {}", e),
            Error::SubGidRangeTooSmall(ref r, ref m) => {
                format!(
                    "Range '{}' in subgid is too small for user, minimum required: '{}'",
                    r,
                    m
                )
            }
            Error::SubUidRangeTooSmall(ref r, ref m) => {
                format!(
                    "Range '{}' in subuid is too small for user, minimum required: '{}'",
                    r,
                    m
                )
            }
            Error::Unlockpt(ref e) => format!("Error calling unlockpt, {}", e),
            Error::Unshare(ref e) => format!("Unshare error: {}", e),
            Error::UserNotInGroup(ref u, ref g) => {
                format!("User '{}' is not a member of the '{}' unix group", u, g)
            }
            Error::UsernameNotFound(ref u) => format!("Could not find local username: '{}'", u),
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
