// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};

use message::Routable;

pub use message::sessionsrv::*;

impl Routable for SessionCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: define a suitable route hash for creating sessions. The gateway should probably
        // perform the oauth authentication to retrieve the ID of the oauth user and then we should
        // route on that.
        None
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
