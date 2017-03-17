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

use dbcache::{self, ExpiringSet, InstaSet};
use hab_net::privilege::{self, FeatureFlags};
use hab_net::server::Envelope;
use protocol::net::{self, ErrCode, NetOk};
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
        Ok(account) => try!(req.reply_complete(sock, &account)),
        Err(dbcache::Error::EntityNotFound) => {
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

pub fn account_search(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let mut msg: proto::AccountSearch = try!(req.parse_msg());
    let result = match msg.get_key() {
        proto::AccountSearchKey::Id => {
            let value: u64 = msg.take_value().parse().unwrap();
            state.datastore.accounts.find(&value)
        }
        proto::AccountSearchKey::Name => {
            state.datastore.accounts.find_by_username(&msg.take_value())
        }
    };
    match result {
        Ok(account) => try!(req.reply_complete(sock, &account)),
        Err(dbcache::Error::EntityNotFound) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "ss:account-search:0");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("{}", e);
            let err = net::err(ErrCode::DATA_STORE, "ss:account-search:1");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn grant_flag(req: &mut Envelope,
                  sock: &mut zmq::Socket,
                  state: &mut ServerState)
                  -> Result<()> {
    let msg: proto::GrantFlagToTeam = try!(req.parse_msg());
    try!(state.datastore.features.grant(msg.get_flag(), msg.get_team_id()));
    try!(req.reply_complete(sock, &NetOk::new()));
    Ok(())
}

pub fn grant_list(req: &mut Envelope,
                  sock: &mut zmq::Socket,
                  state: &mut ServerState)
                  -> Result<()> {
    let msg: proto::ListFlagGrants = try!(req.parse_msg());
    let teams = try!(state.datastore.features.teams(msg.get_flag()));
    let mut grants = proto::FlagGrants::new();
    grants.set_teams(teams);
    try!(req.reply_complete(sock, &grants));
    Ok(())
}

pub fn revoke_flag(req: &mut Envelope,
                   sock: &mut zmq::Socket,
                   state: &mut ServerState)
                   -> Result<()> {
    let msg: proto::RevokeFlagFromTeam = try!(req.parse_msg());
    try!(state.datastore.features.revoke(msg.get_flag(), msg.get_team_id()));
    try!(req.reply_complete(sock, &NetOk::new()));
    Ok(())
}

pub fn session_create(req: &mut Envelope,
                      sock: &mut zmq::Socket,
                      state: &mut ServerState)
                      -> Result<()> {
    let mut msg: proto::SessionCreate = try!(req.parse_msg());
    let account: proto::Account = match state.datastore.sessions.find(&msg.get_token()
                                             .to_string()) {
        Ok(session) => {
            state.datastore
                .accounts
                .find(&session.get_owner_id())
                .unwrap()
        }
        _ => try!(state.datastore.accounts.find_or_create(&msg)),
    };
    let mut session_token = proto::SessionToken::new();
    session_token.set_token(msg.take_token());
    session_token.set_owner_id(account.get_id());
    session_token.set_provider(msg.get_provider());
    if let Some(e) = state.datastore
           .sessions
           .write(&mut session_token)
           .err() {
        error!("{}", e);
        let err = net::err(ErrCode::DATA_STORE, "ss:session-create:0");
        try!(req.reply_complete(sock, &err));
        return Ok(());
    }
    let mut session: proto::Session = account.into();
    session.set_token(session_token.take_token());
    if let Some(err) = set_features(&state, &mut session).err() {
        // JW TODO: handle this and reply with a partial auth (sans features) if they can't be
        // obtained instead of outputting an error
        error!("unable to set features, {}", err);
    }
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
            let account: proto::Account = state.datastore
                .accounts
                .find(&token.get_owner_id())
                .unwrap();
            let mut session: proto::Session = account.into();
            session.set_token(token.take_token());
            if let Some(err) = set_features(&state, &mut session).err() {
                // JW TODO: handle this and reply with a partial auth (sans features) if they can't
                // be obtained instead of outputting an error
                error!("unable to set features, {}", err);
            }
            try!(req.reply_complete(sock, &session));
        }
        Err(dbcache::Error::EntityNotFound) => {
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

// Determine permissions and toggle feature flags on for the given Session
fn set_features(state: &ServerState, session: &mut proto::Session) -> Result<()> {
    let mut flags = FeatureFlags::empty();
    // Initialize some empty flags in case we fail to obtain teams from remote
    session.set_flags(flags.bits());
    let teams = try!(state.github.teams(session.get_token()));
    for team in teams {
        if team.id != 0 && team.id == state.admin_team {
            debug!("Granting feature flag={:?} for team={:?}",
                   privilege::ADMIN,
                   team.name);
            flags.insert(privilege::ADMIN);
            continue;
        }
        if let Some(raw_flags) = state.datastore
               .features
               .flags(team.id)
               .ok() {
            for raw_flag in raw_flags {
                let flag = FeatureFlags::from_bits(raw_flag).unwrap();
                debug!("Granting feature flag={:?} for team={:?}", flag, team.name);
                flags.insert(flag);
            }
        }
    }
    session.set_flags(flags.bits());
    Ok(())
}
