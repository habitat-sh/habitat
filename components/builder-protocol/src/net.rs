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

#[cfg(test)]
mod tests {
    use protobuf::Message;
    use super::*;

    #[test]
    fn message_id() {
        let msg = Ping::new();
        assert_eq!(msg.descriptor().name(), "Ping");
    }
}
