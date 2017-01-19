// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
