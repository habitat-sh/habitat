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

use db;
use hab_core;
use hab_net;
use protobuf;
use protocol;
use postgres;
use r2d2;
use zmq;

pub type SrvResult<T> = Result<T, SrvError>;

#[derive(Debug)]
pub enum SrvError {
    BadPort(String),
    ConnErr(hab_net::conn::ConnErr),
    Db(db::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransactionStart(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    DbListen(postgres::error::Error),
    HabitatCore(hab_core::Error),
    NetError(hab_net::NetError),
    OriginCreate(postgres::error::Error),
    OriginChannelCreate(postgres::error::Error),
    OriginChannelGet(postgres::error::Error),
    OriginChannelList(postgres::error::Error),
    OriginChannelDelete(postgres::error::Error),
    OriginChannelPackageGet(postgres::error::Error),
    OriginChannelPackageLatestGet(postgres::error::Error),
    OriginChannelPackageList(postgres::error::Error),
    OriginCheckAccess(postgres::error::Error),
    OriginGet(postgres::error::Error),
    OriginMemberList(postgres::error::Error),
    OriginIntegrationCreate(postgres::error::Error),
    OriginIntegrationGetNames(postgres::error::Error),
    OriginIntegrationDelete(postgres::error::Error),
    OriginIntegrationRequest(postgres::error::Error),
    OriginInvitationAccept(postgres::error::Error),
    OriginInvitationCreate(postgres::error::Error),
    OriginInvitationGet(postgres::error::Error),
    OriginInvitationIgnore(postgres::error::Error),
    OriginInvitationRescind(postgres::error::Error),
    OriginInvitationListForOrigin(postgres::error::Error),
    OriginInvitationListForAccount(postgres::error::Error),
    OriginInvitationValidate(postgres::error::Error),
    OriginMemberDelete(postgres::error::Error),
    OriginPackageCreate(postgres::error::Error),
    OriginPackageGet(postgres::error::Error),
    OriginPackageLatestGet(postgres::error::Error),
    OriginPackageChannelList(postgres::error::Error),
    OriginPackagePlatformList(postgres::error::Error),
    OriginPackageList(postgres::error::Error),
    OriginPackageVersionList(postgres::error::Error),
    OriginPackageDemote(postgres::error::Error),
    OriginPackageGroupPromote(postgres::error::Error),
    OriginPackagePromote(postgres::error::Error),
    OriginPackageSearch(postgres::error::Error),
    OriginPackageUniqueList(postgres::error::Error),
    OriginPackageUpdate(postgres::error::Error),
    OriginProjectCreate(postgres::error::Error),
    OriginProjectDelete(postgres::error::Error),
    OriginProjectGet(postgres::error::Error),
    OriginProjectListGet(postgres::error::Error),
    OriginProjectUpdate(postgres::error::Error),
    OriginProjectIntegrationCreate(postgres::error::Error),
    OriginProjectIntegrationGet(postgres::error::Error),
    OriginProjectIntegrationRequest(postgres::error::Error),
    OriginSecretKeyCreate(postgres::error::Error),
    OriginSecretKeyGet(postgres::error::Error),
    OriginPublicKeyCreate(postgres::error::Error),
    OriginPublicKeyGet(postgres::error::Error),
    OriginPublicKeyLatestGet(postgres::error::Error),
    OriginPublicKeyListForOrigin(postgres::error::Error),
    OriginUpdate(postgres::error::Error),
    OriginAccountList(postgres::error::Error),
    OriginAccountInOrigin(postgres::error::Error),
    Protocol(protocol::ProtocolError),
    SyncInvitations(postgres::error::Error),
    SyncInvitationsUpdate(postgres::error::Error),
    Protobuf(protobuf::ProtobufError),
    UnknownOriginPackageVisibility(protocol::originsrv::Error),
}

impl fmt::Display for SrvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            SrvError::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            SrvError::ConnErr(ref e) => format!("{}", e),
            SrvError::Db(ref e) => format!("{}", e),
            SrvError::DbTransactionStart(ref e) => {
                format!("Failed to start database transaction, {}", e)
            }
            SrvError::DbTransactionCommit(ref e) => {
                format!("Failed to commit database transaction, {}", e)
            }
            SrvError::DbPoolTimeout(ref e) => {
                format!("Timeout getting connection from the database pool, {}", e)
            }
            SrvError::DbListen(ref e) => {
                format!("Error setting up async database event listener, {}", e)
            }
            SrvError::HabitatCore(ref e) => format!("{}", e),
            SrvError::NetError(ref e) => format!("{}", e),
            SrvError::OriginCreate(ref e) => format!("Error creating origin in database, {}", e),
            SrvError::OriginChannelCreate(ref e) => {
                format!("Error creating channel in database, {}", e)
            }
            SrvError::OriginChannelGet(ref e) => {
                format!("Error getting channel from database, {}", e)
            }
            SrvError::OriginChannelList(ref e) => {
                format!("Error listing channels for an origin from database, {}", e)
            }
            SrvError::OriginChannelDelete(ref e) => {
                format!("Error deleting channel in database, {}", e)
            }
            SrvError::OriginChannelPackageGet(ref e) => {
                format!("Error getting package for a channel from database, {}", e)
            }
            SrvError::OriginChannelPackageLatestGet(ref e) => {
                format!(
                    "Error getting the latest package for a channel from database, {}",
                    e
                )
            }
            SrvError::OriginChannelPackageList(ref e) => {
                format!("Error listing packages for a channel from database, {}", e)
            }
            SrvError::OriginCheckAccess(ref e) => {
                format!("Error checking access to origin in database, {}", e)
            }
            SrvError::OriginGet(ref e) => format!("Error getting origin from database, {}", e),
            SrvError::OriginMemberList(ref e) => {
                format!("Error getting origin members from database, {}", e)
            }
            SrvError::OriginIntegrationCreate(ref e) => {
                format!("Error creating integration in database, {}", e)
            }
            SrvError::OriginIntegrationGetNames(ref e) => {
                format!("Error getting integration names from database, {}", e)
            }
            SrvError::OriginIntegrationDelete(ref e) => {
                format!("Error deleting integration from database, {}", e)
            }
            SrvError::OriginIntegrationRequest(ref e) => {
                format!("Error retrieving integration request from database, {}", e)
            }
            SrvError::OriginInvitationAccept(ref e) => {
                format!("Error accepting origin invitation, {}", e)
            }
            SrvError::OriginInvitationCreate(ref e) => {
                format!("Error creating origin invitation, {}", e)
            }
            SrvError::OriginInvitationGet(ref e) => {
                format!("Error fetching origin invitation, {}", e)
            }
            SrvError::OriginInvitationIgnore(ref e) => {
                format!("Error ignoring origin invitation, {}", e)
            }
            SrvError::OriginInvitationRescind(ref e) => {
                format!("Error rescinding origin invitation, {}", e)
            }
            SrvError::OriginInvitationListForOrigin(ref e) => {
                format!(
                    "Error listing origin invitations for an origin in database, {}",
                    e
                )
            }
            SrvError::OriginInvitationListForAccount(ref e) => {
                format!(
                    "Error listing origin invitations for an account in database, {}",
                    e
                )
            }
            SrvError::OriginInvitationValidate(ref e) => {
                format!(
                    "Error validating origin invitation for an account in database, {}",
                    e
                )
            }
            SrvError::OriginPackageCreate(ref e) => {
                format!("Error creating package in database, {}", e)
            }
            SrvError::OriginMemberDelete(ref e) => {
                format!("Error deleting member of origin in database, {}", e)
            }
            SrvError::OriginPackageGet(ref e) => {
                format!("Error getting package in database, {}", e)
            }
            SrvError::OriginPackageLatestGet(ref e) => {
                format!("Error getting latest package in database, {}", e)
            }
            SrvError::OriginPackageChannelList(ref e) => {
                format!("Error getting list of channels for this package, {}", e)
            }
            SrvError::OriginPackagePlatformList(ref e) => {
                format!("Error getting list of platforms for this package, {}", e)
            }
            SrvError::OriginPackageList(ref e) => {
                format!("Error getting list of packages for this origin, {}", e)
            }
            SrvError::OriginPackageVersionList(ref e) => {
                format!(
                    "Error getting list of package versions for this origin, {}",
                    e
                )
            }
            SrvError::OriginPackageDemote(ref e) => {
                format!("Error demoting package from channel, {}", e)
            }
            SrvError::OriginPackageGroupPromote(ref e) => {
                format!("Error promoting package group to channel, {}", e)
            }
            SrvError::OriginPackagePromote(ref e) => {
                format!("Error promoting package to channel, {}", e)
            }
            SrvError::OriginPackageSearch(ref e) => {
                format!("Error searching list of packages for this origin, {}", e)
            }
            SrvError::OriginPackageUniqueList(ref e) => {
                format!(
                    "Error getting unique list of packages for this origin, {}",
                    e
                )
            }
            SrvError::OriginPackageUpdate(ref e) => {
                format!("Error updating a package in this origin, {}", e)
            }
            SrvError::OriginProjectCreate(ref e) => {
                format!("Error creating project in database, {}", e)
            }
            SrvError::OriginProjectDelete(ref e) => {
                format!("Error deleting project in database, {}", e)
            }
            SrvError::OriginProjectGet(ref e) => {
                format!("Error getting project from database, {}", e)
            }
            SrvError::OriginProjectListGet(ref e) => {
                format!("Error getting project list from database, {}", e)
            }
            SrvError::OriginProjectUpdate(ref e) => {
                format!("Error updating project in database, {}", e)
            }
            SrvError::OriginProjectIntegrationCreate(ref e) => {
                format!("Error creating project integration in database, {}", e)
            }
            SrvError::OriginProjectIntegrationGet(ref e) => {
                format!("Error getting project integration from database, {}", e)
            }
            SrvError::OriginProjectIntegrationRequest(ref e) => {
                format!(
                    "Error retrieving project integration request from database, {}",
                    e
                )
            }
            SrvError::OriginSecretKeyCreate(ref e) => {
                format!("Error creating origin secret key in database, {}", e)
            }
            SrvError::OriginSecretKeyGet(ref e) => {
                format!("Error getting origin secret key from database, {}", e)
            }
            SrvError::OriginPublicKeyCreate(ref e) => {
                format!("Error creating origin public key in database, {}", e)
            }
            SrvError::OriginPublicKeyGet(ref e) => {
                format!("Error getting origin public key from database, {}", e)
            }
            SrvError::OriginPublicKeyLatestGet(ref e) => {
                format!(
                    "Error getting latest origin public key from database, {}",
                    e
                )
            }
            SrvError::OriginPublicKeyListForOrigin(ref e) => {
                format!(
                    "Error listing origin public keys for an origin from database, {}",
                    e
                )
            }
            SrvError::OriginAccountList(ref e) => {
                format!("Error getting list of origins for this account, {}", e)
            }
            SrvError::OriginAccountInOrigin(ref e) => {
                format!("Error checking if this account is in an origin, {}", e)
            }
            SrvError::Protocol(ref e) => format!("{}", e),
            SrvError::SyncInvitations(ref e) => {
                format!("Error syncing invitations for account, {}", e)
            }
            SrvError::SyncInvitationsUpdate(ref e) => {
                format!("Error update invitation sync for account, {}", e)
            }
            SrvError::OriginUpdate(ref e) => format!("Error updating origin, {}", e),
            SrvError::Protobuf(ref e) => format!("{}", e),
            SrvError::UnknownOriginPackageVisibility(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for SrvError {
    fn description(&self) -> &str {
        match *self {
            SrvError::BadPort(_) => {
                "Received an invalid port or a number outside of the valid range."
            }
            SrvError::ConnErr(ref err) => err.description(),
            SrvError::Db(ref err) => err.description(),
            SrvError::DbTransactionStart(ref err) => err.description(),
            SrvError::DbTransactionCommit(ref err) => err.description(),
            SrvError::DbPoolTimeout(ref err) => err.description(),
            SrvError::DbListen(ref err) => err.description(),
            SrvError::HabitatCore(ref err) => err.description(),
            SrvError::NetError(ref err) => err.description(),
            SrvError::OriginCreate(ref err) => err.description(),
            SrvError::OriginChannelCreate(ref err) => err.description(),
            SrvError::OriginChannelGet(ref err) => err.description(),
            SrvError::OriginChannelList(ref err) => err.description(),
            SrvError::OriginChannelPackageGet(ref err) => err.description(),
            SrvError::OriginChannelPackageLatestGet(ref err) => err.description(),
            SrvError::OriginChannelPackageList(ref err) => err.description(),
            SrvError::OriginCheckAccess(ref err) => err.description(),
            SrvError::OriginChannelDelete(ref err) => err.description(),
            SrvError::OriginGet(ref err) => err.description(),
            SrvError::OriginMemberList(ref err) => err.description(),
            SrvError::OriginIntegrationCreate(ref err) => err.description(),
            SrvError::OriginIntegrationGetNames(ref err) => err.description(),
            SrvError::OriginIntegrationDelete(ref err) => err.description(),
            SrvError::OriginIntegrationRequest(ref err) => err.description(),
            SrvError::OriginInvitationAccept(ref err) => err.description(),
            SrvError::OriginInvitationCreate(ref err) => err.description(),
            SrvError::OriginInvitationGet(ref err) => err.description(),
            SrvError::OriginInvitationIgnore(ref err) => err.description(),
            SrvError::OriginInvitationRescind(ref err) => err.description(),
            SrvError::OriginInvitationListForOrigin(ref err) => err.description(),
            SrvError::OriginInvitationListForAccount(ref err) => err.description(),
            SrvError::OriginInvitationValidate(ref err) => err.description(),
            SrvError::OriginMemberDelete(ref err) => err.description(),
            SrvError::OriginPackageCreate(ref err) => err.description(),
            SrvError::OriginPackageGet(ref err) => err.description(),
            SrvError::OriginPackageLatestGet(ref err) => err.description(),
            SrvError::OriginPackageChannelList(ref err) => err.description(),
            SrvError::OriginPackagePlatformList(ref err) => err.description(),
            SrvError::OriginPackageList(ref err) => err.description(),
            SrvError::OriginPackageVersionList(ref err) => err.description(),
            SrvError::OriginPackageDemote(ref err) => err.description(),
            SrvError::OriginPackageGroupPromote(ref err) => err.description(),
            SrvError::OriginPackagePromote(ref err) => err.description(),
            SrvError::OriginPackageSearch(ref err) => err.description(),
            SrvError::OriginPackageUniqueList(ref err) => err.description(),
            SrvError::OriginPackageUpdate(ref err) => err.description(),
            SrvError::OriginProjectCreate(ref err) => err.description(),
            SrvError::OriginProjectDelete(ref err) => err.description(),
            SrvError::OriginProjectGet(ref err) => err.description(),
            SrvError::OriginProjectListGet(ref err) => err.description(),
            SrvError::OriginProjectUpdate(ref err) => err.description(),
            SrvError::OriginProjectIntegrationCreate(ref err) => err.description(),
            SrvError::OriginProjectIntegrationGet(ref err) => err.description(),
            SrvError::OriginProjectIntegrationRequest(ref err) => err.description(),
            SrvError::OriginSecretKeyCreate(ref err) => err.description(),
            SrvError::OriginSecretKeyGet(ref err) => err.description(),
            SrvError::OriginPublicKeyCreate(ref err) => err.description(),
            SrvError::OriginPublicKeyGet(ref err) => err.description(),
            SrvError::OriginPublicKeyLatestGet(ref err) => err.description(),
            SrvError::OriginPublicKeyListForOrigin(ref err) => err.description(),
            SrvError::OriginAccountList(ref err) => err.description(),
            SrvError::OriginAccountInOrigin(ref err) => err.description(),
            SrvError::OriginUpdate(ref err) => err.description(),
            SrvError::Protocol(ref err) => err.description(),
            SrvError::SyncInvitations(ref err) => err.description(),
            SrvError::SyncInvitationsUpdate(ref err) => err.description(),
            SrvError::Protobuf(ref err) => err.description(),
            SrvError::UnknownOriginPackageVisibility(ref err) => err.description(),
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

impl From<db::error::Error> for SrvError {
    fn from(err: db::error::Error) -> Self {
        SrvError::Db(err)
    }
}

impl From<protobuf::ProtobufError> for SrvError {
    fn from(err: protobuf::ProtobufError) -> Self {
        SrvError::Protobuf(err)
    }
}

impl From<protocol::ProtocolError> for SrvError {
    fn from(err: protocol::ProtocolError) -> Self {
        SrvError::Protocol(err)
    }
}

impl From<zmq::Error> for SrvError {
    fn from(err: zmq::Error) -> Self {
        SrvError::from(hab_net::conn::ConnErr::from(err))
    }
}
