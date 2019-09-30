use std::{env,
          error,
          fmt,
          io,
          net,
          path::PathBuf,
          result,
          str,
          string};
use toml;

use crate::{api_client,
            hcore::{self,
                    package::PackageIdent}};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    APIClient(api_client::Error),
    ArtifactIdentMismatch((String, String, String)),
    /// Occurs when there is no valid toml of json in the environment variable
    BadEnvConfig(String),
    BadGlyphStyle(String),
    CantUploadGossipToml,
    ChannelNotFound,
    CryptoKeyError(String),
    DownloadFailed(String),
    EditorEnv(env::VarError),
    EditStatus,
    FileNameError,
    /// Occurs when a file that should exist does not or could not be read.
    FileNotFound(String),
    GossipFileRelativePath(String),
    HabitatCore(hcore::Error),
    InstallHookFailed(PackageIdent),
    InvalidEventStreamToken(String),
    InvalidInstallHookMode(String),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    /// Errors when joining paths :)
    JoinPathsError(env::JoinPathsError),
    MissingCLIInputError(String),
    NamedPipeTimeoutOnStart(String, String, io::Error),
    NetParseError(net::AddrParseError),
    OfflineArtifactNotFound(PackageIdent),
    OfflineOriginKeyNotFound(String),
    OfflinePackageNotFound(PackageIdent),
    PackageNotFound(String),
    /// Occurs upon errors related to file or directory permissions.
    PermissionFailed(String),
    /// When an error occurs serializing rendering context
    RenderContextSerialization(serde_json::Error),
    RootRequired,
    StatusFileCorrupt(PathBuf),
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    /// When an error occurs registering template file
    // Boxed due to clippy::large_enum_variant
    TemplateFileError(Box<handlebars::TemplateFileError>),
    /// When an error occurs rendering template
    /// The error is constructed with a handlebars::RenderError's format string instead
    /// of the handlebars::RenderError itself because the cause field of the
    /// handlebars::RenderError in the handlebars crate version we use implements send
    /// and not sync which can lead to upstream compile errors when dealing with the
    /// failure crate. We should change this to a RenderError after we update the
    /// handlebars crate. See https://github.com/sunng87/handlebars-rust/issues/194
    TemplateRenderError(String),
    /// When an error occurs merging toml
    TomlMergeError(String),
    /// When an error occurs parsing toml
    TomlParser(toml::de::Error),
    TomlSerializeError(toml::ser::Error),
    WireDecode(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::APIClient(ref err) => format!("{}", err),
            Error::ArtifactIdentMismatch((ref a, ref ai, ref i)) => {
                format!("Artifact ident {} for `{}' does not match expected ident {}",
                        ai, a, i)
            }
            Error::BadEnvConfig(ref varname) => {
                format!("Unable to find valid TOML or JSON in {} ENVVAR", varname)
            }
            Error::BadGlyphStyle(ref style) => format!("Unknown symbol style '{}'", style),
            Error::CantUploadGossipToml => {
                "Can't upload gossip.toml, it's a reserved file name".to_string()
            }
            Error::ChannelNotFound => "Channel not found".to_string(),
            Error::CryptoKeyError(ref s) => format!("Missing or invalid key: {}", s),
            Error::DownloadFailed(ref msg) => msg.to_string(),
            Error::EditorEnv(ref e) => format!("Missing EDITOR environment variable: {}", e),
            Error::EditStatus => "Failed edit text command".to_string(),
            Error::FileNameError => "Failed to extract a filename".to_string(),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::GossipFileRelativePath(ref s) => {
                format!("Path for gossip file cannot have relative components (eg: ..): {}",
                        s)
            }
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::MissingCLIInputError(ref arg) => {
                format!("Missing required CLI argument!: {}", arg)
            }
            Error::InstallHookFailed(ref ident) => {
                format!("Install hook exited unsuccessfully: {}", ident)
            }
            Error::InvalidEventStreamToken(ref s) => {
                format!("Invalid event stream token provided: '{}'", s)
            }
            Error::InvalidInstallHookMode(ref e) => {
                format!("Invalid InstallHookMode conversion from {}", e)
            }
            Error::IO(ref err) => format!("{}", err),
            Error::JoinPathsError(ref err) => format!("{}", err),
            Error::NamedPipeTimeoutOnStart(ref group, ref hook, ref err) => {
                format!("Unable to start powershell named pipe for {} hook of {}: {}",
                        hook, group, err)
            }
            Error::NetParseError(ref err) => format!("{}", err),
            Error::OfflineArtifactNotFound(ref ident) => {
                format!("Cached artifact not found in offline mode: {}", ident)
            }
            Error::OfflineOriginKeyNotFound(ref name_with_rev) => {
                format!("Cached origin key not found in offline mode: {}",
                        name_with_rev)
            }
            Error::OfflinePackageNotFound(ref ident) => {
                format!("No installed package or cached artifact could be found locally in \
                         offline mode: {}",
                        ident)
            }
            Error::PackageNotFound(ref e) => format!("Package not found. {}", e),
            Error::PermissionFailed(ref e) => e.to_string(),
            Error::RenderContextSerialization(ref e) => {
                format!("Unable to serialize rendering context, {}", e)
            }
            Error::RootRequired => {
                "Root or administrator permissions required to complete operation".to_string()
            }
            Error::StatusFileCorrupt(ref path) => {
                format!("Unable to decode contents of INSTALL_STATUS file, {}",
                        path.display())
            }
            Error::StrFromUtf8Error(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::TemplateFileError(ref err) => format!("{:?}", err),
            Error::TemplateRenderError(ref err) => err.to_string(),
            Error::TomlMergeError(ref e) => format!("Failed to merge TOML: {}", e),
            Error::TomlParser(ref err) => format!("Failed to parse TOML: {}", err),
            Error::TomlSerializeError(ref e) => format!("Can't serialize TOML: {}", e),
            Error::WireDecode(ref m) => format!("Failed to decode wire message: {}", m),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

impl From<api_client::Error> for Error {
    fn from(err: api_client::Error) -> Self { Error::APIClient(err) }
}

impl From<handlebars::TemplateFileError> for Error {
    fn from(err: handlebars::TemplateFileError) -> Self { Error::TemplateFileError(Box::new(err)) }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Self { Error::HabitatCore(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self { Error::IO(err) }
}

impl From<env::JoinPathsError> for Error {
    fn from(err: env::JoinPathsError) -> Self { Error::JoinPathsError(err) }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self { Error::StrFromUtf8Error(err) }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self { Error::StringFromUtf8Error(err) }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self { Error::TomlSerializeError(err) }
}

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Self { Error::NetParseError(err) }
}
