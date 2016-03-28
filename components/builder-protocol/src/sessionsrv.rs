// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::BTreeMap;

use rustc_serialize::json::{Json, ToJson};

pub use message::sessionsrv::*;

impl ToJson for AuthToken {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("token".to_string(), self.get_token().to_json());
        Json::Object(m)
    }
}
