use std::{env,
          error,
          ffi,
          fmt,
          io,
          num,
          path::{self,
                 PathBuf},
          result};

use crate::{api_client,
            common,
            hcore,
            protocol::net,
            sup_client::SrvClientError};
use handlebars;
use serde_yaml;
use toml;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    APIClient(api_client::Error),
    ArgumentError(&'static str),
    ButterflyError(String),
    CacheSslCertError(String),
    CannotParseBinlinkBinaryName(PathBuf),
    CannotParseBinlinkTarget(PathBuf),
    CannotRemoveDockerStudio,
    CannotRemoveFromChannel((String, String)),
    CannotRemovePackage(hcore::package::PackageIdent, usize),
    CommandNotFoundInPkg((String, String)),
    CryptoCLI(String),
    CtlClient(SrvClientError),
    DockerDaemonDown,
    DockerFileSharingNotEnabled,
    DockerImageNotFound(String),
    DockerNetworkDown(String),
    EnvJoinPathsError(env::JoinPathsError),
    ExecCommandNotFound(PathBuf),
    FFINulError(ffi::NulError),
    FileNotFound(String),
    HabitatCommon(common::Error),
    HabitatCore(hcore::Error),
    // Boxed due to clippy::large_enum_variant
    HandlebarsRenderError(Box<handlebars::TemplateRenderError>),
    IO(io::Error),
    JobGroupPromoteOrDemote(api_client::Error, bool /* promote */),
    JobGroupCancel(api_client::Error),
    JobGroupPromoteOrDemoteUnprocessable(bool /* promote */),
    JsonErr(serde_json::Error),
    LicenseNotAccepted,
    NameLookup,
    NetErr(net::NetErr),
    PackageArchiveMalformed(String),
    ParseIntError(num::ParseIntError),
    PathPrefixError(path::StripPrefixError),
    ProvidesError(String),
    RemoteSupResolutionError(String, io::Error),
    RootRequired,
    ScheduleStatus(api_client::Error),
    SubcommandNotSupported(String),
    UnsupportedExportFormat(String),
    TomlDeserializeError(toml::de::Error),
    TomlSerializeError(toml::ser::Error),
    Utf8Error(String),
    YamlError(serde_yaml::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::APIClient(ref e) => e.to_string(),
            Error::ArgumentError(ref e) => e.to_string(),
            Error::ButterflyError(ref e) => e.to_string(),
            Error::CacheSslCertError(ref e) => format!("Cannot cache SSL_CERT_FILE: {}", e),
            Error::CannotParseBinlinkBinaryName(ref p) => {
                format!("Cannot parse binlink binary name from {}.", p.display())
            }
            Error::CannotParseBinlinkTarget(ref p) => {
                format!("Cannot parse binlink target path from {}.", p.display())
            }
            Error::CannotRemoveDockerStudio => {
                "Docker Studios are not persistent and cannot be removed".to_string()
            }
            Error::CannotRemoveFromChannel((ref p, ref c)) => {
                format!("{} cannot be removed from the {} channel.", p, c)
            }
            Error::CannotRemovePackage(ref p, ref c) => {
                format!("Can't remove package: {}. It is a dependency of {} packages",
                        p, c)
            }
            Error::CommandNotFoundInPkg((ref p, ref c)) => {
                format!("`{}' was not found under any 'PATH' directories in the {} package",
                        c, p)
            }
            Error::CryptoCLI(ref e) => e.to_string(),
            Error::CtlClient(ref e) => e.to_string(),
            Error::DockerDaemonDown => {
                "Can not connect to Docker. Is the Docker daemon running?".to_string()
            }
            #[cfg(not(windows))]
            Error::DockerFileSharingNotEnabled => {
                "File Sharing must be enabled in order to enter a studio.\nPlease enable it in the \
                 Docker preferences and share (at a minimum) your home directory."
                                                                                  .to_string()
            }
            #[cfg(windows)]
            Error::DockerFileSharingNotEnabled => {
                "File Sharing must be enabled in order to enter a studio.\nPlease select a drive \
                 to share in the Docker preferences."
                                                     .to_string()
            }
            Error::DockerImageNotFound(ref e) => {
                format!("The Docker image {} was not found in the docker registry.\nYou can \
                         specify your own Docker image using the HAB_DOCKER_STUDIO_IMAGE \
                         environment variable.",
                        e)
            }
            Error::DockerNetworkDown(ref e) => {
                format!("The Docker image {} is unreachable due to a network error.\nThe image \
                         must be reachable to ensure the versions of hab inside and outside the \
                         studio match.\nYou can specify your own Docker image using the \
                         HAB_DOCKER_STUDIO_IMAGE environment variable.",
                        e)
            }
            Error::EnvJoinPathsError(ref err) => format!("{}", err),
            Error::ExecCommandNotFound(ref c) => {
                format!("`{}' was not found on the filesystem or in PATH",
                        c.display())
            }
            Error::FFINulError(ref e) => e.to_string(),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::HabitatCommon(ref e) => e.to_string(),
            Error::HabitatCore(ref e) => e.to_string(),
            Error::HandlebarsRenderError(ref e) => e.to_string(),
            Error::IO(ref err) => format!("{}", err),
            Error::JobGroupPromoteOrDemoteUnprocessable(true) => {
                "Failed to promote job group, the build job is still in progress".to_string()
            }
            Error::JobGroupPromoteOrDemoteUnprocessable(false) => {
                "Failed to demote job group, the build job is still in progress".to_string()
            }
            Error::JobGroupPromoteOrDemote(ref e, promote) => {
                format!("Failed to {} job group: {:?}",
                        if promote { "promote" } else { "demote" },
                        e)
            }
            Error::JsonErr(ref e) => e.to_string(),
            Error::JobGroupCancel(ref e) => format!("Failed to cancel job group: {:?}", e),
            Error::LicenseNotAccepted => "License agreement not accepted".to_string(),
            Error::NameLookup => "Error resolving a name or IP address".to_string(),
            Error::NetErr(ref e) => e.to_string(),
            Error::PackageArchiveMalformed(ref e) => {
                format!("Package archive was unreadable or contained unexpected contents: {:?}",
                        e)
            }
            Error::ParseIntError(ref err) => format!("{}", err),
            Error::PathPrefixError(ref err) => format!("{}", err),
            Error::ProvidesError(ref err) => format!("Can't find {}", err),
            Error::RemoteSupResolutionError(ref sup_addr, ref err) => {
                format!("Failed to resolve remote supervisor '{}': {}",
                        sup_addr, err,)
            }
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation".to_string()
            }
            Error::ScheduleStatus(ref e) => format!("Failed to retrieve job group status: {:?}", e),
            Error::SubcommandNotSupported(ref e) => {
                format!("Subcommand `{}' not supported on this operating system", e)
            }
            Error::UnsupportedExportFormat(ref e) => format!("Unsupported export format: {}", e),
            Error::TomlDeserializeError(ref e) => format!("Can't deserialize TOML: {}", e),
            Error::TomlSerializeError(ref e) => format!("Can't serialize TOML: {}", e),
            Error::Utf8Error(ref e) => format!("Error processing a string as UTF-8: {}", e),
            Error::YamlError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

impl From<api_client::Error> for Error {
    fn from(err: api_client::Error) -> Error { Error::APIClient(err) }
}

impl From<common::Error> for Error {
    fn from(err: common::Error) -> Error { Error::HabitatCommon(err) }
}

impl From<ffi::NulError> for Error {
    fn from(err: ffi::NulError) -> Error { Error::FFINulError(err) }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error { Error::HabitatCore(err) }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Error {
        Error::HandlebarsRenderError(Box::new(err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::IO(err) }
}

impl From<path::StripPrefixError> for Error {
    fn from(err: path::StripPrefixError) -> Error { Error::PathPrefixError(err) }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self { Error::TomlDeserializeError(err) }
}
impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self { Error::TomlSerializeError(err) }
}

impl From<env::JoinPathsError> for Error {
    fn from(err: env::JoinPathsError) -> Self { Error::EnvJoinPathsError(err) }
}

impl From<SrvClientError> for Error {
    fn from(err: SrvClientError) -> Self { Error::CtlClient(err) }
}

impl From<net::NetErr> for Error {
    fn from(err: net::NetErr) -> Self { Error::NetErr(err) }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self { Error::JsonErr(err) }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error { Error::YamlError(err) }
}
