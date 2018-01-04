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

use hab_net::app::prelude::*;
use postgres;
use protocol::net;
use protocol::originsrv as proto;

use super::ServerState;
use error::{SrvError, SrvResult};

pub fn origin_check_access(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::CheckOriginAccessRequest>()?;
    let is_member = state.datastore.check_account_in_origin(&msg)?;
    let mut reply = proto::CheckOriginAccessResponse::new();
    reply.set_has_access(is_member);
    conn.route_reply(req, &reply)?;
    Ok(())
}

pub fn origin_check_owner(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::CheckOriginOwnerRequest>()?;
    let mut og = proto::OriginGet::new();
    og.set_name(msg.get_origin_name().to_string());
    match state.datastore.get_origin(&og) {
        Ok(Some(ref origin)) => {
            let mut reply = proto::CheckOriginOwnerResponse::new();
            reply.set_is_owner(origin.get_owner_id() == msg.get_account_id());
            conn.route_reply(req, &reply)?;
        }
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-check-owner:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-check-owner:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn my_origins(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::MyOriginsRequest>()?;
    match state.datastore.my_origins(&msg) {
        Ok(ref mor) => conn.route_reply(req, mor)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:my-origins:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_update(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageUpdate>()?;
    match state.datastore.update_origin_package(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-update:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let mut msg = req.parse::<proto::OriginCreate>()?;
    match state.datastore.create_origin(&mut msg) {
        Ok(Some(ref origin)) => conn.route_reply(req, origin)?,
        Ok(None) => {
            // this match branch is likely unnecessary because of the way a unique constraint
            // violation will be handled. see the matching comment in data_store.rs for the
            // create_origin function.
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-create:0");
            conn.route_reply(req, &*err)?;
        }
        Err(SrvError::OriginCreate(ref db))
            if db.code().is_some() && *db.code().unwrap() == postgres::error::UNIQUE_VIOLATION => {
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_update(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginUpdate>()?;
    match state.datastore.update_origin(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-update:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginGet>()?;
    match state.datastore.get_origin(&msg) {
        Ok(Some(ref origin)) => conn.route_reply(req, origin)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_integration_get_names(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginIntegrationGetNames>()?;
    match state.datastore.get_origin_integration_names(&msg) {
        Ok(Some(ref names)) => conn.route_reply(req, names)?,
        Ok(None) => {
            let err = NetError::new(
                ErrCode::ENTITY_NOT_FOUND,
                "vt:origin-integration-get-names:0",
            );
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-integration-get-names:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_integration_request(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginIntegrationRequest>()?;
    match state.datastore.origin_integration_request(&msg) {
        Ok(ref oir) => conn.route_reply(req, oir)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-integration-request:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_integration_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginIntegrationCreate>()?;
    match state.datastore.create_origin_integration(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(SrvError::OriginIntegrationCreate(ref db))
            if db.code().is_some() && *db.code().unwrap() == postgres::error::UNIQUE_VIOLATION => {
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-integration-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-integration-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_integration_delete(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginIntegrationDelete>()?;
    match state.datastore.delete_origin_integration(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-integration-delete:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_invitation_accept(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginInvitationAcceptRequest>()?;
    match state.datastore.accept_origin_invitation(conn, &msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-invitation-accept:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_invitation_ignore(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginInvitationIgnoreRequest>()?;
    match state.datastore.ignore_origin_invitation(conn, &msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-invitation-ignore:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_invitation_rescind(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginInvitationRescindRequest>()?;
    match state.datastore.rescind_origin_invitation(conn, &msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-invitation-rescind:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_invitation_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginInvitationCreate>()?;
    match state.datastore.create_origin_invitation(&msg) {
        Ok(Some(ref invite)) => conn.route_reply(req, invite)?,
        Ok(None) => {
            debug!(
                "User {} is already a member of the origin {}",
                &msg.get_origin_name(),
                &msg.get_account_name()
            );
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-invitation-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-invitation-create:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_invitation_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginInvitationListRequest>()?;
    match state.datastore.list_origin_invitations_for_origin(&msg) {
        Ok(ref oilr) => conn.route_reply(req, oilr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-invitation-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_member_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginMemberListRequest>()?;
    match state.datastore.list_origin_members(&msg) {
        Ok(ref omlr) => conn.route_reply(req, omlr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-member-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_secret_key_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginSecretKeyCreate>()?;
    match state.datastore.create_origin_secret_key(&msg) {
        Ok(ref osk) => conn.route_reply(req, osk)?,
        Err(SrvError::OriginSecretKeyCreate(ref db))
            if db.code().is_some() && *db.code().unwrap() == postgres::error::UNIQUE_VIOLATION => {
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-secret-key-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-secret-key-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_secret_key_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginSecretKeyGet>()?;
    match state.datastore.get_origin_secret_key(&msg) {
        Ok(Some(ref key)) => conn.route_reply(req, key)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-secret-key-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-secret-key-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_public_key_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPublicKeyCreate>()?;
    match state.datastore.create_origin_public_key(&msg) {
        Ok(ref osk) => conn.route_reply(req, osk)?,
        Err(SrvError::OriginPublicKeyCreate(ref db))
            if db.code().is_some() && *db.code().unwrap() == postgres::error::UNIQUE_VIOLATION => {
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-public-key-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-public-key-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_public_key_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPublicKeyGet>()?;
    match state.datastore.get_origin_public_key(&msg) {
        Ok(Some(ref key)) => conn.route_reply(req, key)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-public-key-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-public-key-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_public_key_latest_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPublicKeyLatestGet>()?;
    match state.datastore.get_origin_public_key_latest(&msg) {
        Ok(Some(ref key)) => conn.route_reply(req, key)?,
        Ok(None) => {
            let err = NetError::new(
                ErrCode::ENTITY_NOT_FOUND,
                "vt:origin-public-key-latest-get:0",
            );
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-public-key-latest-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_public_key_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPublicKeyListRequest>()?;
    match state.datastore.list_origin_public_keys_for_origin(&msg) {
        Ok(ref opklr) => conn.route_reply(req, opklr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-public-key-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let opc = req.parse::<proto::OriginProjectCreate>()?;
    match state.datastore.create_origin_project(&opc) {
        Ok(ref project) => conn.route_reply(req, project)?,
        Err(SrvError::OriginProjectCreate(ref db))
            if db.code().is_some() && *db.code().unwrap() == postgres::error::UNIQUE_VIOLATION => {
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-project-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-project-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_delete(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectDelete>()?;
    match state.datastore.delete_origin_project_by_name(
        msg.get_name(),
    ) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-project-delete:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectGet>()?;
    match state.datastore.get_origin_project_by_name(&msg.get_name()) {
        Ok(Some(ref project)) => conn.route_reply(req, project)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-project-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-project-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_update(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectUpdate>()?;
    match state.datastore.update_origin_project(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-project-update:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_list_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectListGet>()?;
    match state.datastore.get_origin_project_list(&msg) {
        Ok(ref projects) => conn.route_reply(req, projects)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-project-list-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_integration_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectIntegrationCreate>()?;
    match state.datastore.create_project_integration(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(SrvError::OriginProjectIntegrationCreate(ref db))
            if db.code().is_some() && *db.code().unwrap() == postgres::error::UNIQUE_VIOLATION => {
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:project-integration-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:project-integration-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_integration_delete(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectIntegrationDelete>()?;

    match state.datastore.delete_project_integration(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:project-integration-delete:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_integration_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectIntegrationGet>()?;

    match state.datastore.get_project_integration(&msg) {
        Ok(Some(ref integration)) => conn.route_reply(req, integration)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:project-integration-get:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:project-integration-get:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_project_integration_request(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginProjectIntegrationRequest>()?;
    match state.datastore.origin_project_integration_request(&msg) {
        Ok(ref opir) => conn.route_reply(req, opir)?,
        Err(e) => {
            let err = NetError::new(
                ErrCode::DATA_STORE,
                "vt:origin-project-integration-request:1",
            );
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_channel_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginChannelCreate>()?;
    match state.datastore.create_origin_channel(&msg) {
        Ok(ref occ) => conn.route_reply(req, occ)?,
        Err(SrvError::OriginChannelCreate(ref db))
            if db.code().is_some() && *db.code().unwrap() == postgres::error::UNIQUE_VIOLATION => {
            let err = NetError::new(ErrCode::ENTITY_CONFLICT, "vt:origin-channel-create:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-channel-create:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_channel_delete(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginChannelDelete>()?;
    match state.datastore.delete_origin_channel_by_id(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-channel-delete:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_channel_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginChannelGet>()?;
    match state.datastore.get_origin_channel(&msg) {
        Ok(Some(ref channel)) => conn.route_reply(req, channel)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-channel-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-channel-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_channel_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginChannelListRequest>()?;
    match state.datastore.list_origin_channels(&msg) {
        Ok(ref oclr) => conn.route_reply(req, oclr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-channel-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageCreate>()?;
    match state.datastore.create_origin_package(&msg) {
        Ok(ref opc) => conn.route_reply(req, opc)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-create:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageGet>()?;
    match state.datastore.get_origin_package(&msg) {
        Ok(Some(ref package)) => conn.route_reply(req, package)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-package-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_channel_package_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginChannelPackageGet>()?;
    match state.datastore.get_origin_channel_package(&msg) {
        Ok(Some(ref package)) => conn.route_reply(req, package)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-channel-package-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-channel-package-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_latest_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageLatestGet>()?;
    match state.datastore.get_origin_package_latest(&msg) {
        Ok(Some(ref package)) => conn.route_reply(req, package)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "vt:origin-package-latest-get:0");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-latest-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_channel_package_latest_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginChannelPackageLatestGet>()?;
    match state.datastore.get_origin_channel_package_latest(&msg) {
        Ok(Some(ref package)) => conn.route_reply(req, package)?,
        Ok(None) => {
            let err = NetError::new(
                ErrCode::ENTITY_NOT_FOUND,
                "vt:origin-channel-package-latest-get:0",
            );
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(
                ErrCode::DATA_STORE,
                "vt:origin-channel-package-latest-get:1",
            );
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_version_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageVersionListRequest>()?;
    match state.datastore.list_origin_package_versions_for_origin(
        &msg,
    ) {
        Ok(ref opvlr) => conn.route_reply(req, opvlr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-version-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_platform_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackagePlatformListRequest>()?;
    match state.datastore.list_origin_package_platforms_for_package(
        &msg,
    ) {
        Ok(ref opplr) => conn.route_reply(req, opplr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-platform-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_channel_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageChannelListRequest>()?;
    match state.datastore.list_origin_package_channels_for_package(
        &msg,
    ) {
        Ok(Some(ref opclr)) => conn.route_reply(req, opclr)?,
        Ok(None) => {
            let err = NetError::new(
                ErrCode::ENTITY_NOT_FOUND,
                "vt:origin-package-channel-list:0",
            );
            error!("{}", err);
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-channel-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageListRequest>()?;
    match state.datastore.list_origin_package_for_origin(&msg) {
        Ok(ref oplr) => conn.route_reply(req, oplr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_channel_package_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginChannelPackageListRequest>()?;
    match state.datastore.list_origin_channel_package_for_channel(
        &msg,
    ) {
        Ok(ref oplr) => conn.route_reply(req, oplr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-channel-package-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_group_promote(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageGroupPromote>()?;
    match state.datastore.promote_origin_package_group(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-group-promote:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_promote(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackagePromote>()?;
    match state.datastore.promote_origin_package(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-promote:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_group_demote(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageGroupDemote>()?;
    match state.datastore.demote_origin_package_group(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-group-demote:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_demote(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageDemote>()?;
    match state.datastore.demote_origin_package(&msg) {
        Ok(()) => conn.route_reply(req, &net::NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-demote:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_unique_list(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageUniqueListRequest>()?;
    match state.datastore.list_origin_package_unique_for_origin(&msg) {
        Ok(ref opulr) => conn.route_reply(req, opulr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-unique-list:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_package_search(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginPackageSearchRequest>()?;
    match state.datastore.search_origin_package_for_origin(&msg) {
        Ok(ref opsr) => conn.route_reply(req, opsr)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-package-search:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn origin_member_delete(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> SrvResult<()> {
    let msg = req.parse::<proto::OriginMemberRemove>()?;
    match state.datastore.delete_origin_member(&msg) {
        Ok(()) => conn.route_reply(req, &NetOk::new())?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "vt:origin-member-delete:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}
