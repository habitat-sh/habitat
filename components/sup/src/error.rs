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

use std::env;
use std::error;
use std::ffi;
use std::fmt;
use std::io;
use std::net;
use std::path::PathBuf;
use std::result;
use std::str;
use std::string;
use std::sync::mpsc;

use api_client;
use butterfly;
use common;
use glob;
use handlebars;
use hcore;
use hcore::os::process::Pid;
use hcore::output::StructuredOutput;
use hcore::package::{self, Identifiable, PackageInstall};
use launcher_client;
use notify;
use protocol;
use serde_json;
use toml;

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
    pub fn new(
        err: Error,
        logkey: &'static str,
        file: &'static str,
        line: u32,
        column: u32,
    ) -> SupError {
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
    Departed,
    BadAddress(String),
    BadCompositesPath(PathBuf, io::Error),
    BadDataFile(PathBuf, io::Error),
    BadDataPath(PathBuf, io::Error),
    BadDesiredState(String),
    BadElectionStatus(String),
    BadPackage(PackageInstall, hcore::error::Error),
    BadSpecsPath(PathBuf, io::Error),
    BadStartStyle(String),
    BadEnvConfig(String),
    BindTimeout(String),
    LockPoisoned,
    TestBootFail,
    ButterflyError(butterfly::error::Error),
    CtlSecretIo(PathBuf, io::Error),
    APIClient(api_client::Error),
    EnvJoinPathsError(env::JoinPathsError),
    ExecCommandNotFound(String),
    FileNotFound(String),
    FileWatcherFileIsRoot,
    GroupNotFound(String),
    HabitatCommon(common::Error),
    HabitatCore(hcore::Error),
    TemplateFileError(handlebars::TemplateFileError),
    TemplateRenderError(handlebars::RenderError),
    InvalidBinding(String),
    InvalidBinds(Vec<String>),
    InvalidCertFile(PathBuf),
    InvalidKeyFile(PathBuf),
    InvalidKeyParameter(String),
    InvalidPidFile,
    InvalidTokioThreadCount,
    InvalidTopology(String),
    InvalidUpdateStrategy(String),
    Io(io::Error),
    IPFailed,
    Launcher(launcher_client::Error),
    MissingRequiredBind(Vec<String>),
    MissingRequiredIdent,
    NameLookup(io::Error),
    NetErr(protocol::net::NetErr),
    NetParseError(net::AddrParseError),
    NoActiveMembers(hcore::service::ServiceGroup),
    NoLauncher,
    NoSuchBind(String),
    NotifyCreateError(notify::Error),
    NotifyError(notify::Error),
    NulError(ffi::NulError),
    PackageNotFound(package::PackageIdent),
    Permissions(String),
    PidFileCorrupt(PathBuf),
    PidFileIO(PathBuf, io::Error),
    ProcessLockCorrupt,
    ProcessLocked(Pid),
    ProcessLockIO(PathBuf, io::Error),
    RecvError(mpsc::RecvError),
    RenderContextSerialization(serde_json::Error),
    ServiceDeserializationError(serde_json::Error),
    ServiceNotLoaded(package::PackageIdent),
    ServiceSerializationError(serde_json::Error),
    ServiceSpecFileIO(PathBuf, io::Error),
    ServiceSpecParse(toml::de::Error),
    ServiceSpecRender(toml::ser::Error),
    SignalFailed,
    SpecWatcherDirNotFound(String),
    SpecWatcherGlob(glob::PatternError),
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    TomlEncode(toml::ser::Error),
    TomlMergeError(String),
    TomlParser(toml::de::Error),
    TryRecvError(mpsc::TryRecvError),
    UnpackFailed,
    UserNotFound(String),
}

impl fmt::Display for SupError {
    // We create a string for each type of error, then create a `StructuredOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match self.err {
            Error::APIClient(ref err) => format!("{}", err),
            Error::BadAddress(ref err) => format!("Unable to bind to address {}.", err),
            Error::BadCompositesPath(ref path, ref err) => format!(
                "Unable to create the composites directory '{}' ({})",
                path.display(),
                err
            ),
            Error::Departed => format!(
                "This Supervisor has been manually departed.\n\nFor the safety of the system, \
                 this Supervisor cannot be started (if we did, we would risk the services on \
                 this machine behaving badly without our knowledge.) If you know that the \
                 services on this system are safe, and want them to rejoin the habitat ring, \
                 you need to:\n\n  rm -rf /hab/sup/default/MEMBER_ID /hab/sup/default/data\n\n \
                 This will cause the Supervisor to join the ring as a new member.\n\n \
                 If you are in doubt, it is better to consider the services managed by this \
                 Supervisor as unsafe to run."
            ),
            Error::BadDataFile(ref path, ref err) => format!(
                "Unable to read or write to data file, {}, {}",
                path.display(),
                err
            ),
            Error::BadDataPath(ref path, ref err) => format!(
                "Unable to read or write to data directory, {}, {}",
                path.display(),
                err
            ),
            Error::BadDesiredState(ref state) => {
                format!("Unknown service desired state style '{}'", state)
            }
            Error::BadElectionStatus(ref status) => format!("Unknown election status '{}'", status),
            Error::BadPackage(ref pkg, ref err) => format!("Bad package, {}, {}", pkg, err),
            Error::BadSpecsPath(ref path, ref err) => format!(
                "Unable to create the specs directory '{}' ({})",
                path.display(),
                err
            ),
            Error::BadStartStyle(ref style) => format!("Unknown service start style '{}'", style),
            Error::BadEnvConfig(ref varname) => {
                format!("Unable to find valid TOML or JSON in {} ENVVAR", varname)
            }
            Error::BindTimeout(ref err) => format!("Timeout waiting to bind to {}", err),
            Error::LockPoisoned => format!("A mutex or read/write lock has failed."),
            Error::TestBootFail => format!("Simulated boot failure"),
            Error::ButterflyError(ref err) => format!("Butterfly error: {}", err),
            Error::CtlSecretIo(ref path, ref err) => format!(
                "IoError while reading or writing ctl secret, {}, {}",
                path.display(),
                err
            ),
            Error::ExecCommandNotFound(ref c) => {
                format!("`{}' was not found on the filesystem or in PATH", c)
            }
            Error::Permissions(ref err) => format!("{}", err),
            Error::HabitatCommon(ref err) => format!("{}", err),
            Error::HabitatCore(ref err) => format!("{}", err),
            Error::TemplateFileError(ref err) => format!("{:?}", err),
            Error::TemplateRenderError(ref err) => format!("{}", err),
            Error::EnvJoinPathsError(ref err) => format!("{}", err),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::FileWatcherFileIsRoot => format!("Watched file is root"),
            Error::GroupNotFound(ref e) => format!("No GID for group '{}' could be found", e),
            Error::InvalidBinding(ref binding) => format!(
                "Invalid binding \"{}\", must be of the form <NAME>:<SERVICE_GROUP> or \
                    <SERVICE_NAME>:<NAME>:<SERVICE_GROUP> where <NAME> is a service name,
                    <SERVICE_GROUP> is a valid service group, and <SERVICE_NAME> is the name of
                    a service within a composite if the given bind is for a composite service.",
                binding
            ),
            Error::InvalidBinds(ref e) => format!("Invalid bind(s), {}", e.join(", ")),
            Error::InvalidCertFile(ref path) => format!("Invalid cert file: {}", path.display()),
            Error::InvalidKeyFile(ref path) => format!("Invalid key file: {}", path.display()),
            Error::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            Error::InvalidPidFile => format!("Invalid child process PID file"),
            Error::InvalidTokioThreadCount => {
                format!("Tokio thread count should be a positive integer")
            }
            Error::InvalidTopology(ref t) => format!("Invalid topology: {}", t),
            Error::InvalidUpdateStrategy(ref s) => format!("Invalid update strategy: {}", s),
            Error::Io(ref err) => format!("{}", err),
            Error::IPFailed => format!("Failed to discover this hosts outbound IP address"),
            Error::Launcher(ref err) => format!("{}", err),
            Error::MissingRequiredBind(ref e) => {
                format!("Missing required bind(s), {}", e.join(", "))
            }
            Error::MissingRequiredIdent => {
                format!("Missing required ident field: (example: ident = \"core/redis\")")
            }
            Error::NameLookup(ref e) => format!("Error resolving a name or IP address: {}", e),
            Error::NetErr(ref err) => format!("{}", err),
            Error::NetParseError(ref e) => format!("Can't parse ip:port: {}", e),
            Error::NoActiveMembers(ref g) => format!("No active members in service group {}", g),
            Error::NoLauncher => format!("Supervisor must be run from `hab-launch`"),
            Error::NoSuchBind(ref b) => format!("No such bind: {}", b),
            Error::NotifyCreateError(ref e) => format!("Notify create error: {}", e),
            Error::NotifyError(ref e) => format!("Notify error: {}", e),
            Error::NulError(ref e) => format!("{}", e),
            Error::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::PidFileCorrupt(ref path) => {
                format!("Unable to decode contents of PID file, {}", path.display())
            }
            Error::PidFileIO(ref path, ref err) => {
                format!("Unable to read PID file, {}, {}", path.display(), err)
            }
            Error::ProcessLockCorrupt => format!("Unable to decode contents of process lock"),
            Error::ProcessLocked(ref pid) => format!(
                "Unable to start Habitat Supervisor because another instance is already \
                 running with the pid {}.",
                pid
            ),
            Error::ProcessLockIO(ref path, ref err) => format!(
                "Unable to start Habitat Supervisor because we weren't able to write or \
                 read to a process lock at {}, {}",
                path.display(),
                err
            ),
            Error::RecvError(ref err) => format!("{}", err),
            Error::RenderContextSerialization(ref e) => {
                format!("Unable to serialize rendering context, {}", e)
            }
            Error::ServiceDeserializationError(ref e) => {
                format!("Can't deserialize service status: {}", e)
            }
            Error::ServiceNotLoaded(ref ident) => format!("Service {} not loaded", ident),
            Error::ServiceSerializationError(ref e) => {
                format!("Can't serialize service to file: {}", e)
            }
            Error::ServiceSpecFileIO(ref path, ref err) => format!(
                "Unable to write or read to a service spec file at {}, {}",
                path.display(),
                err
            ),
            Error::ServiceSpecParse(ref err) => {
                format!("Unable to parse contents of service spec file, {}", err)
            }
            Error::ServiceSpecRender(ref err) => {
                format!("Service spec could not be rendered successfully: {}", err)
            }
            Error::SignalFailed => format!("Failed to send a signal to the child process"),
            Error::SpecWatcherDirNotFound(ref path) => format!(
                "Spec directory '{}' not created or is not a directory",
                path
            ),
            Error::SpecWatcherGlob(ref e) => format!("{}", e),
            Error::StrFromUtf8Error(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TomlEncode(ref e) => format!("Failed to encode TOML: {}", e),
            Error::TomlMergeError(ref e) => format!("Failed to merge TOML: {}", e),
            Error::TomlParser(ref err) => format!("Failed to parse TOML: {}", err),
            Error::TryRecvError(ref err) => format!("{}", err),
            Error::UnpackFailed => format!("Failed to unpack a package"),
            Error::UserNotFound(ref e) => format!("No UID for user '{}' could be found", e),
        };
        let progname = PROGRAM_NAME.as_str();
        let mut so = StructuredOutput::new(
            progname,
            self.logkey,
            self.line,
            self.file,
            self.column,
            &content,
        );
        so.verbose = Some(true);
        write!(f, "{}", so)
    }
}

impl error::Error for SupError {
    fn description(&self) -> &str {
        match self.err {
            Error::APIClient(ref err) => err.description(),
            Error::BadAddress(_) => "Unable to bind to address",
            Error::BadCompositesPath(_, _) => "Unable to create the composites directory",
            Error::Departed => "Supervisor has been manually departed",
            Error::BadDataFile(_, _) => "Unable to read or write to a data file",
            Error::BadDataPath(_, _) => "Unable to read or write to data directory",
            Error::BadElectionStatus(_) => "Unknown election status",
            Error::BadDesiredState(_) => "Unknown desired state in service spec",
            Error::BadPackage(_, _) => "Package was malformed or contained malformed contents",
            Error::BadSpecsPath(_, _) => "Unable to create the specs directory",
            Error::BadStartStyle(_) => "Unknown start style in service spec",
            Error::BadEnvConfig(_) => "Unknown syntax in Env Configuration",
            Error::BindTimeout(_) => "Timeout waiting to bind to an address",
            Error::LockPoisoned => "A mutex or read/write lock has failed",
            Error::TestBootFail => "Simulated boot failure",
            Error::ButterflyError(ref err) => err.description(),
            Error::CtlSecretIo(_, _) => "IoError while reading ctl secret",
            Error::ExecCommandNotFound(_) => "Exec command was not found on filesystem or in PATH",
            Error::GroupNotFound(_) => "No matching GID for group found",
            Error::TemplateFileError(ref err) => err.description(),
            Error::TemplateRenderError(ref err) => err.description(),
            Error::HabitatCommon(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::EnvJoinPathsError(ref err) => err.description(),
            Error::FileNotFound(_) => "File not found",
            Error::FileWatcherFileIsRoot => "Watched file is root",
            Error::InvalidBinding(_) => "Invalid binding parameter",
            Error::InvalidBinds(_) => {
                "Service binds detected that are neither required nor optional package binds"
            }
            Error::InvalidCertFile(_) => "Invalid cert file",
            Error::InvalidKeyFile(_) => "Invalid key file",
            Error::InvalidKeyParameter(_) => "Key parameter error",
            Error::InvalidPidFile => "Invalid child process PID file",
            Error::InvalidTokioThreadCount => "Invalid Tokio thread count",
            Error::InvalidTopology(_) => "Invalid topology",
            Error::InvalidUpdateStrategy(_) => "Invalid update strategy",
            Error::Io(ref err) => err.description(),
            Error::IPFailed => "Failed to discover the outbound IP address",
            Error::Launcher(ref err) => err.description(),
            Error::MissingRequiredBind(_) => {
                "A service to start without specifying a service group for all required binds"
            }
            Error::MissingRequiredIdent => {
                "Missing required ident field: (example: ident = \"core/redis\")"
            }
            Error::NetErr(ref err) => err.description(),
            Error::NetParseError(_) => "Can't parse IP:port",
            Error::NameLookup(_) => "Error resolving a name or IP address",
            Error::NoActiveMembers(_) => "Group has no active members",
            Error::NoLauncher => "Supervisor must be run from `hab-launch`",
            Error::NoSuchBind(_) => "No such bind found for this service",
            Error::NotifyCreateError(_) => "Notify create error",
            Error::NotifyError(_) => "Notify error",
            Error::NulError(_) => {
                "An attempt was made to build a CString with a null byte inside it"
            }
            Error::PackageNotFound(_) => "Cannot find a package",
            Error::Permissions(_) => "File system permissions error",
            Error::PidFileCorrupt(_) => "Unable to decode contents of PID file",
            Error::PidFileIO(_, _) => "Unable to read or write to PID file",
            Error::ProcessLockCorrupt => "Unable to decode contents of process lock",
            Error::ProcessLocked(_) => {
                "Another instance of the Habitat Supervisor is already running"
            }
            Error::ProcessLockIO(_, _) => "Unable to read or write to a process lock",
            Error::RecvError(_) => "A channel failed to receive a response",
            Error::RenderContextSerialization(_) => "Unable to serialize rendering context",
            Error::ServiceDeserializationError(_) => "Can't deserialize service status",
            Error::ServiceNotLoaded(_) => "Service status called when service not loaded",
            Error::ServiceSerializationError(_) => "Can't serialize service to file",
            Error::ServiceSpecFileIO(_, _) => "Unable to write or read to a service spec file",
            Error::ServiceSpecParse(_) => "Service spec could not be parsed successfully",
            Error::ServiceSpecRender(_) => "Service spec TOML could not be rendered successfully",
            Error::SignalFailed => "Failed to send a signal to the child process",
            Error::SpecWatcherDirNotFound(_) => "Spec directory not created or is not a directory",
            Error::SpecWatcherGlob(_) => "Spec watcher file globbing error",
            Error::StrFromUtf8Error(_) => "Failed to convert a str from a &[u8] as UTF-8",
            Error::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            Error::TomlEncode(_) => "Failed to encode toml!",
            Error::TomlMergeError(_) => "Failed to merge TOML!",
            Error::TomlParser(_) => "Failed to parse TOML!",
            Error::TryRecvError(_) => "A channel failed to receive a response",
            Error::UnpackFailed => "Failed to unpack a package",
            Error::UserNotFound(_) => "No matching UID for user found",
        }
    }
}

impl From<api_client::Error> for SupError {
    fn from(err: api_client::Error) -> SupError {
        sup_error!(Error::APIClient(err))
    }
}

impl From<SupError> for protocol::net::NetErr {
    fn from(err: SupError) -> protocol::net::NetErr {
        protocol::net::err(protocol::net::ErrCode::Internal, err)
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

impl From<launcher_client::Error> for SupError {
    fn from(err: launcher_client::Error) -> SupError {
        sup_error!(Error::Launcher(err))
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

impl From<mpsc::RecvError> for SupError {
    fn from(err: mpsc::RecvError) -> SupError {
        sup_error!(Error::RecvError(err))
    }
}

impl From<mpsc::TryRecvError> for SupError {
    fn from(err: mpsc::TryRecvError) -> SupError {
        sup_error!(Error::TryRecvError(err))
    }
}

impl From<notify::Error> for SupError {
    fn from(err: notify::Error) -> SupError {
        sup_error!(Error::NotifyError(err))
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

impl From<protocol::net::NetErr> for SupError {
    fn from(err: protocol::net::NetErr) -> Self {
        sup_error!(Error::NetErr(err))
    }
}
