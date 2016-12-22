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
use std::fmt;
use std::io;
use std::result;

use git2;
use hab_core;
use protobuf;
use zmq;

#[derive(Debug)]
pub enum Error {
    BuildFailure(i32),
    Git(git2::Error),
    HabitatCore(hab_core::Error),
    IO(io::Error),
    Protobuf(protobuf::ProtobufError),
    UnknownVCS,
    WorkspaceSetup(String, io::Error),
    WorkspaceTeardown(String, io::Error),
    Zmq(zmq::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BuildFailure(ref e) => {
                format!("Build studio exited with non-zero exit code, {}", e)
            }
            Error::Git(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::UnknownVCS => format!("Job requires an unknown VCS"),
            Error::Zmq(ref e) => format!("{}", e),
            Error::WorkspaceSetup(ref p, ref e) => {
                format!("Error while setting up workspace at {}, err={:?}", p, e)
            }
            Error::WorkspaceTeardown(ref p, ref e) => {
                format!("Error while tearing down workspace at {}, err={:?}", p, e)
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BuildFailure(_) => "Build studio exited with a non-zero exit code",
            Error::Git(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
            Error::UnknownVCS => "Job requires an unknown VCS",
            Error::WorkspaceSetup(_, _) => "IO Error while creating workspace on disk",
            Error::WorkspaceTeardown(_, _) => "IO Error while destroying workspace on disk",
            Error::Zmq(ref err) => err.description(),
        }
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Error {
        Error::Git(err)
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::Protobuf(err)
    }
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Error {
        Error::Zmq(err)
    }
}
