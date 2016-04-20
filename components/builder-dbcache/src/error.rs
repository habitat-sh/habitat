// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::error;
use std::fmt;
use std::result;

use r2d2;
use redis;

#[derive(Debug)]
pub enum Error {
    ConnectionTimeout(r2d2::GetTimeout),
    DataStore(redis::RedisError),
    EntityNotFound,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ConnectionTimeout(ref e) => format!("Connection timeout, {}", e),
            Error::DataStore(ref e) => format!("DataStore error, {}", e),
            Error::EntityNotFound => format!("No value for key found"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConnectionTimeout(ref e) => e.description(),
            Error::DataStore(_) => "Error querying DataStore",
            Error::EntityNotFound => "Entity not found in DataStore",
        }
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        match err.kind() {
            redis::ErrorKind::TypeError => Error::EntityNotFound,
            _ => Error::DataStore(err),
        }
    }
}

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Self {
        Error::ConnectionTimeout(err)
    }
}
