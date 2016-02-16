// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The rumor system.
//!
//! A rumor is the unit of sharing in the gossip protocol - it piggybacks on the SWIM failure
//! detection protocol to share state.

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use census::CensusEntry;
use election::Election;

use rustc_serialize::Encodable;
use uuid::Uuid;

/// How many times does a rumor get shared with a member before we stop sharing it?
pub const COLD_AFTER: usize = 3;

use gossip::member::{Member, MemberId};

/// A Peer is a representation of a member; it tracks how to contact the member, and whether this
/// request is actually being proxied during a PingReq operation.
#[derive(Clone, Debug, RustcDecodable, RustcEncodable)]
pub struct Peer {
    pub member_id: MemberId,
    pub listening_on: String,
    pub proxy_through: Option<String>,
    pub proxy_to: Option<String>,
}

impl Peer {
    /// Create a new peer.
    pub fn new(member_id: MemberId, listening_on: String) -> Peer {
        Peer {
            member_id: member_id,
            listening_on: listening_on,
            proxy_through: None,
            proxy_to: None,
        }
    }
}

/// The SWIM Protocol.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub enum Protocol {
    Ping(Peer, RumorList),
    Ack(Peer, RumorList),
    PingReq(Peer, RumorList),
}

/// Rumors contain Messages as their payload, which are then processed by the correct internal
/// sub-system.
#[derive(Debug, RustcDecodable, RustcEncodable, Clone, PartialEq, Eq)]
pub enum Message {
    Member(Member),
    CensusEntry(CensusEntry),
    Election(Election),
    Blank,
}

/// A UUID for Rumors. In practice, always matches the UUID of a message payload.
pub type RumorId = Uuid;

/// A Rumor, which contains a Message.
#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq, Clone)]
pub struct Rumor {
    pub id: RumorId,
    pub payload: Message,
}

impl Rumor {
    /// Create a new rumor with a 'Message::Member' payload.
    pub fn member(member: Member) -> Rumor {
        Rumor {
            id: member.id.clone(),
            payload: Message::Member(member),
        }
    }

    /// Create a new rumor with a 'Message::CensusEntry' payload.
    pub fn census_entry(ce: CensusEntry) -> Rumor {
        Rumor {
            id: ce.id.clone(),
            payload: Message::CensusEntry(ce),
        }
    }

    /// Create a new rumor with a 'Message::Election' payload.
    pub fn election(e: Election) -> Rumor {
        Rumor {
            id: e.id.clone(),
            payload: Message::Election(e),
        }
    }


    /// Create a new rumor with a 'Blank' payload.
    pub fn blank() -> Rumor {
        Rumor {
            id: Uuid::new_v4(),
            payload: Message::Blank,
        }
    }
}

/// A list of rumors, and their corresponding heat. Heat determines whether we need to share the
/// rumor with a given Member
#[derive(Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct RumorList {
    pub rumors: HashMap<RumorId, Rumor>,
    pub heat: HashMap<MemberId, HashMap<RumorId, usize>>,
}

impl RumorList {
    /// Create a new RumorList.
    pub fn new() -> RumorList {
        RumorList {
            rumors: HashMap::new(),
            heat: HashMap::new(),
        }
    }

    /// Given a RumorList, we match on the payload of each rumor, and then dispatch the
    /// corresponding entry to the right subsystem (e.g. Membership, Census, etc.).
    //  pub fn process_rumors(&mut self, remote_rumors: RumorList,
    //                        member_list: Arc<RwLock<MemberList>>,
    //                        census_list: Arc<RwLock<CensusList>>) {
    //      for (id, remote_rumor) in remote_rumors.rumors.into_iter() {
    //          match remote_rumor.payload {
    //              Message::Member(m) => {
    //                  debug!("Processing member {:#?}", m);
    //                  {
    //                      let mut ml = member_list.write().unwrap();
    //                      if ml.process(m) {
    //                          // The internals of the object might have changed, but not by
    //                          // replacement. Hence, we don't take the rumor as given - we have to go
    //                          // get the current member for the new rumor
    //                          self.add_rumor(Rumor::member(ml.get(&id).unwrap().clone()));
    //                      }
    //                  }
    //              }
    //              Message::CensusEntry(ce) => {
    //                  debug!("Processing Census Entry {:#?}", ce);
    //                  {
    //                      let mut cl = census_list.write().unwrap();
    //                      if cl.process(ce.clone()) {
    //                          // If we changed, by definition we took the other side.
    //                          self.add_rumor(Rumor::census_entry(ce));
    //                      }
    //                  }
    //              }
    //              Message::Blank => {}
    //          }
    //      }
    //  }
    /// Update a rumor. Resets the heat.
    pub fn update_rumor(&mut self, rumor: Rumor) {
        debug!("Updating rumor {:#?}", rumor);
        self.reset_heat_for(&rumor.id);
        self.add_rumor(rumor);
    }

    /// Resets the heat for a rumor.
    pub fn reset_heat_for(&mut self, rumor_id: &RumorId) {
        for (_member_id, mut rumor_heat) in self.heat.iter_mut() {
            debug!("Reset heat for {:?}", rumor_id);
            if rumor_heat.contains_key(rumor_id) {
                let mut count = rumor_heat.get_mut(rumor_id).unwrap();
                *count = 0;
            } else {
                rumor_heat.insert(rumor_id.clone(), 0);
            }
        }
    }

    /// Increments the heat for a given member and rumor.
    pub fn increment_heat_for(&mut self, member_id: &MemberId, rumor_id: &RumorId) {
        if self.heat.contains_key(member_id) {
            let rumor_heat = self.heat.get_mut(member_id).unwrap();
            if rumor_heat.contains_key(rumor_id) {
                let mut count = rumor_heat.get_mut(rumor_id).unwrap();
                *count = *count + 1;
            } else {
                rumor_heat.insert(rumor_id.clone(), 1);
            }
        } else {
            let mut rumor_heat = HashMap::new();
            rumor_heat.insert(rumor_id.clone(), 1);
            self.heat.insert(member_id.clone(), rumor_heat);
        }
    }

    /// Given a RumorList and a member, update the heat for each Rumor.
    pub fn update_heat_for(&mut self, member_id: &MemberId, rumor_list: &RumorList) {
        for (rumor_id, _rumor) in rumor_list.rumors.iter() {
            debug!("Updating heat for {:?} {:?}", member_id, rumor_list);
            self.increment_heat_for(member_id, rumor_id);
        }
    }

    /// Get a RumorList that contains all the Hot Rumors for the member in question.
    pub fn hot_rumors_for(&self, member_id: &MemberId) -> RumorList {
        let mut hot_rumors = RumorList::new();
        let hot_rumor_iterator = self.rumors.iter().filter(|&(rumor_id, _rumor)| {
            let hot_or_not = if self.heat_for(member_id, rumor_id) <= COLD_AFTER {
                true
            } else {
                false
            };
            hot_or_not
        });
        for (_rid, rumor) in hot_rumor_iterator {
            let rc = rumor.clone();
            hot_rumors.add_rumor(rc);
        }
        hot_rumors
    }

    /// Return the heat for a given member and rumor.
    pub fn heat_for(&self, member_id: &MemberId, rumor_id: &RumorId) -> usize {
        if self.heat.contains_key(member_id) {
            let rumor_heat = self.heat.get(member_id).unwrap();
            if rumor_heat.contains_key(rumor_id) {
                rumor_heat.get(rumor_id).unwrap().clone()
            } else {
                0
            }
        } else {
            0
        }
    }

    /// Add a new rumor to the list.
    pub fn add_rumor(&mut self, rumor: Rumor) {
        debug!("Adding rumor {:?}", rumor);
        self.reset_heat_for(&rumor.id);
        self.rumors.insert(rumor.id, rumor);
    }

    pub fn prune_elections_for(&mut self, service_group: &str) {
        let mut prune_list: Vec<RumorId> = Vec::new();
        for (rid, rumor) in self.rumors.iter() {
            if let Message::Election(ref election) = rumor.payload {
                if &election.service_group() == service_group {
                    prune_list.push(rid.clone());
                }
            }
        }
        for rid in prune_list.iter() {
            self.rumors.remove(&rid);
        }
    }

    pub fn remove_rumor(&mut self, rumor_id: &RumorId) {
        self.rumors.remove(rumor_id);
        for (_member, mut rumor_map) in self.heat.iter_mut() {
            rumor_map.remove(rumor_id);
        }
    }
}

impl Deref for RumorList {
    type Target = HashMap<RumorId, Rumor>;

    fn deref(&self) -> &HashMap<RumorId, Rumor> {
        &self.rumors
    }
}

impl DerefMut for RumorList {
    fn deref_mut(&mut self) -> &mut HashMap<RumorId, Rumor> {
        &mut self.rumors
    }
}

#[cfg(test)]
mod test {
    mod rumor_list {
        use gossip::rumor::{Rumor, RumorId, RumorList};
        use gossip::member::MemberId;

        #[test]
        fn add_rumor() {
            let rumor = Rumor::blank();
            let rumor_id = rumor.id.clone();
            let mut rl = RumorList::new();
            rl.add_rumor(rumor);
            assert_eq!(rumor_id, rl.rumors.get(&rumor_id).unwrap().id);
        }

        #[test]
        fn heat_for() {
            let rumor = Rumor::blank();
            let rumor_id = rumor.id.clone();
            let mut rl = RumorList::new();
            let member_id = MemberId::new_v4();
            rl.add_rumor(rumor);
            // Rumor exists
            assert_eq!(0, rl.heat_for(&member_id, &rumor_id));
            // Rumor does not exist in the heat map
            let fake_rumor = RumorId::new_v4();
            assert_eq!(0, rl.heat_for(&member_id, &fake_rumor));
            // Member does not exist in the heat map
            let fake_member = MemberId::new_v4();
            assert_eq!(0, rl.heat_for(&fake_member, &rumor_id));
        }
    }
}
