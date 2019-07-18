use std::{error,
          fmt,
          io,
          num,
          path::PathBuf,
          result};

use reqwest;
use serde_json;
use url;

use crate::{hab_core,
            hab_http};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    APIError(reqwest::StatusCode, String),
    BadResponseBody(io::Error),
    DownloadWrite(PathBuf, io::Error),
    HabitatCore(hab_core::Error),
    HabitatHttpClient(hab_http::Error),
    ReqwestError(reqwest::Error),
    IO(io::Error),
    Json(serde_json::Error),
    KeyReadError(PathBuf, io::Error),
    NoFilePart,
    PackageReadError(PathBuf, io::Error),
    ParseIntError(num::ParseIntError),
    IdentNotFullyQualified,
    UploadFailed(String),
    UrlParseError(url::ParseError),
    WriteSyncFailed,
    NotSupported,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::APIError(ref c, ref m) if !m.is_empty() => format!("[{}] {}", c, m),
            Error::APIError(ref c, _) => format!("[{}]", c),
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
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::APIError(..) => "Received a non-2XX response code from API",
            Error::BadResponseBody(_) => "Failed to read response body",
            Error::DownloadWrite(..) => "Failed to write response contents to file",
            Error::HabitatCore(ref err) => err.description(),
            Error::HabitatHttpClient(ref err) => err.description(),
            Error::ReqwestError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::KeyReadError(..) => "Failed to read origin key from disk",
            Error::NoFilePart => {
                "An invalid path was passed - we needed a filename, and this path does not have one"
            }
            Error::PackageReadError(..) => "Failed to read package artifact from disk",
            Error::ParseIntError(ref err) => err.description(),
            Error::IdentNotFullyQualified => {
                "Cannot perform the specified operation. Specify a fully qualifed package \
                 identifier (ex: core/busybox-static/1.42.2/20170513215502)"
            }
            Error::UploadFailed(_) => "Upload failed",
            Error::UrlParseError(ref err) => err.description(),
            Error::WriteSyncFailed => {
                "Could not write to destination; bytes written was 0 on a non-0 buffer"
            }
            Error::NotSupported => "The specified operation is not supported.",
        }
    }
}

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
