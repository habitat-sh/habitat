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
use std::error;
use std::fmt;
use std::result;

use protobuf::core::ProtobufEnum;
use rustc_serialize::{Decoder, Decodable};
use rustc_serialize::json::{Json, ToJson};

pub use message::net::*;

pub fn err<M: Into<String>>(code: ErrCode, msg: M) -> NetError {
    let mut err = NetError::new();
    err.set_code(code);
    err.set_msg(msg.into());
    err
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[err: {:?}, msg: {}]", self.get_code(), self.get_msg())
    }
}

impl error::Error for NetError {
    fn description(&self) -> &str {
        match self.get_code() {
            ErrCode::BUG => "An unexpected error occurred.",
            ErrCode::TIMEOUT => "Network timeout.",
            ErrCode::REMOTE_REJECTED => "Remote server rejected request.",
            ErrCode::BAD_REMOTE_REPLY => "Remote server returned a bad response.",
            ErrCode::ENTITY_NOT_FOUND => "Entity not found in datastore.",
            ErrCode::NO_SHARD => "Shard not available.",
            ErrCode::ACCESS_DENIED => "Operation not allowed by authenticated.",
            ErrCode::SESSION_EXPIRED => "Session expired, user should re-authenticate.",
            ErrCode::ENTITY_CONFLICT => "Entity already exists in datastore.",
            ErrCode::ZMQ => "Network error.",
            ErrCode::DATA_STORE => "Database error.",
            ErrCode::AUTH_SCOPE => "Additional authorization scope(s) required for action.",
            ErrCode::WORKSPACE_SETUP => "Worker runner unable to setup build workspace.",
            ErrCode::SECRET_KEY_FETCH => "Worker runner unable to fetch secret key for origin.",
            ErrCode::SECRET_KEY_IMPORT => "Worker runner unable to import secret key for origin.",
            ErrCode::VCS_CLONE => "Worker runner unable to retrieve project source to build.",
            ErrCode::BUILD => "Worker runner failed to build project.",
            ErrCode::POST_PROCESSOR => "One or more post processing step failed in Worker runner.",
        }
    }
}

impl Decodable for NetError {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        d.read_struct("NetError", 2, |d| {
            let mut err = NetError::new();
            let code: i32 = try!(d.read_struct_field("code", 0, |d| Decodable::decode(d)));
            err.set_code(ErrCode::from_i32(code).unwrap());
            err.set_msg(try!(d.read_struct_field("msg", 1, |d| Decodable::decode(d))));
            Ok(err)
        })
    }
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

#[cfg(test)]
mod tests {
    use protobuf::Message;
    use super::*;
    use rustc_serialize::json::{self, ToJson};

    #[test]
    fn message_id() {
        let msg = Ping::new();
        assert_eq!(msg.descriptor().name(), "Ping");
    }

    #[test]
    fn net_err_json_serialization() {
        let err = err(ErrCode::ACCESS_DENIED, "net:1:err");
        let encoded = json::encode(&err.to_json()).unwrap();
        let decoded: NetError = json::decode(&encoded).unwrap();
        assert_eq!(decoded.get_code(), ErrCode::ACCESS_DENIED);
        assert_eq!(decoded.get_msg(), "net:1:err");
    }
}
