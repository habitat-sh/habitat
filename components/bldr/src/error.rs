// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Error handling for Bldr.
//!
//! Errors in bldr are of the type `BldrError`, which contains an `ErrorKind` along with
//! information about where the error was created in the code base, in the same way that the
//! `output` module does. To simplify the creation of these annotated errors, we provide the
//! `bldr_error!` macro, which takes only an `ErrorKind` as its argument.
//!
//! To match on `ErrorKind`, do something like this:
//!
//! ```ignore
//! let error = bldr_error!(ErrorKind::CommandNotImplemented);
//! let result = match error {
//!     BldrError{err: ErrorKind::CommandNotImplemented, ..} => true,
//!     _ => false
//! };
//! assert_eq!(result, true);
//! ```
//!
//! When printing errors, we automatically create a `StructuredOutput` with the `verbose` flag set,
//! ensuring that you can see the file, line number, and column it was created from.
//!
//! Also included in this module is `BldrResult<T>`, a type alias for `Result<T, BldrError>`. Use
//! it instead of the longer `Result` form.

use std::io;
use std::result;
use std::fmt;
use std::error::Error;
use std::string;
use std::ffi;
use std::sync::mpsc;
use std::str;

use core::{self, package};
use depot_client;
use gpgme;
use uuid;
use wonder::actor;
use ansi_term::Colour::Red;
use rustc_serialize::json;
use hyper;
use toml;
use mustache;

use package::HookType;
use output::StructuredOutput;

static LOGKEY: &'static str = "ER";

#[derive(Debug)]
/// All errors in Bldr are kept in this struct. We store `ErrorKind`, an enum with a variant for
/// every type of error we produce. It also stores the location the error was created.
pub struct BldrError {
    pub err: ErrorKind,
    logkey: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
}

impl BldrError {
    /// Create a new `BldrError`. Usually accessed through the `bldr_error!` macro, rather than
    /// called directly.
    pub fn new(err: ErrorKind,
               logkey: &'static str,
               file: &'static str,
               line: u32,
               column: u32)
               -> BldrError {
        BldrError {
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
pub enum ErrorKind {
    ActorError(actor::ActorError),
    BldrCore(core::Error),
    CommandNotImplemented,
    ConfigFileRelativePath(String),
    DbInvalidPath,
    DepotClient(depot_client::Error),
    FileNameError,
    FileNotFound(String),
    GPGError(gpgme::Error),
    HealthCheck(String),
    HookFailed(HookType, i32, String),
    HTTP(hyper::status::StatusCode),
    /// TODO: once discovery/etcd.rs is purged, this error can be removed
    HyperError(hyper::error::Error),
    InvalidKeyParameter(String),
    InvalidPackageIdent(String),
    InvalidPidFile,
    InvalidServiceGroupString(String),
    Io(io::Error),
    IPFailed,
    JsonDecode(json::DecoderError),
    JsonEncode(json::EncoderError),
    KeyNotFound(String),
    MetaFileMalformed(package::MetaFile),
    MetaFileNotFound(package::MetaFile),
    MetaFileIO(io::Error),
    MustacheEncoderError(mustache::encoder::Error),
    NulError(ffi::NulError),
    PackageArchiveMalformed(String),
    PackageNotFound(package::PackageIdent),
    RemotePackageNotFound(package::PackageIdent),
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
pub type BldrResult<T> = result::Result<T, BldrError>;

impl fmt::Display for BldrError {
    // We create a string for each type of error, then create a `StructuedOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self.err {
            ErrorKind::ActorError(ref err) => format!("Actor returned error: {:?}", err),
            ErrorKind::BldrCore(ref err) => format!("{}", err),
            ErrorKind::CommandNotImplemented => format!("Command is not yet implemented!"),
            ErrorKind::ConfigFileRelativePath(ref s) => {
                format!("Path for configuration file cannot have relative components (eg: ..): {}",
                        s)
            }
            ErrorKind::DbInvalidPath => format!("Invalid filepath to internal datastore"),
            ErrorKind::DepotClient(ref err) => format!("{}", err),
            ErrorKind::FileNameError => format!("Failed to extract a filename"),
            ErrorKind::FileNotFound(ref e) => format!("File not found at: {}", e),
            ErrorKind::GPGError(ref e) => format!("{}", e),
            ErrorKind::HealthCheck(ref e) => format!("Health Check failed: {}", e),
            ErrorKind::HookFailed(ref t, ref e, ref o) => {
                format!("Hook failed to run: {}, {}, {}", t, e, o)
            }
            ErrorKind::HTTP(ref e) => format!("{}", e),
            ErrorKind::HyperError(ref err) => format!("{}", err),
            ErrorKind::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            ErrorKind::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: chef/redis)",
                        e)
            }
            ErrorKind::InvalidPidFile => format!("Invalid child process PID file"),
            ErrorKind::InvalidServiceGroupString(ref e) => {
                format!("Invalid service group string: {}", e)
            }
            ErrorKind::Io(ref err) => format!("{}", err),
            ErrorKind::IPFailed => format!("Failed to discover this hosts outbound IP address"),
            ErrorKind::JsonDecode(ref e) => format!("JSON decoding error: {}", e),
            ErrorKind::JsonEncode(ref e) => format!("JSON encoding error: {}", e),
            ErrorKind::KeyNotFound(ref e) => format!("Key not found in key cache: {}", e),
            ErrorKind::MetaFileMalformed(ref e) => {
                format!("MetaFile: {:?}, didn't contain a valid UTF-8 string", e)
            }
            ErrorKind::MetaFileNotFound(ref e) => {
                format!("Couldn't read MetaFile: {}, not found", e)
            }
            ErrorKind::MetaFileIO(ref e) => format!("IO error while accessing MetaFile: {:?}", e),
            ErrorKind::MustacheEncoderError(ref me) => {
                match *me {
                    mustache::encoder::Error::IoError(ref e) => format!("{}", e),
                    _ => format!("Mustache encoder error: {:?}", me),
                }
            }
            ErrorKind::NulError(ref e) => format!("{}", e),
            ErrorKind::PackageArchiveMalformed(ref e) => {
                format!("Package archive was unreadable or contained unexpected contents: {:?}",
                        e)
            }
            ErrorKind::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            ErrorKind::RemotePackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package in any sources: {}", pkg)
                } else {
                    format!("Cannot find a release of package in any sources: {}", pkg)
                }
            }
            ErrorKind::SignalFailed => format!("Failed to send a signal to the child process"),
            ErrorKind::SignalNotifierStarted => format!("Only one instance of a Signal Notifier may be running"),
            ErrorKind::StrFromUtf8Error(ref e) => format!("{}", e),
            ErrorKind::StringFromUtf8Error(ref e) => format!("{}", e),
            ErrorKind::TomlEncode(ref e) => format!("Failed to encode toml: {}", e),
            ErrorKind::TomlParser(ref errs) => {
                format!("Failed to parse toml:\n{}", toml_parser_string(errs))
            }
            ErrorKind::TryRecvError(ref err) => format!("{}", err),
            ErrorKind::UnknownTopology(ref t) => format!("Unknown topology {}!", t),
            ErrorKind::UnpackFailed => format!("Failed to unpack a package"),
            ErrorKind::UuidParseError(ref e) => format!("Uuid Parse Error: {:?}", e),
        };
        let cstring = Red.bold().paint(content).to_string();
        let mut so = StructuredOutput::new("bldr",
                                           self.logkey,
                                           self.line,
                                           self.file,
                                           self.column,
                                           &cstring);
        so.verbose = Some(true);
        write!(f, "{}", so)
    }
}

impl Error for BldrError {
    fn description(&self) -> &str {
        match self.err {
            ErrorKind::ActorError(_) => "A running actor responded with an error",
            ErrorKind::BldrCore(ref err) => err.description(),
            ErrorKind::CommandNotImplemented => "Command is not yet implemented!",
            ErrorKind::ConfigFileRelativePath(_) => "Path for configuration file cannot have relative components (eg: ..)",
            ErrorKind::DbInvalidPath => "A bad filepath was provided for an internal datastore",
            ErrorKind::DepotClient(ref err) => err.description(),
            ErrorKind::FileNameError => "Failed to extract a filename from a path",
            ErrorKind::FileNotFound(_) => "File not found",
            ErrorKind::GPGError(_) => "gpgme error",
            ErrorKind::HealthCheck(_) => "Health Check returned an unknown status code",
            ErrorKind::HookFailed(_, _, _) => "Hook failed to run",
            ErrorKind::HTTP(_) => "Received an HTTP error",
            ErrorKind::HyperError(ref err) => err.description(),
            ErrorKind::InvalidKeyParameter(_) => "Key parameter error",
            ErrorKind::InvalidPackageIdent(_) => "Package identifiers must be in origin/name format (example: chef/redis)",
            ErrorKind::InvalidPidFile => "Invalid child process PID file",
            ErrorKind::InvalidServiceGroupString(_) => "Service group strings must be in service.group format (example: redis.default)",
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::IPFailed => "Failed to discover the outbound IP address",
            ErrorKind::JsonDecode(_) => "JSON decoding error: {:?}",
            ErrorKind::JsonEncode(_) => "JSON encoding error",
            ErrorKind::KeyNotFound(_) => "Key not found in key cache",
            ErrorKind::MetaFileMalformed(_) => "MetaFile didn't contain a valid UTF-8 string",
            ErrorKind::MetaFileNotFound(_) => "Failed to read an archive's metafile",
            ErrorKind::MetaFileIO(_) => "MetaFile could not be read or written to",
            ErrorKind::MustacheEncoderError(_) => "Failed to encode mustache template",
            ErrorKind::NulError(_) => "An attempt was made to build a CString with a null byte inside it",
            ErrorKind::PackageArchiveMalformed(_) => "Package archive was unreadable or had unexpected contents",
            ErrorKind::PackageNotFound(_) => "Cannot find a package",
            ErrorKind::RemotePackageNotFound(_) => "Cannot find a package in any sources",
            ErrorKind::SignalFailed => "Failed to send a signal to the child process",
            ErrorKind::SignalNotifierStarted => "Only one instance of a Signal Notifier may be running",
            ErrorKind::StrFromUtf8Error(_) => "Failed to convert a str from a &[u8] as UTF-8",
            ErrorKind::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            ErrorKind::TomlEncode(_) => "Failed to encode toml!",
            ErrorKind::TomlParser(_) => "Failed to parse toml!",
            ErrorKind::TryRecvError(_) => "A channel failed to recieve a response",
            ErrorKind::UnknownTopology(_) => "Unknown topology",
            ErrorKind::UnpackFailed => "Failed to unpack a package",
            ErrorKind::UuidParseError(_) => "Uuid Parse Error",
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

impl From<core::Error> for BldrError {
    fn from(err: core::Error) -> BldrError {
        bldr_error!(ErrorKind::BldrCore(err))
    }
}

impl From<depot_client::Error> for BldrError {
    fn from(err: depot_client::Error) -> BldrError {
        bldr_error!(ErrorKind::DepotClient(err))
    }
}

impl From<uuid::ParseError> for BldrError {
    fn from(err: uuid::ParseError) -> BldrError {
        bldr_error!(ErrorKind::UuidParseError(err))
    }
}

impl From<ffi::NulError> for BldrError {
    fn from(err: ffi::NulError) -> BldrError {
        bldr_error!(ErrorKind::NulError(err))
    }
}

impl From<mustache::encoder::Error> for BldrError {
    fn from(err: mustache::encoder::Error) -> BldrError {
        bldr_error!(ErrorKind::MustacheEncoderError(err))
    }
}

impl From<io::Error> for BldrError {
    fn from(err: io::Error) -> BldrError {
        bldr_error!(ErrorKind::Io(err))
    }
}

impl From<hyper::error::Error> for BldrError {
    fn from(err: hyper::error::Error) -> BldrError {
        bldr_error!(ErrorKind::HyperError(err))
    }
}

impl From<string::FromUtf8Error> for BldrError {
    fn from(err: string::FromUtf8Error) -> BldrError {
        bldr_error!(ErrorKind::StringFromUtf8Error(err))
    }
}

impl From<str::Utf8Error> for BldrError {
    fn from(err: str::Utf8Error) -> BldrError {
        bldr_error!(ErrorKind::StrFromUtf8Error(err))
    }
}

impl From<mpsc::TryRecvError> for BldrError {
    fn from(err: mpsc::TryRecvError) -> BldrError {
        bldr_error!(ErrorKind::TryRecvError(err))
    }
}

impl From<gpgme::Error> for BldrError {
    fn from(err: gpgme::Error) -> BldrError {
        bldr_error!(ErrorKind::GPGError(err))
    }
}

impl From<actor::ActorError> for BldrError {
    fn from(err: actor::ActorError) -> Self {
        bldr_error!(ErrorKind::ActorError(err))
    }
}

impl From<json::EncoderError> for BldrError {
    fn from(err: json::EncoderError) -> Self {
        bldr_error!(ErrorKind::JsonEncode(err))
    }
}

impl From<json::DecoderError> for BldrError {
    fn from(err: json::DecoderError) -> Self {
        bldr_error!(ErrorKind::JsonDecode(err))
    }
}

impl From<toml::Error> for BldrError {
    fn from(err: toml::Error) -> Self {
        bldr_error!(ErrorKind::TomlEncode(err))
    }
}
