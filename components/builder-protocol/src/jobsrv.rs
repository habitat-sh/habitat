// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::collections::BTreeMap;
use std::result;
use std::str::FromStr;

use protobuf::ProtobufEnum;
use rustc_serialize::{Decoder, Decodable, Encoder, Encodable};
use rustc_serialize::json::{Json, ToJson};

use message::Routable;
use sharding::InstaId;

pub use message::jobsrv::*;

#[derive(Debug)]
pub enum Error {
    BadJobState,
}

impl Routable for JobCreate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_owner_id().to_string())
    }
}

impl Routable for JobGet {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_id()))
    }
}

impl ToJson for Job {
    fn to_json(&self) -> Json {
        let mut m = BTreeMap::new();
        m.insert("id".to_string(), self.get_id().to_json());
        m.insert("state".to_string(), self.get_state().value().to_json());
        Json::Object(m)
    }
}

impl Default for JobState {
    fn default() -> JobState {
        JobState::Pending
    }
}

impl Encodable for JobState {
    fn encode<S: Encoder>(&self, s: &mut S) -> result::Result<(), S::Error> {
        try!(s.emit_i32(self.value()));
        Ok(())
    }
}

impl Decodable for JobState {
    fn decode<D: Decoder>(d: &mut D) -> result::Result<Self, D::Error> {
        let val = try!(d.read_i32());
        let state = JobState::from_i32(val).unwrap_or_default();
        Ok(state)
    }
}

impl FromStr for JobState {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value.parse() {
            Ok(id) => {
                if let Some(state) = JobState::from_i32(id) {
                    Ok(state)
                } else {
                    Err(Error::BadJobState)
                }
            }
            Err(_) => Err(Error::BadJobState),
        }
    }
}
