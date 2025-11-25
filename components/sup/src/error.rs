use crate::event;
use futures::channel::oneshot;
use habitat_core::{self,
                   package::{self,
                             Identifiable}};
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
          sync::mpsc,
          time::Duration};
use tokio::task::JoinError;

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
    EnvVarError(env::VarError),
    ExecCommandNotFound(String),
    EventError(event::Error),
    FileNotFound(String),
    FileWatcherFileIsRoot,
    GroupNotFound(String),
    Hab(Box<hab::error::Error>),
    HabitatCommon(Box<habitat_common::Error>),
    HabitatCore(Box<habitat_core::Error>),
    InvalidBinds(Vec<String>),
    InvalidCertFile(PathBuf),
    InvalidHealthCheckResult(i32),
    InvalidKeyFile(PathBuf),
    InvalidKeyParameter(String),
    InvalidPidFile,
    InvalidTopology(String),
    InvalidUpdateStrategy(String),
    Io(io::Error),
    TaskJoin(JoinError),
    LauncherIPCCommand(habitat_launcher_client::IPCCommandError),
    LauncherTryIPCCommand(habitat_launcher_client::TryIPCCommandError),
    LockFileError(crate::lock_file::Error),
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
    TLSError(rustls::Error),
    TomlEncode(toml::ser::Error),
    TryRecvError(mpsc::TryRecvError),
    UnpackFailed,
    UserNotFound(String),
    WithDuration(Box<Self>, Duration),
}

impl Error {
    /// Give this error an associated duration.
    ///
    /// This is useful for providing feedback on failible, long running tasks.
    pub fn with_duration(self, duration: Duration) -> Self {
        Self::WithDuration(Box::new(self), duration)
    }
}

impl fmt::Display for Error {
    // We create a string for each type of error, then create a `StructuredOutput` for it, flip
    // verbose on, and print it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content = match self {
            Error::APIClient(err) => err.to_string(),
            Error::BadAddress(err) => format!("Unable to bind to address {}.", err),
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
            Error::BadDataFile(path, err) => {
                format!("Unable to read or write to data file, {}, {}",
                        path.display(),
                        err)
            }
            Error::BadDataPath(path, err) => {
                format!("Unable to read or write to data directory, {}, {}",
                        path.display(),
                        err)
            }
            Error::BadDesiredState(state) => {
                format!("Unknown service desired state style '{}'", state)
            }
            Error::BadElectionStatus(status) => format!("Unknown election status '{}'", status),
            Error::BadSpecsPath(path, err) => {
                format!("Unable to create the specs directory '{}' ({})",
                        path.display(),
                        err)
            }
            Error::BadStartStyle(style) => format!("Unknown service start style '{}'", style),
            Error::BindTimeout(err) => format!("Timeout waiting to bind to {}", err),
            Error::LockPoisoned => "A mutex or read/write lock has failed.".to_string(),

            // This '->' formatting is taken from the thiserror crate and how it
            // presents source error information. Ultimately, it would be good
            // to standardize on that, as it provides an easy way to generate
            // informative error messages for users.
            Error::LockFileError(e) => format!("Lock file error -> {}", e),

            Error::TestBootFail => "Simulated boot failure".to_string(),
            Error::ButterflyError(err) => format!("Butterfly error: {}", err),
            Error::CtlSecretIo(path, err) => {
                format!("IoError while reading or writing ctl secret, {}, {}",
                        path.display(),
                        err)
            }
            Error::ExecCommandNotFound(c) => {
                format!("`{}' was not found on the filesystem or in PATH", c)
            }
            Error::EventError(err) => err.to_string(),
            Error::Permissions(err) => err.to_string(),
            Error::Hab(err) => err.to_string(),
            Error::HabitatCommon(err) => err.to_string(),
            Error::HabitatCore(err) => err.to_string(),
            Error::EnvJoinPathsError(err) => err.to_string(),
            Error::EnvVarError(err) => err.to_string(),
            Error::FileNotFound(e) => format!("File not found at: {}", e),
            Error::FileWatcherFileIsRoot => "Watched file is root".to_string(),
            Error::GroupNotFound(e) => format!("No GID for group '{}' could be found", e),
            Error::InvalidBinds(e) => format!("Invalid bind(s), {}", e.join(", ")),
            Error::InvalidCertFile(path) => format!("Invalid cert file: {}", path.display()),
            Error::InvalidHealthCheckResult(code) => {
                format!("Invalid health check result: {}", code)
            }
            Error::InvalidKeyFile(path) => format!("Invalid key file: {}", path.display()),
            Error::InvalidKeyParameter(e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            Error::InvalidPidFile => "Invalid child process PID file".to_string(),
            Error::InvalidTopology(t) => format!("Invalid topology: {}", t),
            Error::InvalidUpdateStrategy(s) => format!("Invalid update strategy: {}", s),
            Error::Io(err) => err.to_string(),
            Error::TaskJoin(err) => err.to_string(),
            Error::LauncherIPCCommand(err) => {
                let mut chain: Vec<String> = vec![format!("{}", err)];
                let mut root = err.source();
                while let Some(cause) = root {
                    chain.push(format!("{}", cause));
                    root = cause.source();
                }
                format!("Supervisor failed to execute launcher command via IPC: {}",
                        chain.join(", "))
            }
            Error::LauncherTryIPCCommand(err) => {
                let mut chain: Vec<String> = vec![format!("{}", err)];
                let mut root = err.source();
                while let Some(cause) = root {
                    chain.push(format!("{}", cause));
                    root = cause.source();
                }
                format!("Supervisor failed to try executing launcher command via IPC: {}",
                        chain.join(", "))
            }
            Error::MissingRequiredBind(e) => {
                format!("Missing required bind(s), {}", e.join(", "))
            }
            Error::MissingRequiredIdent => {
                "Missing required ident field: (example: ident = \"core/redis\")".to_string()
            }
            Error::NameLookup(e) => format!("Error resolving a name or IP address: {}", e),
            Error::NetErr(err) => err.to_string(),
            Error::NetParseError(e) => {
                format!("Can't parse IP address or socket address: {}", e)
            }
            Error::NoActiveMembers(g) => format!("No active members in service group {}", g),
            Error::NoLauncher => "Supervisor must be run from `hab-launch`".to_string(),
            Error::NoSuchBind(b) => format!("No such bind: {}", b),
            Error::NotifyCreateError(e) => format!("Notify create error: {}", e),
            Error::NotifyError(e) => format!("Notify error: {}", e),
            Error::NulError(e) => e.to_string(),
            Error::OneshotCanceled(e) => e.to_string(),
            Error::PackageNotFound(pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::PackageNotRunnable(pkg) => format!("Package is not runnable: {}", pkg),
            Error::RecvError(err) => err.to_string(),
            Error::RecvTimeoutError(err) => err.to_string(),
            Error::ServiceDeserializationError(e) => {
                format!("Can't deserialize service status: {}", e)
            }
            Error::ServiceNotLoaded(ident) => format!("Service {} not loaded", ident),
            Error::ServiceSerializationError(e) => {
                format!("Can't serialize service to file: {}", e)
            }
            Error::ServiceSpecFileIO(path, err) => {
                format!("Unable to write or read to a service spec file at {}, {}",
                        path.display(),
                        err)
            }
            Error::ServiceSpecParse(err) => {
                format!("Unable to parse contents of service spec file, {}", err)
            }
            Error::ServiceSpecRender(err) => {
                format!("Service spec could not be rendered successfully: {}", err)
            }
            Error::SignalFailed => "Failed to send a signal to the child process".to_string(),
            Error::SpecWatcherNotCreated => "Failed to create a SpecWatcher".to_string(),
            Error::SpecDirNotFound(path) => {
                format!("Spec directory '{}' not created or is not a directory",
                        path)
            }
            Error::SpecWatcherGlob(e) => e.to_string(),
            Error::StrFromUtf8Error(e) => e.to_string(),
            Error::StringFromUtf8Error(e) => e.to_string(),
            Error::TLSError(e) => e.to_string(),
            Error::TomlEncode(e) => format!("Failed to encode TOML: {}", e),
            Error::TryRecvError(err) => err.to_string(),
            Error::UnpackFailed => "Failed to unpack a package".to_string(),
            Error::UserNotFound(e) => format!("No UID for user '{}' could be found", e),
            Error::WithDuration(e, duration) => {
                format!("{} ({} s)", e, duration.as_secs_f64())
            }
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
            Error::EventError(e) => e.source(),
            Error::LockFileError(e) => e.source(),
            _ => None,
        }
    }
}

impl From<rustls::Error> for Error {
    fn from(err: rustls::Error) -> Error { Error::TLSError(err) }
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

impl From<hab::error::Error> for Error {
    fn from(err: hab::error::Error) -> Error { Error::Hab(Box::new(err)) }
}

impl From<habitat_common::Error> for Error {
    fn from(err: habitat_common::Error) -> Error { Error::HabitatCommon(Box::new(err)) }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Error { Error::SpecWatcherGlob(err) }
}

impl From<habitat_core::Error> for Error {
    fn from(err: habitat_core::Error) -> Error { Error::HabitatCore(Box::new(err)) }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error { Error::NulError(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Error { Error::TaskJoin(err) }
}

impl From<env::JoinPathsError> for Error {
    fn from(err: env::JoinPathsError) -> Error { Error::EnvJoinPathsError(err) }
}

impl From<habitat_launcher_client::IPCCommandError> for Error {
    fn from(err: habitat_launcher_client::IPCCommandError) -> Error {
        Error::LauncherIPCCommand(err)
    }
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

impl From<env::VarError> for Error {
    fn from(err: env::VarError) -> Error { Error::EnvVarError(err) }
}

impl From<crate::lock_file::Error> for Error {
    fn from(src: crate::lock_file::Error) -> Self { Error::LockFileError(src) }
}
