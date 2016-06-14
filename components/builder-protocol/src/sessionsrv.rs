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

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};

use message::{Persistable, Routable};

pub use message::sessionsrv::*;

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

impl ToJson for Account {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("id".to_string(), self.get_id().to_string().to_json());
        m.insert("name".to_string(), self.get_name().to_json());
        m.insert("email".to_string(), self.get_email().to_json());
        Json::Object(m)
    }
}

impl Routable for AccountGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_name().to_string())
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

impl ToJson for Session {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("token".to_string(), self.get_token().to_json());
        m.insert("email".to_string(), self.get_email().to_json());
        m.insert("name".to_string(), self.get_name().to_json());
        m.insert("id".to_string(), self.get_id().to_json());
        Json::Object(m)
    }
}
