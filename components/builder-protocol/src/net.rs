// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::BTreeMap;

use protobuf::core::ProtobufEnum;
use rustc_serialize::json::{Json, ToJson};

pub use message::net::*;

pub fn err<M: Into<String>>(code: ErrCode, msg: M) -> NetError {
    let mut err = NetError::new();
    err.set_code(code);
    err.set_msg(msg.into());
    err
}

impl ToJson for ErrCode {
    fn to_json(&self) -> Json {
        Json::U64(self.value() as u64)
    }
}

impl ToJson for NetError {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("code".to_string(), self.get_code().to_json());
        m.insert("msg".to_string(), self.get_msg().to_json());
        Json::Object(m)
    }
}
