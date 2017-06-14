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

use db;
use hab_core;
use hab_net;
use protocol;
use postgres;
use protobuf;
use r2d2;

#[derive(Debug)]
pub enum Error {
    BadPort(String),
    Db(db::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransaction(postgres::error::Error),
    HabitatCore(hab_core::Error),
    IO(io::Error),
    PackageInsert(postgres::error::Error),
    PackagesGet(postgres::error::Error),
    NetError(hab_net::Error),
    ProtoNetError(protocol::net::NetError),
    Protobuf(protobuf::ProtobufError),
    UnknownPackage,
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
            Error::DbTransaction(ref e) => format!("Database transaction error, {}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::PackageInsert(ref e) => format!("Database error inserting a new package, {}", e),
            Error::PackagesGet(ref e) => format!("Database error retrieving packages, {}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::ProtoNetError(ref e) => format!("{}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::UnknownPackage => format!("Unknown Package"),
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
            Error::DbTransaction(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::PackageInsert(ref err) => err.description(),
            Error::PackagesGet(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::ProtoNetError(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
            Error::UnknownPackage => "Unknown Package",
        }
    }
}

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Error {
        Error::DbPoolTimeout(err)
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Self {
        Error::NetError(err)
    }
}

impl From<protocol::net::NetError> for Error {
    fn from(err: protocol::net::NetError) -> Self {
        Error::ProtoNetError(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::Protobuf(err)
    }
}
