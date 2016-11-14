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

use dbcache::{self, BasicSet, IndexSet, InstaSet};
use hab_net::server::Envelope;
use protobuf::RepeatedField;
use protocol::net::{self, NetOk, ErrCode};
use protocol::vault as proto;
use zmq;

use super::ServerState;
use error::Result;

pub fn account_invitation_list(req: &mut Envelope,
                               sock: &mut zmq::Socket,
                               state: &mut ServerState)
                               -> Result<()> {
    let msg: proto::AccountInvitationListRequest = try!(req.parse_msg());
    let invites = try!(state.datastore.origins.invites.get_by_account_id(msg.get_account_id()));
    debug!("Got invites for account {} ", &msg.get_account_id());
    let mut resp = proto::AccountInvitationListResponse::new();
    resp.set_account_id(msg.get_account_id());

    let mut r_invites = RepeatedField::new();
    for invite in invites {
        r_invites.push(invite);
    }
    resp.set_invitations(r_invites);
    try!(req.reply_complete(sock, &resp));
    Ok(())
}

pub fn origin_check_access(req: &mut Envelope,
                           sock: &mut zmq::Socket,
                           state: &mut ServerState)
                           -> Result<()> {
    let msg: proto::CheckOriginAccessRequest = try!(req.parse_msg());
    let is_member = try!(state.datastore
        .origins
        .is_origin_member(msg.get_account_id(), msg.get_origin_name()));
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
    let mut origin = proto::Origin::new();
    origin.set_name(msg.get_name().to_string());
    origin.set_owner_id(msg.get_owner_id());
    if let Ok(_origin) = state.datastore
        .origins
        .name_idx
        .find(&msg.get_name().to_string()) {
        let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-create:0");
        try!(req.reply_complete(sock, &err));
    }
    try!(state.datastore.origins.write(&mut origin));
    debug!("Adding owner as origin member: {}", &msg.get_owner_name());
    try!(state.datastore
        .origins
        .add_origin_member(msg.get_owner_id(), msg.get_owner_name(), msg.get_name()));
    try!(req.reply_complete(sock, &origin));
    Ok(())
}

pub fn origin_get(req: &mut Envelope,
                  sock: &mut zmq::Socket,
                  state: &mut ServerState)
                  -> Result<()> {
    let mut msg: proto::OriginGet = try!(req.parse_msg());
    match state.datastore.origins.name_idx.find(&msg.take_name()) {
        Ok(origin_id) => {
            let mut origin = state.datastore.origins.find(&origin_id).unwrap();

            match state.datastore.origins.key_idx.find(&origin.get_name().to_string()) {
                Ok(key_id) => {
                    let secret_key =
                        state.datastore.origins.origin_secret_keys.find(&key_id).unwrap();
                    origin.set_private_key_name(format!("{}-{}",
                                                        secret_key.get_name(),
                                                        secret_key.get_revision()));
                }
                Err(e) => debug!("error looking for secret key {:?}", e),
            };

            try!(req.reply_complete(sock, &origin));
        }
        Err(dbcache::Error::EntityNotFound) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:origin-get:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("OriginGet, err={:?}", e);
            let err = net::err(ErrCode::BUG, "vt:origin-get:0");
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
    match state.datastore.origins.invites.find(&msg.get_invite_id()) {
        Ok(invite) => {
            debug!("REQ    {:?}", &msg);
            debug!("INVITE {:?}", &invite);
            if msg.get_account_accepting_request() != invite.get_account_id() {
                let err = net::err(ErrCode::ACCESS_DENIED, "vt:origin-invite-accept:0");
                try!(req.reply_complete(sock, &err));
            }

            match state.datastore.origins.modify_invite(&invite, msg.get_ignore()) {
                Ok(()) => (),
                Err(e) => {
                    debug!("Error accepting invite: {}", e);
                }
            };
        }
        Err(e) => {
            debug!("Error accepting invite, maybe it's already been accepted? {}",
                   e);
        }
    };

    let resp = proto::OriginInvitationAcceptResponse::new();
    try!(req.reply_complete(sock, &resp));
    Ok(())
}

pub fn origin_invitation_create(req: &mut Envelope,
                                sock: &mut zmq::Socket,
                                state: &mut ServerState)
                                -> Result<()> {
    let msg: proto::OriginInvitationCreate = try!(req.parse_msg());
    let mut invitation = proto::OriginInvitation::new();
    if !try!(state.datastore
        .origins
        .is_origin_member(msg.get_account_id(), msg.get_origin_name())) {
        debug!("Can't invite to this org unless your already a member");
        let err = net::err(ErrCode::ACCESS_DENIED, "vt:origin-create:0");
        try!(req.reply_complete(sock, &err));
    }

    let existing_invites =
        try!(state.datastore.origins.invites.get_by_account_id(msg.get_account_id()));

    for invite in &existing_invites {
        if invite.get_origin_name() == msg.get_origin_name() {
            debug!("Invite for origin {} for user {} already exists",
                   &msg.get_origin_name(),
                   &msg.get_account_name());
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-create:1");
            try!(req.reply_complete(sock, &err));
            return Ok(());
        }
    }

    invitation.set_account_id(msg.get_account_id());
    invitation.set_account_name(msg.get_account_name().to_string());
    invitation.set_origin_id(msg.get_origin_id());
    invitation.set_origin_name(msg.get_origin_name().to_string());
    invitation.set_owner_id(msg.get_owner_id());

    try!(state.datastore.origins.invites.write(&mut invitation));
    try!(req.reply_complete(sock, &invitation));
    Ok(())
}

pub fn origin_invitation_list(req: &mut Envelope,
                              sock: &mut zmq::Socket,
                              state: &mut ServerState)
                              -> Result<()> {
    let msg: proto::OriginInvitationListRequest = try!(req.parse_msg());
    let invites = try!(state.datastore
        .origins
        .invites
        .get_by_origin_id(msg.get_origin_id()));
    let mut resp = proto::OriginInvitationListResponse::new();
    resp.set_origin_id(msg.get_origin_id());
    let mut r_invites = RepeatedField::new();
    for invite in invites {
        r_invites.push(invite);
    }
    resp.set_invitations(r_invites);
    try!(req.reply_complete(sock, &resp));
    Ok(())
}

pub fn origin_list(req: &mut Envelope,
                   sock: &mut zmq::Socket,
                   _state: &mut ServerState)
                   -> Result<()> {
    let origin1 = proto::Origin::new();
    let origin2 = proto::Origin::new();
    let origins = vec![origin1, origin2];
    for (i, origin) in origins.iter().enumerate() {
        if i == origins.len() {
            try!(req.reply_complete(sock, origin));
        } else {
            try!(req.reply(sock, origin));
        }
    }
    Ok(())
}

pub fn origin_member_list(req: &mut Envelope,
                          sock: &mut zmq::Socket,
                          state: &mut ServerState)
                          -> Result<()> {
    let msg: proto::OriginMemberListRequest = try!(req.parse_msg());
    let members = try!(state.datastore.origins.list_origin_members(msg.get_origin_id()));
    let mut r_members = RepeatedField::new();
    for member in members {
        r_members.push(member);
    }
    let mut resp = proto::OriginMemberListResponse::new();
    resp.set_origin_id(msg.get_origin_id());
    resp.set_members(r_members);
    try!(req.reply_complete(sock, &resp));
    Ok(())
}

pub fn account_origin_list(req: &mut Envelope,
                           sock: &mut zmq::Socket,
                           state: &mut ServerState)
                           -> Result<()> {
    let msg: proto::AccountOriginListRequest = try!(req.parse_msg());
    let origins = try!(state.datastore.origins.list_account_origins(msg.get_account_id()));
    let mut r_origins = RepeatedField::new();
    for origin in origins {
        r_origins.push(origin);
    }
    let mut resp = proto::AccountOriginListResponse::new();
    resp.set_account_id(msg.get_account_id());
    resp.set_origins(r_origins);
    try!(req.reply_complete(sock, &resp));
    Ok(())
}

pub fn origin_secret_key_create(req: &mut Envelope,
                                sock: &mut zmq::Socket,
                                state: &mut ServerState)
                                -> Result<()> {
    let msg: proto::OriginSecretKeyCreate = try!(req.parse_msg());
    let mut pk = proto::OriginSecretKey::new();
    pk.set_name(msg.get_name().to_string());
    pk.set_revision(msg.get_revision().to_string());
    pk.set_origin_id(msg.get_origin_id());
    pk.set_owner_id(msg.get_owner_id());
    pk.set_body(msg.get_body().to_vec());
    match state.datastore.origins.origin_secret_keys.write(&mut pk) {
        Ok(true) => try!(req.reply_complete(sock, &pk)),
        Ok(false) => {
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:origin-secret-key-create:0");
            try!(req.reply_complete(sock, &err));
        }
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
    let mut msg: proto::OriginSecretKeyGet = try!(req.parse_msg());
    match state.datastore.origins.key_idx.find(&msg.take_origin()) {
        Ok(ref id) => {
            let key = state.datastore.origins.origin_secret_keys.find(id).unwrap();
            try!(req.reply_complete(sock, &key));
        }
        Err(dbcache::Error::EntityNotFound) => {
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
    let mut project = try!(req.parse_msg::<proto::ProjectCreate>()).take_project();
    match state.datastore.projects.write(&mut project) {
        Ok(true) => try!(req.reply_complete(sock, &project)),
        Ok(false) => {
            let err = net::err(ErrCode::ENTITY_CONFLICT, "vt:project-create:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("ProjectCreate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:project-create:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn project_delete(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let mut msg: proto::ProjectDelete = try!(req.parse_msg());
    try!(state.datastore.projects.delete(&msg.take_id()));
    try!(req.reply_complete(sock, &NetOk::new()));
    Ok(())
}

pub fn project_get(req: &mut Envelope,
                   sock: &mut zmq::Socket,
                   state: &mut ServerState)
                   -> Result<()> {
    let mut msg: proto::ProjectGet = try!(req.parse_msg());
    match state.datastore.projects.find(&msg.take_id()) {
        Ok(ref project) => try!(req.reply_complete(sock, project)),
        Err(dbcache::Error::EntityNotFound) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:project-get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("ProjectGet, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:project-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn project_update(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let msg: proto::ProjectUpdate = try!(req.parse_msg());
    match state.datastore.projects.update(&msg.get_project()) {
        Ok(()) => try!(req.reply_complete(sock, &NetOk::new())),
        Err(dbcache::Error::EntityNotFound) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "vt:project-update:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(err) => {
            error!("ProjectUpdate, err={:?}", err);
            let err = net::err(ErrCode::DATA_STORE, "vt:project-update:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}
