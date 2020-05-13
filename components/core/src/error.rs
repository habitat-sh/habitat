use std::{env,
          error,
          ffi,
          fmt,
          io,
          num,
          path::PathBuf,
          result,
          str,
          string};

use regex;
use toml;

use crate::package::{self,
                     Identifiable};

pub type Result<T> = result::Result<T, Error>;

/// Core error types
#[derive(Debug)]
pub enum Error {
    BadBindingMode(String),
    /// An invalid path to a keyfile was given.
    BadKeyPath(String),
    /// An operation expected a composite package
    CompositePackageExpected(String),
    /// Error reading raw contents of configuration file.
    ConfigFileIO(PathBuf, io::Error),
    /// Parsing error while reading a configuration file.
    ConfigFileSyntax(toml::de::Error),
    /// Expected an array of socket addrs for configuration field value.
    ConfigInvalidArraySocketAddr(&'static str),
    /// Expected an array of tables containing string feilds and values for configuration
    /// field value.
    ConfigInvalidArrayTableString(&'static str),
    /// Expected an array of PackageTarget entries for configuration field value.
    ConfigInvalidArrayTarget(&'static str),
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
    /// Occurs when a call to CreateProcessAsUserW fails
    CreateProcessAsUserFailed(io::Error),
    /// Occurs when a call to CryptProtectData fails
    CryptProtectDataFailed(String),
    /// Occurs when a call to CryptUnprotectData fails
    CryptUnprotectDataFailed(String),
    /// Occurs when unable to locate the docker cli on the path
    DockerCommandNotFound(&'static str),
    /// Occurs when a file that should exist does not or could not be read.
    FileNotFound(String),
    /// Occurs when a fully-qualified package identifier is required,
    /// but a non-qualified identifier (e.g. "foo/bar" or
    /// "foo/bar/1.0.0") was given instead.
    FullyQualifiedPackageIdentRequired(String),
    /// Occurs when a service binding cannot be successfully parsed.
    InvalidBinding(String),
    /// Occurs when a package identifier string cannot be successfully parsed.
    InvalidPackageIdent(String),
    /// Occurs when a package target string cannot be successfully parsed.
    InvalidPackageTarget(String),
    /// Occurs when a package type is not recognized.
    InvalidPackageType(String),
    /// Occurs when a service group string cannot be successfully parsed.
    InvalidServiceGroup(String),
    /// Occurs when an origin is in an invalid format
    InvalidOrigin(String),
    /// Occurs when an OsString path cannot be converted to a String
    InvalidPathString(ffi::OsString),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    /// Errors when joining paths :)
    JoinPathsError(env::JoinPathsError),
    // When LogonUserW does not have the correct logon type
    LogonTypeNotGranted,
    /// Occurs when a call to LogonUserW fails
    LogonUserFailed(io::Error),
    /// Occurs when a BIND, BIND_OPTIONAL, or BIND_MAP MetaFile is
    /// read and contains a bad entry.
    MetaFileBadBind,
    /// Occurs when a package metadata file cannot be opened, read, or parsed.
    MetaFileMalformed(package::metadata::MetaFile),
    /// Occurs when a particular package metadata file is not found.
    MetaFileNotFound(package::metadata::MetaFile),
    /// When an IO error while accessing a MetaFile.
    MetaFileIO(io::Error),
    /// Occurs when we can't find an outbound IP address
    NoOutboundIpAddr(io::Error),
    /// Occurs when a call to OpenDesktopW fails
    OpenDesktopFailed(String),
    /// Occurs when a suitable installed package cannot be found.
    PackageNotFound(package::PackageIdent),
    /// Occurs where trying to unpack a package
    PackageUnpackFailed(String),
    /// When an error occurs parsing an integer.
    ParseIntError(num::ParseIntError),
    /// When parsing a string as an OS signal fails
    ParseSignalError(String),
    /// Occurs upon errors related to file or directory permissions.
    PermissionFailed(String),
    /// Error parsing the contents of a plan file were incomplete or malformed.
    PlanMalformed,
    // When CreateProcessAsUserW does not have the correct privileges
    PrivilegeNotHeld,
    /// When an error occurs parsing or compiling a regular expression.
    RegexParse(regex::Error),
    /// When an error occurs serializing rendering context
    RenderContextSerialization(serde_json::Error),
    /// When an error occurs converting a `String` from a UTF-8 byte vector.
    StringFromUtf8Error(string::FromUtf8Error),
    /// When the system target (platform and architecture) do not match the package target.
    TargetMatchError(String),
    /// Occurs when a `uname` libc call returns an error.
    UnameFailed(String),
    /// Occurs when a `waitpid` libc call returns an error.
    WaitpidFailed(String),
    /// Occurs when a `kill` libc call returns an error.
    SignalFailed(i32, io::Error),
    /// Occurs when the sodium library cannot be initialized.
    SodiumInitFailed,
    /// Occurs when a `CreateToolhelp32Snapshot` win32 call returns an error.
    CreateToolhelp32SnapshotFailed(String),
    /// Occurs when a `GetExitCodeProcess` win32 call returns an error.
    GetExitCodeProcessFailed(String),
    /// Occurs when a `WaitForSingleObject` win32 call returns an error.
    WaitForSingleObjectFailed(String),
    /// Occurs when a `TerminateProcess` win32 call returns an error.
    TerminateProcessFailed(u32, io::Error),
    /// Occurs if the host os kernel does not have a supported docker image
    UnsupportedDockerHostKernel(String),
    /// When an error occurs attempting to interpret a sequence of u8 as a string.
    Utf8Error(str::Utf8Error),
    /// When a `PackageTaget` for a package does not match the active `PackageTarget` for this
    /// system.
    WrongActivePackageTarget(package::PackageTarget, package::PackageTarget),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::BadBindingMode(ref value) => format!("Unknown binding mode '{}'", value),
            Error::BadKeyPath(ref e) => {
                format!("Invalid keypath: {}. Specify an absolute path to a file on disk.",
                        e)
            }
            Error::CompositePackageExpected(ref ident) => {
                format!("The package is not a composite: {}", ident)
            }
            Error::ConfigFileIO(ref f, ref e) => {
                format!("Error reading configuration file, {}, {}", f.display(), e)
            }
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
            Error::ConfigInvalidArrayTarget(ref f) => {
                format!("Invalid array value of targets containing string fields and values in \
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
            Error::CreateProcessAsUserFailed(ref e) => {
                format!("Failure calling CreateProcessAsUserW: {:?}", e)
            }
            Error::CryptoError(ref e) => format!("Crypto error: {}", e),
            Error::CryptProtectDataFailed(ref e) => e.to_string(),
            Error::CryptUnprotectDataFailed(ref e) => e.to_string(),
            Error::DockerCommandNotFound(ref c) => {
                format!("Docker command `{}' was not found on the filesystem or in PATH",
                        c)
            }
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::FullyQualifiedPackageIdentRequired(ref ident) => {
                format!("Fully-qualified package identifier was expected, but found: {:?}",
                        ident)
            }
            Error::InvalidBinding(ref binding) => {
                format!("Invalid binding '{}', must be of the form <NAME>:<SERVICE_GROUP> where \
                         <NAME> is a service name, and <SERVICE_GROUP> is a valid service group",
                        binding)
            }
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
            Error::InvalidPackageType(ref e) => format!("Invalid package type: {}.", e),
            Error::InvalidServiceGroup(ref e) => {
                format!("Invalid service group: {}. A valid service group string is in the form \
                         service.group (example: redis.production)",
                        e)
            }
            Error::InvalidOrigin(ref origin) => {
                format!("Invalid origin: {}. Origins must begin with a lowercase letter or \
                         number. Allowed characters include lowercase letters, numbers, -, and _. \
                         No more than 255 characters.",
                        origin)
            }
            Error::InvalidPathString(ref s) => {
                format!("Could not generate String from path: {:?}", s)
            }
            Error::IO(ref err) => format!("{}", err),
            Error::JoinPathsError(ref err) => format!("{}", err),
            Error::LogonTypeNotGranted => {
                "hab_svc_user user must possess the 'SE_SERVICE_LOGON_NAME' account right to be \
                 spawned as a service by the Supervisor"
                                                        .to_string()
            }
            Error::LogonUserFailed(ref e) => format!("Failure calling LogonUserW: {:?}", e),
            Error::MetaFileBadBind => {
                "Bad value parsed from BIND, BIND_OPTIONAL, or BIND_MAP".to_string()
            }
            Error::MetaFileMalformed(ref e) => {
                format!("MetaFile: {:?}, didn't contain a valid UTF-8 string", e)
            }
            Error::MetaFileNotFound(ref e) => format!("Couldn't read MetaFile: {}, not found", e),
            Error::MetaFileIO(ref e) => format!("IO error while accessing MetaFile: {:?}", e),
            Error::NoOutboundIpAddr(ref e) => {
                format!("Failed to discover this host's outbound IP address: {}", e)
            }
            Error::OpenDesktopFailed(ref e) => e.to_string(),
            Error::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::PackageUnpackFailed(ref e) => format!("Package could not be unpacked. {}", e),
            Error::ParseIntError(ref e) => format!("{}", e),
            Error::ParseSignalError(ref s) => format!("Failed to parse '{}' as a signal", s),
            Error::PlanMalformed => "Failed to read or parse contents of Plan file".to_string(),
            Error::PermissionFailed(ref e) => e.to_string(),
            Error::PrivilegeNotHeld => "Current user must possess the 'SE_INCREASE_QUOTA_NAME' \
                                        and 'SE_ASSIGNPRIMARYTOKEN_NAME' privilege to spawn a new \
                                        process as a different user"
                                                                    .to_string(),
            Error::RenderContextSerialization(ref e) => {
                format!("Unable to serialize rendering context, {}", e)
            }
            Error::RegexParse(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TargetMatchError(ref e) => e.to_string(),
            Error::UnameFailed(ref e) => e.to_string(),
            Error::WaitpidFailed(ref e) => e.to_string(),
            Error::SignalFailed(ref r, ref e) => {
                format!("Failed to send a signal to the child process: {}, {}", r, e)
            }
            Error::SodiumInitFailed => "Sodium library initialization failed".to_string(),
            Error::GetExitCodeProcessFailed(ref e) => e.to_string(),
            Error::CreateToolhelp32SnapshotFailed(ref e) => e.to_string(),
            Error::WaitForSingleObjectFailed(ref e) => e.to_string(),
            Error::TerminateProcessFailed(ref r, ref e) => {
                format!("Failed to terminate process: {}, {}", r, e)
            }
            Error::UnsupportedDockerHostKernel(ref e) => {
                format!("Unsupported Docker host kernel: {}", e)
            }
            Error::Utf8Error(ref e) => format!("{}", e),
            Error::WrongActivePackageTarget(ref active, ref wrong) => {
                format!("Package target '{}' is not supported as this system has a different \
                         active package target '{}'",
                        wrong, active)
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

impl From<env::JoinPathsError> for Error {
    fn from(err: env::JoinPathsError) -> Self { Error::JoinPathsError(err) }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self { Error::StringFromUtf8Error(err) }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self { Error::Utf8Error(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self { Error::IO(err) }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self { Error::ParseIntError(err) }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self { Error::RegexParse(err) }
}
