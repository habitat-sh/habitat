// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::error;
use std::fmt;
use std::io;
use std::result;

use core;
use dbcache;
use hnet;
use hyper;
use protobuf;
use rustc_serialize::json;
use zmq;

use oauth;

#[derive(Debug)]
pub enum Error {
    Auth(oauth::github::AuthErr),
    BadPort(String),
    DataStore(dbcache::Error),
    EntityNotFound,
    HabitatCore(core::Error),
    HTTP(hyper::status::StatusCode),
    HyperError(hyper::error::Error),
    IO(io::Error),
    NetError(hnet::Error),
    JsonDecode(json::DecoderError),
    MissingScope(String),
    Protobuf(protobuf::ProtobufError),
    Zmq(zmq::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Auth(ref e) => format!("GitHub Authentication error, {}", e),
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::DataStore(ref e) => format!("DataStore error, {}", e),
            Error::EntityNotFound => format!("No value for key found"),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::HyperError(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::JsonDecode(ref e) => format!("JSON decoding error, {}", e),
            Error::MissingScope(ref e) => format!("Missing GitHub permission: {}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::Zmq(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Auth(_) => "GitHub authorization error.",
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::DataStore(ref err) => err.description(),
            Error::EntityNotFound => "Entity not found in database.",
            Error::HabitatCore(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::HyperError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::JsonDecode(ref err) => err.description(),
            Error::MissingScope(_) => "Missing GitHub authorization scope.",
            Error::Protobuf(ref err) => err.description(),
            Error::Zmq(ref err) => err.description(),
        }
    }
}

impl From<core::Error> for Error {
    fn from(err: core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<dbcache::Error> for Error {
    fn from(err: dbcache::Error) -> Self {
        Error::DataStore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Self {
        Error::HyperError(err)
    }
}

impl From<json::DecoderError> for Error {
    fn from(err: json::DecoderError) -> Self {
        Error::JsonDecode(err)
    }
}

impl From<hnet::Error> for Error {
    fn from(err: hnet::Error) -> Self {
        Error::NetError(err)
    }
}

impl From<oauth::github::AuthErr> for Error {
    fn from(err: oauth::github::AuthErr) -> Self {
        Error::Auth(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Self {
        Error::Protobuf(err)
    }
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Self {
        Error::Zmq(err)
    }
}
