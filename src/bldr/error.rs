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
use std::num;
use std::string;
use std::ffi;
use std::sync::mpsc;
use std::str;

use gpgme;
use libarchive;
use uuid;
use wonder::actor;
use ansi_term::Colour::Red;
use rustc_serialize::json;
use hyper;
use toml;
use mustache;
use regex;

use depot::data_store;
use package;
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

#[derive(Debug)]
/// All the kinds of errors we produce.
pub enum ErrorKind {
    ArchiveReadFailed(String),
    ArchiveError(libarchive::error::ArchiveError),
    Io(io::Error),
    CommandNotImplemented,
    DbInvalidPath,
    InstallFailed,
    WriteSyncFailed,
    HyperError(hyper::error::Error),
    HTTP(hyper::status::StatusCode),
    CannotParseFileName,
    PathUTF8,
    GPGError(gpgme::Error),
    UnpackFailed,
    TomlParser(Vec<toml::ParserError>),
    TomlEncode(toml::Error),
    MdbError(data_store::MdbError),
    MustacheEncoderError(mustache::encoder::Error),
    MetaFileNotFound(package::MetaFile),
    MetaFileMalformed(package::MetaFile),
    MetaFileIO(io::Error),
    PackageArchiveMalformed(String),
    PermissionFailed,
    BadVersion,
    RegexParse(regex::Error),
    ParseIntError(num::ParseIntError),
    FileNameError,
    FileNotFound(String),
    KeyNotFound(String),
    PackageLoad(String),
    PackageNotFound(package::PackageIdent),
    PackageIdentMismatch(String, String),
    RemotePackageNotFound(package::PackageIdent),
    MustacheMergeOnlyMaps,
    SupervisorSignalFailed,
    StringFromUtf8Error(string::FromUtf8Error),
    StrFromUtf8Error(str::Utf8Error),
    SupervisorDied,
    NulError(ffi::NulError),
    IPFailed,
    HostnameFailed,
    UnknownTopology(String),
    NoConfiguration,
    HealthCheck(String),
    HookFailed(package::HookType, i32, String),
    TryRecvError(mpsc::TryRecvError),
    BadWatch(String),
    NoXFilename,
    NoFilePart,
    SignalNotifierStarted,
    ActorError(actor::ActorError),
    CensusNotFound(String),
    UuidParseError(uuid::ParseError),
    InvalidPackageIdent(String),
    InvalidKeyParameter(String),
    JsonEncode(json::EncoderError),
    JsonDecode(json::DecoderError),
    InitialPeers,
}

/// Our result type alias, for easy coding.
pub type BldrResult<T> = result::Result<T, BldrError>;

impl fmt::Display for BldrError {
    // We create a string for each type of error, then create a `StructuedOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self.err {
            ErrorKind::ArchiveReadFailed(ref e) => format!("Failed to read package archive, {}", e),
            ErrorKind::ArchiveError(ref err) => format!("{}", err),
            ErrorKind::Io(ref err) => format!("{}", err),
            ErrorKind::CommandNotImplemented => format!("Command is not yet implemented!"),
            ErrorKind::DbInvalidPath => format!("Invalid filepath to internal datastore"),
            ErrorKind::InstallFailed => format!("Could not install package!"),
            ErrorKind::HyperError(ref err) => format!("{}", err),
            ErrorKind::HTTP(ref e) => format!("{}", e),
            ErrorKind::WriteSyncFailed => format!("Could not write to destination; perhaps the disk is full?"),
            ErrorKind::CannotParseFileName => format!("Cannot determine the filename from the given URI"),
            ErrorKind::PathUTF8 => format!("Paths must not contain non-UTF8 characters"),
            ErrorKind::GPGError(ref e) => format!("{}", e),
            ErrorKind::UnpackFailed => format!("Failed to unpack a package"),
            ErrorKind::TomlParser(ref errs) => {
                format!("Failed to parse toml:\n{}", toml_parser_string(errs))
            }
            ErrorKind::TomlEncode(ref e) => format!("Failed to encode toml: {}", e),
            ErrorKind::MdbError(ref err) => format!("{}", err),
            ErrorKind::MustacheEncoderError(ref me) => {
                match *me {
                    mustache::encoder::Error::IoError(ref e) => format!("{}", e),
                    _ => format!("Mustache encoder error: {:?}", me),
                }
            }
            ErrorKind::MetaFileNotFound(ref e) => {
                format!("Couldn't read MetaFile: {}, not found", e)
            }
            ErrorKind::MetaFileMalformed(ref e) => {
                format!("MetaFile: {:?}, didn't contain a valid UTF-8 string", e)
            }
            ErrorKind::MetaFileIO(ref e) => format!("IO error while accessing MetaFile: {:?}", e),
            ErrorKind::PackageArchiveMalformed(ref e) => {
                format!("Package archive was unreadable or contained unexpected contents: {:?}",
                        e)
            }
            ErrorKind::PermissionFailed => format!("Failed to set permissions"),
            ErrorKind::BadVersion => format!("Failed to parse a version number"),
            ErrorKind::RegexParse(ref e) => format!("{}", e),
            ErrorKind::ParseIntError(ref e) => format!("{}", e),
            ErrorKind::FileNameError => format!("Failed to extract a filename"),
            ErrorKind::FileNotFound(ref e) => format!("File not found at: {}", e),
            ErrorKind::KeyNotFound(ref e) => format!("Key not found in key cache: {}", e),
            ErrorKind::PackageLoad(ref e) => format!("Unable to load package from: {}", e),
            ErrorKind::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            ErrorKind::PackageIdentMismatch(ref expect, ref got) => {
                format!("Encountered an unexpected package identity: expected={}, got={}",
                        expect,
                        got)
            }
            ErrorKind::RemotePackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package in any sources: {}", pkg)
                } else {
                    format!("Cannot find a release of package in any sources: {}", pkg)
                }
            }
            ErrorKind::MustacheMergeOnlyMaps => format!("Can only merge two Mustache::Data::Maps"),
            ErrorKind::SupervisorSignalFailed => format!("Failed to send a signal to the process supervisor"),
            ErrorKind::StringFromUtf8Error(ref e) => format!("{}", e),
            ErrorKind::StrFromUtf8Error(ref e) => format!("{}", e),
            ErrorKind::SupervisorDied => format!("The supervisor died"),
            ErrorKind::NulError(ref e) => format!("{}", e),
            ErrorKind::IPFailed => format!("Failed to discover this hosts outbound IP address"),
            ErrorKind::HostnameFailed => format!("Failed to discover this hosts hostname"),
            ErrorKind::UnknownTopology(ref t) => format!("Unknown topology {}!", t),
            ErrorKind::NoConfiguration => format!("No configuration data - cannot continue"),
            ErrorKind::HealthCheck(ref e) => format!("Health Check failed: {}", e),
            ErrorKind::HookFailed(ref t, ref e, ref o) => {
                format!("Hook failed to run: {}, {}, {}", t, e, o)
            }
            ErrorKind::TryRecvError(ref err) => format!("{}", err),
            ErrorKind::BadWatch(ref e) => format!("Bad watch format: {} is not valid", e),
            ErrorKind::NoXFilename => format!("Invalid download from a Depot - missing X-Filename header"),
            ErrorKind::NoFilePart => {
                format!("An invalid path was passed - we needed a filename, and this path does \
                         not have one")
            }
            ErrorKind::SignalNotifierStarted => format!("Only one instance of a Signal Notifier may be running"),
            ErrorKind::ActorError(ref err) => format!("Actor returned error: {:?}", err),
            ErrorKind::CensusNotFound(ref s) => format!("Census entry not found: {:?}", s),
            ErrorKind::UuidParseError(ref e) => format!("Uuid Parse Error: {:?}", e),
            ErrorKind::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: chef/redis)",
                        e)
            }
            ErrorKind::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            ErrorKind::JsonEncode(ref e) => format!("JSON encoding error: {}", e),
            ErrorKind::JsonDecode(ref e) => format!("JSON decoding error: {}", e),
            ErrorKind::InitialPeers => format!("Failed to contact initial peers"),
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
            ErrorKind::ArchiveError(ref err) => err.description(),
            ErrorKind::ArchiveReadFailed(_) => "Failed to read contents of package archive",
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::CommandNotImplemented => "Command is not yet implemented!",
            ErrorKind::DbInvalidPath => "A bad filepath was provided for an internal datastore",
            ErrorKind::InstallFailed => "Could not install package!",
            ErrorKind::WriteSyncFailed => "Could not write to destination; bytes written was 0 on a non-0 buffer",
            ErrorKind::CannotParseFileName => "Cannot determine the filename from the given URI",
            ErrorKind::HyperError(ref err) => err.description(),
            ErrorKind::HTTP(_) => "Received an HTTP error",
            ErrorKind::PathUTF8 => "Paths must not contain non-UTF8 characters",
            ErrorKind::GPGError(_) => "gpgme error",
            ErrorKind::UnpackFailed => "Failed to unpack a package",
            ErrorKind::TomlParser(_) => "Failed to parse toml!",
            ErrorKind::TomlEncode(_) => "Failed to encode toml!",
            ErrorKind::MdbError(_) => "Database error",
            ErrorKind::MustacheEncoderError(_) => "Failed to encode mustache template",
            ErrorKind::MetaFileNotFound(_) => "Failed to read an archive's metafile",
            ErrorKind::MetaFileMalformed(_) => "MetaFile didn't contain a valid UTF-8 string",
            ErrorKind::MetaFileIO(_) => "MetaFile could not be read or written to",
            ErrorKind::PackageArchiveMalformed(_) => "Package archive was unreadable or had unexpected contents",
            ErrorKind::PermissionFailed => "Failed to set permissions",
            ErrorKind::BadVersion => "Failed to parse a version number",
            ErrorKind::RegexParse(_) => "Failed to parse a regular expression",
            ErrorKind::ParseIntError(_) => "Failed to parse an integer from a string!",
            ErrorKind::FileNameError => "Failed to extract a filename from a path",
            ErrorKind::FileNotFound(_) => "File not found",
            ErrorKind::KeyNotFound(_) => "Key not found in key cache",
            ErrorKind::PackageLoad(_) => "Unable to load package from path",
            ErrorKind::PackageNotFound(_) => "Cannot find a package",
            ErrorKind::PackageIdentMismatch(_, _) => "Expected a package identity but received another",
            ErrorKind::RemotePackageNotFound(_) => "Cannot find a package in any sources",
            ErrorKind::MustacheMergeOnlyMaps => "Can only merge two Mustache::Data::Maps",
            ErrorKind::SupervisorSignalFailed => "Failed to send a signal to the process supervisor",
            ErrorKind::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            ErrorKind::StrFromUtf8Error(_) => "Failed to convert a str from a &[u8] as UTF-8",
            ErrorKind::SupervisorDied => "The supervisor died",
            ErrorKind::NulError(_) => "An attempt was made to build a CString with a null byte inside it",
            ErrorKind::IPFailed => "Failed to discover the outbound IP address",
            ErrorKind::HostnameFailed => "Failed to discover this hosts hostname",
            ErrorKind::UnknownTopology(_) => "Unknown topology",
            ErrorKind::NoConfiguration => "No configuration data available",
            ErrorKind::HealthCheck(_) => "Health Check returned an unknown status code",
            ErrorKind::HookFailed(_, _, _) => "Hook failed to run",
            ErrorKind::TryRecvError(_) => "A channel failed to recieve a response",
            ErrorKind::BadWatch(_) => "An invalid watch was specified",
            ErrorKind::NoXFilename => "Invalid download from a Depot - missing X-Filename header",
            ErrorKind::NoFilePart => "An invalid path was passed - we needed a filename, and this path does not have one",
            ErrorKind::SignalNotifierStarted => "Only one instance of a Signal Notifier may be running",
            ErrorKind::ActorError(_) => "A running actor responded with an error",
            ErrorKind::CensusNotFound(_) => "A census entry does not exist",
            ErrorKind::UuidParseError(_) => "Uuid Parse Error",
            ErrorKind::InvalidPackageIdent(_) => "Package identifiers must be in origin/name format (example: chef/redis)",
            ErrorKind::InvalidKeyParameter(_) => "Key parameter error",
            ErrorKind::JsonEncode(_) => "JSON encoding error",
            ErrorKind::JsonDecode(_) => "JSON decoding error: {:?}",
            ErrorKind::InitialPeers => "Failed to contact initial peers",
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

impl From<regex::Error> for BldrError {
    fn from(err: regex::Error) -> BldrError {
        bldr_error!(ErrorKind::RegexParse(err))
    }
}

impl From<num::ParseIntError> for BldrError {
    fn from(err: num::ParseIntError) -> BldrError {
        bldr_error!(ErrorKind::ParseIntError(err))
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

impl From<libarchive::error::ArchiveError> for BldrError {
    fn from(err: libarchive::error::ArchiveError) -> BldrError {
        bldr_error!(ErrorKind::ArchiveError(err))
    }
}

impl From<data_store::MdbError> for BldrError {
    fn from(err: data_store::MdbError) -> BldrError {
        bldr_error!(ErrorKind::MdbError(err))
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
