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

use bldr_core;
use db;
use hab_core;
use hab_net;
use protocol;
use postgres;
use protobuf;
use r2d2;
use zmq;
use depot_client;
use api_client;

#[derive(Debug)]
pub enum Error {
    APIClient(api_client::Error),
    BadPort(String),
    BuilderCore(bldr_core::Error),
    ChannelCreate(depot_client::Error),
    Db(db::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransaction(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    DbTransactionStart(postgres::error::Error),
    GroupCreate(postgres::error::Error),
    GroupGet(postgres::error::Error),
    GroupPending(postgres::error::Error),
    GroupSetState(postgres::error::Error),
    HabitatCore(hab_core::Error),
    IO(io::Error),
    MessageDelete(postgres::error::Error),
    MessageGet(postgres::error::Error),
    MessageInsert(postgres::error::Error),
    NetError(hab_net::Error),
    PackageInsert(postgres::error::Error),
    PackagePromote(depot_client::Error),
    PackageStats(postgres::error::Error),
    PackagesGet(postgres::error::Error),
    ProjectSetState(postgres::error::Error),
    ProtoNetError(protocol::net::NetError),
    Protobuf(protobuf::ProtobufError),
    UnknownGroup,
    UnknownGroupState,
    UnknownJobState(protocol::jobsrv::Error),
    UnknownPackage,
    UnknownProjectState,
    Zmq(zmq::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::APIClient(ref e) => format!("{}", e),
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::BuilderCore(ref e) => format!("{}", e),
            Error::ChannelCreate(ref e) => format!("{}", e),
            Error::Db(ref e) => format!("{}", e),
            Error::DbPoolTimeout(ref e) => {
                format!("Timeout getting connection from the database pool, {}", e)
            }
            Error::DbTransaction(ref e) => format!("Database transaction error, {}", e),
            Error::DbTransactionCommit(ref e) => {
                format!("Database transaction commit error, {}", e)
            }
            Error::DbTransactionStart(ref e) => format!("Database transaction start error, {}", e),
            Error::GroupCreate(ref e) => format!("Database error creating a new group, {}", e),
            Error::GroupGet(ref e) => format!("Database error getting group data, {}", e),
            Error::GroupPending(ref e) => format!("Database error getting pending group, {}", e),
            Error::GroupSetState(ref e) => format!("Database error setting group state, {}", e),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::MessageDelete(ref e) => {
                format!(
                    "Database error deleting a message from the message queue, {}",
                    e
                )
            }
            Error::MessageGet(ref e) => {
                format!(
                    "Database error retrieving a message from the message queue, {}",
                    e
                )
            }
            Error::MessageInsert(ref e) => {
                format!(
                    "Database error inserting a message to the message queue, {}",
                    e
                )
            }
            Error::NetError(ref e) => format!("{}", e),
            Error::PackageInsert(ref e) => format!("Database error inserting a new package, {}", e),
            Error::PackagePromote(ref e) => format!("{}", e),
            Error::PackageStats(ref e) => {
                format!("Database error retrieving package statistics, {}", e)
            }
            Error::PackagesGet(ref e) => format!("Database error retrieving packages, {}", e),
            Error::ProjectSetState(ref e) => format!("Database error setting project state, {}", e),
            Error::ProtoNetError(ref e) => format!("{}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::UnknownGroup => format!("Unknown Group"),
            Error::UnknownGroupState => format!("Unknown Group State"),
            Error::UnknownJobState(ref e) => format!("{}", e),
            Error::UnknownPackage => format!("Unknown Package"),
            Error::UnknownProjectState => format!("Unknown Project State"),
            Error::Zmq(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::APIClient(ref err) => err.description(),
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::BuilderCore(ref err) => err.description(),
            Error::ChannelCreate(ref err) => err.description(),
            Error::Db(ref err) => err.description(),
            Error::DbPoolTimeout(ref err) => err.description(),
            Error::DbTransaction(ref err) => err.description(),
            Error::DbTransactionCommit(ref err) => err.description(),
            Error::DbTransactionStart(ref err) => err.description(),
            Error::GroupCreate(ref err) => err.description(),
            Error::GroupGet(ref err) => err.description(),
            Error::GroupPending(ref err) => err.description(),
            Error::GroupSetState(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::MessageDelete(ref err) => err.description(),
            Error::MessageGet(ref err) => err.description(),
            Error::MessageInsert(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::PackageInsert(ref err) => err.description(),
            Error::PackagePromote(ref err) => err.description(),
            Error::PackageStats(ref err) => err.description(),
            Error::PackagesGet(ref err) => err.description(),
            Error::ProjectSetState(ref err) => err.description(),
            Error::ProtoNetError(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
            Error::UnknownGroup => "Unknown Group",
            Error::UnknownGroupState => "Unknown Group State",
            Error::UnknownJobState(ref err) => err.description(),
            Error::UnknownPackage => "Unknown Package",
            Error::UnknownProjectState => "Unknown Project State",
            Error::Zmq(ref err) => err.description(),
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

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Error {
        Error::Zmq(err)
    }
}
