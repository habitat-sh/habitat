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
use std::ffi;
use std::fmt;
use std::io;
use std::path;
use std::result;

use depot_client;
use common;
use hcore;
use handlebars;
use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    ArgumentError(&'static str),
    ButterflyError(String),
    CommandNotFoundInPkg((String, String)),
    CryptoCLI(String),
    DepotClient(depot_client::Error),
    DockerDaemonDown,
    DockerFileSharingNotEnabled,
    DockerImageNotFound(String),
    DockerNetworkDown(String),
    ExecCommandNotFound(String),
    FFINulError(ffi::NulError),
    FileNotFound(String),
    HabitatCommon(common::Error),
    HabitatCore(hcore::Error),
    HandlebarsRenderError(handlebars::TemplateRenderError),
    IO(io::Error),
    PackageArchiveMalformed(String),
    PathPrefixError(path::StripPrefixError),
    ProvidesError(String),
    RootRequired,
    SubcommandNotSupported(String),
    UnsupportedExportFormat(String),
    TomlDeserializeError(toml::de::Error),
    TomlSerializeError(toml::ser::Error),
    Utf8Error(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ArgumentError(ref e) => format!("{}", e),
            Error::ButterflyError(ref e) => format!("{}", e),
            Error::CommandNotFoundInPkg((ref p, ref c)) => {
                format!("`{}' was not found under any 'PATH' directories in the {} package",
                        c,
                        p)
            }
            Error::CryptoCLI(ref e) => format!("{}", e),
            Error::DepotClient(ref err) => format!("{}", err),
            Error::DockerDaemonDown => {
                format!("Can not connect to Docker. Is the Docker daemon running?")
            }
            #[cfg(not(windows))]
            Error::DockerFileSharingNotEnabled => {
                format!("File Sharing must be enabled in order to enter a studio.\nPlease enable \
                         it in the Docker preferences and share (at a minimum) your home \
                         directory.")
            }
            #[cfg(windows)]
            Error::DockerFileSharingNotEnabled => {
                format!("File Sharing must be enabled in order to enter a studio.\nPlease select \
                         a drive to share in the Docker preferences.")
            }
            Error::DockerImageNotFound(ref e) => {
                format!("The Docker image {} was not found in the docker registry.\nYou can \
                         specify your own Docker image using the HAB_DOCKER_STUDIO_IMAGE \
                         environment variable.",
                        e)
            }
            Error::DockerNetworkDown(ref e) => {
                format!("The Docker image {} is unreachable due to a network error.\nThe \
                         image must be reachable to ensure the versions of hab inside and \
                         outside the studio match.\nYou can specify your own Docker image using \
                         the HAB_DOCKER_STUDIO_IMAGE environment variable.",
                        e)
            }
            Error::ExecCommandNotFound(ref c) => {
                format!("`{}' was not found on the filesystem or in PATH", c)
            }
            Error::FFINulError(ref e) => format!("{}", e),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::HabitatCommon(ref e) => format!("{}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HandlebarsRenderError(ref e) => format!("{}", e),
            Error::IO(ref err) => format!("{}", err),
            Error::PackageArchiveMalformed(ref e) => {
                format!("Package archive was unreadable or contained unexpected contents: {:?}",
                        e)
            }
            Error::PathPrefixError(ref err) => format!("{}", err),
            Error::ProvidesError(ref err) => format!("Can't find {}", err),
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation".to_string()
            }
            Error::SubcommandNotSupported(ref e) => {
                format!("Subcommand `{}' not supported on this operating system", e)
            }
            Error::UnsupportedExportFormat(ref e) => format!("Unsupported export format: {}", e),
            Error::TomlDeserializeError(ref e) => format!("Can't deserialize TOML: {}", e),
            Error::TomlSerializeError(ref e) => format!("Can't serialize TOML: {}", e),
            Error::Utf8Error(ref e) => format!("Error processing a string as UTF-8: {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ArgumentError(_) => "There was an error parsing an error or with it's value",
            Error::ButterflyError(_) => "Butterfly has had an error",
            Error::CommandNotFoundInPkg(_) => {
                "Command was not found under any 'PATH' directories in the package"
            }
            Error::CryptoCLI(_) => "A cryptographic error has occurred",
            Error::DepotClient(ref err) => err.description(),
            Error::DockerDaemonDown => "The Docker daemon could not be found.",
            Error::DockerFileSharingNotEnabled => "Docker file sharing is not enabled.",
            Error::DockerImageNotFound(_) => "The Docker image was not found.",
            Error::DockerNetworkDown(_) => "The Docker registry is unreachable.",
            Error::ExecCommandNotFound(_) => "Exec command was not found on filesystem or in PATH",
            Error::FFINulError(ref err) => err.description(),
            Error::FileNotFound(_) => "File not found",
            Error::HabitatCommon(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::HandlebarsRenderError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::PackageArchiveMalformed(_) => {
                "Package archive was unreadable or had unexpected contents"
            }
            Error::PathPrefixError(ref err) => err.description(),
            Error::ProvidesError(_) => {
                "Can't find a package that provides the given search parameter"
            }
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation"
            }
            Error::SubcommandNotSupported(_) => "Subcommand not supported on this operating system",
            Error::UnsupportedExportFormat(_) => "Unsupported export format",
            Error::TomlDeserializeError(_) => "Can't deserialize TOML",
            Error::TomlSerializeError(_) => "Can't serialize TOML",
            Error::Utf8Error(_) => "Error processing string as UTF-8",
        }
    }
}

impl From<common::Error> for Error {
    fn from(err: common::Error) -> Error {
        Error::HabitatCommon(err)
    }
}

impl From<depot_client::Error> for Error {
    fn from(err: depot_client::Error) -> Error {
        Error::DepotClient(err)
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

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Error {
        Error::HandlebarsRenderError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(err: path::StripPrefixError) -> Error {
        Error::PathPrefixError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::TomlDeserializeError(err)
    }
}
impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlSerializeError(err)
    }
}
