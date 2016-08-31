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
use std::result;
use std::str::FromStr;

use protobuf::ProtobufEnum;
use rustc_serialize::{Decoder, Decodable, Encoder, Encodable};
use rustc_serialize::json::{Json, ToJson};

use message::{Persistable, Routable};
use sharding::InstaId;

pub use message::jobsrv::*;

#[derive(Debug)]
pub enum Error {
    BadJobState,
}

impl Into<Job> for JobSpec {
    fn into(mut self) -> Job {
        let mut job = Job::new();
        job.set_owner_id(self.get_owner_id());
        job.set_state(JobState::default());
        job.set_project(self.take_project());
        job
    }
}

impl Routable for JobSpec {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
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

impl Persistable for Job {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}
