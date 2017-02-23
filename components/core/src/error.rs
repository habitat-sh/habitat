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
use std::num;
use std::result;
use std::str;
use std::string;

use libarchive;
use regex;

use package::{self, Identifiable};

pub type Result<T> = result::Result<T, Error>;

/// Core error types
#[derive(Debug)]
pub enum Error {
    /// Occurs when a `habitat_core::package::PackageArchive` is being read.
    ArchiveError(libarchive::error::ArchiveError),
    /// An invalid path to a keyfile was given.
    BadKeyPath(String),
    /// Error reading raw contents of configuration file.
    ConfigFileIO(io::Error),
    /// Parsing error while reading a configuration file.
    ConfigFileSyntax(String),
    /// Expected an array of socket addrs for configuration field value.
    ConfigInvalidArraySocketAddr(&'static str),
    /// Expected an array of tables containing string feilds and values for configuration
    /// field value.
    ConfigInvalidArrayTableString(&'static str),
    /// Expected an array of u16 entries for configuration field value.
    ConfigInvalidArrayU16(&'static str),
    /// Expected an array of u32 entries for configuration field value.
    ConfigInvalidArrayU32(&'static str),
    /// Expected an array of u64 entries for configuration field value.
    ConfigInvalidArrayU64(&'static str),
    /// Expected a boolean for configuration field value.
    ConfigInvalidBool(&'static str),
    /// Expected a package ident for configuration field value.
    ConfigInvalidIdent(&'static str),
    /// Expected a network address for configuration field value.
    ConfigInvalidIpAddr(&'static str),
    /// Expected a network address pair for configuration field value.
    ConfigInvalidSocketAddr(&'static str),
    /// Expected a string for configuration field value.
    ConfigInvalidString(&'static str),
    /// Expected a table of string fields and values for configuration field value.
    ConfigInvalidTableString(&'static str),
    /// Expected a package target for configuration field value.
    ConfigInvalidTarget(&'static str),
    /// Expected a u16 for configuration field value.
    ConfigInvalidU16(&'static str),
    /// Expected a u32 for configuration field value.
    ConfigInvalidU32(&'static str),
    /// Expected a u64 for configuration field value.
    ConfigInvalidU64(&'static str),
    /// Expected a usize for configuration field value.
    ConfigInvalidUsize(&'static str),
    /// Crypto library error
    CryptoError(String),
    /// Occurs when a file that should exist does not or could not be read.
    FileNotFound(String),
    /// Occurs when a package identifier string cannot be successfully parsed.
    InvalidPackageIdent(String),
    /// Occurs when a package target string cannot be successfully parsed.
    InvalidPackageTarget(String),
    /// Occurs when validating a package target for an unsupported architecture.
    InvalidArchitecture(String),
    /// Occurs when validating a package target for an unsupported platform.
    InvalidPlatform(String),
    /// Occurs when a service group string cannot be successfully parsed.
    InvalidServiceGroup(String),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    /// Occurs when a BIND or BIND_OPTIONAL MetaFile is read and contains a bad entry.
    MetaFileBadBind,
    /// Occurs when a package metadata file cannot be opened, read, or parsed.
    MetaFileMalformed(package::metadata::MetaFile),
    /// Occurs when a particular package metadata file is not found.
    MetaFileNotFound(package::metadata::MetaFile),
    /// When an IO error while accessing a MetaFile.
    MetaFileIO(io::Error),
    /// Occurs when we can't find an outbound IP address
    NoOutboundAddr,
    /// Occurs when a suitable installed package cannot be found.
    PackageNotFound(package::PackageIdent),
    /// When an error occurs parsing an integer.
    ParseIntError(num::ParseIntError),
    /// Occurs when setting ownership or permissions on a file or directory fails.
    PermissionFailed(String),
    /// Error parsing the contents of a plan file were incomplete or malformed.
    PlanMalformed,
    /// When an error occurs parsing or compiling a regular expression.
    RegexParse(regex::Error),
    /// When an error occurs converting a `String` from a UTF-8 byte vector.
    StringFromUtf8Error(string::FromUtf8Error),
    /// When the system target (platform and architecture) do not match the package target.
    TargetMatchError(String),
    /// Occurs when a `uname` libc call returns an error.
    UnameFailed(String),
    /// Occurs when a `waitpid` libc call returns an error.
    WaitpidFailed(String),
    /// Occurs when a `kill` libc call returns an error.
    SignalFailed(i32),
    /// Occurs when a `GetExitCodeProcess` win32 call returns an error.
    GetExitCodeProcessFailed(String),
    /// Occurs when a `HabChild` constructor fails to return a process.
    GetHabChildFailed(String),
    /// Occurs when a `TerminateProcess` win32 call returns an error.
    TerminateProcessFailed(String),
    /// When an error occurs attempting to interpret a sequence of u8 as a string.
    Utf8Error(str::Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ArchiveError(ref err) => format!("{}", err),
            Error::BadKeyPath(ref e) => {
                format!("Invalid keypath: {}. Specify an absolute path to a file on disk.",
                        e)
            }
            Error::ConfigFileIO(ref e) => format!("Error reading configuration file: {}", e),
            Error::ConfigFileSyntax(ref e) => {
                format!("Syntax errors while parsing TOML configuration file:\n\n{}",
                        e)
            }
            Error::ConfigInvalidArraySocketAddr(ref f) => {
                format!("Invalid array value of network address pair strings config, field={}. \
                         (example: [\"127.0.0.1:8080\", \"10.0.0.4:22\"])",
                        f)
            }
            Error::ConfigInvalidArrayTableString(ref f) => {
                format!("Invalid array value of tables containing string fields and values in \
                         config, field={}",
                        f)
            }
            Error::ConfigInvalidArrayU16(ref f) => {
                format!("Invalid array value of u16 entries in config, field={}. (example: [1, 2])",
                        f)
            }
            Error::ConfigInvalidArrayU32(ref f) => {
                format!("Invalid array value of u32 entries in config, field={}. (example: [1, 2])",
                        f)
            }
            Error::ConfigInvalidArrayU64(ref f) => {
                format!("Invalid array value of u64 entries in config, field={}. (example: [1, 2])",
                        f)
            }
            Error::ConfigInvalidBool(ref f) => {
                format!("Invalid boolean value in config, field={}. (example: true)",
                        f)
            }
            Error::ConfigInvalidIdent(ref f) => {
                format!("Invalid package identifier string value in config, field={}. (example: \
                         \"core/redis\")",
                        f)
            }
            Error::ConfigInvalidIpAddr(ref f) => {
                format!("Invalid IP address string value in config, field={}. (example: \
                         \"127.0.0.0\")",
                        f)
            }
            Error::ConfigInvalidSocketAddr(ref f) => {
                format!("Invalid network address pair string value in config, field={}. (example: \
                         \"127.0.0.0:8080\")",
                        f)
            }
            Error::ConfigInvalidString(ref f) => {
                format!("Invalid string value in config, field={}.", f)
            }
            Error::ConfigInvalidTableString(ref f) => {
                format!("Invalid table value of string fields and values in config, field={}",
                        f)
            }
            Error::ConfigInvalidTarget(ref f) => {
                format!("Invalid package target string value in config, field={}. (example: \
                         \"x86_64-linux\")",
                        f)
            }
            Error::ConfigInvalidU16(ref f) => format!("Invalid u16 value in config, field={}", f),
            Error::ConfigInvalidU32(ref f) => format!("Invalid u32 value in config, field={}", f),
            Error::ConfigInvalidU64(ref f) => format!("Invalid u64 value in config, field={}", f),
            Error::ConfigInvalidUsize(ref f) => {
                format!("Invalid usize value in config, field={}", f)
            }
            Error::CryptoError(ref e) => format!("Crypto error: {}", e),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: acme/redis)",
                        e)
            }
            Error::InvalidPackageTarget(ref e) => {
                format!("Invalid package target: {}. A valid target is in the form \
                         architecture-platform (example: x86_64-linux)",
                        e)
            }
            Error::InvalidArchitecture(ref e) => format!("Invalid architecture: {}.", e),
            Error::InvalidPlatform(ref e) => format!("Invalid platform: {}.", e),
            Error::InvalidServiceGroup(ref e) => {
                format!("Invalid service group: {}. A valid service group string is in the form \
                         service.group (example: redis.production)",
                        e)
            }
            Error::IO(ref err) => format!("{}", err),
            Error::MetaFileBadBind => format!("Bad value parsed from BIND or BIND_OPTIONAL"),
            Error::MetaFileMalformed(ref e) => {
                format!("MetaFile: {:?}, didn't contain a valid UTF-8 string", e)
            }
            Error::MetaFileNotFound(ref e) => format!("Couldn't read MetaFile: {}, not found", e),
            Error::MetaFileIO(ref e) => format!("IO error while accessing MetaFile: {:?}", e),
            Error::NoOutboundAddr => format!("Failed to discover this hosts outbound IP address"),
            Error::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::ParseIntError(ref e) => format!("{}", e),
            Error::PlanMalformed => format!("Failed to read or parse contents of Plan file"),
            Error::PermissionFailed(ref e) => format!("{}", e),
            Error::RegexParse(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TargetMatchError(ref e) => format!("{}", e),
            Error::UnameFailed(ref e) => format!("{}", e),
            Error::WaitpidFailed(ref e) => format!("{}", e),
            Error::SignalFailed(ref e) => {
                format!("Failed to send a signal to the child process: {}", e)
            }
            Error::GetExitCodeProcessFailed(ref e) => format!("{}", e),
            Error::GetHabChildFailed(ref e) => format!("{}", e),
            Error::TerminateProcessFailed(ref e) => format!("{}", e),
            Error::Utf8Error(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ArchiveError(ref err) => err.description(),
            Error::BadKeyPath(_) => "An absolute path to a file on disk is required",
            Error::ConfigFileIO(_) => "Unable to read the raw contents of a configuration file",
            Error::ConfigFileSyntax(_) => "Error parsing contents of configuration file",
            Error::ConfigInvalidArraySocketAddr(_) => {
                "Invalid array value of network address pair strings encountered while parsing a \
                 configuration file"
            }
            Error::ConfigInvalidArrayTableString(_) => {
                "Invalid array value of tables containing string fields and values encountered \
                 while parsing a configuration file"
            }
            Error::ConfigInvalidArrayU16(_) => {
                "Invalid array value of u16 entries encountered while parsing a configuration file"
            }
            Error::ConfigInvalidArrayU32(_) => {
                "Invalid array value of u32 entries encountered while parsing a configuration file"
            }
            Error::ConfigInvalidArrayU64(_) => {
                "Invalid array value of u64 entries encountered while parsing a configuration file"
            }
            Error::ConfigInvalidBool(_) => {
                "Invalid boolean value encountered while parsing a configuration file"
            }
            Error::ConfigInvalidIdent(_) => {
                "Invalid package identifier string value encountered while parsing a configuration \
                 file"
            }
            Error::ConfigInvalidIpAddr(_) => {
                "Invalid IP address string value encountered while parsing a configuration file"
            }
            Error::ConfigInvalidSocketAddr(_) => {
                "Invalid network address pair string value encountered while parsing a \
                 configuration file"
            }
            Error::ConfigInvalidString(_) => {
                "Invalid string value encountered while parsing a configuration file"
            }
            Error::ConfigInvalidTableString(_) => {
                "Invalid table value of string fields and values encountered while parsing a \
                 configuration file"
            }
            Error::ConfigInvalidTarget(_) => {
                "Invalid package target string value encountered while parsing a configuration file"
            }
            Error::ConfigInvalidU16(_) => {
                "Invalid u16 value encountered while parsing a configuration file"
            }
            Error::ConfigInvalidU32(_) => {
                "Invalid u32 value encountered while parsing a configuration file"
            }
            Error::ConfigInvalidU64(_) => {
                "Invalid u64 value encountered while parsing a configuration file"
            }
            Error::ConfigInvalidUsize(_) => {
                "Invalid usize value encountered while parsing a configuration file"
            }
            Error::CryptoError(_) => "Crypto error",
            Error::FileNotFound(_) => "File not found",
            Error::InvalidPackageIdent(_) => {
                "Package identifiers must be in origin/name format (example: acme/redis)"
            }
            Error::InvalidPackageTarget(_) => {
                "Package targets must be in architecture-platform format (example: x86_64-linux)"
            }
            Error::InvalidArchitecture(_) => "Unsupported target architecture supplied.",
            Error::InvalidPlatform(_) => "Unsupported target platform supplied.",
            Error::InvalidServiceGroup(_) => {
                "Service group strings must be in service.group format (example: redis.production)"
            }
            Error::IO(ref err) => err.description(),
            Error::MetaFileBadBind => "Bad value parsed from BIND or BIND_OPTIONAL MetaFile",
            Error::MetaFileMalformed(_) => "MetaFile didn't contain a valid UTF-8 string",
            Error::MetaFileNotFound(_) => "Failed to read an archive's metafile",
            Error::MetaFileIO(_) => "MetaFile could not be read or written to",
            Error::NoOutboundAddr => "Failed to discover the outbound IP address",
            Error::PackageNotFound(_) => "Cannot find a package",
            Error::ParseIntError(_) => "Failed to parse an integer from a string!",
            Error::PermissionFailed(_) => "Failed to set permissions",
            Error::PlanMalformed => "Failed to read or parse contents of Plan file",
            Error::RegexParse(_) => "Failed to parse a regular expression",
            Error::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            Error::TargetMatchError(_) => "System target does not match package target",
            Error::UnameFailed(_) => "uname failed",
            Error::SignalFailed(_) => "Failed to send a signal to the child process",
            Error::WaitpidFailed(_) => "waitpid failed",
            Error::GetExitCodeProcessFailed(_) => "GetExitCodeProcess failed",
            Error::GetHabChildFailed(_) => "Failed to return a HabChild",
            Error::TerminateProcessFailed(_) => "Failed to call TerminateProcess",
            Error::Utf8Error(_) => "Failed to interpret a sequence of bytes as a string",
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<libarchive::error::ArchiveError> for Error {
    fn from(err: libarchive::error::ArchiveError) -> Self {
        Error::ArchiveError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::RegexParse(err)
    }
}
