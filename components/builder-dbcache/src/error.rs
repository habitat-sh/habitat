// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::result;

use redis;

#[derive(Debug)]
pub enum Error {
    DataStore(redis::RedisError),
    EntityNotFound,
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::DataStore(ref e) => format!("DataStore error, {}", e),
            Error::EntityNotFound => format!("No value for key found"),
        };
        write!(f, "{}", msg)
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::DataStore(err)
    }
}
