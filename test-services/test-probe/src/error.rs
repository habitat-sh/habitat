use serde_json;
use std::{error::Error as StdError,
          fmt,
          io,
          num,
          result};
use toml;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Parse(num::ParseIntError),
    Toml(toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Json(e) => e.fmt(f),
            Error::Parse(e) => e.fmt(f),
            Error::Toml(e) => e.fmt(f),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Io(err) }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error { Error::Json(err) }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error { Error::Parse(err) }
}
