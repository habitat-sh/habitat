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

use hab_net::server::Envelope;
use postgres::error::Error as PostgresError;
use postgres::error::SqlState::UniqueViolation;
use protocol::net::{self, NetOk, ErrCode};
use protocol::originsrv as proto;
use zmq;

use super::ServerState;
use error::Result;
use error::Error;

pub fn origin_check_access(req: &mut Envelope,
                           sock: &mut zmq::Socket,
                           state: &mut ServerState)
                           -> Result<()> {
    let msg: proto::CheckOriginAccessRequest = try!(req.parse_msg());

    let is_member = try!(state.datastore.check_account_in_origin(&msg));
    let mut resp = proto::CheckOriginAccessResponse::new();
    resp.set_has_access(is_member);
    try!(req.reply_complete(sock, &resp));
    Ok(())
}

pub fn origin_create(req: &mut Envelope,
                     sock: &mut zmq::Socket,
                     state: &mut ServerState)
                     -> Result<()> {
    let msg: proto::OriginCreate = try!(req.parse_msg());

    match state.datastore.create_origin(&msg) {
        Ok(Some(ref origin)) => try!(req.reply_complete(sock, origin)),
        Ok(None) => {
            // this match branch is likely unnecessary because of the way a unique constraint
            // violation will be handled. see the matching comment in data_store.rs for the
            // create_origin function.
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-create:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(Error::OriginCreate(PostgresError::Db(ref db))) if db.code == UniqueViolation => {
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-create:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-create:2");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_get(req: &mut Envelope,
                  sock: &mut zmq::Socket,
                  state: &mut ServerState)
                  -> Result<()> {
    let msg: proto::OriginGet = try!(req.parse_msg());

    match state.datastore.get_origin(&msg) {
        Ok(Some(ref origin)) => try!(req.reply_complete(sock, origin)),
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_invitation_accept(req: &mut Envelope,
                                sock: &mut zmq::Socket,
                                state: &mut ServerState)
                                -> Result<()> {
    let msg: proto::OriginInvitationAcceptRequest = try!(req.parse_msg());

    match state.datastore.accept_origin_invitation(&msg) {
        Ok(()) => try!(req.reply_complete(sock, &NetOk::new())),
        Err(err) => {
            error!("OriginInvitationList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-invitation-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_invitation_create(req: &mut Envelope,
                                sock: &mut zmq::Socket,
                                state: &mut ServerState)
                                -> Result<()> {
    let msg: proto::OriginInvitationCreate = try!(req.parse_msg());

    match state.datastore.create_origin_invitation(&msg) {
        Ok(Some(ref invite)) => try!(req.reply_complete(sock, invite)),
        Ok(None) => {
            debug!("User {} is already a member of the origin {}",
                   &msg.get_origin_name(),
                   &msg.get_account_name());
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-invitation-create:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginInvitationCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-invitation-create:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_invitation_list(req: &mut Envelope,
                              sock: &mut zmq::Socket,
                              state: &mut ServerState)
                              -> Result<()> {
    let msg: proto::OriginInvitationListRequest = try!(req.parse_msg());

    match state.datastore.list_origin_invitations_for_origin(&msg) {
        Ok(ref oilr) => try!(req.reply_complete(sock, oilr)),
        Err(err) => {
            error!("OriginInvitationList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-invitation-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_member_list(req: &mut Envelope,
                          sock: &mut zmq::Socket,
                          state: &mut ServerState)
                          -> Result<()> {
    let msg: proto::OriginMemberListRequest = try!(req.parse_msg());
    match state.datastore.list_origin_members(&msg) {
        Ok(ref omlr) => try!(req.reply_complete(sock, omlr)),
        Err(err) => {
            error!("OriginMemberList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-member-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_secret_key_create(req: &mut Envelope,
                                sock: &mut zmq::Socket,
                                state: &mut ServerState)
                                -> Result<()> {
    let msg: proto::OriginSecretKeyCreate = try!(req.parse_msg());

    match state.datastore.create_origin_secret_key(&msg) {
        Ok(ref osk) => try!(req.reply_complete(sock, osk)),
        Err(Error::OriginSecretKeyCreate(PostgresError::Db(ref db))) if db.code ==
                                                                        UniqueViolation => {
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-secret-key-create:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginSecretKeyCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-secret-key-create:2");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_secret_key_get(req: &mut Envelope,
                             sock: &mut zmq::Socket,
                             state: &mut ServerState)
                             -> Result<()> {
    let msg: proto::OriginSecretKeyGet = try!(req.parse_msg());
    match state.datastore.get_origin_secret_key(&msg) {
        Ok(Some(ref key)) => {
            try!(req.reply_complete(sock, key));
        }
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-secret-key-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginSecretKeyGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-secret-key-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_public_key_create(req: &mut Envelope,
                                sock: &mut zmq::Socket,
                                state: &mut ServerState)
                                -> Result<()> {
    let msg: proto::OriginPublicKeyCreate = try!(req.parse_msg());

    match state.datastore.create_origin_public_key(&msg) {
        Ok(ref osk) => try!(req.reply_complete(sock, osk)),
        Err(Error::OriginPublicKeyCreate(PostgresError::Db(ref db))) if db.code ==
                                                                        UniqueViolation => {
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-public-key-create:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginPublicKeyCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-public-key-create:2");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_public_key_get(req: &mut Envelope,
                             sock: &mut zmq::Socket,
                             state: &mut ServerState)
                             -> Result<()> {
    let msg: proto::OriginPublicKeyGet = try!(req.parse_msg());
    match state.datastore.get_origin_public_key(&msg) {
        Ok(Some(ref key)) => {
            try!(req.reply_complete(sock, key));
        }
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-public-key-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginPublicKeyGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-public-key-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_public_key_latest_get(req: &mut Envelope,
                                    sock: &mut zmq::Socket,
                                    state: &mut ServerState)
                                    -> Result<()> {
    let msg: proto::OriginPublicKeyLatestGet = try!(req.parse_msg());
    match state.datastore.get_origin_public_key_latest(&msg) {
        Ok(Some(ref key)) => {
            try!(req.reply_complete(sock, key));
        }
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND,
                               "vt:origin-public-key-latest-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginPublicKeyLatestGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-public-key-latest-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_public_key_list(req: &mut Envelope,
                              sock: &mut zmq::Socket,
                              state: &mut ServerState)
                              -> Result<()> {
    let msg: proto::OriginPublicKeyListRequest = try!(req.parse_msg());
    match state.datastore.list_origin_public_keys_for_origin(&msg) {
        Ok(ref opklr) => try!(req.reply_complete(sock, opklr)),
        Err(err) => {
            error!("OriginPublicKeyListForOrigin, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-public-key-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn project_create(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let opc = try!(req.parse_msg::<proto::OriginProjectCreate>());

    match state.datastore.create_origin_project(&opc) {
        Ok(ref project) => try!(req.reply_complete(sock, project)),
        Err(Error::OriginProjectCreate(PostgresError::Db(ref db))) if db.code ==
                                                                      UniqueViolation => {
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-project-create:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("ProjectCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-project-create:2");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn project_delete(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let msg: proto::OriginProjectDelete = try!(req.parse_msg());

    match state
              .datastore
              .delete_origin_project_by_name(&msg.get_name()) {
        Ok(()) => try!(req.reply_complete(sock, &NetOk::new())),
        Err(err) => {
            error!("OriginProjectGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-project-delete:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn project_get(req: &mut Envelope,
                   sock: &mut zmq::Socket,
                   state: &mut ServerState)
                   -> Result<()> {
    let msg: proto::OriginProjectGet = try!(req.parse_msg());
    match state.datastore.get_origin_project_by_name(&msg.get_name()) {
        Ok(Some(ref project)) => try!(req.reply_complete(sock, project)),
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-project-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginProjectGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-project-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn project_update(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let msg: proto::OriginProjectUpdate = try!(req.parse_msg());

    match state.datastore.update_origin_project(&msg) {
        Ok(()) => try!(req.reply_complete(sock, &NetOk::new())),
        Err(err) => {
            error!("OriginProjectUpdate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-project-update:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_channel_create(req: &mut Envelope,
                             sock: &mut zmq::Socket,
                             state: &mut ServerState)
                             -> Result<()> {
    let msg: proto::OriginChannelCreate = try!(req.parse_msg());

    match state.datastore.create_origin_channel(&msg) {
        Ok(ref occ) => try!(req.reply_complete(sock, occ)),
        Err(Error::OriginChannelCreate(PostgresError::Db(ref db))) if db.code ==
                                                                      UniqueViolation => {
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-channel-create:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginChannelCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-channel-create:2");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_channel_delete(req: &mut Envelope,
                             sock: &mut zmq::Socket,
                             state: &mut ServerState)
                             -> Result<()> {
    let msg: proto::OriginChannelDelete = try!(req.parse_msg());
    match state.datastore.delete_origin_channel_by_id(&msg) {
        Ok(()) =>  try!(req.reply_complete(sock, &net::NetOk::new())),
        Err(err) => {
            error!("OriginChannelDelete, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-channel-delete:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_channel_get(req: &mut Envelope,
                          sock: &mut zmq::Socket,
                          state: &mut ServerState)
                          -> Result<()> {
    let msg: proto::OriginChannelGet = try!(req.parse_msg());
    match state.datastore.get_origin_channel(&msg) {
        Ok(Some(ref channel)) => try!(req.reply_complete(sock, channel)),
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-channel-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginChannelGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-channel-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_channel_list(req: &mut Envelope,
                           sock: &mut zmq::Socket,
                           state: &mut ServerState)
                           -> Result<()> {
    let msg: proto::OriginChannelListRequest = try!(req.parse_msg());
    match state.datastore.list_origin_channels(&msg) {
        Ok(ref oclr) => try!(req.reply_complete(sock, oclr)),
        Err(err) => {
            error!("OriginChannelList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-channel-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_create(req: &mut Envelope,
                             sock: &mut zmq::Socket,
                             state: &mut ServerState)
                             -> Result<()> {
    let msg: proto::OriginPackageCreate = try!(req.parse_msg());

    match state.datastore.create_origin_package(&msg) {
        Ok(ref opc) => try!(req.reply_complete(sock, opc)),
        Err(err) => {
            error!("OriginPackageCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-create:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_get(req: &mut Envelope,
                          sock: &mut zmq::Socket,
                          state: &mut ServerState)
                          -> Result<()> {
    let msg: proto::OriginPackageGet = try!(req.parse_msg());
    match state.datastore.get_origin_package(&msg) {
        Ok(Some(ref package)) => try!(req.reply_complete(sock, package)),
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-package-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginPackageGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_channel_package_get(req: &mut Envelope,
                                  sock: &mut zmq::Socket,
                                  state: &mut ServerState)
                                  -> Result<()> {
    let msg: proto::OriginChannelPackageGet = try!(req.parse_msg());
    match state.datastore.get_origin_channel_package(&msg) {
        Ok(Some(ref package)) => try!(req.reply_complete(sock, package)),
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-channel-package-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginChannelPackageGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-channel-package-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_latest_get(req: &mut Envelope,
                                 sock: &mut zmq::Socket,
                                 state: &mut ServerState)
                                 -> Result<()> {
    let msg: proto::OriginPackageLatestGet = try!(req.parse_msg());
    match state.datastore.get_origin_package_latest(&msg) {
        Ok(Some(ref package)) => try!(req.reply_complete(sock, package)),
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-package-latest-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginPackageLatestGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-latest-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_channel_package_latest_get(req: &mut Envelope,
                                         sock: &mut zmq::Socket,
                                         state: &mut ServerState)
                                         -> Result<()> {
    let msg: proto::OriginChannelPackageLatestGet = try!(req.parse_msg());
    match state.datastore.get_origin_channel_package_latest(&msg) {
        Ok(Some(ref package)) => try!(req.reply_complete(sock, package)),
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND,
                               "vt:origin-channel-package-latest-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginChannelPackageLatestGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE,
                               "vt:origin-channel-package-latest-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_version_list(req: &mut Envelope,
                                   sock: &mut zmq::Socket,
                                   state: &mut ServerState)
                                   -> Result<()> {
    let msg: proto::OriginPackageVersionListRequest = try!(req.parse_msg());

    match state
              .datastore
              .list_origin_package_versions_for_origin(&msg) {
        Ok(ref opvlr) => try!(req.reply_complete(sock, opvlr)),
        Err(err) => {
            error!("OriginPackageVersionList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-version-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_list(req: &mut Envelope,
                           sock: &mut zmq::Socket,
                           state: &mut ServerState)
                           -> Result<()> {
    let msg: proto::OriginPackageListRequest = try!(req.parse_msg());
    match state.datastore.list_origin_package_for_origin(&msg) {
        Ok(ref oplr) => try!(req.reply_complete(sock, oplr)),
        Err(err) => {
            error!("OriginPackageList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_channel_package_list(req: &mut Envelope,
                                   sock: &mut zmq::Socket,
                                   state: &mut ServerState)
                                   -> Result<()> {
    let msg: proto::OriginChannelPackageListRequest = try!(req.parse_msg());
    match state
              .datastore
              .list_origin_channel_package_for_channel(&msg) {
        Ok(ref oplr) => try!(req.reply_complete(sock, oplr)),
        Err(err) => {
            error!("OriginChannelPackageList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-channel-package-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_promote(req: &mut Envelope,
                              sock: &mut zmq::Socket,
                              state: &mut ServerState)
                              -> Result<()> {
    let msg: proto::OriginPackagePromote = try!(req.parse_msg());
    match state.datastore.promote_origin_package(&msg) {
        Ok(()) =>  try!(req.reply_complete(sock, &net::NetOk::new())),
        Err(err) => {
            error!("OriginPackagePromote, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-promote:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_unique_list(req: &mut Envelope,
                                  sock: &mut zmq::Socket,
                                  state: &mut ServerState)
                                  -> Result<()> {
    let msg: proto::OriginPackageUniqueListRequest = try!(req.parse_msg());
    match state.datastore.list_origin_package_unique_for_origin(&msg) {
        Ok(ref opulr) => try!(req.reply_complete(sock, opulr)),
        Err(err) => {
            error!("OriginPackageUniqueList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-unique-list:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn origin_package_search(req: &mut Envelope,
                             sock: &mut zmq::Socket,
                             state: &mut ServerState)
                             -> Result<()> {
    let msg: proto::OriginPackageSearchRequest = try!(req.parse_msg());
    match state.datastore.search_origin_package_for_origin(&msg) {
        Ok(ref opsr) => try!(req.reply_complete(sock, opsr)),
        Err(err) => {
            error!("OriginPackageSearch, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-package-search:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}
