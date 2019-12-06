use crate::event;
use futures::channel::oneshot;
use glob;
use habitat_api_client;
use habitat_butterfly;
use habitat_common;
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

/// Our result type alias, for easy coding.
pub type Result<T> = result::Result<T, Error>;

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
    InvalidHealthCheckResult(i32),
    InvalidKeyFile(PathBuf),
    InvalidKeyParameter(String),
    InvalidPidFile,
    InvalidTopology(String),
    InvalidUpdateStrategy(String),
    Io(io::Error),
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

impl fmt::Display for Error {
    // We create a string for each type of error, then create a `StructuredOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self {
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
            Error::InvalidHealthCheckResult(code) => {
                format!("Invalid health check result: {}", code)
            }
            Error::InvalidKeyFile(ref path) => format!("Invalid key file: {}", path.display()),
            Error::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            Error::InvalidPidFile => "Invalid child process PID file".to_string(),
            Error::InvalidTopology(ref t) => format!("Invalid topology: {}", t),
            Error::InvalidUpdateStrategy(ref s) => format!("Invalid update strategy: {}", s),
            Error::Io(ref err) => err.to_string(),
            Error::Launcher(ref err) => err.to_string(),
            Error::MissingRequiredBind(ref e) => {
                format!("Missing required bind(s), {}", e.join(", "))
            }
            Error::MissingRequiredIdent => {
                "Missing required ident field: (example: ident = \"core/redis\")".to_string()
            }
            Error::NameLookup(ref e) => format!("Error resolving a name or IP address: {}", e),
            Error::NetErr(ref err) => err.to_string(),
            Error::NetParseError(ref e) => {
                format!("Can't parse IP address or socket address: {}", e)
            }
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

        // TODO (CM): Consider implementing Error::source() for all
        // our errors as a more formalized way of exposing an
        // underlying error. See
        // https://github.com/habitat-sh/habitat/issues/6556 for details.
        if let Some(source) = self.source() {
            write!(f, "{} -> {}", content, source)
        } else {
            write!(f, "{}", content)
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            // Nothing else implements source yet
            Error::EventError(ref e) => e.source(),
            _ => None,
        }
    }
}

impl From<rustls::TLSError> for Error {
    fn from(err: rustls::TLSError) -> Error { Error::TLSError(err) }
}

impl From<habitat_api_client::Error> for Error {
    fn from(err: habitat_api_client::Error) -> Error { Error::APIClient(err) }
}

impl From<Error> for habitat_sup_protocol::net::NetErr {
    fn from(err: Error) -> habitat_sup_protocol::net::NetErr {
        match err {
            Error::MissingRequiredBind(_) | Error::InvalidBinds(_) => {
                habitat_sup_protocol::net::err(habitat_sup_protocol::net::ErrCode::InvalidPayload,
                                               err)
            }
            _ => habitat_sup_protocol::net::err(habitat_sup_protocol::net::ErrCode::Internal, err),
        }
    }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error { Error::NetParseError(err) }
}

impl From<habitat_butterfly::error::Error> for Error {
    fn from(err: habitat_butterfly::error::Error) -> Error { Error::ButterflyError(err) }
}

impl From<habitat_common::Error> for Error {
    fn from(err: habitat_common::Error) -> Error { Error::HabitatCommon(err) }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Error { Error::SpecWatcherGlob(err) }
}

impl From<habitat_core::Error> for Error {
    fn from(err: habitat_core::Error) -> Error { Error::HabitatCore(err) }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error { Error::NulError(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}

impl From<env::JoinPathsError> for Error {
    fn from(err: env::JoinPathsError) -> Error { Error::EnvJoinPathsError(err) }
}

impl From<habitat_launcher_client::Error> for Error {
    fn from(err: habitat_launcher_client::Error) -> Error { Error::Launcher(err) }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error { Error::StringFromUtf8Error(err) }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error { Error::StrFromUtf8Error(err) }
}

impl From<mpsc::RecvError> for Error {
    fn from(err: mpsc::RecvError) -> Error { Error::RecvError(err) }
}

impl From<mpsc::RecvTimeoutError> for Error {
    fn from(err: mpsc::RecvTimeoutError) -> Error { Error::RecvTimeoutError(err) }
}

impl From<mpsc::TryRecvError> for Error {
    fn from(err: mpsc::TryRecvError) -> Error { Error::TryRecvError(err) }
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Error { Error::NotifyError(err) }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Error { Error::TomlEncode(err) }
}

impl From<habitat_sup_protocol::net::NetErr> for Error {
    fn from(err: habitat_sup_protocol::net::NetErr) -> Error { Error::NetErr(err) }
}

impl From<oneshot::Canceled> for Error {
    fn from(err: oneshot::Canceled) -> Error { Error::OneshotCanceled(err) }
}

impl From<event::Error> for Error {
    fn from(err: event::Error) -> Error { Error::EventError(err) }
}
