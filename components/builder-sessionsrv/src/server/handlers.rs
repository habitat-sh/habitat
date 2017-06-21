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

use std::env;

use hab_net::privilege;
use hab_net::server::Envelope;
use protocol::net::{self, ErrCode};
use protocol::sessionsrv as proto;
use zmq;

use super::ServerState;
use error::Result;

pub fn account_get_id(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::AccountGetId = try!(req.parse_msg());
    match state.datastore.get_account_by_id(&msg) {
        Ok(Some(account)) => req.reply_complete(sock, &account)?,
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "ss:account-get-id:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("{}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account-get-id:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn account_get(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::AccountGet = try!(req.parse_msg());
    match state.datastore.get_account(&msg) {
        Ok(Some(account)) => req.reply_complete(sock, &account)?,
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "ss:account-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("{}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn session_create(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::SessionCreate = try!(req.parse_msg());

    let mut is_admin = false;
    let mut is_early_access = false;
    let mut is_build_worker = false;

    if env::var_os("HAB_FUNC_TEST").is_some() {
        is_admin = true;
        is_early_access = true;
        is_build_worker = true;
    } else {
        let teams = match state.github.teams(msg.get_token()) {
            Ok(teams) => teams,
            Err(e) => {
                error!("Cannot retrieve teams from github; failing: {}", e);
                let err = net::err(ErrCode::DATA_STORE, "ss:session-create:0");
                req.reply_complete(sock, &err)?;
                return Ok(());
            }
        };
        for team in teams {
            if team.id != 0 && team.id == state.permissions.admin_team {
                debug!(
                    "Granting feature flag={:?} for team={:?}",
                    privilege::ADMIN,
                    team.name
                );
                is_admin = true;
            }
            if team.id != 0 && state.permissions.early_access_teams.contains(&team.id) {
                debug!(
                    "Granting feature flag={:?} for team={:?}",
                    privilege::EARLY_ACCESS,
                    team.name
                );
                is_early_access = true;
            }
            if team.id != 0 && state.permissions.build_worker_teams.contains(&team.id) {
                debug!(
                    "Granting feature flag={:?} for team={:?}",
                    privilege::BUILD_WORKER,
                    team.name
                );
                is_build_worker = true;
            }
        }
    }
    match state.datastore.find_or_create_account_via_session(
        &msg,
        is_admin,
        is_early_access,
        is_build_worker,
    ) {
        Ok(session) => req.reply_complete(sock, &session)?,
        Err(e) => {
            error!("{}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:session-create:1");
            req.reply_complete(sock, &err)?;
        }
    }
    Ok(())
}

pub fn session_get(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::SessionGet = try!(req.parse_msg());
    match state.datastore.get_session(&msg) {
        Ok(Some(session)) => {
            try!(req.reply_complete(sock, &session));
        }
        Ok(None) => {
            let err = net::err(ErrCode::SESSION_EXPIRED, "ss:auth:4");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("{}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:auth:5");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn account_origin_invitation_create(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::AccountOriginInvitationCreate = try!(req.parse_msg());
    match state.datastore.create_account_origin_invitation(&msg) {
        Ok(()) => {
            try!(req.reply_complete(sock, &net::NetOk::new()));
        }
        Err(e) => {
            error!("Error creating invitation, {}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account_origin_invitation_create:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn account_origin_invitation_accept(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::AccountOriginInvitationAcceptRequest = try!(req.parse_msg());
    match state.datastore.accept_origin_invitation(&msg) {
        Ok(()) => {
            try!(req.reply_complete(sock, &net::NetOk::new()));
        }
        Err(e) => {
            error!("Error accepting invitation, {}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account_origin_invitation_accept:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn account_origin_create(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::AccountOriginCreate = try!(req.parse_msg());
    match state.datastore.create_origin(&msg) {
        Ok(()) => {
            try!(req.reply_complete(sock, &net::NetOk::new()));
        }
        Err(e) => {
            error!("Error adding origin for account, {}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account_origin_create:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn account_origin_list_request(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::AccountOriginListRequest = try!(req.parse_msg());
    match state.datastore.get_origins_by_account(&msg) {
        Ok(reply) => {
            try!(req.reply_complete(sock, &reply));
        }
        Err(e) => {
            error!("Error listing origins for account, {}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account_origin_list_request:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn account_invitation_list(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::AccountInvitationListRequest = try!(req.parse_msg());
    match state.datastore.list_invitations(&msg) {
        Ok(response) => {
            try!(req.reply_complete(sock, &response));
        }
        Err(e) => {
            error!("Failed to list account invitations, {}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account_invitation_list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}
