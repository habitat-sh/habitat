// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};

use message::{Persistable, Routable};
use sharding::InstaId;

pub use message::vault::*;

impl Persistable for Origin {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Routable for OriginGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: This won't acurately find the origin without it. We can switch to using the ID
        // of the origin or perform a reverse lookup by storing the name->ID map on a particular
        // vault server.
        Some(self.get_name().to_string())
    }
}

impl Routable for OriginCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl ToJson for Origin {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("id".to_string(), self.get_id().to_json());
        m.insert("name".to_string(), self.get_name().to_json());
        m.insert("owner_id".to_string(), self.get_owner_id().to_json());
        Json::Object(m)
    }
}
