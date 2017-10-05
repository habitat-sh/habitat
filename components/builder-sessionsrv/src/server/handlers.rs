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

use base64;
use hab_net::app::prelude::*;
use hab_net::privilege::{self, FeatureFlags};

use protocol::{message, net};
use protocol::sessionsrv as proto;

use super::{ServerState, Session};
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

pub fn account_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountCreate>()?;
    match state.datastore.create_account(&msg) {
        Ok(account) => conn.route_reply(req, &account)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-create:0");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_find_or_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountFindOrCreate>()?;
    match state.datastore.account_find_or_create(&msg) {
        Ok(account) => conn.route_reply(req, &account)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-foc:0");
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
    let mut session = Session::default();
    let mut flags = FeatureFlags::default();
    if env::var_os("HAB_FUNC_TEST").is_some() {
        flags = FeatureFlags::all();
    } else {
        assign_permissions(msg.get_name(), &mut flags, state)
    }

    let mut account_req = proto::AccountFindOrCreate::default();
    account_req.set_name(msg.take_name());
    account_req.set_email(msg.take_email());

    match conn.route::<proto::AccountFindOrCreate, proto::Account>(&account_req) {
        Ok(mut account) => {
            let mut token = proto::SessionToken::new();
            token.set_account_id(account.get_id());
            token.set_extern_id(msg.get_extern_id());
            token.set_provider(msg.get_provider());
            token.set_token(msg.get_token().to_string().into_bytes());
            let bytes = message::encode(&token)?;
            let encoded_token = base64::encode(&bytes);

            session.set_id(account.get_id());
            session.set_email(account.take_email());
            session.set_name(account.take_name());
            session.set_token(encoded_token);
            session.set_flags(flags.bits());
            session.set_oauth_token(msg.take_token());
            {
                state.sessions.write().unwrap().replace(session.clone());
            }
            conn.route_reply(req, &*session)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:session-create:5");
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
    let encoded = message::encode(msg.get_token())?;
    let base64_token = base64::encode(&encoded);
    match state.sessions.read().unwrap().get(base64_token.as_str()) {
        Some(session) => {
            if session.expired() {
                let err = NetError::new(ErrCode::SESSION_EXPIRED, "ss:session-get:1");
                conn.route_reply(req, &*err)?;
            } else {
                conn.route_reply(req, &**session)?;
            }
        }
        None => {
            let err = NetError::new(ErrCode::SESSION_EXPIRED, "ss:session-get:0");
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

pub fn account_origin_invitation_ignore(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountOriginInvitationIgnoreRequest>()?;
    match state.datastore.ignore_origin_invitation(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-origin-invitation-ignore:1");
            error!("{}, {}", e, err);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn account_origin_invitation_rescind(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountOriginInvitationRescindRequest>()?;
    match state.datastore.rescind_origin_invitation(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(
                ErrCode::DATA_STORE,
                "ss:account-origin-invitation-rescind:1",
            );
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

pub fn account_origin_remove(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountOriginRemove>()?;
    match state.datastore.delete_origin(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-origin-remove:1");
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

fn assign_permissions(name: &str, flags: &mut FeatureFlags, state: &ServerState) {
    match state.github.app_installation_token(
        state.permissions.app_install_id,
    ) {
        Ok(token) => {
            match state.github.check_team_membership(
                &token,
                state.permissions.admin_team,
                name,
            ) {
                Ok(membership) => {
                    if membership.active() {
                        debug!("Granting feature flag={:?}", privilege::ADMIN);
                        flags.set(privilege::ADMIN, true);
                    }
                }
                Err(err) => warn!("Failed to check team membership, {}", err),
            }
            for team in state.permissions.early_access_teams.iter() {
                match state.github.check_team_membership(&token, *team, name) {
                    Ok(membership) => {
                        if membership.active() {
                            debug!("Granting feature flag={:?}", privilege::EARLY_ACCESS);
                            flags.set(privilege::EARLY_ACCESS, true);
                            break;
                        }
                    }
                    Err(err) => warn!("Failed to check team membership, {}", err),
                }
            }
            for team in state.permissions.build_worker_teams.iter() {
                match state.github.check_team_membership(&token, *team, name) {
                    Ok(membership) => {
                        if membership.active() {
                            debug!("Granting feature flag={:?}", privilege::BUILD_WORKER);
                            flags.set(privilege::BUILD_WORKER, true);
                            break;
                        }
                    }
                    Err(err) => warn!("Failed to check team membership, {}", err),
                }
            }
        }
        Err(err) => warn!("Failed to obtain installation token, {}", err),
    }
}
