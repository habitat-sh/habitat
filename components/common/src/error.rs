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
use std::io;
use std::fmt;
use std::result;
use std::str;
use std::string;

use toml;

use depot_client;
use hcore;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ArtifactIdentMismatch((String, String, String)),
    CantUploadGossipToml,
    CryptoKeyError(String),
    GossipFileRelativePath(String),
    DepotClient(depot_client::Error),
    FileNameError,
    HabitatCore(hcore::Error),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    RootRequired,
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    TomlSerializeError(toml::ser::Error),
    WireDecode(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ArtifactIdentMismatch((ref a, ref ai, ref i)) => {
                format!("Artifact ident {} for `{}' does not match expected ident {}",
                        ai,
                        a,
                        i)
            }
            Error::CantUploadGossipToml => {
                format!("Can't upload gossip.toml, it's a reserved file name")
            }
            Error::CryptoKeyError(ref s) => format!("Missing or invalid key: {}", s),
            Error::GossipFileRelativePath(ref s) => {
                format!("Path for gossip file cannot have relative components (eg: ..): {}",
                        s)
            }
            Error::DepotClient(ref err) => format!("{}", err),
            Error::FileNameError => format!("Failed to extract a filename"),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref err) => format!("{}", err),
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation".to_string()
            }
            Error::StrFromUtf8Error(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TomlSerializeError(ref e) => format!("Can't serialize TOML: {}", e),
            Error::WireDecode(ref m) => format!("Failed to decode wire message: {}", m),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ArtifactIdentMismatch((_, _, _)) => {
                "Artifact ident does not match expected ident"
            }
            Error::CantUploadGossipToml => "Can't upload gossip.toml, it's a reserved filename",
            Error::CryptoKeyError(_) => "Missing or invalid key",
            Error::GossipFileRelativePath(_) => {
                "Path for gossip file cannot have relative components (eg: ..)"
            }
            Error::DepotClient(ref err) => err.description(),
            Error::FileNameError => "Failed to extract a filename from a path",
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation"
            }
            Error::StrFromUtf8Error(_) => "Failed to convert a string as UTF-8",
            Error::StringFromUtf8Error(_) => "Failed to convert a string as UTF-8",
            Error::TomlSerializeError(_) => "Can't serialize TOML",
            Error::WireDecode(_) => "Failed to decode wire message",
        }
    }
}

impl From<depot_client::Error> for Error {
    fn from(err: depot_client::Error) -> Self {
        Error::DepotClient(err)
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Self {
        Error::HabitatCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::StrFromUtf8Error(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::TomlSerializeError(err)
    }
}
