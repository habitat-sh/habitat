// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use api_client;
use bldr_core;
use db;
use depot_client;
use hab_core;
use hab_net;
use protocol;
use postgres;
use protobuf;
use r2d2;
use zmq;

pub type SrvResult<T> = Result<T, SrvError>;

#[derive(Debug)]
pub enum SrvError {
    APIClient(api_client::Error),
    BadPort(String),
    BuilderCore(bldr_core::Error),
    ChannelCreate(depot_client::Error),
    ConnErr(hab_net::conn::ConnErr),
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
    NetError(hab_net::NetError),
    PackageInsert(postgres::error::Error),
    PackagePromote(depot_client::Error),
    PackageStats(postgres::error::Error),
    PackagesGet(postgres::error::Error),
    ProjectSetState(postgres::error::Error),
    Protobuf(protobuf::ProtobufError),
    Protocol(protocol::ProtocolError),
    UnknownGroup,
    UnknownGroupState,
    UnknownJobState(protocol::jobsrv::Error),
    UnknownPackage,
    UnknownProjectState,
}

impl fmt::Display for SrvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            SrvError::APIClient(ref e) => format!("{}", e),
            SrvError::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            SrvError::BuilderCore(ref e) => format!("{}", e),
            SrvError::ChannelCreate(ref e) => format!("{}", e),
            SrvError::ConnErr(ref e) => format!("{}", e),
            SrvError::Db(ref e) => format!("{}", e),
            SrvError::DbPoolTimeout(ref e) => {
                format!("Timeout getting connection from the database pool, {}", e)
            }
            SrvError::DbTransaction(ref e) => format!("Database transaction error, {}", e),
            SrvError::DbTransactionCommit(ref e) => {
                format!("Database transaction commit error, {}", e)
            }
            SrvError::DbTransactionStart(ref e) => {
                format!("Database transaction start error, {}", e)
            }
            SrvError::GroupCreate(ref e) => format!("Database error creating a new group, {}", e),
            SrvError::GroupGet(ref e) => format!("Database error getting group data, {}", e),
            SrvError::GroupPending(ref e) => format!("Database error getting pending group, {}", e),
            SrvError::GroupSetState(ref e) => format!("Database error setting group state, {}", e),
            SrvError::HabitatCore(ref e) => format!("{}", e),
            SrvError::IO(ref e) => format!("{}", e),
            SrvError::MessageDelete(ref e) => {
                format!(
                    "Database error deleting a message from the message queue, {}",
                    e
                )
            }
            SrvError::MessageGet(ref e) => {
                format!(
                    "Database error retrieving a message from the message queue, {}",
                    e
                )
            }
            SrvError::MessageInsert(ref e) => {
                format!(
                    "Database error inserting a message to the message queue, {}",
                    e
                )
            }
            SrvError::NetError(ref e) => format!("{}", e),
            SrvError::PackageInsert(ref e) => {
                format!("Database error inserting a new package, {}", e)
            }
            SrvError::PackagePromote(ref e) => format!("{}", e),
            SrvError::PackageStats(ref e) => {
                format!("Database error retrieving package statistics, {}", e)
            }
            SrvError::PackagesGet(ref e) => format!("Database error retrieving packages, {}", e),
            SrvError::ProjectSetState(ref e) => {
                format!("Database error setting project state, {}", e)
            }
            SrvError::Protobuf(ref e) => format!("{}", e),
            SrvError::Protocol(ref e) => format!("{}", e),
            SrvError::UnknownGroup => format!("Unknown Group"),
            SrvError::UnknownGroupState => format!("Unknown Group State"),
            SrvError::UnknownJobState(ref e) => format!("{}", e),
            SrvError::UnknownPackage => format!("Unknown Package"),
            SrvError::UnknownProjectState => format!("Unknown Project State"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for SrvError {
    fn description(&self) -> &str {
        match *self {
            SrvError::APIClient(ref err) => err.description(),
            SrvError::BadPort(_) => {
                "Received an invalid port or a number outside of the valid range."
            }
            SrvError::BuilderCore(ref err) => err.description(),
            SrvError::ChannelCreate(ref err) => err.description(),
            SrvError::ConnErr(ref err) => err.description(),
            SrvError::Db(ref err) => err.description(),
            SrvError::DbPoolTimeout(ref err) => err.description(),
            SrvError::DbTransaction(ref err) => err.description(),
            SrvError::DbTransactionCommit(ref err) => err.description(),
            SrvError::DbTransactionStart(ref err) => err.description(),
            SrvError::GroupCreate(ref err) => err.description(),
            SrvError::GroupGet(ref err) => err.description(),
            SrvError::GroupPending(ref err) => err.description(),
            SrvError::GroupSetState(ref err) => err.description(),
            SrvError::HabitatCore(ref err) => err.description(),
            SrvError::IO(ref err) => err.description(),
            SrvError::MessageDelete(ref err) => err.description(),
            SrvError::MessageGet(ref err) => err.description(),
            SrvError::MessageInsert(ref err) => err.description(),
            SrvError::NetError(ref err) => err.description(),
            SrvError::PackageInsert(ref err) => err.description(),
            SrvError::PackagePromote(ref err) => err.description(),
            SrvError::PackageStats(ref err) => err.description(),
            SrvError::PackagesGet(ref err) => err.description(),
            SrvError::ProjectSetState(ref err) => err.description(),
            SrvError::Protobuf(ref err) => err.description(),
            SrvError::Protocol(ref err) => err.description(),
            SrvError::UnknownGroup => "Unknown Group",
            SrvError::UnknownGroupState => "Unknown Group State",
            SrvError::UnknownJobState(ref err) => err.description(),
            SrvError::UnknownPackage => "Unknown Package",
            SrvError::UnknownProjectState => "Unknown Project State",
        }
    }
}

impl From<r2d2::GetTimeout> for SrvError {
    fn from(err: r2d2::GetTimeout) -> Self {
        SrvError::DbPoolTimeout(err)
    }
}

impl From<hab_core::Error> for SrvError {
    fn from(err: hab_core::Error) -> Self {
        SrvError::HabitatCore(err)
    }
}

impl From<db::error::Error> for SrvError {
    fn from(err: db::error::Error) -> Self {
        SrvError::Db(err)
    }
}

impl From<io::Error> for SrvError {
    fn from(err: io::Error) -> Self {
        SrvError::IO(err)
    }
}

impl From<hab_net::NetError> for SrvError {
    fn from(err: hab_net::NetError) -> Self {
        SrvError::NetError(err)
    }
}

impl From<hab_net::conn::ConnErr> for SrvError {
    fn from(err: hab_net::conn::ConnErr) -> Self {
        SrvError::ConnErr(err)
    }
}

impl From<protocol::ProtocolError> for SrvError {
    fn from(err: protocol::ProtocolError) -> Self {
        SrvError::Protocol(err)
    }
}

impl From<protobuf::ProtobufError> for SrvError {
    fn from(err: protobuf::ProtobufError) -> Self {
        SrvError::Protobuf(err)
    }
}

impl From<zmq::Error> for SrvError {
    fn from(err: zmq::Error) -> Self {
        SrvError::ConnErr(hab_net::conn::ConnErr::from(err))
    }
}
