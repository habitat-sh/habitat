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

use std::env;

use hab_net::app::prelude::*;
use hab_net::privilege;

use protocol::net;
use protocol::sessionsrv as proto;

use super::ServerState;
use error::SrvResult;

pub fn account_get_id(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountGetId>()?;
    match state.datastore.get_account_by_id(&msg) {
        Ok(Some(account)) => conn.route_reply(req, &account)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "ss:account-get-id:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-get-id:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountGet>()?;
    match state.datastore.get_account(&msg) {
        Ok(Some(account)) => conn.route_reply(req, &account)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "ss:account-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-get:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn session_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let mut msg = req.parse::<proto::SessionCreate>()?;
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
            Err(_) => {
                let err = NetError::new(ErrCode::ACCESS_DENIED, "ss:session-create:0");
                conn.route_reply(req, &*err)?;
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

    // If only a token was filled in, let's grab the rest of the data from GH. We check email in
    // this case because although email is an optional field in the protobuf message, email is
    // required for access to builder.
    if msg.get_email().is_empty() {
        match state.github.user(msg.get_token()) {
            Ok(user) => {
                // Select primary email. If no primary email can be found, use any email. If
                // no email is associated with account return an access denied error.
                let email = match state.github.emails(msg.get_token()) {
                    Ok(ref emails) => {
                        emails
                            .iter()
                            .find(|e| e.primary)
                            .unwrap_or(&emails[0])
                            .email
                            .clone()
                    }
                    Err(_) => {
                        let err = NetError::new(ErrCode::ACCESS_DENIED, "ss:session-create:2");
                        conn.route_reply(req, &*err)?;
                        return Ok(());
                    }
                };

                msg.set_extern_id(user.id);
                msg.set_email(email);
                msg.set_name(user.login);
                msg.set_provider(proto::OAuthProvider::GitHub);
            }
            Err(_) => {
                let err = NetError::new(ErrCode::ACCESS_DENIED, "ss:session-create:3");
                conn.route_reply(req, &*err)?;
                return Ok(());
            }
        }
    }

    match state.datastore.find_or_create_account_via_session(
        &msg,
        is_admin,
        is_early_access,
        is_build_worker,
    ) {
        Ok(session) => conn.route_reply(req, &session)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:session-create:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn session_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::SessionGet>()?;
    match state.datastore.get_session(&msg) {
        Ok(Some(session)) => conn.route_reply(req, &session)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::SESSION_EXPIRED, "ss:auth:4");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:auth:5");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_origin_invitation_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountOriginInvitationCreate>()?;
    match state.datastore.create_account_origin_invitation(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-origin-invitation-create:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_origin_invitation_accept(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountOriginInvitationAcceptRequest>()?;
    match state.datastore.accept_origin_invitation(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-origin-invitation-accept:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_origin_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountOriginCreate>()?;
    match state.datastore.create_origin(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-origin-create:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_origin_list_request(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountOriginListRequest>()?;
    match state.datastore.get_origins_by_account(&msg) {
        Ok(reply) => conn.route_reply(req, &reply)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-origin-list-request:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_invitation_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountInvitationListRequest>()?;
    match state.datastore.list_invitations(&msg) {
        Ok(response) => conn.route_reply(req, &response)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-invitation-list:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}
