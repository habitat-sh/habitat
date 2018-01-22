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
use std::io;
use std::path::PathBuf;
use std::process;
use std::result;
use std::sync::mpsc;

use bldr_core;
use hab_core;
use git2;
use github_api_client;
use protobuf;
use protocol;
use retry;
use url;
use zmq;

use runner::studio;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AirlockNetworking(PathBuf, io::Error),
    AirlockFailure(process::ExitStatus),
    BuildEnvFile(PathBuf, io::Error),
    BuildFailure(i32),
    BuilderCore(bldr_core::Error),
    CannotAddCreds,
    Chown(PathBuf, u32, u32, io::Error),
    ChownWait(io::Error),
    Exporter(io::Error),
    Git(git2::Error),
    GithubAppAuthErr(github_api_client::HubError),
    HabitatCore(hab_core::Error),
    InvalidIntegrations(String),
    NoNetworkGatewayError,
    NoNetworkInterfaceError,
    NotHTTPSCloneUrl(url::Url),
    NoStudioGroup,
    NoStudioUser,
    Protobuf(protobuf::ProtobufError),
    Protocol(protocol::ProtocolError),
    Retry(retry::RetryError),
    StudioBuild(PathBuf, io::Error),
    StudioTeardown(PathBuf, io::Error),
    UrlParseError(url::ParseError),
    WorkspaceSetup(String, io::Error),
    WorkspaceTeardown(String, io::Error),
    Zmq(zmq::Error),
    Mpsc(mpsc::SendError<bldr_core::job::Job>),
    JobCanceled,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::AirlockFailure(ref e) => {
                format!("Airlock networking exited with non-zero exit code, {}", e)
            }
            Error::AirlockNetworking(ref p, ref e) => {
                format!(
                    "Error while running airlock networking command for {}, err={}",
                    p.display(),
                    e
                )
            }
            Error::BuildEnvFile(ref p, ref e) => {
                format!(
                    "Unable to read workspace build env file, {}, {}",
                    p.display(),
                    e
                )
            }
            Error::BuildFailure(ref e) => {
                format!("Build studio exited with non-zero exit code, {}", e)
            }
            Error::BuilderCore(ref e) => format!("{}", e),
            Error::CannotAddCreds => format!("Cannot add credentials to url"),
            Error::Chown(ref p, ref u, ref g, ref e) => {
                format!(
                    "Unable to recursively chown path, {} with '{}:{}', {}",
                    p.display(),
                    u,
                    g,
                    e
                )
            }
            Error::ChownWait(ref e) => format!("Unable to complete chown process, {}", e),
            Error::Exporter(ref e) => {
                format!("Unable to spawn or pipe data from exporter proc, {}", e)
            }
            Error::Git(ref e) => format!("{}", e),
            Error::GithubAppAuthErr(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::InvalidIntegrations(ref s) => format!("Invalid integration: {}", s),
            Error::NoNetworkGatewayError => format!("No network_gateway config specified"),
            Error::NoNetworkInterfaceError => format!("No network_interface config specified"),
            Error::NotHTTPSCloneUrl(ref e) => {
                format!(
                    "Attempted to clone {}. Only HTTPS clone urls are supported",
                    e
                )
            }
            Error::NoStudioGroup => {
                format!("System is missing studio group, {}", studio::STUDIO_GROUP)
            }
            Error::NoStudioUser => {
                format!("System is missing studio user, {}", studio::STUDIO_USER)
            }
            Error::Protobuf(ref e) => format!("{}", e),
            Error::Protocol(ref e) => format!("{}", e),
            Error::Retry(ref e) => format!("{}", e),
            Error::StudioBuild(ref p, ref e) => {
                format!(
                    "Error while running studio build at {}, err={}",
                    p.display(),
                    e
                )
            }
            Error::StudioTeardown(ref p, ref e) => {
                format!(
                    "Error while tearing down studio at {}, err={}",
                    p.display(),
                    e
                )
            }
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::WorkspaceSetup(ref p, ref e) => {
                format!("Error while setting up workspace at {}, err={}", p, e)
            }
            Error::WorkspaceTeardown(ref p, ref e) => {
                format!("Error while tearing down workspace at {}, err={}", p, e)
            }
            Error::Zmq(ref e) => format!("{}", e),
            Error::Mpsc(ref e) => format!("{}", e),
            Error::JobCanceled => format!("Job was canceled"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AirlockFailure(_) => "Airlock networking exited with a non-zero exit code",
            Error::AirlockNetworking(_, _) => "IO Error while running airlock networking command",
            Error::BuildEnvFile(_, _) => "Unable to read workspace build env file",
            Error::BuildFailure(_) => "Build studio exited with a non-zero exit code",
            Error::BuilderCore(ref err) => err.description(),
            Error::CannotAddCreds => "Cannot add credentials to url",
            Error::Chown(_, _, _, _) => "Unable to recursively chown path",
            Error::ChownWait(_) => "Unable to complete chown process",
            Error::Exporter(_) => "IO Error while spawning or piping data from exporter proc",
            Error::Git(ref err) => err.description(),
            Error::GithubAppAuthErr(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::InvalidIntegrations(_) => "Invalid integrations detected",
            Error::NoNetworkGatewayError => "No network_gateway config specified",
            Error::NoNetworkInterfaceError => "No network_interface config specified",
            Error::NotHTTPSCloneUrl(_) => "Only HTTPS clone urls are supported",
            Error::NoStudioGroup => "System missing group to run studio",
            Error::NoStudioUser => "System missing user to run studio",
            Error::Protobuf(ref err) => err.description(),
            Error::Protocol(ref err) => err.description(),
            Error::Retry(ref err) => err.description(),
            Error::StudioBuild(_, _) => "IO Error while running studio build",
            Error::StudioTeardown(_, _) => "IO Error while tearing down studio",
            Error::WorkspaceSetup(_, _) => "IO Error while creating workspace on disk",
            Error::WorkspaceTeardown(_, _) => "IO Error while destroying workspace on disk",
            Error::Zmq(ref err) => err.description(),
            Error::UrlParseError(ref err) => err.description(),
            Error::Mpsc(ref err) => err.description(),
            Error::JobCanceled => "Job was canceled",
        }
    }
}

impl From<bldr_core::Error> for Error {
    fn from(err: bldr_core::Error) -> Error {
        Error::BuilderCore(err)
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::Protobuf(err)
    }
}

impl From<protocol::ProtocolError> for Error {
    fn from(err: protocol::ProtocolError) -> Self {
        Error::Protocol(err)
    }
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Error {
        Error::Zmq(err)
    }
}
