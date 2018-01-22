// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use hab_net::conn;
use protocol;
use zmq;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Connection(conn::ConnErr),
    Protocol(protocol::ProtocolError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Connection(ref e) => format!("{}", e),
            AppError::Protocol(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::Connection(ref err) => err.description(),
            AppError::Protocol(ref err) => err.description(),
        }
    }
}

impl From<conn::ConnErr> for AppError {
    fn from(err: conn::ConnErr) -> AppError {
        AppError::Connection(err)
    }
}

impl From<protocol::ProtocolError> for AppError {
    fn from(err: protocol::ProtocolError) -> AppError {
        AppError::Protocol(err)
    }
}

impl From<zmq::Error> for AppError {
    fn from(err: zmq::Error) -> AppError {
        Self::from(conn::ConnErr::from(err))
    }
}
