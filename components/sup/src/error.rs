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
use std::env;
use std::error;
use std::ffi;
use std::fmt;
use std::net;
use std::path::PathBuf;
use std::result;
use std::str;
use std::string;
use std::sync::mpsc;

use ansi_term::Colour::Red;
use butterfly;
use common;
use depot_client;
use glob;
use handlebars;
use hcore::{self, package};
use hcore::package::Identifiable;
use notify;
use toml;

use output::StructuredOutput;
use PROGRAM_NAME;

static LOGKEY: &'static str = "ER";

/// Our result type alias, for easy coding.
pub type Result<T> = result::Result<T, SupError>;

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
    BadDataFile(PathBuf, io::Error),
    BadDataPath(PathBuf, io::Error),
    BadDesiredState(String),
    BadSpecsPath(PathBuf, io::Error),
    BadStartStyle(String),
    ButterflyError(butterfly::error::Error),
    DepotClient(depot_client::Error),
    EnvJoinPathsError(env::JoinPathsError),
    ExecCommandNotFound(String),
    FileNotFound(String),
    HabitatCommon(common::Error),
    HabitatCore(hcore::Error),
    TemplateFileError(handlebars::TemplateFileError),
    TemplateRenderError(handlebars::RenderError),
    InvalidBinding(String),
    InvalidKeyParameter(String),
    InvalidPidFile,
    InvalidTopology(String),
    InvalidUpdateStrategy(String),
    Io(io::Error),
    IPFailed,
    MissingRequiredBind(Vec<String>),
    MissingRequiredIdent,
    NameLookup(io::Error),
    NetParseError(net::AddrParseError),
    NulError(ffi::NulError),
    PackageNotFound(package::PackageIdent),
    Permissions(String),
    ProcessLockCorrupt,
    ProcessLocked(u32),
    ProcessLockIO(PathBuf, io::Error),
    ServiceLoaded(package::PackageIdent),
    ServiceSpecFileIO(PathBuf, io::Error),
    ServiceSpecParse(toml::de::Error),
    ServiceSpecRender(toml::ser::Error),
    SignalFailed,
    SpecWatcherDirNotFound(String),
    SpecWatcherGlob(glob::PatternError),
    SpecWatcherNotify(notify::Error),
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    TomlEncode(toml::ser::Error),
    TomlMergeError(String),
    TomlParser(toml::de::Error),
    TryRecvError(mpsc::TryRecvError),
    UnpackFailed,
}

impl fmt::Display for SupError {
    // We create a string for each type of error, then create a `StructuredOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self.err {
            Error::BadDataFile(ref path, ref err) => {
                format!("Unable to read or write to data file, {}, {}",
                        path.display(),
                        err)
            }
            Error::BadDataPath(ref path, ref err) => {
                format!("Unable to read or write to data directory, {}, {}",
                        path.display(),
                        err)
            }
            Error::BadDesiredState(ref state) => {
                format!("Unknown service desired state style '{}'", state)
            }
            Error::BadSpecsPath(ref path, ref err) => {
                format!("Unable to create the specs directory '{}' ({})",
                        path.display(),
                        err)
            }
            Error::BadStartStyle(ref style) => format!("Unknown service start style '{}'", style),
            Error::ButterflyError(ref err) => format!("Butterfly error: {}", err),
            Error::ExecCommandNotFound(ref c) => {
                format!("`{}' was not found on the filesystem or in PATH", c)
            }
            Error::Permissions(ref err) => format!("{}", err),
            Error::HabitatCommon(ref err) => format!("{}", err),
            Error::HabitatCore(ref err) => format!("{}", err),
            Error::TemplateFileError(ref err) => format!("{:?}", err),
            Error::TemplateRenderError(ref err) => format!("{}", err),
            Error::DepotClient(ref err) => format!("{}", err),
            Error::EnvJoinPathsError(ref err) => format!("{}", err),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::InvalidBinding(ref binding) => {
                format!("Invalid binding \"{}\", must be of the form <NAME>:<SERVICE_GROUP> where \
                         <NAME> is a service name and <SERVICE_GROUP> is a valid service group",
                        binding)
            }
            Error::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            Error::InvalidPidFile => format!("Invalid child process PID file"),
            Error::InvalidTopology(ref t) => format!("Invalid topology: {}", t),
            Error::InvalidUpdateStrategy(ref s) => format!("Invalid update strategy: {}", s),
            Error::Io(ref err) => format!("{}", err),
            Error::IPFailed => format!("Failed to discover this hosts outbound IP address"),
            Error::MissingRequiredBind(ref e) => {
                format!("Missing required bind(s), {}", e.join(", "))
            }
            Error::MissingRequiredIdent => format!("Missing required ident field: (example: ident = \"core/redis\")"),
            Error::NameLookup(ref e) => format!("Error resolving a name or IP address: {}", e),
            Error::NetParseError(ref e) => format!("Can't parse ip:port: {}", e),
            Error::NulError(ref e) => format!("{}", e),
            Error::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::ProcessLockCorrupt => format!("Unable to decode contents of process lock"),
            Error::ProcessLocked(ref pid) => {
                format!("Unable to start Habitat Supervisor because another instance is already \
                    running with the pid {}. If your intention was to run multiple Supervisors - \
                    that can be done by setting a value for `--override-name` at startup - but \
                    it is not recommended.",
                        pid)
            }
            Error::ProcessLockIO(ref path, ref err) => {
                format!("Unable to start Habitat Supervisor because we weren't able to write or \
                    read to a process lock at {}, {}",
                        path.display(),
                        err)
            }
            Error::ServiceLoaded(ref ident) => {
                format!("Service already loaded, unload '{}' and try again", ident)
            }
            Error::ServiceSpecFileIO(ref path, ref err) => {
                format!("Unable to write or read to a service spec file at {}, {}",
                        path.display(),
                        err)
            }
            Error::ServiceSpecParse(ref err) => {
                format!("Unable to parse contents of service spec file, {}", err)
            }
            Error::ServiceSpecRender(ref err) => {
                format!("Service spec could not be rendered successfully: {}", err)
            }
            Error::SignalFailed => format!("Failed to send a signal to the child process"),
            Error::SpecWatcherDirNotFound(ref path) => {
                format!("Spec directory '{}' not created or is not a directory",
                        path)
            }
            Error::SpecWatcherGlob(ref e) => format!("{}", e),
            Error::SpecWatcherNotify(ref e) => format!("{}", e),
            Error::StrFromUtf8Error(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TomlEncode(ref e) => format!("Failed to encode TOML: {}", e),
            Error::TomlMergeError(ref e) => format!("Failed to merge TOML: {}", e),
            Error::TomlParser(ref err) => format!("Failed to parse TOML: {}", err),
            Error::TryRecvError(ref err) => format!("{}", err),
            Error::UnpackFailed => format!("Failed to unpack a package"),
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
            Error::BadDataFile(_, _) => "Unable to read or write to a data file",
            Error::BadDataPath(_, _) => "Unable to read or write to data directory",
            Error::BadDesiredState(_) => "Unknown desired state in service spec",
            Error::BadSpecsPath(_, _) => "Unable to create the specs directory",
            Error::BadStartStyle(_) => "Unknown start style in service spec",
            Error::ButterflyError(ref err) => err.description(),
            Error::ExecCommandNotFound(_) => "Exec command was not found on filesystem or in PATH",
            Error::TemplateFileError(ref err) => err.description(),
            Error::TemplateRenderError(ref err) => err.description(),
            Error::HabitatCommon(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::DepotClient(ref err) => err.description(),
            Error::EnvJoinPathsError(ref err) => err.description(),
            Error::FileNotFound(_) => "File not found",
            Error::InvalidBinding(_) => "Invalid binding parameter",
            Error::InvalidKeyParameter(_) => "Key parameter error",
            Error::InvalidPidFile => "Invalid child process PID file",
            Error::InvalidTopology(_) => "Invalid topology",
            Error::InvalidUpdateStrategy(_) => "Invalid update strategy",
            Error::Io(ref err) => err.description(),
            Error::IPFailed => "Failed to discover the outbound IP address",
            Error::MissingRequiredBind(_) => "A service to start without specifying a service group for all required binds",
            Error::MissingRequiredIdent => "Missing required ident field: (example: ident = \"core/redis\")",
            Error::NetParseError(_) => "Can't parse IP:port",
            Error::NameLookup(_) => "Error resolving a name or IP address",
            Error::NulError(_) => "An attempt was made to build a CString with a null byte inside it",
            Error::PackageNotFound(_) => "Cannot find a package",
            Error::Permissions(_) => "File system permissions error",
            Error::ProcessLockCorrupt => "Unable to decode contents of process lock",
            Error::ProcessLocked(_) => "Another instance of the Habitat Supervisor is already running",
            Error::ProcessLockIO(_, _) => "Unable to write or read to a process lock",
            Error::ServiceLoaded(_) => "Service load or start called when service already loaded",
            Error::ServiceSpecFileIO(_, _) => "Unable to write or read to a service spec file",
            Error::ServiceSpecParse(_) => "Service spec could not be parsed successfully",
            Error::ServiceSpecRender(_) => "Service spec TOML could not be rendered successfully",
            Error::SignalFailed => "Failed to send a signal to the child process",
            Error::SpecWatcherDirNotFound(_) => "Spec directory not created or is not a directory",
            Error::SpecWatcherGlob(_) => "Spec watcher file globbing error",
            Error::SpecWatcherNotify(_) => "Spec watcher error",
            Error::StrFromUtf8Error(_) => "Failed to convert a str from a &[u8] as UTF-8",
            Error::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            Error::TomlEncode(_) => "Failed to encode toml!",
            Error::TomlMergeError(_) => "Failed to merge TOML!",
            Error::TomlParser(_) => "Failed to parse TOML!",
            Error::TryRecvError(_) => "A channel failed to receive a response",
            Error::UnpackFailed => "Failed to unpack a package",
        }
    }
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

impl From<glob::PatternError> for SupError {
    fn from(err: glob::PatternError) -> SupError {
        sup_error!(Error::SpecWatcherGlob(err))
    }
}

impl From<handlebars::RenderError> for SupError {
    fn from(err: handlebars::RenderError) -> SupError {
        sup_error!(Error::TemplateRenderError(err))
    }
}

impl From<handlebars::TemplateFileError> for SupError {
    fn from(err: handlebars::TemplateFileError) -> SupError {
        sup_error!(Error::TemplateFileError(err))
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

impl From<env::JoinPathsError> for SupError {
    fn from(err: env::JoinPathsError) -> SupError {
        sup_error!(Error::EnvJoinPathsError(err))
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

impl From<notify::Error> for SupError {
    fn from(err: notify::Error) -> SupError {
        sup_error!(Error::SpecWatcherNotify(err))
    }
}

impl From<toml::de::Error> for SupError {
    fn from(err: toml::de::Error) -> Self {
        sup_error!(Error::TomlParser(err))
    }
}

impl From<toml::ser::Error> for SupError {
    fn from(err: toml::ser::Error) -> Self {
        sup_error!(Error::TomlEncode(err))
    }
}
