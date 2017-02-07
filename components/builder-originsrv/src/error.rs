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

use hab_core;
use hab_net;
use protobuf;
use postgres;
use r2d2;
use zmq;
use db;

#[derive(Debug)]
pub enum Error {
    BadPort(String),
    Db(db::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransactionStart(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    HabitatCore(hab_core::Error),
    IO(io::Error),
    NetError(hab_net::Error),
    OriginCreate(postgres::error::Error),
    OriginCheckAccess(postgres::error::Error),
    OriginGet(postgres::error::Error),
    OriginMemberList(postgres::error::Error),
    OriginInvitationAccept(postgres::error::Error),
    OriginInvitationCreate(postgres::error::Error),
    OriginInvitationListForOrigin(postgres::error::Error),
    OriginInvitationListForAccount(postgres::error::Error),
    OriginInvitationValidate(postgres::error::Error),
    OriginProjectCreate(postgres::error::Error),
    OriginProjectDelete(postgres::error::Error),
    OriginProjectGet(postgres::error::Error),
    OriginProjectUpdate(postgres::error::Error),
    OriginSecretKeyCreate(postgres::error::Error),
    OriginSecretKeyGet(postgres::error::Error),
    OriginAccountList(postgres::error::Error),
    OriginAccountInOrigin(postgres::error::Error),
    Protobuf(protobuf::ProtobufError),
    Zmq(zmq::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::Db(ref e) => format!("{}", e),
            Error::DbTransactionStart(ref e) => {
                format!("Failed to start database transaction, {}", e)
            }
            Error::DbTransactionCommit(ref e) => {
                format!("Failed to commit database transaction, {}", e)
            }
            Error::DbPoolTimeout(ref e) => {
                format!("Timeout getting connection from the database pool, {}", e)
            }
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::OriginCreate(ref e) => format!("Error creating origin in database, {}", e),
            Error::OriginCheckAccess(ref e) => {
                format!("Error checking access to origin in database, {}", e)
            }
            Error::OriginGet(ref e) => format!("Error getting origin from database, {}", e),
            Error::OriginMemberList(ref e) => {
                format!("Error getting origin members from database, {}", e)
            }
            Error::OriginInvitationAccept(ref e) => {
                format!("Error accepting origin invitation in database, {}", e)
            }
            Error::OriginInvitationCreate(ref e) => {
                format!("Error creating origin invitation in database, {}", e)
            }
            Error::OriginInvitationListForOrigin(ref e) => {
                format!("Error listing origin invitations for an origin in database, {}",
                        e)
            }
            Error::OriginInvitationListForAccount(ref e) => {
                format!("Error listing origin invitations for an account in database, {}",
                        e)
            }
            Error::OriginInvitationValidate(ref e) => {
                format!("Error validating origin invitation for an account in database, {}",
                        e)
            }

            Error::OriginProjectCreate(ref e) => {
                format!("Error creating project in database, {}", e)
            }
            Error::OriginProjectDelete(ref e) => {
                format!("Error deleting project in database, {}", e)
            }
            Error::OriginProjectGet(ref e) => format!("Error getting project from database, {}", e),
            Error::OriginProjectUpdate(ref e) => {
                format!("Error updating project in database, {}", e)
            }
            Error::OriginSecretKeyCreate(ref e) => {
                format!("Error creating origin secret key in database, {}", e)
            }
            Error::OriginSecretKeyGet(ref e) => {
                format!("Error getting origin secret key from database, {}", e)
            }
            Error::OriginAccountList(ref e) => {
                format!("Error getting list of origins for this account, {}", e)
            }
            Error::OriginAccountInOrigin(ref e) => {
                format!("Error checking if this account is in an origin, {}", e)
            }
            Error::Protobuf(ref e) => format!("{}", e),
            Error::Zmq(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::Db(ref err) => err.description(),
            Error::DbTransactionStart(ref err) => err.description(),
            Error::DbTransactionCommit(ref err) => err.description(),
            Error::DbPoolTimeout(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::NetError(ref err) => err.description(),
            Error::OriginCreate(ref err) => err.description(),
            Error::OriginCheckAccess(ref err) => err.description(),
            Error::OriginGet(ref err) => err.description(),
            Error::OriginMemberList(ref err) => err.description(),
            Error::OriginInvitationAccept(ref err) => err.description(),
            Error::OriginInvitationCreate(ref err) => err.description(),
            Error::OriginInvitationListForOrigin(ref err) => err.description(),
            Error::OriginInvitationListForAccount(ref err) => err.description(),
            Error::OriginInvitationValidate(ref err) => err.description(),
            Error::OriginProjectCreate(ref err) => err.description(),
            Error::OriginProjectDelete(ref err) => err.description(),
            Error::OriginProjectGet(ref err) => err.description(),
            Error::OriginProjectUpdate(ref err) => err.description(),
            Error::OriginSecretKeyCreate(ref err) => err.description(),
            Error::OriginSecretKeyGet(ref err) => err.description(),
            Error::OriginAccountList(ref err) => err.description(),
            Error::OriginAccountInOrigin(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
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

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Self {
        Error::NetError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
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
