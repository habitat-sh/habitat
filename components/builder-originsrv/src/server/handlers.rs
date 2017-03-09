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
use protobuf::RepeatedField;
use protocol::net::{self, NetOk, ErrCode};
use protocol::originsrv as proto;
use zmq;

use super::ServerState;
use error::Result;

pub fn account_invitation_list(req: &mut Envelope,
                               sock: &mut zmq::Socket,
                               state: &mut ServerState)
                               -> Result<()> {
    let mut resp = proto::AccountInvitationListResponse::new();

    let msg: proto::AccountInvitationListRequest = try!(req.parse_msg());

    match state.datastore.list_origin_invitations_for_account(&msg) {
        Ok(Some(invites)) => {
            debug!("Got invites for account {} ", &msg.get_account_id());
            resp.set_account_id(msg.get_account_id());
            let mut r_invites = RepeatedField::new();
            for invite in invites {
                r_invites.push(invite);
            }
            resp.set_invitations(r_invites);
            try!(req.reply_complete(sock, &resp));
        }
        Ok(None) => {
            debug!("No invites for account {} ", &msg.get_account_id());
            try!(req.reply_complete(sock, &resp));
        }
        Err(e) => {
            error!("Account Invitation List, err={:?}", e);
            let err = net::err(ErrCode::BUG, "vt:account_invitation_list:0");
            try!(req.reply_complete(sock, &err));
        }
    }

    Ok(())
}

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
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-create:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("OriginCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-create:1");
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

pub fn origin_invitation_validate(req: &mut Envelope,
                                  sock: &mut zmq::Socket,
                                  state: &mut ServerState)
                                  -> Result<()> {
    let msg: proto::OriginInvitationValidateRequest = try!(req.parse_msg());

    match state.datastore.validate_origin_invitation(&msg) {
        Ok(ref response) => try!(req.reply_complete(sock, response)),
        Err(err) => {
            error!("OriginInvitationValidate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-invitation-validate:1");
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

    let in_origin = state.datastore
        .check_account_in_origin_by_origin_and_account_id(msg.get_origin_name(),
                                                          msg.get_account_id() as i64)?;
    if !in_origin {
        debug!("Can't invite to this org unless your already a member");
        let err = net::err(ErrCode::ACCESS_DENIED, "vt:origin-invitation-create:0");
        try!(req.reply_complete(sock, &err));
    } else {
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

pub fn account_origin_list(req: &mut Envelope,
                           sock: &mut zmq::Socket,
                           state: &mut ServerState)
                           -> Result<()> {
    let msg: proto::AccountOriginListRequest = try!(req.parse_msg());

    match state.datastore.list_origins_by_account(&msg) {
        Ok(ref aolr) => try!(req.reply_complete(sock, aolr)),
        Err(err) => {
            error!("OriginAccountList, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-account-list:1");
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
        Err(err) => {
            error!("OriginSecretKeyCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-secret-key-create:1");
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

pub fn project_create(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let opc = try!(req.parse_msg::<proto::OriginProjectCreate>());

    match state.datastore.create_origin_project(&opc) {
        Ok(ref project) => try!(req.reply_complete(sock, project)),
        Err(err) => {
            error!("ProjectCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:origin-project-create:1");
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

    match state.datastore.delete_origin_project_by_name(&msg.get_name()) {
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
