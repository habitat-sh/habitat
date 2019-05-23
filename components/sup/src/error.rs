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

use crate::event;
use futures::sync::oneshot;
use glob;
use habitat_api_client;
use habitat_butterfly;
use habitat_common::{self,
                     output::{self,
                              OutputContext,
                              OutputVerbosity,
                              StructuredOutput},
                     PROGRAM_NAME};
use habitat_core::{self,
                   os::process::Pid,
                   package::{self,
                             Identifiable}};
use habitat_launcher_client;
use habitat_sup_protocol;
use notify;
use rustls;
use serde_json;
use std::{env,
          error::{self,
                  Error as _},
          ffi,
          fmt,
          io,
          net,
          path::PathBuf,
          result,
          str,
          string,
          sync::mpsc};
use toml;

static LOGKEY: &'static str = "ER";

/// Our result type alias, for easy coding.
pub type Result<T> = result::Result<T, SupError>;

#[derive(Debug)]
/// All errors in the Supervisor are kept in this struct. We store `Error`, an enum with a variant
/// for every type of error we produce. It also stores the location the error was created.
pub struct SupError {
    pub err: Error,
    logkey:  &'static str,
    file:    &'static str,
    line:    u32,
    column:  u32,
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
        SupError { err,
                   logkey,
                   file,
                   line,
                   column }
    }
}

/// All the kinds of errors we produce.
#[derive(Debug)]
pub enum Error {
    Departed,
    BadAddress(String),
    BadDataFile(PathBuf, io::Error),
    BadDataPath(PathBuf, io::Error),
    BadDesiredState(String),
    BadElectionStatus(String),
    BadSpecsPath(PathBuf, io::Error),
    BadStartStyle(String),
    BindTimeout(String),
    LockPoisoned,
    TestBootFail,
    ButterflyError(habitat_butterfly::error::Error),
    CtlSecretIo(PathBuf, io::Error),
    APIClient(habitat_api_client::Error),
    EnvJoinPathsError(env::JoinPathsError),
    ExecCommandNotFound(String),
    EventError(event::Error),
    FileNotFound(String),
    FileWatcherFileIsRoot,
    GroupNotFound(String),
    HabitatCommon(habitat_common::Error),
    HabitatCore(habitat_core::Error),
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
    Launcher(habitat_launcher_client::Error),
    MissingRequiredBind(Vec<String>),
    MissingRequiredIdent,
    NameLookup(io::Error),
    NetErr(habitat_sup_protocol::net::NetErr),
    NetParseError(net::AddrParseError),
    NoActiveMembers(habitat_core::service::ServiceGroup),
    NoLauncher,
    NoSuchBind(String),
    NotifyCreateError(notify::Error),
    NotifyError(notify::Error),
    NulError(ffi::NulError),
    OneshotCanceled(oneshot::Canceled),
    PackageNotFound(package::PackageIdent),
    PackageNotRunnable(package::PackageIdent),
    Permissions(String),
    PidFileCorrupt(PathBuf),
    PidFileIO(PathBuf, io::Error),
    ProcessLockCorrupt,
    ProcessLocked(Pid),
    ProcessLockIO(PathBuf, io::Error),
    RecvError(mpsc::RecvError),
    RecvTimeoutError(mpsc::RecvTimeoutError),
    ServiceDeserializationError(serde_json::Error),
    ServiceNotLoaded(package::PackageIdent),
    ServiceSerializationError(serde_json::Error),
    ServiceSpecFileIO(PathBuf, io::Error),
    ServiceSpecParse(toml::de::Error),
    ServiceSpecRender(toml::ser::Error),
    SignalFailed,
    SpecWatcherNotCreated,
    SpecDirNotFound(String),
    SpecWatcherGlob(glob::PatternError),
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    TLSError(rustls::TLSError),
    TomlEncode(toml::ser::Error),
    TryRecvError(mpsc::TryRecvError),
    UnpackFailed,
    UserNotFound(String),
}

impl fmt::Display for SupError {
    // We create a string for each type of error, then create a `StructuredOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self.err {
            Error::APIClient(ref err) => err.to_string(),
            Error::BadAddress(ref err) => format!("Unable to bind to address {}.", err),
            Error::Departed => "This Supervisor has been manually departed.\n\nFor the safety of \
                                the system, this Supervisor cannot be started (if we did, we \
                                would risk the services on this machine behaving badly without \
                                our knowledge.) If you know that the services on this system are \
                                safe, and want them to rejoin the habitat ring, you need to:\n\n  \
                                rm -rf /hab/sup/default/MEMBER_ID /hab/sup/default/data\n\n This \
                                will cause the Supervisor to join the ring as a new member.\n\n \
                                If you are in doubt, it is better to consider the services \
                                managed by this Supervisor as unsafe to run."
                                                                             .to_string(),
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
            Error::BadElectionStatus(ref status) => format!("Unknown election status '{}'", status),
            Error::BadSpecsPath(ref path, ref err) => {
                format!("Unable to create the specs directory '{}' ({})",
                        path.display(),
                        err)
            }
            Error::BadStartStyle(ref style) => format!("Unknown service start style '{}'", style),
            Error::BindTimeout(ref err) => format!("Timeout waiting to bind to {}", err),
            Error::LockPoisoned => "A mutex or read/write lock has failed.".to_string(),
            Error::TestBootFail => "Simulated boot failure".to_string(),
            Error::ButterflyError(ref err) => format!("Butterfly error: {}", err),
            Error::CtlSecretIo(ref path, ref err) => {
                format!("IoError while reading or writing ctl secret, {}, {}",
                        path.display(),
                        err)
            }
            Error::ExecCommandNotFound(ref c) => {
                format!("`{}' was not found on the filesystem or in PATH", c)
            }
            Error::EventError(ref err) => err.to_string(),
            Error::Permissions(ref err) => err.to_string(),
            Error::HabitatCommon(ref err) => err.to_string(),
            Error::HabitatCore(ref err) => err.to_string(),
            Error::EnvJoinPathsError(ref err) => err.to_string(),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::FileWatcherFileIsRoot => "Watched file is root".to_string(),
            Error::GroupNotFound(ref e) => format!("No GID for group '{}' could be found", e),
            Error::InvalidBinds(ref e) => format!("Invalid bind(s), {}", e.join(", ")),
            Error::InvalidCertFile(ref path) => format!("Invalid cert file: {}", path.display()),
            Error::InvalidKeyFile(ref path) => format!("Invalid key file: {}", path.display()),
            Error::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            Error::InvalidPidFile => "Invalid child process PID file".to_string(),
            Error::InvalidTokioThreadCount => {
                "Tokio thread count should be a positive integer".to_string()
            }
            Error::InvalidTopology(ref t) => format!("Invalid topology: {}", t),
            Error::InvalidUpdateStrategy(ref s) => format!("Invalid update strategy: {}", s),
            Error::Io(ref err) => err.to_string(),
            Error::IPFailed => "Failed to discover this hosts outbound IP address".to_string(),
            Error::Launcher(ref err) => err.to_string(),
            Error::MissingRequiredBind(ref e) => {
                format!("Missing required bind(s), {}", e.join(", "))
            }
            Error::MissingRequiredIdent => {
                "Missing required ident field: (example: ident = \"core/redis\")".to_string()
            }
            Error::NameLookup(ref e) => format!("Error resolving a name or IP address: {}", e),
            Error::NetErr(ref err) => err.to_string(),
            Error::NetParseError(ref e) => format!("Can't parse ip:port: {}", e),
            Error::NoActiveMembers(ref g) => format!("No active members in service group {}", g),
            Error::NoLauncher => "Supervisor must be run from `hab-launch`".to_string(),
            Error::NoSuchBind(ref b) => format!("No such bind: {}", b),
            Error::NotifyCreateError(ref e) => format!("Notify create error: {}", e),
            Error::NotifyError(ref e) => format!("Notify error: {}", e),
            Error::NulError(ref e) => e.to_string(),
            Error::OneshotCanceled(ref e) => e.to_string(),
            Error::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::PackageNotRunnable(ref pkg) => format!("Package is not runnable: {}", pkg),
            Error::PidFileCorrupt(ref path) => {
                format!("Unable to decode contents of PID file, {}", path.display())
            }
            Error::PidFileIO(ref path, ref err) => {
                format!("Unable to read PID file, {}, {}", path.display(), err)
            }
            Error::ProcessLockCorrupt => "Unable to decode contents of process lock".to_string(),
            Error::ProcessLocked(ref pid) => {
                format!("Unable to start Habitat Supervisor because another instance is already \
                         running with the pid {}.",
                        pid)
            }
            Error::ProcessLockIO(ref path, ref err) => {
                format!("Unable to start Habitat Supervisor because we weren't able to write or \
                         read to a process lock at {}, {}",
                        path.display(),
                        err)
            }
            Error::RecvError(ref err) => err.to_string(),
            Error::RecvTimeoutError(ref err) => err.to_string(),
            Error::ServiceDeserializationError(ref e) => {
                format!("Can't deserialize service status: {}", e)
            }
            Error::ServiceNotLoaded(ref ident) => format!("Service {} not loaded", ident),
            Error::ServiceSerializationError(ref e) => {
                format!("Can't serialize service to file: {}", e)
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
            Error::SignalFailed => "Failed to send a signal to the child process".to_string(),
            Error::SpecWatcherNotCreated => "Failed to create a SpecWatcher".to_string(),
            Error::SpecDirNotFound(ref path) => {
                format!("Spec directory '{}' not created or is not a directory",
                        path)
            }
            Error::SpecWatcherGlob(ref e) => e.to_string(),
            Error::StrFromUtf8Error(ref e) => e.to_string(),
            Error::StringFromUtf8Error(ref e) => e.to_string(),
            Error::TLSError(ref e) => e.to_string(),
            Error::TomlEncode(ref e) => format!("Failed to encode TOML: {}", e),
            Error::TryRecvError(ref err) => err.to_string(),
            Error::UnpackFailed => "Failed to unpack a package".to_string(),
            Error::UserNotFound(ref e) => format!("No UID for user '{}' could be found", e),
        };

        let progname = PROGRAM_NAME.as_str();
        let so = StructuredOutput::new(progname,
                                       self.logkey,
                                       OutputContext { line:   self.line,
                                                       file:   self.file,
                                                       column: self.column, },
                                       output::get_format(),
                                       OutputVerbosity::Verbose,
                                       &content);

        // TODO (CM): Consider implementing Error::source() for all
        // our errors as a more formalized way of exposing an
        // underlying error. See
        // https://github.com/habitat-sh/habitat/issues/6556 for details.
        if let Some(source) = self.source() {
            write!(f, "{} -> {}", so, source)
        } else {
            write!(f, "{}", so)
        }
    }
}

impl error::Error for SupError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.err {
            // Nothing else implements source yet
            Error::EventError(ref e) => e.source(),
            _ => None,
        }
    }
}

impl From<rustls::TLSError> for SupError {
    fn from(err: rustls::TLSError) -> SupError { sup_error!(Error::TLSError(err)) }
}

impl From<habitat_api_client::Error> for SupError {
    fn from(err: habitat_api_client::Error) -> SupError { sup_error!(Error::APIClient(err)) }
}

impl From<SupError> for habitat_sup_protocol::net::NetErr {
    fn from(err: SupError) -> habitat_sup_protocol::net::NetErr {
        habitat_sup_protocol::net::err(habitat_sup_protocol::net::ErrCode::Internal, err)
    }
}

impl From<net::AddrParseError> for SupError {
    fn from(err: net::AddrParseError) -> SupError { sup_error!(Error::NetParseError(err)) }
}

impl From<habitat_butterfly::error::Error> for SupError {
    fn from(err: habitat_butterfly::error::Error) -> SupError {
        sup_error!(Error::ButterflyError(err))
    }
}

impl From<habitat_common::Error> for SupError {
    fn from(err: habitat_common::Error) -> SupError { sup_error!(Error::HabitatCommon(err)) }
}

impl From<glob::PatternError> for SupError {
    fn from(err: glob::PatternError) -> SupError { sup_error!(Error::SpecWatcherGlob(err)) }
}

impl From<habitat_core::Error> for SupError {
    fn from(err: habitat_core::Error) -> SupError { sup_error!(Error::HabitatCore(err)) }
}

impl From<ffi::NulError> for SupError {
    fn from(err: ffi::NulError) -> SupError { sup_error!(Error::NulError(err)) }
}

impl From<io::Error> for SupError {
    fn from(err: io::Error) -> SupError { sup_error!(Error::Io(err)) }
}

impl From<env::JoinPathsError> for SupError {
    fn from(err: env::JoinPathsError) -> SupError { sup_error!(Error::EnvJoinPathsError(err)) }
}

impl From<habitat_launcher_client::Error> for SupError {
    fn from(err: habitat_launcher_client::Error) -> SupError { sup_error!(Error::Launcher(err)) }
}

impl From<string::FromUtf8Error> for SupError {
    fn from(err: string::FromUtf8Error) -> SupError { sup_error!(Error::StringFromUtf8Error(err)) }
}

impl From<str::Utf8Error> for SupError {
    fn from(err: str::Utf8Error) -> SupError { sup_error!(Error::StrFromUtf8Error(err)) }
}

impl From<mpsc::RecvError> for SupError {
    fn from(err: mpsc::RecvError) -> SupError { sup_error!(Error::RecvError(err)) }
}

impl From<mpsc::RecvTimeoutError> for SupError {
    fn from(err: mpsc::RecvTimeoutError) -> SupError { sup_error!(Error::RecvTimeoutError(err)) }
}

impl From<mpsc::TryRecvError> for SupError {
    fn from(err: mpsc::TryRecvError) -> SupError { sup_error!(Error::TryRecvError(err)) }
}

impl From<notify::Error> for SupError {
    fn from(err: notify::Error) -> SupError { sup_error!(Error::NotifyError(err)) }
}

impl From<toml::ser::Error> for SupError {
    fn from(err: toml::ser::Error) -> Self { sup_error!(Error::TomlEncode(err)) }
}

impl From<habitat_sup_protocol::net::NetErr> for SupError {
    fn from(err: habitat_sup_protocol::net::NetErr) -> Self { sup_error!(Error::NetErr(err)) }
}

impl From<oneshot::Canceled> for SupError {
    fn from(err: oneshot::Canceled) -> Self { sup_error!(Error::OneshotCanceled(err)) }
}

impl From<event::Error> for SupError {
    fn from(err: event::Error) -> Self { sup_error!(Error::EventError(err)) }
}
