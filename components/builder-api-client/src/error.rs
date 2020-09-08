use crate::{hab_core::{self,
                       package::{PackageIdent,
                                 PackageTarget}},
            hab_http};
use std::{error,
          fmt,
          io,
          num,
          path::PathBuf,
          result};
use tokio::task::JoinError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum APIFailure {
    DownloadPackageFailed(usize, PackageIdent, PackageTarget, Box<Error>),
    DownloadLatestKeyFailed(usize, String, Box<Error>),
    DownloadKeyFailed(usize, String, Box<Error>),
    UploadKeyFailed(usize, String, Box<Error>),
}

impl std::fmt::Display for APIFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DownloadPackageFailed(retries, ident, target, error) => {
                write!(f,
                       "When applicable, we try once, then re-attempt {} times to
                                      download a package. Unfortunately, we failed to download
                                      {} for {}. Last error: {}",
                       retries, ident, target, error)
            }

            Self::DownloadLatestKeyFailed(retries, name, error) => {
                write!(f,
                       "When applicable, we try once, then re-attempt {} times to download an \
                        origin key. Unfortunately, we failed to download the latest {} key. Last \
                        error: {}",
                       retries, name, error)
            }
            Self::DownloadKeyFailed(retries, named_revision, error) => {
                write!(f,
                       "When applicable, we try once, then re-attempt {} times to download an \
                        origin key. Unfortunately, we failed to download the {} origin key. Last \
                        error: {}",
                       retries, named_revision, error)
            }

            Self::UploadKeyFailed(retries, named_revision, error) => {
                write!(f,
                       "When applicable, we try once, then re-attempt {} times to upload an \
                        origin key. Unfortunately, we failed to upload the {} key. Last error: {}",
                       retries, named_revision, error)
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    APIError(reqwest::StatusCode, String),
    APIClientError(APIFailure),
    BadResponseBody(reqwest::Error),
    DownloadWrite(PathBuf, io::Error),
    HabitatCore(hab_core::Error),
    HabitatHttpClient(hab_http::Error),
    ReqwestError(reqwest::Error),
    IO(io::Error),
    Json(serde_json::Error),
    KeyReadError(PathBuf, io::Error),
    MissingHeader(String),
    InvalidHeader(String),
    NoFilePart,
    PackageReadError(PathBuf, io::Error),
    ParseIntError(num::ParseIntError),
    IdentNotFullyQualified,
    UploadFailed(String),
    UrlParseError(url::ParseError),
    WriteSyncFailed,
    NotSupported,
    TokioJoinError(JoinError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::APIError(ref c, ref m) if !m.is_empty() => format!("[{}] {}", c, m),
            Error::APIError(ref c, _) => format!("[{}]", c),
            Error::APIClientError(ref e) => format!("{}", e),
            Error::BadResponseBody(ref e) => format!("Failed to read response body, {}", e),
            Error::DownloadWrite(ref p, ref e) => {
                format!("Failed to write contents of builder response, {}, {}",
                        p.display(),
                        e)
            }
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HabitatHttpClient(ref e) => format!("{}", e),
            Error::ReqwestError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::KeyReadError(ref p, ref e) => {
                format!("Failed to read origin key, {}, {}", p.display(), e)
            }
            Error::MissingHeader(ref s) => format!("Response is missing a required header: {}", s),
            Error::InvalidHeader(ref s) => format!("Response header is invalid: {}", s),
            Error::NoFilePart => "An invalid path was passed - we needed a filename, and this \
                                  path does not have one"
                                                         .to_string(),
            Error::PackageReadError(ref p, ref e) => {
                format!("Failed to read package artifact, {}, {}", p.display(), e)
            }
            Error::ParseIntError(ref err) => format!("{}", err),
            Error::IdentNotFullyQualified => {
                "Cannot perform the specified operation. Specify a fully qualifed package \
                 identifier (ex: core/busybox-static/1.42.2/20170513215502)"
                                                                            .to_string()
            }
            Error::UploadFailed(ref s) => format!("Upload failed: {}", s),
            Error::UrlParseError(ref e) => format!("{}", e),
            Error::WriteSyncFailed => {
                "Could not write to destination; perhaps the disk is full?".to_string()
            }
            Error::NotSupported => "The specified operation is not supported.".to_string(),
            Error::TokioJoinError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error { Error::HabitatCore(err) }
}

impl From<hab_http::Error> for Error {
    fn from(err: hab_http::Error) -> Error { Error::HabitatHttpClient(err) }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error { Error::ReqwestError(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::IO(err) }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error { Error::Json(err) }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error { Error::UrlParseError(err) }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Error { Error::TokioJoinError(err) }
}
