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

use dbcache::{self, ExpiringSet, IndexSet, InstaSet};
use hab_net::server::Envelope;
use protocol::net::{self, ErrCode};
use protocol::sessionsrv as proto;
use zmq;

use super::ServerState;
use error::Result;

pub fn account_get(req: &mut Envelope,
                   sock: &mut zmq::Socket,
                   state: &mut ServerState)
                   -> Result<()> {
    let msg: proto::AccountGet = try!(req.parse_msg());
    match state.datastore.accounts.find_by_username(&msg.get_name().to_string()) {
        Ok(account) => {
            try!(req.reply_complete(sock, &account));
        }
        Err(dbcache::Error::EntityNotFound) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "ss:account_get:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("datastore error, err={:?}", e);
            let err = net::err(ErrCode::INTERNAL, "ss:account_get:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn session_create(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let mut msg: proto::SessionCreate = try!(req.parse_msg());
    let mut account: proto::Account = match state.datastore
        .sessions
        .find(&msg.get_token().to_string()) {
        Ok(session) => state.datastore.accounts.find(&session.get_owner_id()).unwrap(),
        _ => try!(state.datastore.accounts.find_or_create(&msg)),
    };
    let mut session_token = proto::SessionToken::new();
    session_token.set_owner_id(account.get_id());
    session_token.set_token(msg.take_token());
    try!(state.datastore.sessions.write(&mut session_token));
    let mut session = proto::Session::new();
    session.set_token(session_token.take_token());
    session.set_id(session_token.get_owner_id());
    session.set_email(account.take_email());
    session.set_name(account.take_name());
    try!(req.reply_complete(sock, &session));
    Ok(())
}

pub fn session_get(req: &mut Envelope,
                   sock: &mut zmq::Socket,
                   state: &mut ServerState)
                   -> Result<()> {
    let msg: proto::SessionGet = try!(req.parse_msg());
    match state.datastore.sessions.find(&msg.get_token().to_string()) {
        Ok(mut token) => {
            let account: proto::Account =
                state.datastore.accounts.find(&token.get_owner_id()).unwrap();
            let mut session: proto::Session = account.into();
            session.set_token(token.take_token());
            try!(req.reply_complete(sock, &session));
        }
        Err(dbcache::Error::EntityNotFound) => {
            let err = net::err(ErrCode::SESSION_EXPIRED, "ss:auth:4");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("datastore error, err={:?}", e);
            let err = net::err(ErrCode::INTERNAL, "ss:auth:5");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}
