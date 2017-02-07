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

pub mod swim;

use std::result;
use std::str;

use habitat_core::crypto::SymKey;
use habitat_core::util;
use serde::{Serialize, Serializer};
use toml;

use error::Result;
use message::swim::Wire;
use rumor::service::SysInfo;
use protobuf::{self, Message};

pub fn generate_wire(payload: Vec<u8>, ring_key: &Option<SymKey>) -> Result<Vec<u8>> {
    let mut wire = Wire::new();
    if let Some(ref ring_key) = *ring_key {
        wire.set_encrypted(true);
        let (nonce, encrypted_payload) = try!(ring_key.encrypt(&payload));
        wire.set_nonce(nonce);
        wire.set_payload(encrypted_payload);
    } else {
        wire.set_payload(payload);
    }
    Ok(try!(wire.write_to_bytes()))
}

pub fn unwrap_wire(payload: &[u8], ring_key: &Option<SymKey>) -> Result<Vec<u8>> {
    let mut wire: Wire = try!(protobuf::parse_from_bytes(payload));
    if let Some(ref ring_key) = *ring_key {
        Ok(try!(ring_key.decrypt(wire.get_nonce(), wire.get_payload())))
    } else {
        Ok(wire.take_payload())
    }
}

impl Serialize for swim::Election {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("election", 6));
        try!(serializer.serialize_struct_elt(&mut state, "member_id", self.get_member_id()));
        try!(serializer.serialize_struct_elt(&mut state,
                                             "service_group", self.get_service_group()));
        try!(serializer.serialize_struct_elt(&mut state, "term", self.get_term()));
        try!(serializer.serialize_struct_elt(&mut state, "suitability", self.get_suitability()));
        try!(serializer.serialize_struct_elt(&mut state, "status", self.get_status()));
        try!(serializer.serialize_struct_elt(&mut state, "votes", self.get_votes()));
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for swim::Member {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("member", 6));
        try!(serializer.serialize_struct_elt(&mut state, "id", self.get_id()));
        try!(serializer.serialize_struct_elt(&mut state, "incarnation", self.get_incarnation()));
        try!(serializer.serialize_struct_elt(&mut state, "address", self.get_address()));
        try!(serializer.serialize_struct_elt(&mut state, "swim_port", self.get_swim_port()));
        try!(serializer.serialize_struct_elt(&mut state, "gossip_port", self.get_gossip_port()));
        try!(serializer.serialize_struct_elt(&mut state, "persistent", self.get_persistent()));
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for swim::Membership {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("membership", 2));
        try!(serializer.serialize_struct_elt(&mut state, "member", self.get_member()));
        try!(serializer.serialize_struct_elt(&mut state, "health", self.get_health()));
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for swim::Rumor {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("rumor", 8));
        try!(serializer.serialize_struct_elt(&mut state, "type", self.get_field_type()));
        try!(serializer.serialize_struct_elt(&mut state, "tag", self.get_tag()));
        try!(serializer.serialize_struct_elt(&mut state, "from_id", self.get_from_id()));
        if self.has_member() {
            try!(serializer.serialize_struct_elt(&mut state, "member", self.get_member()));
        }
        if self.has_service() {
            try!(serializer.serialize_struct_elt(&mut state, "service", self.get_service()));
        }
        if self.has_service_config() {
            try!(serializer.serialize_struct_elt(&mut state,
                                                 "service_config",
                                                 self.get_service_config()));
        }
        if self.has_service_file() {
            try!(serializer.serialize_struct_elt(&mut state,
                                                 "service_file",
                                                 self.get_service_file()));
        }
        if self.has_election() {
            try!(serializer.serialize_struct_elt(&mut state, "election", self.get_election()));
        }
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for swim::Service {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("service", 11));
        let cfg = util::toml::table_from_bytes(self.get_cfg()).unwrap_or(toml::Table::default());
        let sys = str::from_utf8(self.get_sys())
            .ok()
            .and_then(|v| toml::decode_str(v))
            .unwrap_or(SysInfo::default());
        try!(serializer.serialize_struct_elt(&mut state, "member_id", self.get_member_id()));
        try!(serializer.serialize_struct_elt(&mut state,
                                             "service_group",
                                             self.get_service_group()));
        try!(serializer.serialize_struct_elt(&mut state, "package", self.get_pkg()));
        try!(serializer.serialize_struct_elt(&mut state, "incarnation", self.get_incarnation()));
        try!(serializer.serialize_struct_elt(&mut state, "cfg", &cfg));
        try!(serializer.serialize_struct_elt(&mut state, "sys", &sys));
        try!(serializer.serialize_struct_elt(&mut state, "initialized", self.get_initialized()));
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for swim::ServiceConfig {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("service_config", 4));
        try!(serializer.serialize_struct_elt(&mut state,
                                             "service_group",
                                             self.get_service_group()));
        try!(serializer.serialize_struct_elt(&mut state, "incarnation", self.get_incarnation()));
        try!(serializer.serialize_struct_elt(&mut state, "encrypted", self.get_encrypted()));
        match str::from_utf8(self.get_config()) {
            Ok(c) => try!(serializer.serialize_struct_elt(&mut state, "config", c)),
            Err(_) => {
                try!(serializer.serialize_struct_elt(&mut state, "config", self.get_config()))
            }
        };
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for swim::ServiceFile {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("service_file", 5));
        try!(serializer.serialize_struct_elt(&mut state,
                                             "service_group",
                                             self.get_service_group()));
        try!(serializer.serialize_struct_elt(&mut state, "incarnation", self.get_incarnation()));
        try!(serializer.serialize_struct_elt(&mut state, "encrypted", self.get_encrypted()));
        try!(serializer.serialize_struct_elt(&mut state, "filename", self.get_filename()));
        match str::from_utf8(self.get_body()) {
            Ok(c) => try!(serializer.serialize_struct_elt(&mut state, "body", c)),
            Err(_) => try!(serializer.serialize_struct_elt(&mut state, "body", self.get_body())),
        };
        serializer.serialize_struct_end(state)
    }
}

impl Serialize for swim::Election_Status {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl Serialize for swim::Membership_Health {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl Serialize for swim::Rumor_Type {
    fn serialize<S>(&self, serializer: &mut S) -> result::Result<(), S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}
