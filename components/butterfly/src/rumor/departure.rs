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

//! The Departure rumor.
//!
//! Deaprture rumors declare that a given member has "departed" the gossip ring manually. When this
//! happens, we ensure that the member can no longer come back into the fold, unless an
//! administrator reverses the decision.

use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

use protobuf::{self, Message};

use error::Result;
use message::swim::{Departure as ProtoDeparture, Rumor as ProtoRumor,
                    Rumor_Type as ProtoRumor_Type};
use rumor::Rumor;

#[derive(Debug, Clone, Serialize)]
pub struct Departure(ProtoRumor);

impl PartialOrd for Departure {
    fn partial_cmp(&self, other: &Departure) -> Option<Ordering> {
        if self.get_member_id() != other.get_member_id() {
            None
        } else {
            Some(self.get_member_id().cmp(&other.get_member_id()))
        }
    }
}

impl PartialEq for Departure {
    fn eq(&self, other: &Departure) -> bool {
        self.get_member_id() == other.get_member_id()
    }
}

impl From<ProtoRumor> for Departure {
    fn from(pr: ProtoRumor) -> Departure {
        Departure(pr)
    }
}

impl From<Departure> for ProtoRumor {
    fn from(departure: Departure) -> ProtoRumor {
        departure.0
    }
}

impl Deref for Departure {
    type Target = ProtoDeparture;

    fn deref(&self) -> &ProtoDeparture {
        self.0.get_departure()
    }
}

impl DerefMut for Departure {
    fn deref_mut(&mut self) -> &mut ProtoDeparture {
        self.0.mut_departure()
    }
}

impl Departure {
    pub fn new<U>(member_id: U) -> Self
    where
        U: ToString,
    {
        let mut rumor = ProtoRumor::new();
        rumor.set_from_id(String::from("butterflyclient"));
        rumor.set_field_type(ProtoRumor_Type::Departure);

        let mut proto = ProtoDeparture::new();
        proto.set_member_id(member_id.to_string());
        rumor.set_departure(proto);
        Departure(rumor)
    }
}

impl Rumor for Departure {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let rumor = protobuf::parse_from_bytes::<ProtoRumor>(bytes)?;
        Ok(Departure::from(rumor))
    }

    fn merge(&mut self, other: Departure) -> bool {
        if *self >= other { false } else { true }
    }

    fn kind(&self) -> ProtoRumor_Type {
        ProtoRumor_Type::Departure
    }

    fn id(&self) -> &str {
        self.get_member_id()
    }

    fn key(&self) -> &str {
        "departure"
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>> {
        Ok(try!(self.0.write_to_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Departure;
    use rumor::Rumor;

    fn create_departure(member_id: &str) -> Departure {
        Departure::new(member_id)
    }

    #[test]
    fn identical_departures_are_equal() {
        let s1 = create_departure("mastodon");
        let s2 = create_departure("mastodon");
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn departures_with_different_member_ids_are_not_equal() {
        let s1 = create_departure("mastodon");
        let s2 = create_departure("limpbizkit");
        assert_eq!(s1, s2);
    }

    // Order
    #[test]
    fn departures_that_are_identical_are_equal_via_cmp() {
        let s1 = create_departure("adam");
        let s2 = create_departure("adam");
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Equal));
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_departure("mastodon");
        let s1_check = s1.clone();
        let s2 = create_departure("mastodon");
        assert_eq!(s1.merge(s2), false);
        assert_eq!(s1, s1_check);
    }
}
