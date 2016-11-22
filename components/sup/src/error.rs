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

//! Error handling for the Supervisor.
//!
//! Errors in the Supervisor are of the type `SupError`, which contains an `Error` along with
//! information about where the error was created in the code base, in the same way that the
//! `output` module does. To simplify the creation of these annotated errors, we provide the
//! `sup_error!` macro, which takes only an `Error` as its argument.
//!
//! To match on `Error`, do something like this:
//!
//! ```ignore
//! let error = sup_error!(Error::CommandNotImplemented);
//! let result = match error {
//!     SupError{err: Error::CommandNotImplemented, ..} => true,
//!     _ => false
//! };
//! assert_eq!(result, true);
//! ```
//!
//! When printing errors, we automatically create a `StructuredOutput` with the `verbose` flag set,
//! ensuring that you can see the file, line number, and column it was created from.
//!
//! Also included in this module is `Result<T>`, a type alias for `Result<T, SupError>`. Use
//! it instead of the longer `Result` form.

use std::io;
use std::error;
use std::ffi;
use std::fmt;
use std::net;
use std::num;
use std::result;
use std::str;
use std::string;
use std::sync::mpsc;

use ansi_term::Colour::Red;
use handlebars;
use hcore::package::Identifiable;
use butterfly;
use hyper;
use rustc_serialize::json;
use toml;
use uuid;
use wonder::actor;

use common;
use depot_client;
use hcore::{self, package};
use output::StructuredOutput;
use package::HookType;
use PROGRAM_NAME;

static LOGKEY: &'static str = "ER";

#[derive(Debug)]
/// All errors in the Supervisor are kept in this struct. We store `Error`, an enum with a variant
/// for every type of error we produce. It also stores the location the error was created.
pub struct SupError {
    pub err: Error,
    logkey: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
}

impl SupError {
    /// Create a new `SupError`. Usually accessed through the `sup_error!` macro, rather than
    /// called directly.
    pub fn new(err: Error,
               logkey: &'static str,
               file: &'static str,
               line: u32,
               column: u32)
               -> SupError {
        SupError {
            err: err,
            logkey: logkey,
            file: file,
            line: line,
            column: column,
        }
    }
}

/// All the kinds of errors we produce.
#[derive(Debug)]
pub enum Error {
    ActorError(actor::ActorError),
    ButterflyError(butterfly::error::Error),
    CommandNotImplemented,
    DbInvalidPath,
    DepotClient(depot_client::Error),
    ExecCommandNotFound(String),
    FileNotFound(String),
    HabitatCommon(common::Error),
    HabitatCore(hcore::Error),
    HandlebarsTemplateFileError(handlebars::TemplateFileError),
    HandlebarsRenderError(handlebars::RenderError),
    HealthCheck(String),
    HookFailed(HookType, i32, String),
    HTTP(hyper::status::StatusCode),
    /// TODO: once discovery/etcd.rs is purged, this error can be removed
    HyperError(hyper::error::Error),
    InvalidBinding(String),
    InvalidKeyParameter(String),
    InvalidPidFile,
    InvalidPort(num::ParseIntError),
    InvalidServiceGroupString(String),
    Io(io::Error),
    IPFailed,
    JsonDecode(json::DecoderError),
    JsonEncode(json::EncoderError),
    KeyNotFound(String),
    MetaFileIO(io::Error),
    NetParseError(net::AddrParseError),
    NoRunFile,
    NulError(ffi::NulError),
    PackageArchiveMalformed(String),
    PackageNotFound(package::PackageIdent),
    Permissions(String),
    RemotePackageNotFound(package::PackageIdent),
    RootRequired,
    SignalFailed,
    SignalNotifierStarted,
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    TomlEncode(toml::Error),
    TomlParser(Vec<toml::ParserError>),
    TryRecvError(mpsc::TryRecvError),
    UnknownTopology(String),
    UnpackFailed,
    UuidParseError(uuid::ParseError),
}

/// Our result type alias, for easy coding.
pub type Result<T> = result::Result<T, SupError>;

impl fmt::Display for SupError {
    // We create a string for each type of error, then create a `StructuedOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self.err {
            Error::ActorError(ref err) => format!("Actor returned error: {:?}", err),
            Error::ButterflyError(ref err) => format!("Butterfly error: {}", err),
            Error::ExecCommandNotFound(ref c) => {
                format!("`{}' was not found on the filesystem or in PATH", c)
            }
            Error::Permissions(ref err) => format!("{}", err),
            Error::HabitatCommon(ref err) => format!("{}", err),
            Error::HabitatCore(ref err) => format!("{}", err),
            Error::HandlebarsTemplateFileError(ref err) => format!("{:?}", err),
            Error::HandlebarsRenderError(ref err) => format!("{}", err),
            Error::CommandNotImplemented => format!("Command is not yet implemented!"),
            Error::DbInvalidPath => format!("Invalid filepath to internal datastore"),
            Error::DepotClient(ref err) => format!("{}", err),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::HealthCheck(ref e) => format!("Health Check failed: {}", e),
            Error::HookFailed(ref t, ref e, ref o) => {
                format!("Hook failed to run: {}, {}, {}", t, e, o)
            }
            Error::HTTP(ref e) => format!("{}", e),
            Error::HyperError(ref err) => format!("{}", err),
            Error::InvalidBinding(ref binding) => {
                format!("Invalid binding - must be ':' delimited: {}", binding)
            }
            Error::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            Error::InvalidPort(ref e) => {
                format!("Invalid port number in package expose metadata: {}", e)
            }
            Error::InvalidPidFile => format!("Invalid child process PID file"),
            Error::InvalidServiceGroupString(ref e) => {
                format!("Invalid service group string: {}", e)
            }
            Error::Io(ref err) => format!("{}", err),
            Error::IPFailed => format!("Failed to discover this hosts outbound IP address"),
            Error::JsonDecode(ref e) => format!("JSON decoding error: {}", e),
            Error::JsonEncode(ref e) => format!("JSON encoding error: {}", e),
            Error::KeyNotFound(ref e) => format!("Key not found in key cache: {}", e),
            Error::MetaFileIO(ref e) => format!("IO error while accessing MetaFile: {:?}", e),
            Error::NetParseError(ref e) => format!("Can't parse ip:port: {}", e),
            Error::NoRunFile => {
                format!("No run file is present for this package; specify a run hook or \
                         $pkg_svc_run in your plan")
            }
            Error::NulError(ref e) => format!("{}", e),
            Error::PackageArchiveMalformed(ref e) => {
                format!("Package archive was unreadable or contained unexpected contents: {:?}",
                        e)
            }
            Error::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::RemotePackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package in any sources: {}", pkg)
                } else {
                    format!("Cannot find a release of package in any sources: {}", pkg)
                }
            }
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation".to_string()
            }
            Error::SignalFailed => format!("Failed to send a signal to the child process"),
            Error::SignalNotifierStarted => {
                format!("Only one instance of a Signal Notifier may be running")
            }
            Error::StrFromUtf8Error(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TomlEncode(ref e) => format!("Failed to encode toml: {}", e),
            Error::TomlParser(ref errs) => {
                format!("Failed to parse toml:\n{}", toml_parser_string(errs))
            }
            Error::TryRecvError(ref err) => format!("{}", err),
            Error::UnknownTopology(ref t) => format!("Unknown topology {}!", t),
            Error::UnpackFailed => format!("Failed to unpack a package"),
            Error::UuidParseError(ref e) => format!("Uuid Parse Error: {:?}", e),
        };
        let cstring = Red.bold().paint(content).to_string();
        let progname = PROGRAM_NAME.as_str();
        let mut so = StructuredOutput::new(progname,
                                           self.logkey,
                                           self.line,
                                           self.file,
                                           self.column,
                                           &cstring);
        so.verbose = Some(true);
        write!(f, "{}", so)
    }
}

impl error::Error for SupError {
    fn description(&self) -> &str {
        match self.err {
            Error::ActorError(_) => "A running actor responded with an error",
            Error::ButterflyError(ref err) => err.description(),
            Error::ExecCommandNotFound(_) => "Exec command was not found on filesystem or in PATH",
            Error::HandlebarsRenderError(ref err) => err.description(),
            Error::HandlebarsTemplateFileError(ref err) => err.description(),
            Error::HabitatCommon(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::CommandNotImplemented => "Command is not yet implemented!",
            Error::DbInvalidPath => "A bad filepath was provided for an internal datastore",
            Error::DepotClient(ref err) => err.description(),
            Error::FileNotFound(_) => "File not found",
            Error::HealthCheck(_) => "Health Check returned an unknown status code",
            Error::HookFailed(_, _, _) => "Hook failed to run",
            Error::HTTP(_) => "Received an HTTP error",
            Error::HyperError(ref err) => err.description(),
            Error::InvalidBinding(_) => "Invalid binding parameter",
            Error::InvalidKeyParameter(_) => "Key parameter error",
            Error::InvalidPort(_) => "Invalid port number in package expose metadata",
            Error::InvalidPidFile => "Invalid child process PID file",
            Error::InvalidServiceGroupString(_) => {
                "Service group strings must be in service.group format (example: redis.default)"
            }
            Error::Io(ref err) => err.description(),
            Error::IPFailed => "Failed to discover the outbound IP address",
            Error::JsonDecode(_) => "JSON decoding error: {:?}",
            Error::JsonEncode(_) => "JSON encoding error",
            Error::KeyNotFound(_) => "Key not found in key cache",
            Error::MetaFileIO(_) => "MetaFile could not be read or written to",
            Error::NetParseError(_) => "Can't parse IP:port",
            Error::NoRunFile => {
                "No run file is present for this package; specify a run hook or $pkg_svc_run \
                 in your plan"
            }
            Error::NulError(_) => {
                "An attempt was made to build a CString with a null byte inside it"
            }
            Error::PackageArchiveMalformed(_) => {
                "Package archive was unreadable or had unexpected contents"
            }
            Error::PackageNotFound(_) => "Cannot find a package",
            Error::Permissions(_) => "File system permissions error",
            Error::RemotePackageNotFound(_) => "Cannot find a package in any sources",
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation"
            }
            Error::SignalFailed => "Failed to send a signal to the child process",
            Error::SignalNotifierStarted => "Only one instance of a Signal Notifier may be running",
            Error::StrFromUtf8Error(_) => "Failed to convert a str from a &[u8] as UTF-8",
            Error::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            Error::TomlEncode(_) => "Failed to encode toml!",
            Error::TomlParser(_) => "Failed to parse toml!",
            Error::TryRecvError(_) => "A channel failed to recieve a response",
            Error::UnknownTopology(_) => "Unknown topology",
            Error::UnpackFailed => "Failed to unpack a package",
            Error::UuidParseError(_) => "Uuid Parse Error",
        }
    }
}

fn toml_parser_string(errs: &Vec<toml::ParserError>) -> String {
    let mut errors = String::new();
    for err in errs.iter() {
        errors.push_str(&format!("{}", err));
        errors.push_str("\n");
    }
    return errors;
}

impl From<net::AddrParseError> for SupError {
    fn from(err: net::AddrParseError) -> SupError {
        sup_error!(Error::NetParseError(err))
    }
}

impl From<butterfly::error::Error> for SupError {
    fn from(err: butterfly::error::Error) -> SupError {
        sup_error!(Error::ButterflyError(err))
    }
}

impl From<common::Error> for SupError {
    fn from(err: common::Error) -> SupError {
        sup_error!(Error::HabitatCommon(err))
    }
}

impl From<handlebars::RenderError> for SupError {
    fn from(err: handlebars::RenderError) -> SupError {
        sup_error!(Error::HandlebarsRenderError(err))
    }
}

impl From<handlebars::TemplateFileError> for SupError {
    fn from(err: handlebars::TemplateFileError) -> SupError {
        sup_error!(Error::HandlebarsTemplateFileError(err))
    }
}

impl From<hcore::Error> for SupError {
    fn from(err: hcore::Error) -> SupError {
        sup_error!(Error::HabitatCore(err))
    }
}

impl From<depot_client::Error> for SupError {
    fn from(err: depot_client::Error) -> SupError {
        sup_error!(Error::DepotClient(err))
    }
}

impl From<uuid::ParseError> for SupError {
    fn from(err: uuid::ParseError) -> SupError {
        sup_error!(Error::UuidParseError(err))
    }
}

impl From<ffi::NulError> for SupError {
    fn from(err: ffi::NulError) -> SupError {
        sup_error!(Error::NulError(err))
    }
}

impl From<io::Error> for SupError {
    fn from(err: io::Error) -> SupError {
        sup_error!(Error::Io(err))
    }
}

impl From<hyper::error::Error> for SupError {
    fn from(err: hyper::error::Error) -> SupError {
        sup_error!(Error::HyperError(err))
    }
}

impl From<string::FromUtf8Error> for SupError {
    fn from(err: string::FromUtf8Error) -> SupError {
        sup_error!(Error::StringFromUtf8Error(err))
    }
}

impl From<str::Utf8Error> for SupError {
    fn from(err: str::Utf8Error) -> SupError {
        sup_error!(Error::StrFromUtf8Error(err))
    }
}

impl From<mpsc::TryRecvError> for SupError {
    fn from(err: mpsc::TryRecvError) -> SupError {
        sup_error!(Error::TryRecvError(err))
    }
}

impl From<actor::ActorError> for SupError {
    fn from(err: actor::ActorError) -> Self {
        sup_error!(Error::ActorError(err))
    }
}

impl From<json::EncoderError> for SupError {
    fn from(err: json::EncoderError) -> Self {
        sup_error!(Error::JsonEncode(err))
    }
}

impl From<json::DecoderError> for SupError {
    fn from(err: json::DecoderError) -> Self {
        sup_error!(Error::JsonDecode(err))
    }
}

impl From<toml::Error> for SupError {
    fn from(err: toml::Error) -> Self {
        sup_error!(Error::TomlEncode(err))
    }
}
