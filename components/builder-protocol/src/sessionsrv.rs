// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
