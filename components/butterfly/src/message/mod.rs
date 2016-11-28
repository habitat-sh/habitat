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

pub mod swim;

use std::result;
use std::str;

use habitat_core::crypto::SymKey;
use rustc_serialize::{Encoder, Encodable};

use error::Result;
use message::swim::Wire;
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

impl Encodable for swim::Election {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("election", 6, |s| {
            try!(s.emit_struct_field("member_id", 0, |s| self.get_member_id().encode(s)));
            try!(s.emit_struct_field("service_group", 1, |s| self.get_service_group().encode(s)));
            try!(s.emit_struct_field("term", 2, |s| self.get_term().encode(s)));
            try!(s.emit_struct_field("suitability", 3, |s| self.get_suitability().encode(s)));
            try!(s.emit_struct_field("status", 4, |s| (self.get_status() as usize).encode(s)));
            try!(s.emit_struct_field("votes", 5, |s| self.get_votes().encode(s)));
            Ok(())
        }));
        Ok(())
    }
}

impl Encodable for swim::Member {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("member", 6, |s| {
            try!(s.emit_struct_field("id", 0, |s| self.get_id().encode(s)));
            try!(s.emit_struct_field("incarnation", 1, |s| self.get_incarnation().encode(s)));
            try!(s.emit_struct_field("address", 2, |s| self.get_address().encode(s)));
            try!(s.emit_struct_field("swim_port", 3, |s| self.get_swim_port().encode(s)));
            try!(s.emit_struct_field("gossip_port", 4, |s| self.get_gossip_port().encode(s)));
            try!(s.emit_struct_field("persistent", 5, |s| self.get_persistent().encode(s)));
            Ok(())
        }));
        Ok(())
    }
}

impl Encodable for swim::Membership {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("membership", 2, |s| {
            try!(s.emit_struct_field("member", 0, |s| self.get_member().encode(s)));
            try!(s.emit_struct_field("health", 1, |s| (self.get_health() as usize).encode(s)));
            Ok(())
        }));
        Ok(())
    }
}

impl Encodable for swim::Rumor {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("rumor", 8, |s| {
            try!(s.emit_struct_field("type", 0, |s| (self.get_field_type() as u8).encode(s)));
            try!(s.emit_struct_field("tag", 1, |s| self.get_tag().encode(s)));
            try!(s.emit_struct_field("from_id", 2, |s| self.get_from_id().encode(s)));
            if self.has_member() {
                try!(s.emit_struct_field("member", 3, |s| self.get_member().encode(s)));
            }
            if self.has_service() {
                try!(s.emit_struct_field("service", 4, |s| self.get_service().encode(s)));
            }
            if self.has_service_config() {
                try!(s.emit_struct_field("service_config",
                                         5,
                                         |s| self.get_service_config().encode(s)));
            }
            if self.has_service_file() {
                try!(s.emit_struct_field("service_file", 6, |s| self.get_service_file().encode(s)));
            }
            if self.has_election() {
                try!(s.emit_struct_field("election", 7, |s| self.get_election().encode(s)));
            }
            Ok(())
        }));
        Ok(())
    }
}

impl Encodable for swim::Service {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("service", 8, |s| {
            try!(s.emit_struct_field("member_id", 0, |s| self.get_member_id().encode(s)));
            try!(s.emit_struct_field("service_group", 1, |s| self.get_service_group().encode(s)));
            try!(s.emit_struct_field("incarnation", 2, |s| self.get_incarnation().encode(s)));
            try!(s.emit_struct_field("ip", 3, |s| self.get_ip().encode(s)));
            try!(s.emit_struct_field("hostname", 4, |s| self.get_hostname().encode(s)));
            try!(s.emit_struct_field("port", 5, |s| self.get_port().encode(s)));
            try!(s.emit_struct_field("exposes", 6, |s| self.get_exposes().encode(s)));
            try!(s.emit_struct_field("initialized", 7, |s| self.get_initialized().encode(s)));
            Ok(())
        }));
        Ok(())
    }
}

impl Encodable for swim::ServiceConfig {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("service_config", 4, |s| {
            try!(s.emit_struct_field("service_group", 0, |s| self.get_service_group().encode(s)));
            try!(s.emit_struct_field("incarnation", 1, |s| self.get_incarnation().encode(s)));
            try!(s.emit_struct_field("encrypted", 2, |s| self.get_encrypted().encode(s)));
            match str::from_utf8(self.get_config()) {
                Ok(c) => try!(s.emit_struct_field("config", 3, |s| c.encode(s))),
                Err(_) => try!(s.emit_struct_field("config", 3, |s| self.get_config().encode(s))),
            }
            Ok(())
        }));
        Ok(())
    }
}

impl Encodable for swim::ServiceFile {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_struct("service_file", 5, |s| {
            try!(s.emit_struct_field("service_group", 0, |s| self.get_service_group().encode(s)));
            try!(s.emit_struct_field("incarnation", 1, |s| self.get_incarnation().encode(s)));
            try!(s.emit_struct_field("encrypted", 2, |s| self.get_encrypted().encode(s)));
            try!(s.emit_struct_field("filename", 3, |s| self.get_filename().encode(s)));
            match str::from_utf8(self.get_body()) {
                Ok(c) => try!(s.emit_struct_field("body", 3, |s| c.encode(s))),
                Err(_) => try!(s.emit_struct_field("body", 3, |s| self.get_body().encode(s))),
            }
            Ok(())
        }));
        Ok(())
    }
}
