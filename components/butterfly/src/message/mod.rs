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
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
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
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("election", 6));
        try!(strukt.serialize_field("member_id", self.get_member_id()));
        try!(strukt.serialize_field("service_group", self.get_service_group()));
        try!(strukt.serialize_field("term", &self.get_term()));
        try!(strukt.serialize_field("suitability", &self.get_suitability()));
        try!(strukt.serialize_field("status", &self.get_status()));
        try!(strukt.serialize_field("votes", self.get_votes()));
        strukt.end()
    }
}

impl Serialize for swim::Member {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("member", 6));
        try!(strukt.serialize_field("id", self.get_id()));
        try!(strukt.serialize_field("incarnation", &self.get_incarnation()));
        try!(strukt.serialize_field("address", self.get_address()));
        try!(strukt.serialize_field("swim_port", &self.get_swim_port()));
        try!(strukt.serialize_field("gossip_port", &self.get_gossip_port()));
        try!(strukt.serialize_field("persistent", &self.get_persistent()));
        strukt.end()
    }
}

impl Serialize for swim::Membership {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("membership", 2));
        try!(strukt.serialize_field("member", self.get_member()));
        try!(strukt.serialize_field("health", &self.get_health()));
        strukt.end()
    }
}

impl Serialize for swim::Rumor {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("rumor", 8));
        try!(strukt.serialize_field("type", &self.get_field_type()));
        try!(strukt.serialize_field("tag", self.get_tag()));
        try!(strukt.serialize_field("from_id", self.get_from_id()));
        if self.has_member() {
            try!(strukt.serialize_field("member", self.get_member()));
        }
        if self.has_service() {
            try!(strukt.serialize_field("service", self.get_service()));
        }
        if self.has_service_config() {
            try!(strukt.serialize_field("service_config", self.get_service_config()));
        }
        if self.has_service_file() {
            try!(strukt.serialize_field("service_file", self.get_service_file()));
        }
        if self.has_election() {
            try!(strukt.serialize_field("election", self.get_election()));
        }
        strukt.end()
    }
}

impl Serialize for swim::Service {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("service", 11));
        let cfg = toml::from_slice(self.get_cfg()).unwrap_or(toml::value::Table::default());
        let sys = toml::from_slice(self.get_sys()).unwrap_or(SysInfo::default());
        try!(strukt.serialize_field("member_id", self.get_member_id()));
        try!(strukt.serialize_field("service_group", self.get_service_group()));
        try!(strukt.serialize_field("package", self.get_pkg()));
        try!(strukt.serialize_field("incarnation", &self.get_incarnation()));
        try!(strukt.serialize_field("cfg", &cfg));
        try!(strukt.serialize_field("sys", &sys));
        try!(strukt.serialize_field("initialized", &self.get_initialized()));
        strukt.end()
    }
}

impl Serialize for swim::ServiceConfig {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("service_config", 4));
        try!(strukt.serialize_field("service_group", self.get_service_group()));
        try!(strukt.serialize_field("incarnation", &self.get_incarnation()));
        try!(strukt.serialize_field("encrypted", &self.get_encrypted()));
        match str::from_utf8(self.get_config()) {
            Ok(c) => try!(strukt.serialize_field("config", c)),
            Err(_) => try!(strukt.serialize_field("config", self.get_config())),
        };
        strukt.end()
    }
}

impl Serialize for swim::ServiceFile {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("service_file", 5));
        try!(strukt.serialize_field("service_group", self.get_service_group()));
        try!(strukt.serialize_field("incarnation", &self.get_incarnation()));
        try!(strukt.serialize_field("encrypted", &self.get_encrypted()));
        try!(strukt.serialize_field("filename", self.get_filename()));
        match str::from_utf8(self.get_body()) {
            Ok(c) => try!(strukt.serialize_field("body", c)),
            Err(_) => try!(strukt.serialize_field("body", self.get_body())),
        };
        strukt.end()
    }
}

impl Serialize for swim::Election_Status {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl Serialize for swim::Membership_Health {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl Serialize for swim::Rumor_Type {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u8(*self as u8)
    }
}
