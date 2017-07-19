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

//! Leader election.
//!
//! This module does leader election for services. It consists of an `Election` that implements
//! `Rumor`, and uses a variant of the [Bully
//! Algorithm](https://en.wikipedia.org/wiki/Bully_algorithm) to select the leader.
//!
//! It uses a particular variant I think of as the "highlander" model. A given election will
//! devolve to a single, universal rumor, which when it is received by the winner will result in
//! the election finishing. There can, in the end, be only one.

use std::ops::{Deref, DerefMut};

use habitat_core::service::ServiceGroup;
use protobuf::{self, Message, RepeatedField};

pub use message::swim::Election_Status;
use error::Result;
use message::swim::{Election as ProtoElection, Rumor as ProtoRumor, Rumor_Type as ProtoRumor_Type};
use rumor::Rumor;

#[derive(Debug, Clone, Serialize)]
pub struct Election(ProtoRumor);

impl From<ProtoRumor> for Election {
    fn from(pr: ProtoRumor) -> Election {
        Election(pr)
    }
}

impl From<Election> for ProtoRumor {
    fn from(election: Election) -> ProtoRumor {
        election.0
    }
}

impl Deref for Election {
    type Target = ProtoElection;

    fn deref(&self) -> &ProtoElection {
        self.0.get_election()
    }
}

impl DerefMut for Election {
    fn deref_mut(&mut self) -> &mut ProtoElection {
        self.0.mut_election()
    }
}

impl Election {
    /// Create a new election, voting for the given member id, for the given service group, and
    /// with the given suitability.
    pub fn new<S1: Into<String>>(
        member_id: S1,
        service_group: ServiceGroup,
        suitability: u64,
    ) -> Election {
        let mut rumor = ProtoRumor::new();
        let from_id = member_id.into();
        let real_member_id = from_id.clone();
        let vote_member_id = from_id.clone();
        rumor.set_from_id(from_id);
        rumor.set_field_type(ProtoRumor_Type::Election);

        let mut proto = ProtoElection::new();
        proto.set_member_id(real_member_id);
        proto.set_service_group(format!("{}", service_group));
        proto.set_term(0);
        proto.set_suitability(suitability);
        proto.set_status(Election_Status::Running);
        proto.set_votes(RepeatedField::from_vec(vec![vote_member_id]));

        rumor.set_election(proto);
        Election(rumor)
    }

    /// Insert a vote for the election.
    pub fn insert_vote(&mut self, member_id: &str) {
        if !self.get_votes().contains(&String::from(member_id)) {
            self.mut_votes().push(String::from(member_id));
        }
    }

    /// Steal all the votes from another election for ourselves.
    pub fn steal_votes(&mut self, other: &mut Election) {
        for x in other.mut_votes().iter() {
            self.insert_vote(x);
        }
    }

    /// Sets the status of the election to "running".
    pub fn running(&mut self) {
        self.set_status(Election_Status::Running);
    }

    /// Sets the status of the election to "finished"
    pub fn finish(&mut self) {
        self.set_status(Election_Status::Finished);
    }

    /// Sets the status of the election to "NoQuorum"
    pub fn no_quorum(&mut self) {
        self.set_status(Election_Status::NoQuorum);
    }

    /// Returns true if the election is finished.
    pub fn is_finished(&self) -> bool {
        self.get_status() == Election_Status::Finished
    }
}

impl PartialEq for Election {
    /// We ignore id in equality checking, because we only have one per service group
    fn eq(&self, other: &Election) -> bool {
        self.get_service_group() == other.get_service_group() &&
            self.get_member_id() == other.get_member_id() &&
            self.get_suitability() == other.get_suitability() &&
            self.get_votes() == other.get_votes() &&
            self.get_status() == other.get_status() && self.get_term() == other.get_term()
    }
}

impl Rumor for Election {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let rumor = protobuf::parse_from_bytes::<ProtoRumor>(bytes)?;
        Ok(Election::from(rumor))
    }

    /// Updates this election based on the contents of another election.
    fn merge(&mut self, mut other: Election) -> bool {
        if *self == other {
            // If we are the same object, just return false
            // println!("Equal: {:?} {:?}", self, other);
            false
        } else if other.get_term() >= self.get_term() &&
                   other.get_status() == Election_Status::Finished
        {
            // If the new rumors term is bigger or equal to ours, and it has a leader, we take it as
            // the leader and move on.
            *self = other;
            true
        } else if other.get_term() == self.get_term() &&
                   self.get_status() == Election_Status::Finished
        {
            // If the terms are equal, and we are finished, then we drop the other side on the
            // floor
            false
        } else if self.get_term() > other.get_term() {
            // If the rumor we got has a term that's lower than ours, keep sharing our rumor no
            // matter what term they are on.
            true
        } else if self.get_suitability() > other.get_suitability() {
            // If we are more suitable than the other side, we want to steal
            // the other sides votes, and keep sharing.
            // println!("Self suitable: {:?} {:?}", self, other);
            self.steal_votes(&mut other);
            true
        } else if other.get_suitability() > self.get_suitability() {
            // If the other side is more suitable than we are, we want to add our votes
            // to its tally, then take it as our rumor.
            // println!("Other suitable: {:?} {:?}", self, other);
            other.steal_votes(self);
            *self = other;
            true
        } else {
            if self.get_member_id() >= other.get_member_id() {
                // If we are equally suitable, and our id sorts before the other, we want to steal
                // it's votes, and mark it as having voted for us.
                // println!("Self sorts equal or greater than other: {:?} {:?}",
                //         self,
                //         other);
                self.steal_votes(&mut other);
                true
            } else {
                // If we are equally suitable, but the other id sorts before ours, then we give it
                // our votes, vote for it ourselves, and spread it as the new rumor
                // println!("Self sorts less than other: {:?} {:?}", self, other);
                other.steal_votes(self);
                *self = other;
                true
            }
        }
    }

    /// We are the Election rumor!
    fn kind(&self) -> ProtoRumor_Type {
        ProtoRumor_Type::Election
    }

    /// There can be only
    fn id(&self) -> &str {
        "election"
    }

    fn key(&self) -> &str {
        self.get_service_group()
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.0.write_to_bytes()?)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ElectionUpdate(Election);

impl ElectionUpdate {
    pub fn new<S1: Into<String>>(
        member_id: S1,
        service_group: ServiceGroup,
        suitability: u64,
    ) -> ElectionUpdate {
        let mut election = Election::new(member_id, service_group, suitability);
        election.0.set_field_type(ProtoRumor_Type::ElectionUpdate);
        ElectionUpdate(election)
    }
}

impl Deref for ElectionUpdate {
    type Target = Election;

    fn deref(&self) -> &Election {
        &self.0
    }
}

impl DerefMut for ElectionUpdate {
    fn deref_mut(&mut self) -> &mut Election {
        &mut self.0
    }
}

impl From<ProtoRumor> for ElectionUpdate {
    fn from(pr: ProtoRumor) -> ElectionUpdate {
        let mut election = Election::from(pr);
        election.0.set_field_type(ProtoRumor_Type::ElectionUpdate);
        ElectionUpdate(election)
    }
}

impl From<ElectionUpdate> for ProtoRumor {
    fn from(election: ElectionUpdate) -> ProtoRumor {
        (election.0).0
    }
}

impl Rumor for ElectionUpdate {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let rumor = Election::from_bytes(bytes)?;
        Ok(ElectionUpdate(rumor))
    }

    fn merge(&mut self, other: ElectionUpdate) -> bool {
        self.0.merge(other.0)
    }

    fn kind(&self) -> ProtoRumor_Type {
        ProtoRumor_Type::ElectionUpdate
    }

    fn id(&self) -> &str {
        "election"
    }

    fn key(&self) -> &str {
        self.0.key()
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>> {
        self.0.write_to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use rumor::Rumor;
    use rumor::election::Election;
    use habitat_core::service::ServiceGroup;

    fn create_election(member_id: &str, suitability: u64) -> Election {
        Election::new(
            member_id,
            ServiceGroup::new(None, "tdep", "prod", None).unwrap(),
            suitability,
        )
    }

    #[test]
    fn merge_two_identical_elections_returns_false() {
        let mut e1 = create_election("a", 0);
        let e2 = e1.clone();
        assert_eq!(e1.merge(e2), false);
    }

    #[test]
    fn merge_four_one_higher_suitability() {
        let mut e1 = create_election("a", 0);
        let e2 = create_election("b", 0);
        let e3 = create_election("c", 1);
        let e4 = create_election("d", 0);
        assert_eq!(e1.merge(e2), true);
        assert_eq!(e1.merge(e3), true);
        assert_eq!(e1.merge(e4), true);
        assert_eq!(e1.get_member_id(), "c");
        assert_eq!(e1.get_votes().len(), 4);
    }

    #[test]
    fn merge_four() {
        let mut e1 = create_election("a", 0);
        let e2 = create_election("b", 0);
        let e3 = create_election("c", 0);
        let e4 = create_election("d", 0);
        assert_eq!(e1.merge(e2), true);
        assert_eq!(e1.merge(e3), true);
        assert_eq!(e1.merge(e4), true);
        assert_eq!(e1.get_member_id(), "d");
        assert_eq!(e1.get_votes().len(), 4);
    }
}
