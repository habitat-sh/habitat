use std::{error,
          fmt,
          io,
          result};

use habitat_core as hab_core;
use hyper;
use openssl::{self,
              ssl};
use serde_json;
use url;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HabitatCore(hab_core::Error),
    HyperError(hyper::error::Error),
    /// Occurs when an improper http or https proxy value is given.
    InvalidProxyValue(String),
    IO(io::Error),
    Json(serde_json::Error),
    SslError(ssl::Error),
    SslErrorStack(openssl::error::ErrorStack),
    /// When an error occurs attempting to parse a string into a URL.
    UrlParseError(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HyperError(ref err) => format!("{}", err),
            Error::IO(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::InvalidProxyValue(ref e) => format!("Invalid proxy value: {:?}", e),
            Error::SslError(ref e) => format!("{}", e),
            Error::SslErrorStack(ref e) => format!("{}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error { Error::HabitatCore(err) }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error { Error::HyperError(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::IO(err) }
}

impl From<ssl::Error> for Error {
    fn from(err: ssl::Error) -> Error { Error::SslError(err) }
}

impl From<openssl::error::ErrorStack> for Error {
    fn from(err: openssl::error::ErrorStack) -> Error { Error::SslErrorStack(err) }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self { Error::UrlParseError(err) }
}
