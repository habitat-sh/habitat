// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::error;
use std::io;
use std::fmt;
use std::result;
use std::str;
use std::string;

use depot_client;
use hcore;
use rustc_serialize::json;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CryptoKeyError(String),
    GossipFileRelativePath(String),
    DepotClient(depot_client::Error),
    FileNameError,
    HabitatCore(hcore::Error),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    JsonDecode(json::DecoderError),
    JsonEncode(json::EncoderError),
    StrFromUtf8Error(str::Utf8Error),
    StringFromUtf8Error(string::FromUtf8Error),
    WireDecode(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::CryptoKeyError(ref s) => format!("Missing or invalid key: {}", s),
            Error::GossipFileRelativePath(ref s) => {
                format!("Path for gossip file cannot have relative components (eg: ..): {}",
                        s)
            }
            Error::DepotClient(ref err) => format!("{}", err),
            Error::FileNameError => format!("Failed to extract a filename"),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref err) => format!("{}", err),
            Error::JsonDecode(ref e) => format!("JSON decoding error: {}", e),
            Error::JsonEncode(ref e) => format!("JSON encoding error: {}", e),
            Error::StrFromUtf8Error(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
            Error::WireDecode(ref m) => format!("Failed to decode wire message: {}", m),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::CryptoKeyError(_) => "Missing or invalid key",
            Error::GossipFileRelativePath(_) => "Path for gossip file cannot have relative components (eg: ..)",
            Error::DepotClient(ref err) => err.description(),
            Error::FileNameError => "Failed to extract a filename from a path",
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::JsonDecode(_) => "JSON decoding error: {:?}",
            Error::JsonEncode(_) => "JSON encoding error",
            Error::StrFromUtf8Error(_) => "Failed to convert a string as UTF-8",
            Error::StringFromUtf8Error(_) => "Failed to convert a string as UTF-8",
            Error::WireDecode(_) => "Failed to decode wire message",
        }
    }
}

impl From<depot_client::Error> for Error {
    fn from(err: depot_client::Error) -> Self {
        Error::DepotClient(err)
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Self {
        Error::HabitatCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Self {
        Error::JsonDecode(err)
    }
}

impl From<json::EncoderError> for Error {
    fn from(err: json::EncoderError) -> Self {
        Error::JsonEncode(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::StrFromUtf8Error(err)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(err)
    }
}
