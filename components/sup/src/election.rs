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

//! The election sub-protocol.
//!
//! This module handles doing leader election via the gossip layer. It has a struct called
//! 'ElectionList', which tracks elections according to service group, and `Election`, which is an
//! individual election.
//!
//! The way an Election works is this: every supervisor creates a new Election rumor, and sends it
//! out. It then recieves every other Election rumor, and overrides its own rumor with any inbound
//! rumor that is more 'suitable', where 'suitable' == "either has a higher suitability or sorts
//! its supervisor id first".
//!
//! The `leader` topology then evaluates these rumors.

use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Deref, DerefMut};

use uuid::Uuid;

use gossip::member::MemberId;
use gossip::rumor::{RumorList, Rumor};

pub type ElectionId = Uuid;

/// The election status
#[derive(PartialEq, Eq, Debug, RustcEncodable, RustcDecodable, Clone)]
pub enum ElectionStatus {
    Running,
    Finished,
}

impl fmt::Display for ElectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            &ElectionStatus::Running => "Running",
            &ElectionStatus::Finished => "Finished",
        };
        write!(f, "{}", output)
    }
}

/// The election struct.
#[derive(Eq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Election {
    pub id: ElectionId,
    pub service: String,
    pub group: String,
    pub leader_id: MemberId,
    pub suitability: u32,
    pub votes: HashSet<MemberId>,
    pub status: ElectionStatus,
    pub term: u32,
}

impl fmt::Display for Election {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Election {}.{} {} (L: {} S: {} V: {} T: {})",
               self.service,
               self.group,
               self.status,
               self.leader_id,
               self.suitability,
               self.votes.len(),
               self.term)
    }
}

impl PartialEq for Election {
    /// We ignore id in equality checking, because we only have one per service group
    fn eq(&self, other: &Election) -> bool {
        self.service == other.service && self.group == other.group &&
        self.leader_id == other.leader_id && self.suitability == other.suitability &&
        self.votes == other.votes && self.status == other.status && self.term == other.term
    }
}

impl Election {
    pub fn new(service: String,
               group: String,
               leader_id: MemberId,
               suitability: u32,
               term: u32)
               -> Election {
        let mut votes = HashSet::new();
        votes.insert(leader_id);
        Election {
            id: ElectionId::new_v4(),
            service: service,
            group: group,
            leader_id: leader_id,
            votes: votes,
            term: term,
            suitability: suitability,
            status: ElectionStatus::Running,
        }
    }

    /// Returns true if the election is finished.
    pub fn finished(&self) -> bool {
        self.status == ElectionStatus::Finished
    }

    /// Returns the service group string.
    pub fn service_group(&self) -> String {
        format!("{}.{}", self.service, self.group)
    }

    /// Returns true if the alive population and the number of votes are equivalent
    pub fn should_finish(&self, member_id: &MemberId, alive_population: usize) -> bool {
        self.votes.len() == alive_population && self.leader_id == *member_id
    }

    /// Updates this struct via another. If true, we have changed the election, and the rumor
    /// should stay hot. If false, we have not changed, and the rumor can start going cold.
    ///
    /// * If we are the same, return false
    /// * If we are running and the inbound is finsihed, and our term is the same, take the remote
    /// and return true
    /// * If our suitability is higher then theirs, vote for them, and return true
    /// * If their suitability is higher than ours, vote for them, and return true
    /// * If we are suitablely equal, but our string is higher, vote for ourselves with the remote
    /// and return true
    /// * Otherwise, vote for them, and take theirs - return true
    /// * Otherwise, we are the same, but our votes are different - add all our votes and return
    /// true
    pub fn update_via(&mut self, remote_election: Election) -> bool {
        if *self == remote_election {
            false
        } else if self.status == ElectionStatus::Running &&
                  remote_election.status == ElectionStatus::Finished &&
                  self.term == remote_election.term {
            *self = remote_election;
            true
        } else if self.suitability > remote_election.suitability {
            for x in remote_election.votes.iter() {
                self.votes.insert(*x);
            }
            self.votes.insert(remote_election.leader_id);
            true
        } else if remote_election.suitability > self.suitability {
            let old_votes = self.votes.clone();
            let old_id = self.leader_id.clone();
            *self = remote_election;
            for x in old_votes.iter() {
                self.votes.insert(*x);
            }
            self.votes.insert(old_id);
            true
        } else {
            if self.leader_id.simple().to_string() >
               remote_election.leader_id.simple().to_string() {
                for x in remote_election.votes.iter() {
                    self.votes.insert(*x);
                }
                self.votes.insert(remote_election.leader_id);
                true
            } else if self.leader_id.simple().to_string() <
                      remote_election.leader_id.simple().to_string() {
                let old_votes = self.votes.clone();
                let old_id = self.leader_id.clone();
                *self = remote_election;
                for x in old_votes.iter() {
                    self.votes.insert(*x);
                }
                self.votes.insert(old_id);
                true
            } else {
                let votes = self.votes.clone();
                let start_len = votes.len();
                let differences = remote_election.votes
                    .difference(&votes);
                for id in differences {
                    self.votes.insert(id.clone());
                }
                if self.votes.len() > start_len {
                    true
                } else {
                    if remote_election.status == ElectionStatus::Finished &&
                       self.status == ElectionStatus::Running {
                        self.status = ElectionStatus::Finished;
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

/// The election list tracks elections across service groups.
#[derive(Debug)]
pub struct ElectionList {
    pub elections: HashMap<String, Election>,
    my_service_group: String,
    pub member_id: MemberId,
}

impl ElectionList {
    pub fn new(service_group: String, member_id: MemberId) -> ElectionList {
        ElectionList {
            elections: HashMap::new(),
            my_service_group: service_group,
            member_id: member_id,
        }
    }

    /// Returns this supervisors election
    pub fn election(&self) -> Option<&Election> {
        self.elections.get(&self.my_service_group)
    }

    /// Returns a mutable ref for this supervisors election
    pub fn election_mut(&mut self) -> Option<&mut Election> {
        self.elections.get_mut(&self.my_service_group)
    }

    /// Update an Election based on a Remote Election. Called from gossip::server.
    pub fn process(&mut self, mut remote_election: Election) -> bool {
        let mut updated_term = false;
        if (remote_election.term > self.current_term_for(&remote_election)) &&
           (self.my_service_group == remote_election.service_group()) {
            let mut e = self.generate_election_for(remote_election.service.clone(),
                                                   remote_election.group.clone());
            e.term = remote_election.term;
            self.elections.insert(e.service_group(), e);
            updated_term = true;
        }
        if let Some(mut current_election) = self.elections
            .get_mut(&remote_election.service_group()) {
            let result = current_election.update_via(remote_election);
            if updated_term {
                if !current_election.votes.contains(&self.member_id) {
                    current_election.votes.insert(self.member_id);
                }
                return true;
            } else {
                if result {
                    if !current_election.votes.contains(&self.member_id) {
                        current_election.votes.insert(self.member_id);
                    }
                    return true;
                } else {
                    // This is a backstop against elections getting deadlocked because we got a new
                    // object without voting for it.
                    if !current_election.votes.contains(&self.member_id) {
                        current_election.votes.insert(self.member_id);
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
        remote_election.votes.insert(self.member_id);
        self.elections.insert(remote_election.service_group(), remote_election);
        return true;
    }

    /// Return the current term for a given election
    pub fn current_term_for(&self, election: &Election) -> u32 {
        self.elections
            .get(&format!("{}.{}", election.service, election.group))
            .map_or(0, |e| e.term.clone())
    }

    /// Build a new election for a service group.
    pub fn generate_election_for(&self, service: String, group: String) -> Election {
        let current_term: u32 = self.elections
            .get(&format!("{}.{}", service, group))
            .map_or(0, |e| e.term.clone());

        // Eventually, you need to actually get the suitability here, or in election::new
        Election::new(service, group, self.member_id.clone(), 0, current_term)
    }

    /// Finish the election
    pub fn finished_election_for(&self, service: String, group: String) -> Election {
        let mut election = self.elections
            .get(&format!("{}.{}", service, group))
            .unwrap()
            .clone();
        election.status = ElectionStatus::Finished;
        election
    }

    /// Generate a rumor list for a finished election
    pub fn finished_rumor_list_for(&self, service: String, group: String) -> RumorList {
        let election = self.finished_election_for(service, group);
        let election_rumor = Rumor::election(election.clone());
        let mut rumor_list = RumorList::new();
        rumor_list.add_rumor(election_rumor);
        rumor_list
    }

    /// Generate a new rumor list for an election. Passed to `server::process_rumors` to kick
    /// things off.
    pub fn generate_rumor_list_for(&self, service: String, group: String) -> RumorList {
        let service_group = format!("{}.{}", &service, &group);
        let mut election = self.generate_election_for(service, group);
        if self.elections.contains_key(&service_group) {
            election.term = self.current_term_for(&election) + 1;
        }
        let election_rumor = Rumor::election(election.clone());
        let mut rumor_list = RumorList::new();
        rumor_list.add_rumor(election_rumor);
        rumor_list
    }
}

impl Deref for ElectionList {
    type Target = HashMap<String, Election>;

    fn deref(&self) -> &HashMap<String, Election> {
        &self.elections
    }
}

impl DerefMut for ElectionList {
    fn deref_mut(&mut self) -> &mut HashMap<String, Election> {
        &mut self.elections
    }
}


#[cfg(test)]
mod test {
    use gossip::member::MemberId;
    use election::Election;

    fn generate_election() -> (MemberId, Election) {
        let id = MemberId::new_v4();
        let election = Election::new("handy".to_string(), "manny".to_string(), id, 1, 1);
        (id, election)
    }

    mod election {
        use gossip::member::MemberId;
        use election::Election;
        use super::generate_election;

        #[test]
        fn new() {
            let member_id = MemberId::new_v4();
            let e = Election::new("handy".to_string(), "manny".to_string(), member_id, 1, 1);
            assert_eq!(e.service, "handy");
            assert_eq!(e.group, "manny");
            assert_eq!(e.leader_id, member_id);
            assert_eq!(e.suitability, 1);
            assert!(e.votes.contains(&member_id));
        }

        #[test]
        fn update_via_when_suitability_is_equal_uses_ids() {
            let (local_id, mut local_election) = generate_election();
            let (remote_id, remote_election) = generate_election();

            assert!(local_election.update_via(remote_election));

            if local_id.simple().to_string() > remote_id.simple().to_string() {
                assert_eq!(local_election.leader_id, local_id);
                assert!(local_election.votes.contains(&local_id));
                assert!(local_election.votes.contains(&remote_id));
            } else {
                assert_eq!(local_election.leader_id, remote_id);
                assert!(local_election.votes.contains(&local_id));
                assert!(local_election.votes.contains(&remote_id));
            }
        }

        #[test]
        fn update_via_when_suitability_is_higher() {
            let (local_id, mut local_election) = generate_election();
            let (remote_id, mut remote_election) = generate_election();

            remote_election.suitability = 50;
            local_election.suitability = 100;
            assert!(local_election.update_via(remote_election.clone()));
            assert_eq!(local_election.leader_id, local_id);
            assert!(local_election.votes.contains(&local_id));
            assert!(local_election.votes.contains(&remote_id));

            remote_election.suitability = 150;
            assert!(local_election.update_via(remote_election.clone()));
            assert_eq!(local_election.leader_id, remote_id);
            assert!(local_election.votes.contains(&local_id));
            assert!(local_election.votes.contains(&remote_id));
        }

        #[test]
        fn update_via_when_both_sides_equal() {
            let (local_id, mut local_election) = generate_election();

            let remote_election = local_election.clone();

            assert_eq!(local_election.update_via(remote_election.clone()), false);
            assert!(local_election.votes.contains(&local_id));
        }

        #[test]
        fn update_via_when_both_sides_equal_but_votes_differ() {
            let (local_id, mut local_election) = generate_election();
            let (second_id, second_election) = generate_election();
            let (third_id, third_election) = generate_election();

            local_election.suitability = 100;

            let mut remote_election = local_election.clone();

            assert!(local_election.update_via(second_election));
            assert!(local_election.update_via(third_election));
            assert!(remote_election.update_via(local_election));

            assert!(remote_election.votes.contains(&local_id));
            assert!(remote_election.votes.contains(&second_id));
            assert!(remote_election.votes.contains(&third_id));
        }

        // Given 5 different election entries, if we update all of them with every other entry, we will
        // wind up with 5 identical entries.
        #[test]
        fn n_way_updates_converge_on_winner() {
            let (_a_id, mut a_election) = generate_election();
            let (_b_id, mut b_election) = generate_election();
            let (_c_id, mut c_election) = generate_election();
            let (_d_id, mut d_election) = generate_election();
            let (_e_id, mut e_election) = generate_election();

            let pristine_a = a_election.clone();

            a_election.suitability = 100;

            a_election.update_via(b_election.clone());
            a_election.update_via(c_election.clone());
            a_election.update_via(d_election.clone());
            a_election.update_via(e_election.clone());

            b_election.update_via(c_election.clone());
            b_election.update_via(d_election.clone());
            b_election.update_via(e_election.clone());
            b_election.update_via(pristine_a.clone());
            b_election.update_via(a_election.clone());

            c_election.update_via(b_election.clone());
            c_election.update_via(d_election.clone());
            c_election.update_via(e_election.clone());
            c_election.update_via(pristine_a.clone());
            c_election.update_via(a_election.clone());

            d_election.update_via(b_election.clone());
            d_election.update_via(c_election.clone());
            d_election.update_via(e_election.clone());
            d_election.update_via(pristine_a.clone());
            d_election.update_via(a_election.clone());

            e_election.update_via(b_election.clone());
            e_election.update_via(c_election.clone());
            e_election.update_via(d_election.clone());
            e_election.update_via(pristine_a.clone());
            e_election.update_via(a_election.clone());

            assert!(a_election == b_election);
            assert!(a_election == c_election);
            assert!(a_election == d_election);
            assert!(a_election == e_election);
        }
    }

}
