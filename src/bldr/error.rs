//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::io;
use std::result;
use std::fmt;
use std::error::Error;
use std::num;
use std::string;
use std::ffi;
use std::sync::mpsc;

use wonder::actor;

use hyper;
use toml;
use mustache;
use regex;

#[derive(Debug)]
pub enum BldrError {
    Io(io::Error),
    CommandNotImplemented,
    InstallFailed,
    WriteSyncFailed,
    HyperError(hyper::error::Error),
    CannotParseFileName,
    PathUTF8,
    GPGVerifyFailed,
    UnpackFailed,
    TomlParser(Vec<toml::ParserError>),
    MustacheEncoderError(mustache::encoder::Error),
    GPGImportFailed,
    PermissionFailed,
    BadVersion,
    RegexParse(regex::Error),
    ParseIntError(num::ParseIntError),
    FileNameError,
    PackageNotFound,
    MustacheMergeOnlyMaps,
    SupervisorSignalFailed,
    StringFromUtf8Error(string::FromUtf8Error),
    SupervisorDied,
    NulError(ffi::NulError),
    IPFailed,
    HostnameFailed,
    UnknownTopology(String),
    NoConfiguration,
    HealthCheck(String),
    TryRecvError(mpsc::TryRecvError),
    BadWatch(String),
    NoXFilename,
    NoFilePart,
    SignalNotifierStarted,
    ActorError(actor::ActorError),
}

pub type BldrResult<T> = result::Result<T, BldrError>;

impl fmt::Display for BldrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BldrError::Io(ref err) => err.fmt(f),
            BldrError::CommandNotImplemented => write!(f, "Command is not yet implemented!"),
            BldrError::InstallFailed => write!(f, "Could not install package!"),
            BldrError::HyperError(ref err) => err.fmt(f),
            BldrError::WriteSyncFailed => write!(f, "Could not write to destination; perhaps the disk is full?"),
            BldrError::CannotParseFileName => write!(f, "Cannot determine the filename from the given URI"),
            BldrError::PathUTF8 => write!(f, "Paths must not contain non-UTF8 characters"),
            BldrError::GPGVerifyFailed => write!(f, "Failed to verify a GPG Signature"),
            BldrError::UnpackFailed => write!(f, "Failed to unpack a package"),
            BldrError::TomlParser(ref errs) => {
                write!(f, "Failed to parse toml:\n{}", toml_parser_string(errs))
            },
            BldrError::MustacheEncoderError(ref me) => {
                match *me {
                    mustache::encoder::Error::IoError(ref e) => e.fmt(f),
                    _ => write!(f, "Mustache encoder error: {:?}", me),
                }
            },
            BldrError::GPGImportFailed => write!(f, "Failed to import a GPG key"),
            BldrError::PermissionFailed => write!(f, "Failed to set permissions"),
            BldrError::BadVersion => write!(f, "Failed to parse a version number"),
            BldrError::RegexParse(ref e) => e.fmt(f),
            BldrError::ParseIntError(ref e) => e.fmt(f),
            BldrError::FileNameError => write!(f, "Failed to extract a filename"),
            BldrError::PackageNotFound => write!(f, "Cannot find a package"),
            BldrError::MustacheMergeOnlyMaps => write!(f, "Can only merge two Mustache::Data::Maps"),
            BldrError::SupervisorSignalFailed => write!(f, "Failed to send a signal to the process supervisor"),
            BldrError::StringFromUtf8Error(ref e) => e.fmt(f),
            BldrError::SupervisorDied => write!(f, "The supervisor died"),
            BldrError::NulError(ref e) => e.fmt(f),
            BldrError::IPFailed => write!(f, "Failed to discover this hosts outbound IP address"),
            BldrError::HostnameFailed => write!(f, "Failed to discover this hosts hostname"),
            BldrError::UnknownTopology(ref t) => write!(f, "Unknown topology {}!", t),
            BldrError::NoConfiguration => write!(f, "No configuration data - cannot continue"),
            BldrError::HealthCheck(ref e) => write!(f, "Health Check failed: {}", e),
            BldrError::TryRecvError(ref err) => err.fmt(f),
            BldrError::BadWatch(ref e) => write!(f, "Bad watch format: {} is not valid", e),
            BldrError::NoXFilename => write!(f, "Invalid download from a repository - missing X-Filename header"),
            BldrError::NoFilePart => write!(f, "An invalid path was passed - we needed a filename, and this path does not have one"),
            BldrError::SignalNotifierStarted => write!(f, "Only one instance of a Signal Notifier may be running"),
            BldrError::ActorError(ref err) => write!(f, "Actor returned error: {:?}", err),
        }
    }
}

impl Error for BldrError {
    fn description(&self) -> &str {
        match *self {
            BldrError::Io(ref err) => err.description(),
            BldrError::CommandNotImplemented => "Command is not yet implemented!",
            BldrError::InstallFailed => "Could not install package!",
            BldrError::WriteSyncFailed => "Could not write to destination; bytes written was 0 on a non-0 buffer",
            BldrError::CannotParseFileName => "Cannot determine the filename from the given URI",
            BldrError::HyperError(ref err) => err.description(),
            BldrError::PathUTF8 => "Paths must not contain non-UTF8 characters",
            BldrError::GPGVerifyFailed => "Failed to verify a GPG Signature",
            BldrError::UnpackFailed => "Failed to unpack a package",
            BldrError::TomlParser(_) => "Failed to parse toml!",
            BldrError::MustacheEncoderError(_) => "Failed to encode mustache template",
            BldrError::GPGImportFailed => "Failed to import a GPG key",
            BldrError::PermissionFailed => "Failed to set permissions",
            BldrError::BadVersion => "Failed to parse a version number",
            BldrError::RegexParse(_) => "Failed to parse a regular expression",
            BldrError::ParseIntError(_) => "Failed to parse an integer from a string!",
            BldrError::FileNameError => "Failed to extract a filename from a path",
            BldrError::PackageNotFound => "Cannot find a package",
            BldrError::MustacheMergeOnlyMaps => "Can only merge two Mustache::Data::Maps",
            BldrError::SupervisorSignalFailed => "Failed to send a signal to the process supervisor",
            BldrError::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            BldrError::SupervisorDied => "The supervisor died",
            BldrError::NulError(_) => "An attempt was made to build a CString with a null byte inside it",
            BldrError::IPFailed => "Failed to discover the outbound IP address",
            BldrError::HostnameFailed => "Failed to discover this hosts hostname",
            BldrError::UnknownTopology(_) => "Unknown topology",
            BldrError::NoConfiguration => "No configuration data available",
            BldrError::HealthCheck(_) => "Health Check returned an unknown status code",
            BldrError::TryRecvError(_) => "A channel failed to recieve a response",
            BldrError::BadWatch(_) => "An invalid watch was specified",
            BldrError::NoXFilename => "Invalid download from a repository - missing X-Filename header",
            BldrError::NoFilePart => "An invalid path was passed - we needed a filename, and this path does not have one",
            BldrError::SignalNotifierStarted => "Only one instance of a Signal Notifier may be running",
            BldrError::ActorError(_) => "A running actor responded with an error",
        }
    }
}

fn toml_parser_string(errs: &Vec<toml::ParserError>) -> String {
    let mut errors = String::new();
    for err in errs.iter() {
        errors.push_str(&format!("{}", err));
        errors.push_str("\n");
    }
    return errors
}

impl From<ffi::NulError> for BldrError {
    fn from(err: ffi::NulError) -> BldrError {
        BldrError::NulError(err)
    }
}

impl From<mustache::encoder::Error> for BldrError {
    fn from(err: mustache::encoder::Error) -> BldrError {
        BldrError::MustacheEncoderError(err)
    }
}

impl From<io::Error> for BldrError {
    fn from(err: io::Error) -> BldrError {
        BldrError::Io(err)
    }
}

impl From<hyper::error::Error> for BldrError {
    fn from(err: hyper::error::Error) -> BldrError {
        BldrError::HyperError(err)
    }
}

impl From<regex::Error> for BldrError {
    fn from(err: regex::Error) -> BldrError {
        BldrError::RegexParse(err)
    }
}

impl From<num::ParseIntError> for BldrError {
    fn from(err: num::ParseIntError) -> BldrError {
        BldrError::ParseIntError(err)
    }
}

impl From<string::FromUtf8Error> for BldrError {
    fn from(err: string::FromUtf8Error) -> BldrError {
        BldrError::StringFromUtf8Error(err)
    }
}

impl From<mpsc::TryRecvError> for BldrError {
    fn from(err: mpsc::TryRecvError) -> BldrError {
        BldrError::TryRecvError(err)
    }
}

impl From<actor::ActorError> for BldrError {
    fn from(err: actor::ActorError) -> Self {
        BldrError::ActorError(err)
    }
}
