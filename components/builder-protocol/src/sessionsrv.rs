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

use std::result;
use std::str::FromStr;

use serde::{Serialize, Serializer};
use serde::ser::{SerializeSeq, SerializeStruct};

use error::{ProtocolError, ProtocolResult};
use message::{Persistable, Routable};
use search::FromSearchPair;

pub use message::sessionsrv::*;

impl FromStr for AccountSearchKey {
    type Err = ProtocolError;

    fn from_str(value: &str) -> ProtocolResult<Self> {
        let value = value.to_lowercase();
        match value.as_ref() {
            "name" => Ok(AccountSearchKey::Name),
            "id" => Ok(AccountSearchKey::Id),
            _ => Err(ProtocolError::BadSearchKey(value)),
        }
    }
}

impl Routable for SessionCreate {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_extern_id())
    }
}

impl Routable for SessionGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: how do we know the shard from the session key? Is it embedded? Is this a
        // composite key that contains the shard plus the token?
        None
    }
}

impl Persistable for Account {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Into<Session> for Account {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_name(self.get_name().to_owned());
        session
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("account", 3));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("email", self.get_email()));
        strukt.end()
    }
}

impl Routable for AccountGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_name().to_string())
    }
}

impl FromSearchPair for AccountSearch {
    fn from_search_pair<K: AsRef<str>, V: Into<String>>(key: K, value: V) -> ProtocolResult<Self> {
        let key = try!(AccountSearchKey::from_str(key.as_ref()));
        let mut search = AccountSearch::new();
        search.set_key(key);
        search.set_value(value.into());
        Ok(search)
    }
}

impl Routable for AccountSearch {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_value().to_string())
    }
}

impl Routable for ListFlagGrants {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: We need to define a new type of routable message - Broadcast. This message
        // needs to hit every session server and not just one.
        //
        // An alternative implementation would be to elect a SessionServer as master and have it
        // broadcast state to the other session servers. For now, this is fine since we run
        // one session server in all environments.
        None
    }
}

impl Routable for GrantFlagToTeam {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: We need to define a new type of routable message - Broadcast. This message
        // needs to hit every session server and not just one.
        //
        // An alternative implementation would be to elect a SessionServer as master and have it
        // broadcast state to the other session servers. For now, this is fine since we run
        // one session server in all environments.
        None
    }
}

impl Routable for RevokeFlagFromTeam {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: We need to define a new type of routable message - Broadcast. This message
        // needs to hit every session server and not just one.
        //
        // An alternative implementation would be to elect a SessionServer as master and have it
        // broadcast state to the other session servers. For now, this is fine since we run
        // one session server in all environments.
        None
    }
}

impl Persistable for SessionToken {
    type Key = String;

    fn primary_key(&self) -> Self::Key {
        self.get_token().to_string()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_token(value)
    }
}

impl Serialize for FlagGrants {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut seq = try!(serializer.serialize_seq(Some(self.get_teams().len())));
        for e in self.get_teams() {
            try!(seq.serialize_element(&e));
        }
        seq.end()
    }
}

impl Serialize for Session {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("session", 5));
        try!(strukt.serialize_field("token", self.get_token()));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("email", self.get_email()));
        try!(strukt.serialize_field("flags", &self.get_flags()));
        strukt.end()
    }
}
