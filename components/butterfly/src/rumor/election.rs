//! Leader election.
//!
//! This module does leader election for services. It consists of an `Election` that implements
//! `Rumor`, and uses a variant of the [Bully
//! Algorithm](https://en.wikipedia.org/wiki/Bully_algorithm) to select the leader.
//!
//! It uses a particular variant I think of as the "highlander" model. A given election will
//! devolve to a single, universal rumor, which when it is received by the winner will result in
//! the election finishing. There can, in the end, be only one.

pub use crate::protocol::newscast::{election::Status as ElectionStatus,
                                    Election as ProtoElection};
use crate::{error::{Error,
                    Result},
            protocol::{self,
                       newscast::{self,
                                  Rumor as ProtoRumor},
                       FromProto},
            rumor::{ConstIdRumor,
                    Rumor,
                    RumorPayload,
                    RumorType}};
use std::{fmt,
          ops::{Deref,
                DerefMut}};

pub trait ElectionRumor: ConstIdRumor {
    fn member_id(&self) -> &str;

    fn is_finished(&self) -> bool;

    fn term(&self) -> u64;
}

pub type Term = u64;

#[derive(Debug, Clone, Serialize)]
pub struct Election {
    pub member_id:     String,
    pub service_group: String,
    pub term:          u64,
    pub suitability:   u64,
    pub status:        ElectionStatus,
    pub votes:         Vec<String>,
}

impl fmt::Display for Election {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "Election m/{} sg/{}, t/{}, su/{}, st/{:?}",
               self.member_id, self.service_group, self.term, self.suitability, self.status)
    }
}

impl Election {
    /// Create a new election, voting for the given member id, for the given service group, and
    /// with the given suitability.
    pub fn new<S1>(member_id: S1,
                   service_group: &str,
                   term: u64,
                   suitability: u64,
                   has_quorum: bool)
                   -> Election
        where S1: Into<String>
    {
        let from_id = member_id.into();
        Election { member_id: from_id.clone(),
                   service_group: service_group.into(),
                   term,
                   suitability,
                   status: if has_quorum {
                       ElectionStatus::Running
                   } else {
                       ElectionStatus::NoQuorum
                   },
                   votes: vec![from_id] }
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
    pub fn running(&mut self) { self.status = ElectionStatus::Running; }

    /// Sets the status of the election to "finished"
    pub fn finish(&mut self) { self.status = ElectionStatus::Finished; }

    /// Sets the status of the election to "NoQuorum"
    pub fn no_quorum(&mut self) { self.status = ElectionStatus::NoQuorum; }
}

impl ElectionRumor for Election {
    fn member_id(&self) -> &str { &self.member_id }

    fn is_finished(&self) -> bool { self.status == ElectionStatus::Finished }

    fn term(&self) -> u64 { self.term }
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

impl protocol::Message<ProtoRumor> for Election {
    const MESSAGE_ID: &'static str = "Election";
}

impl FromProto<ProtoRumor> for Election {
    fn from_proto(rumor: ProtoRumor) -> Result<Self> {
        let payload = match rumor.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::Election(payload) => payload,
            _ => panic!("from-bytes election"),
        };
        let from_id = rumor.from_id.ok_or(Error::ProtocolMismatch("from-id"))?;
        Ok(Election { member_id:     from_id,
                      service_group: payload.service_group
                                            .ok_or(Error::ProtocolMismatch("service-group"))?,
                      term:          payload.term.unwrap_or(0),
                      suitability:   payload.suitability.unwrap_or(0),
                      status:        payload.status
                                            .and_then(ElectionStatus::from_i32)
                                            .unwrap_or(ElectionStatus::Running),
                      votes:         payload.votes, })
    }
}

impl From<Election> for newscast::Election {
    fn from(value: Election) -> Self {
        newscast::Election { member_id:     Some(value.member_id),
                             service_group: Some(value.service_group.to_string()),
                             term:          Some(value.term),
                             suitability:   Some(value.suitability),
                             status:        Some(value.status as i32),
                             votes:         value.votes, }
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
            debug!("received rumor is more suitable; take stored rumor's votes, replace stored \
                    and share");
            other.steal_votes(self);
            *self = other;
            true
        } else if self.member_id >= other.member_id {
            debug!("stored rumor wins tie-breaker; take received rumor's votes and share");
            self.steal_votes(&mut other);
            true
        } else {
            debug!("received rumor wins tie-breaker; take stored rumor's votes, replace stored \
                    and share");
            other.steal_votes(self);
            *self = other;
            true
        }
    }

    fn kind(&self) -> RumorType { RumorType::Election }

    fn id(&self) -> &str { Self::const_id() }

    fn key(&self) -> &str { self.service_group.as_ref() }
}

impl ConstIdRumor for Election {
    fn const_id() -> &'static str { "election" }
}

#[derive(Debug, Clone, Serialize)]
pub struct ElectionUpdate(Election);

impl fmt::Display for ElectionUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "ElectionUpdate m/{} sg/{}, t/{}, su/{}, st/{:?}",
               self.0.member_id,
               self.0.service_group,
               self.0.term,
               self.0.suitability,
               self.0.status)
    }
}

impl ElectionUpdate {
    pub fn new<S1>(member_id: S1,
                   service_group: &str,
                   term: u64,
                   suitability: u64,
                   has_quorum: bool)
                   -> ElectionUpdate
        where S1: Into<String>
    {
        let election = Election::new(member_id, service_group, term, suitability, has_quorum);
        ElectionUpdate(election)
    }
}

impl ElectionRumor for ElectionUpdate {
    fn member_id(&self) -> &str { &self.member_id }

    fn is_finished(&self) -> bool { self.status == ElectionStatus::Finished }

    fn term(&self) -> u64 { self.term }
}

impl Deref for ElectionUpdate {
    type Target = Election;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ElectionUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<Election> for ElectionUpdate {
    fn from(other: Election) -> Self { ElectionUpdate(other) }
}

impl protocol::Message<ProtoRumor> for ElectionUpdate {
    const MESSAGE_ID: &'static str = "ElectionUpdate";
}

impl FromProto<ProtoRumor> for ElectionUpdate {
    fn from_proto(rumor: ProtoRumor) -> Result<Self> {
        Ok(ElectionUpdate(Election::from_proto(rumor)?))
    }
}

impl From<ElectionUpdate> for newscast::Election {
    fn from(value: ElectionUpdate) -> Self { value.0.into() }
}

impl Rumor for ElectionUpdate {
    fn merge(&mut self, other: ElectionUpdate) -> bool { self.0.merge(other.0) }

    fn kind(&self) -> RumorType { RumorType::ElectionUpdate }

    fn id(&self) -> &str { Self::const_id() }

    fn key(&self) -> &str { self.0.key() }
}

impl ConstIdRumor for ElectionUpdate {
    fn const_id() -> &'static str { "election" }
}

#[cfg(test)]
mod tests {
    use crate::rumor::{election::{Election,
                                  ElectionUpdate,
                                  Term},
                       ConstIdRumor as _,
                       Rumor,
                       RumorStore};
    use habitat_core::service::ServiceGroup;

    fn create_election_rumor_store() -> RumorStore<Election> { RumorStore::default() }

    fn create_election_update_rumor_store() -> RumorStore<ElectionUpdate> { RumorStore::default() }

    fn create_election(member_id: &str, suitability: u64) -> Election {
        Election::new(member_id,
                      &ServiceGroup::new("tdep", "prod", None).unwrap(),
                      Term::default(),
                      suitability,
                      true /* has_quorum */)
    }

    fn create_election_update(member_id: &str, suitability: u64) -> ElectionUpdate {
        ElectionUpdate::new(member_id,
                            &ServiceGroup::new("tdep", "prod", None).unwrap(),
                            Term::default(),
                            suitability,
                            true /* has_quorum */)
    }

    #[test]
    fn only_the_latest_election_is_kept() {
        let rs = create_election_rumor_store();
        let e1 = create_election("member_1", 1);
        let e2 = create_election("member_2", 2);
        rs.insert_rsw(e1);
        rs.insert_rsw(e2);

        let list = rs.lock_rsr();
        assert_eq!(list.len(), 1); // because we only have 1 service group

        let sub_list = list.get("tdep.prod").unwrap();
        assert_eq!(sub_list.len(), 1); // because only the latest election is kept
        assert!(sub_list.get(Election::const_id()).is_some());
    }

    #[test]
    fn only_the_latest_election_update_is_kept() {
        let rs = create_election_update_rumor_store();
        let e1 = create_election_update("member_1", 1);
        let e2 = create_election_update("member_2", 2);
        rs.insert_rsw(e1);
        rs.insert_rsw(e2);

        let list = rs.lock_rsr();
        assert_eq!(list.len(), 1); // because we only have 1 service group

        let sub_list = list.get("tdep.prod").unwrap();
        assert_eq!(sub_list.len(), 1); // because only the latest election is kept
        assert!(sub_list.get(ElectionUpdate::const_id()).is_some());
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
