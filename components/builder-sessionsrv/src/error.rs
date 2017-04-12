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
use std::io;
use std::result;
use std::num;

use hab_core;
use hyper;
use hab_net;
use postgres;
use protobuf;
use zmq;
use db;
use r2d2;

#[derive(Debug)]
pub enum Error {
    BadPort(String),
    Db(db::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransactionStart(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    EntityNotFound,
    HabitatCore(hab_core::Error),
    HTTP(hyper::status::StatusCode),
    HyperError(hyper::error::Error),
    IO(io::Error),
    NetError(hab_net::Error),
    Protobuf(protobuf::ProtobufError),
    Zmq(zmq::Error),
    AccountIdFromString(num::ParseIntError),
    AccountCreate(postgres::error::Error),
    AccountGet(postgres::error::Error),
    AccountGetById(postgres::error::Error),
    SessionGet(postgres::error::Error),
    AccountOriginInvitationCreate(postgres::error::Error),
    AccountOriginInvitationList(postgres::error::Error),
    AccountOriginInvitationAccept(postgres::error::Error),
    OriginAccountList(postgres::error::Error),
    OriginCreate(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::Db(ref e) => format!("{}", e),
            Error::DbPoolTimeout(ref e) => {
                format!("Timeout getting connection from the database pool, {}", e)
            }
            Error::DbTransactionStart(ref e) => {
                format!("Failed to start database transaction, {}", e)
            }
            Error::DbTransactionCommit(ref e) => {
                format!("Failed to commit database transaction, {}", e)
            }
            Error::EntityNotFound => format!("No value for key found"),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::HTTP(ref e) => format!("{}", e),
            Error::HyperError(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::Zmq(ref e) => format!("{}", e),
            Error::AccountIdFromString(ref e) => {
                format!("Cannot convert from string to Account ID, {}", e)
            }
            Error::AccountCreate(ref e) => format!("Error creating account in database, {}", e),
            Error::AccountGet(ref e) => format!("Error getting account from database, {}", e),
            Error::AccountGetById(ref e) => format!("Error getting account from database, {}", e),
            Error::SessionGet(ref e) => format!("Error getting session from database, {}", e),
            Error::AccountOriginInvitationCreate(ref e) => {
                format!("Error creating invitation in database, {}", e)
            }
            Error::AccountOriginInvitationList(ref e) => {
                format!("Error listing invitation in database, {}", e)
            }
            Error::AccountOriginInvitationAccept(ref e) => {
                format!("Error accepting invitation in database, {}", e)
            }
            Error::OriginAccountList(ref e) => {
                format!("Error listing origins for account in database, {}", e)
            }
            Error::OriginCreate(ref e) => {
                format!("Error creating origin for account in database, {}", e)
            }

        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::Db(ref err) => err.description(),
            Error::DbPoolTimeout(ref err) => err.description(),
            Error::DbTransactionStart(ref err) => err.description(),
            Error::DbTransactionCommit(ref err) => err.description(),
            Error::EntityNotFound => "Entity not found in database.",
            Error::HabitatCore(ref err) => err.description(),
            Error::HTTP(_) => "Non-200 HTTP response.",
            Error::HyperError(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
            Error::Zmq(ref err) => err.description(),
            Error::AccountIdFromString(ref err) => err.description(),
            Error::AccountCreate(ref err) => err.description(),
            Error::AccountGet(ref err) => err.description(),
            Error::AccountGetById(ref err) => err.description(),
            Error::SessionGet(ref err) => err.description(),
            Error::AccountOriginInvitationCreate(ref err) => err.description(),
            Error::AccountOriginInvitationList(ref err) => err.description(),
            Error::AccountOriginInvitationAccept(ref err) => err.description(),
            Error::OriginAccountList(ref err) => err.description(),
            Error::OriginCreate(ref err) => err.description(),
        }
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
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

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Self {
        Error::NetError(err)
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

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Error {
        Error::DbPoolTimeout(err)
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::AccountIdFromString(err)
    }
}
