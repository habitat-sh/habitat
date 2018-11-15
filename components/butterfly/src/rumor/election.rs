// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use error::{Error, Result};
use protocol::newscast::Rumor as ProtoRumor;
pub use protocol::newscast::{election::Status as ElectionStatus, Election as ProtoElection};
use protocol::{self, newscast, FromProto};
use rumor::{Rumor, RumorPayload, RumorType};

// The default implementations leverage ElectionUpdate's Deref -> Election behavior, but this
// generates a warning. In practice, it's fine:
// See https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=162d64d74390df6a3a8bb815cd3a2c73
#[allow(unconditional_recursion)]
pub trait ElectionRumor {
    fn member_id(&self) -> &str {
        self.member_id()
    }

    fn is_finished(&self) -> bool {
        self.is_finished()
    }

    fn term(&self) -> u64 {
        self.term()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Election {
    pub from_id: String,
    pub member_id: String,
    pub service_group: String,
    pub term: u64,
    pub suitability: u64,
    pub status: ElectionStatus,
    pub votes: Vec<String>,
}

impl Election {
    /// Create a new election, voting for the given member id, for the given service group, and
    /// with the given suitability.
    pub fn new<S1>(member_id: S1, service_group: &str, suitability: u64) -> Election
    where
        S1: Into<String>,
    {
        let from_id = member_id.into();
        Election {
            from_id: from_id.clone(),
            member_id: from_id.clone(),
            service_group: service_group.into(),
            term: 0,
            suitability: suitability,
            status: ElectionStatus::Running,
            votes: vec![from_id],
        }
    }

    /// Insert a vote for the election.
    pub fn insert_vote(&mut self, member_id: &str) {
        if !self.votes.contains(&String::from(member_id)) {
            self.votes.push(String::from(member_id));
        }
    }

    /// Steal all the votes from another election for ourselves.
    pub fn steal_votes(&mut self, other: &mut Election) {
        for x in other.votes.iter() {
            self.insert_vote(x);
        }
    }

    /// Sets the status of the election to "running".
    pub fn running(&mut self) {
        self.status = ElectionStatus::Running;
    }

    /// Sets the status of the election to "finished"
    pub fn finish(&mut self) {
        self.status = ElectionStatus::Finished;
    }

    /// Sets the status of the election to "NoQuorum"
    pub fn no_quorum(&mut self) {
        self.status = ElectionStatus::NoQuorum;
    }
}

impl ElectionRumor for Election {
    fn member_id(&self) -> &str {
        &self.member_id
    }

    fn is_finished(&self) -> bool {
        self.status == ElectionStatus::Finished
    }

    fn term(&self) -> u64 {
        self.term
    }
}

impl PartialEq for Election {
    /// We ignore id in equality checking, because we only have one per service group
    fn eq(&self, other: &Election) -> bool {
        self.service_group == other.service_group
            && self.member_id == other.member_id
            && self.suitability == other.suitability
            && self.votes == other.votes
            && self.status == other.status
            && self.term == other.term
    }
}

impl protocol::Message<ProtoRumor> for Election {}

impl FromProto<ProtoRumor> for Election {
    fn from_proto(rumor: ProtoRumor) -> Result<Self> {
        let payload = match rumor.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::Election(payload) => payload,
            _ => panic!("from-bytes election"),
        };
        let from_id = rumor.from_id.ok_or(Error::ProtocolMismatch("from-id"))?;
        Ok(Election {
            from_id: from_id.clone(),
            member_id: from_id.clone(),
            service_group: payload
                .service_group
                .ok_or(Error::ProtocolMismatch("service-group"))?,
            term: payload.term.unwrap_or(0),
            suitability: payload.suitability.unwrap_or(0),
            status: payload
                .status
                .and_then(ElectionStatus::from_i32)
                .unwrap_or(ElectionStatus::Running),
            votes: payload.votes,
        })
    }
}

impl From<Election> for newscast::Election {
    fn from(value: Election) -> Self {
        newscast::Election {
            member_id: Some(value.member_id),
            service_group: Some(value.service_group.to_string()),
            term: Some(value.term),
            suitability: Some(value.suitability),
            status: Some(value.status as i32),
            votes: value.votes,
        }
    }
}

impl Rumor for Election {
    /// Updates this election based on the contents of another election.
    fn merge(&mut self, mut other: Election) -> bool {
        debug!("merging stored {:?}", self);
        debug!(" with received {:?}", other);
        if *self == other {
            debug!("stored and received rumors are equal; nothing to do");
            false
        } else if other.term >= self.term && other.status == ElectionStatus::Finished {
            debug!("received is finished and represents a newer term; replace stored and share");
            *self = other;
            true
        } else if other.term == self.term && self.status == ElectionStatus::Finished {
            debug!("stored rumor is finished and received rumor is for same term; nothing to do");
            false
        } else if self.term > other.term {
            debug!("stored rumor represents a newer term than received; keep sharing it");
            true
        } else if self.suitability > other.suitability {
            debug!("stored rumor is more suitable; take received rumor's votes and share");
            self.steal_votes(&mut other);
            true
        } else if other.suitability > self.suitability {
            debug!("received rumor is more suitable; take stored rumor's votes, replace stored and share");
            other.steal_votes(self);
            *self = other;
            true
        } else {
            if self.member_id >= other.member_id {
                debug!("stored rumor wins tie-breaker; take received rumor's votes and share");
                self.steal_votes(&mut other);
                true
            } else {
                debug!("received rumor wins tie-breaker; take stored rumor's votes, replace stored and share");
                other.steal_votes(self);
                *self = other;
                true
            }
        }
    }

    /// We are the Election rumor!
    fn kind(&self) -> RumorType {
        RumorType::Election
    }

    /// There can be only
    fn id(&self) -> &str {
        "election"
    }

    fn key(&self) -> &str {
        self.service_group.as_ref()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ElectionUpdate(Election);

impl ElectionUpdate {
    pub fn new<S1>(member_id: S1, service_group: &str, suitability: u64) -> ElectionUpdate
    where
        S1: Into<String>,
    {
        let election = Election::new(member_id, service_group, suitability);
        ElectionUpdate(election)
    }
}

impl ElectionRumor for ElectionUpdate {}

impl Deref for ElectionUpdate {
    type Target = Election;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElectionUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Election> for ElectionUpdate {
    fn from(other: Election) -> Self {
        ElectionUpdate(other)
    }
}

impl protocol::Message<ProtoRumor> for ElectionUpdate {}

impl FromProto<ProtoRumor> for ElectionUpdate {
    fn from_proto(rumor: ProtoRumor) -> Result<Self> {
        Ok(ElectionUpdate(Election::from_proto(rumor)?))
    }
}

impl From<ElectionUpdate> for newscast::Election {
    fn from(value: ElectionUpdate) -> Self {
        value.0.into()
    }
}

impl Rumor for ElectionUpdate {
    fn merge(&mut self, other: ElectionUpdate) -> bool {
        self.0.merge(other.0)
    }

    fn kind(&self) -> RumorType {
        RumorType::ElectionUpdate
    }

    fn id(&self) -> &str {
        "election"
    }

    fn key(&self) -> &str {
        self.0.key()
    }
}

#[cfg(test)]
mod tests {
    use habitat_core::service::ServiceGroup;
    use rumor::{
        election::{Election, ElectionUpdate},
        Rumor, RumorStore,
    };

    fn create_election_rumor_store() -> RumorStore<Election> {
        RumorStore::default()
    }

    fn create_election_update_rumor_store() -> RumorStore<ElectionUpdate> {
        RumorStore::default()
    }

    fn create_election(member_id: &str, suitability: u64) -> Election {
        Election::new(
            member_id,
            &ServiceGroup::new(None, "tdep", "prod", None).unwrap(),
            suitability,
        )
    }

    fn create_election_update(member_id: &str, suitability: u64) -> ElectionUpdate {
        ElectionUpdate::new(
            member_id,
            &ServiceGroup::new(None, "tdep", "prod", None).unwrap(),
            suitability,
        )
    }

    #[test]
    fn only_the_latest_election_is_kept() {
        let rs = create_election_rumor_store();
        let e1 = create_election("member_1", 1);
        let e2 = create_election("member_2", 2);
        rs.insert(e1);
        rs.insert(e2);

        let list = rs.list.read().expect("Rumor store lock poisoned");
        assert_eq!(list.len(), 1); // because we only have 1 service group

        let sub_list = list.get("tdep.prod").unwrap();
        assert_eq!(sub_list.len(), 1); // because only the latest election is kept
        assert!(sub_list.get("election").is_some());
    }

    #[test]
    fn only_the_latest_election_update_is_kept() {
        let rs = create_election_update_rumor_store();
        let e1 = create_election_update("member_1", 1);
        let e2 = create_election_update("member_2", 2);
        rs.insert(e1);
        rs.insert(e2);

        let list = rs.list.read().expect("Rumor store lock poisoned");
        assert_eq!(list.len(), 1); // because we only have 1 service group

        let sub_list = list.get("tdep.prod").unwrap();
        assert_eq!(sub_list.len(), 1); // because only the latest election is kept
        assert!(sub_list.get("election").is_some());
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
        assert_eq!(e1.member_id, "c");
        assert_eq!(e1.votes.len(), 4);
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
        assert_eq!(e1.member_id, "d");
        assert_eq!(e1.votes.len(), 4);
    }
}
