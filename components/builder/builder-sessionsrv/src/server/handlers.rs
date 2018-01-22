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
use hab_net::privilege::FeatureFlags;

use protocol::net;
use protocol::sessionsrv as proto;

use super::{encode_token, ServerState, Session};
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

pub fn account_update(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::AccountUpdate>()?;
    match state.datastore.update_account(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "ss:account-update:0");
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
    debug!("session-create, {:?}", msg);
    let mut flags = FeatureFlags::default();
    if env::var_os("HAB_FUNC_TEST").is_some() ||
        msg.get_session_type() == proto::SessionType::Builder
    {
        flags = FeatureFlags::all();
    } else if msg.get_provider() == proto::OAuthProvider::GitHub {
        assign_permissions(msg.get_name(), &mut flags, state)
    }

    let account = if msg.get_session_type() == proto::SessionType::Builder {
        let mut account = proto::Account::new();
        account.set_id(0);
        account.set_email(msg.take_email());
        account.set_name(msg.take_name());
        account
    } else {
        let mut account_req = proto::AccountFindOrCreate::default();
        account_req.set_name(msg.take_name());
        account_req.set_email(msg.take_email());

        match conn.route::<proto::AccountFindOrCreate, proto::Account>(&account_req) {
            Ok(account) => account,
            Err(e) => {
                let err = NetError::new(ErrCode::DATA_STORE, "ss:session-create:5");
                error!("{}, {}", e, err);
                conn.route_reply(req, &*err)?;
                return Ok(());
            }
        }
    };

    let session = Session::build(msg, account, flags)?;
    {
        debug!("issuing session, {:?}", session);
        state.sessions.write().unwrap().insert(session.clone());
    }
    conn.route_reply(req, &*session)?;
    Ok(())
}

pub fn session_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::SessionGet>()?;
    let token = encode_token(msg.get_token())?;
    let expire_session = {
        match state.sessions.read().unwrap().get(token.as_str()) {
            Some(session) => {
                if session.expired() {
                    true
                } else {
                    conn.route_reply(req, &**session)?;
                    false
                }
            }
            None => {
                let err = NetError::new(ErrCode::SESSION_EXPIRED, "ss:session-get:0");
                conn.route_reply(req, &*err)?;
                false
            }
        }
    };
    // JW TODO: We should renew the session if it's within X time of expiring since the
    // user just confirmed they're still using this session.
    if expire_session {
        state.sessions.write().unwrap().remove(token.as_str());
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
                Ok(Some(membership)) => {
                    if membership.active() {
                        debug!("Granting feature flag={:?}", FeatureFlags::ADMIN);
                        flags.set(FeatureFlags::ADMIN, true);
                    }
                }
                Ok(None) => (),
                Err(err) => warn!("Failed to check team membership, {}", err),
            }
            for team in state.permissions.early_access_teams.iter() {
                match state.github.check_team_membership(&token, *team, name) {
                    Ok(Some(membership)) => {
                        if membership.active() {
                            debug!("Granting feature flag={:?}", FeatureFlags::EARLY_ACCESS);
                            flags.set(FeatureFlags::EARLY_ACCESS, true);
                            break;
                        }
                    }
                    Ok(None) => (),
                    Err(err) => warn!("Failed to check team membership, {}", err),
                }
            }
            for team in state.permissions.build_worker_teams.iter() {
                match state.github.check_team_membership(&token, *team, name) {
                    Ok(Some(membership)) => {
                        if membership.active() {
                            debug!("Granting feature flag={:?}", FeatureFlags::BUILD_WORKER);
                            flags.set(FeatureFlags::BUILD_WORKER, true);
                            break;
                        }
                    }
                    Ok(None) => (),
                    Err(err) => warn!("Failed to check team membership, {}", err),
                }
            }
        }
        Err(err) => warn!("Failed to obtain installation token, {}", err),
    }
}
