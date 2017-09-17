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

use protocol;
use zmq;

use conn;
use error::NetError;

pub type AppResult<T, E> = Result<T, AppError<E>>;

#[derive(Debug)]
pub enum AppError<E>
where
    E: error::Error,
{
    /// Wrapper for network connection send and receive errors.
    Connection(conn::ConnErr),
    /// Occurs when the Application fails to initialize.
    Init(E),
    /// Occurs when no active RouteSrv can be selected to route a request originating from a
    /// dispatch worker to.
    NoRouter,
    /// Wrapper for protocol serialization and deserialization errors.
    Protocol(protocol::ProtocolError),
    /// RouteSrv asked application to terminate.
    Terminated(NetError),
}

impl<T> fmt::Display for AppError<T>
where
    T: error::Error,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::Connection(ref e) => write!(f, "{}", e),
            AppError::Init(ref e) => write!(f, "Application failed to initialize, {}", e),
            AppError::NoRouter => write!(f, "Failed to route request, no reachable RouteSrv"),
            AppError::Protocol(ref e) => write!(f, "{}", e),
            AppError::Terminated(ref e) => write!(f, "received termination request, {}", e),
        }
    }
}

impl<T> error::Error for AppError<T>
where
    T: error::Error,
{
    fn description(&self) -> &str {
        match *self {
            AppError::Connection(ref err) => err.description(),
            AppError::Init(_) => "Application failed to initialize.",
            AppError::NoRouter => "Failed to route request, no reachable RouteSrv.",
            AppError::Protocol(ref err) => err.description(),
            AppError::Terminated(_) => "RouteSrv asked application to terminate.",
        }
    }
}

impl<T> From<conn::ConnErr> for AppError<T>
where
    T: error::Error,
{
    fn from(err: conn::ConnErr) -> AppError<T> {
        AppError::Connection(err)
    }
}

impl<T> From<protocol::ProtocolError> for AppError<T>
where
    T: error::Error,
{
    fn from(err: protocol::ProtocolError) -> AppError<T> {
        AppError::Protocol(err)
    }
}

impl<T> From<zmq::Error> for AppError<T>
where
    T: error::Error,
{
    fn from(err: zmq::Error) -> AppError<T> {
        Self::from(conn::ConnErr::from(err))
    }
}
