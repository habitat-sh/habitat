// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io;
use std::result;

use hyper;
use protobuf;
use rustc_serialize::json;
use zmq;

use oauth;

#[derive(Debug)]
pub enum Error {
    Auth(oauth::github::AuthErr),
    GitHubAPI(HashMap<String, String>),
    IO(io::Error),
    HyperError(hyper::error::Error),
    JsonDecode(json::DecoderError),
    MaxHops,
    HTTP(hyper::status::StatusCode),
    MissingScope(String),
    Protobuf(protobuf::ProtobufError),
    Sys,
    Zmq(zmq::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Auth(ref e) => format!("GitHub Authentication error, {}", e),
            Error::GitHubAPI(ref e) => format!("GitHub API error, {:?}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::HyperError(ref e) => format!("{}", e),
            Error::JsonDecode(ref e) => format!("JSON decoding error, {}", e),
            Error::MaxHops => format!("Received a message containing too many network hops"),
            Error::HTTP(ref e) => format!("{}", e),
            Error::MissingScope(ref e) => format!("Missing GitHub permission: {}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::Sys => format!("Internal system error"),
            Error::Zmq(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Auth(_) => "GitHub authorization error.",
            Error::GitHubAPI(_) => "GitHub API error.",
            Error::IO(ref err) => err.description(),
            Error::HyperError(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::JsonDecode(ref err) => err.description(),
            Error::MaxHops => "Received a message containing too many network hops",
            Error::MissingScope(_) => "Missing GitHub authorization scope.",
            Error::Protobuf(ref err) => err.description(),
            Error::Sys => "Internal system error",
            Error::Zmq(ref err) => err.description(),
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Self {
        Error::HyperError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Self {
        Error::JsonDecode(err)
    }
}

impl From<oauth::github::AuthErr> for Error {
    fn from(err: oauth::github::AuthErr) -> Self {
        Error::Auth(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::Protobuf(err)
    }
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Error {
        Error::Zmq(err)
    }
}
