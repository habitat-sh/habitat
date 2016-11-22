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

//! Tracks membership. Contains both the `Member` struct and the `MemberList`.

use std::collections::{hash_map, HashMap};
use std::fmt;
use std::iter::IntoIterator;
use std::net::SocketAddr;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use uuid::Uuid;
use rand::{thread_rng, Rng};
use time::SteadyTime;

use rumor::RumorKey;
use message::swim::{Member as ProtoMember, Membership as ProtoMembership,
                    Membership_Health as ProtoMembership_Health, Rumor_Type};

/// How many nodes do we target when we need to run PingReq.
const PINGREQ_TARGETS: usize = 5;

/// The health of a node.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Health {
    Alive,
    Suspect,
    Confirmed,
}

/// Maps our internal health to the wire protocols health.
impl From<ProtoMembership_Health> for Health {
    fn from(pm_health: ProtoMembership_Health) -> Health {
        match pm_health {
            ProtoMembership_Health::ALIVE => Health::Alive,
            ProtoMembership_Health::SUSPECT => Health::Suspect,
            ProtoMembership_Health::CONFIRMED => Health::Confirmed,
        }
    }
}

impl From<Health> for ProtoMembership_Health {
    fn from(pm_health: Health) -> ProtoMembership_Health {
        match pm_health {
            Health::Alive => ProtoMembership_Health::ALIVE,
            Health::Suspect => ProtoMembership_Health::SUSPECT,
            Health::Confirmed => ProtoMembership_Health::CONFIRMED,
        }
    }
}

impl<'a> From<&'a Health> for ProtoMembership_Health {
    fn from(pm_health: &'a Health) -> ProtoMembership_Health {
        match pm_health {
            &Health::Alive => ProtoMembership_Health::ALIVE,
            &Health::Suspect => ProtoMembership_Health::SUSPECT,
            &Health::Confirmed => ProtoMembership_Health::CONFIRMED,
        }
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Health::Alive => write!(f, "Alive"),
            &Health::Suspect => write!(f, "Suspect"),
            &Health::Confirmed => write!(f, "Confirmed"),
        }
    }
}

/// A member in the swim group. Passes most of its functionality along to the internal protobuf
/// representation.
#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    pub proto: ProtoMember,
}

impl Member {
    /// Creates a new member with a unique UUID and an incarnation of zero.
    pub fn new() -> Member {
        let mut proto_member = ProtoMember::new();
        proto_member.set_id(Uuid::new_v4().simple().to_string());
        proto_member.set_incarnation(0);
        Member { proto: proto_member }
    }

    /// Returns the socket address of this member.
    ///
    /// # Panics
    ///
    /// This function panics if the address is un-parseable. In practice, it shouldn't be
    /// un-parseable, since its set from the inbound socket directly.
    pub fn swim_socket_address(&self) -> SocketAddr {
        let address_str = format!("{}:{}", self.get_address(), self.get_swim_port());
        match address_str.parse() {
            Ok(addr) => addr,
            Err(e) => {
                panic!("Cannot parse member {:?} address: {}", self, e);
            }
        }
    }
}

impl Deref for Member {
    type Target = ProtoMember;

    fn deref(&self) -> &ProtoMember {
        &self.proto
    }
}

impl DerefMut for Member {
    fn deref_mut(&mut self) -> &mut ProtoMember {
        &mut self.proto
    }
}

impl From<ProtoMember> for Member {
    fn from(member: ProtoMember) -> Member {
        Member { proto: member }
    }
}

impl<'a> From<&'a ProtoMember> for Member {
    fn from(member: &'a ProtoMember) -> Member {
        Member { proto: member.clone() }
    }
}

impl From<Member> for RumorKey {
    fn from(member: Member) -> RumorKey {
        RumorKey::new(Rumor_Type::Member, member.get_id(), "")
    }
}

impl<'a> From<&'a Member> for RumorKey {
    fn from(member: &'a Member) -> RumorKey {
        RumorKey::new(Rumor_Type::Member, member.get_id(), "")
    }
}

impl<'a> From<&'a &'a Member> for RumorKey {
    fn from(member: &'a &'a Member) -> RumorKey {
        RumorKey::new(Rumor_Type::Member, member.get_id(), "")
    }
}

// This is a Uuid type turned to a string
pub type UuidSimple = String;

/// Tracks lists of members, their health, and how long they have been suspect.
#[derive(Debug, Clone)]
pub struct MemberList {
    members: Arc<RwLock<HashMap<UuidSimple, Member>>>,
    health: Arc<RwLock<HashMap<UuidSimple, Health>>>,
    suspect: Arc<RwLock<HashMap<UuidSimple, SteadyTime>>>,
    initial_members: Arc<RwLock<Vec<Member>>>,
    update_counter: Arc<AtomicUsize>,
}

impl MemberList {
    /// Creates a new, empty, MemberList.
    pub fn new() -> MemberList {
        MemberList {
            members: Arc::new(RwLock::new(HashMap::new())),
            health: Arc::new(RwLock::new(HashMap::new())),
            suspect: Arc::new(RwLock::new(HashMap::new())),
            initial_members: Arc::new(RwLock::new(Vec::new())),
            update_counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Increment the update counter for this store.
    ///
    /// We don't care if this repeats - it just needs to be unique for any given two states, which
    /// it will be.
    pub fn increment_update_counter(&self) {
        self.update_counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_update_counter(&self) -> usize {
        self.update_counter.load(Ordering::Relaxed)
    }

    pub fn add_initial_member(&self, member: Member) {
        let mut im = self.initial_members.write().expect("Initial members lock is poisoned");
        im.push(member);
    }

    pub fn with_initial_members<F>(&self, mut with_closure: F) -> ()
        where F: FnMut(&Member)
    {
        let im = self.initial_members.read().expect("Initial members lock is poisoned");
        for member in im.iter() {
            with_closure(member);
        }
    }

    /// Inserts a member into the member list with the given health.
    pub fn insert(&self, member: Member, health: Health) -> bool {
        let share_rumor: bool;
        let mut start_suspicion: bool = false;
        let mut stop_suspicion: bool = false;

        // If we have an existing member record..
        if let Some(current_member) = self.members
            .read()
            .expect("Member List read lock poisoned")
            .get(member.get_id()) {
            // If my incarnation is newer than the member we are being asked
            // to insert, we want to prefer our member, health and all.
            if current_member.get_incarnation() > member.get_incarnation() {
                share_rumor = false;
                // If the new rumor has a higher incarnation than our status, we want
                // to prefer it.
            } else if member.get_incarnation() > current_member.get_incarnation() {
                share_rumor = true;
            } else {
                // We know we have a health if we have a record
                let hl = self.health.read().expect("Health lock is poisoned");
                let current_health = hl.get(current_member.get_id())
                    .expect("No health for a membership record should be impossible; did you use \
                             insert?");
                // If curently healthy and the rumor is suspicion, then we are now suspicious.
                if *current_health == Health::Alive && health == Health::Suspect {
                    start_suspicion = true;
                    share_rumor = true;
                    // If currently healthy and the rumor is confirmation, then we are now confirmed
                } else if *current_health == Health::Alive && health == Health::Confirmed {
                    share_rumor = true;
                    // If we are both alive, then nothing to see here.
                } else if *current_health == Health::Alive && health == Health::Alive {
                    share_rumor = false;
                    // If currently suspicous and the rumor is alive, then we are still suspicious
                } else if *current_health == Health::Suspect && health == Health::Alive {
                    share_rumor = false;
                    // If currently suspicous and the rumor is suspicion, then nothing to see here.
                } else if *current_health == Health::Suspect && health == Health::Suspect {
                    share_rumor = false;
                    // If currently suspicious and the rumor is confirmation, then we are now confirmed
                } else if *current_health == Health::Suspect && health == Health::Confirmed {
                    stop_suspicion = true;
                    share_rumor = true;
                    // When we are currently confirmed, we stay that way until something with a
                    // higher incarnation changes our mind.
                } else {
                    share_rumor = false;
                }
            }
        } else {
            share_rumor = true;
        }

        if share_rumor == true {
            self.increment_update_counter();
            self.health
                .write()
                .expect("Health lock is poisoned")
                .insert(String::from(member.get_id()), health);
            if start_suspicion == true {
                self.suspect
                    .write()
                    .expect("Suspect lock is poisoned")
                    .insert(String::from(member.get_id()), SteadyTime::now());
            }
            if stop_suspicion == true {
                self.suspect.write().expect("Suspect lock is poisoned").remove(member.get_id());
            }
            self.members
                .write()
                .expect("Member list lock is poisoned")
                .insert(String::from(member.get_id()), member);
        }
        share_rumor
    }

    /// Returns the health of the member, if the member exists.
    pub fn health_of(&self, member: &Member) -> Option<Health> {
        match self.health.read().expect("Health lock is poisoned").get(member.get_id()) {
            Some(health) => Some(health.clone()),
            None => None,
        }
    }

    /// Returns the health of the member, if the member exists.
    pub fn health_of_by_id(&self, member_id: &str) -> Option<Health> {
        match self.health.read().expect("Health lock is poisoned").get(member_id) {
            Some(health) => Some(health.clone()),
            None => None,
        }
    }

    /// Returns true if the members health is the same as `health`. False otherwise.
    pub fn check_health_of(&self, member: &Member, health: Health) -> bool {
        match self.health.read().expect("Health lock is poisoned").get(member.get_id()) {
            Some(real_health) if *real_health == health => true,
            Some(_) => false,
            None => false,
        }
    }

    /// Returns true if the members health is the same as `health`. False otherwise.
    pub fn check_health_of_by_id(&self, member_id: &str, health: Health) -> bool {
        match self.health.read().expect("Health lock is poisoned").get(member_id) {
            Some(real_health) if *real_health == health => true,
            Some(_) => false,
            None => false,
        }
    }

    /// Returns true if the member is alive, suspect, or persistent; used during the target
    /// selection phase of the outbound thread.
    pub fn pingable(&self, member: &Member) -> bool {
        if member.get_persistent() {
            return true;
        }
        self.check_health_of(member, Health::Alive) || self.check_health_of(member, Health::Suspect)
    }

    /// Returns true if we are pinging this member because they are persistent, but we think they
    /// are gone.
    pub fn persistent_and_confirmed(&self, member: &Member) -> bool {
        member.get_persistent() && self.check_health_of(member, Health::Confirmed)
    }

    /// Updates the health of a member without touching the member itself. Returns true if the
    /// health changed, false otherwise.
    pub fn insert_health_by_id(&self, member_id: &str, health: Health) -> bool {
        if let Some(current_health) = self.health
            .read()
            .expect("Health read lock is poisoned")
            .get(member_id) {
            if *current_health == health {
                return false;
            }
        }
        if health == Health::Suspect {
            let mut sl = self.suspect.write().expect("Suspect lock is poisoned");
            sl.insert(String::from(member_id), SteadyTime::now());
        }
        self.health
            .write()
            .expect("Health write lock is poisoned")
            .insert(String::from(member_id), health);
        self.increment_update_counter();
        true
    }

    /// The same as `insert_health_by_id`, but takes a member rather than an id.
    pub fn insert_health(&self, member: &Member, health: Health) -> bool {
        self.insert_health_by_id(member.get_id(), health)
    }

    /// Returns a protobuf membership record for the given member id.
    pub fn membership_for(&self, member_id: &str) -> ProtoMembership {
        let mut pm = ProtoMembership::new();
        let mhealth: ProtoMembership_Health = self.health
            .read()
            .expect("Health lock is poisoned")
            .get(member_id)
            .expect("Should have membership before calling membership_for")
            .into();
        let ml = self.members.read().expect("Member list lock is poisoned");
        let member = ml.get(member_id)
            .expect("Should have membership before calling membership_for");
        pm.set_health(mhealth);
        pm.set_member(member.proto.clone());
        pm
    }

    /// Returns the number of members.
    pub fn len(&self) -> usize {
        self.members.read().expect("Member list lock is poisoned").len()
    }

    /// A randomized list of members to check.
    pub fn check_list(&self, exclude_id: &str) -> Vec<Member> {
        let mut members: Vec<Member> = self.members
            .read()
            .expect("Member list lock is poisoned")
            .values()
            .filter(|v| v.get_id() != exclude_id)
            .map(|v| v.clone())
            .collect();
        let mut rng = thread_rng();
        rng.shuffle(&mut members);
        members
    }

    /// Takes a function whose first argument is a member, and calls it for every pingreq target.
    pub fn with_pingreq_targets<F>(&self,
                                   sending_member_id: &str,
                                   target_member_id: &str,
                                   mut with_closure: F)
                                   -> ()
        where F: FnMut(Member) -> ()
    {
        // This will lead to nested read locks if you don't deal with making a copy
        let mut members: Vec<Member> = {
            let ml = self.members.read().expect("Member list lock is poisoned");
            ml.values().map(|v| v.clone()).collect()
        };
        let mut rng = thread_rng();
        rng.shuffle(&mut members);
        for member in members.into_iter()
            .filter(|m| {
                m.get_id() != sending_member_id && m.get_id() != target_member_id &&
                self.check_health_of_by_id(m.get_id(), Health::Alive)
            })
            .take(PINGREQ_TARGETS) {
            with_closure(member);
        }
    }

    /// Takes a function whose argument is a `HashMap::Values` iterator, with the ID and Membership
    /// entry.
    pub fn with_member_iter<F>(&self, mut with_closure: F) -> ()
        where F: FnMut(hash_map::Values<String, Member>) -> ()
    {
        with_closure(self.members.read().expect("Member list lock is poisoned").values());
    }

    /// Takes a function whose argument is a reference to the member list hashmap.
    pub fn with_member_list<F>(&self, mut with_closure: F) -> ()
        where F: FnMut(&HashMap<String, Member>) -> ()
    {
        with_closure(self.members.read().expect("Member list lock is poisoned").deref());
    }

    /// Calls a function whose argument is a reference to a membership entry matching the given ID.
    pub fn with_member<F>(&self, member_id: &str, mut with_closure: F) -> ()
        where F: FnMut(Option<&Member>) -> ()
    {
        let ml = self.members.read().expect("Member list lock poisoned");
        let member = ml.get(member_id);
        with_closure(member);
    }

    /// Iterates over the member list, calling the function for each member.
    pub fn with_members<F>(&self, mut with_closure: F) -> ()
        where F: FnMut(&Member) -> ()
    {
        for member in self.members.read().expect("Member list lock is poisoned").values() {
            with_closure(member);
        }
    }

    /// Iterates over every suspected membership entry, calling the given closure.
    pub fn with_suspects<F>(&self, mut with_closure: F) -> ()
        where F: FnMut((&str, &SteadyTime)) -> ()
    {
        for (id, suspect) in self.suspect.read().expect("Suspect list lock is poisoned").iter() {
            with_closure((id, suspect));
        }
    }

    /// Expires a member from the suspect list.
    pub fn expire(&self, member_id: &str) {
        let mut suspects = self.suspect.write().expect("Suspect list lock is poisoned");
        suspects.remove(member_id);
    }

    pub fn contains_member(&self, member_id: &str) -> bool {
        self.members.read().expect("Member list lock is poisoned").contains_key(member_id)
    }
}

#[cfg(test)]
mod tests {
    mod member {
        use uuid::Uuid;
        use message::swim;
        use member::Member;

        // Sets the uuid to simple, and the incarnation to zero.
        #[test]
        fn new() {
            let member = Member::new();
            assert_eq!(member.proto.get_id().len(), 32);
            assert_eq!(member.proto.get_incarnation(), 0);
        }

        // Takes a member in from a protobuf
        #[test]
        fn new_from_proto() {
            let mut proto = swim::Member::new();
            let uuid = Uuid::new_v4();
            proto.set_id(uuid.simple().to_string());
            proto.set_incarnation(0);
            let proto2 = proto.clone();
            let member: Member = proto.into();
            assert_eq!(proto2, member.proto);
        }
    }

    mod member_list {
        use member::{Member, MemberList, Health, PINGREQ_TARGETS};

        fn populated_member_list(size: u64) -> MemberList {
            let ml = MemberList::new();
            for _x in 0..size {
                let m = Member::new();
                ml.insert(m, Health::Alive);
            }
            ml
        }

        #[test]
        fn new() {
            let ml = MemberList::new();
            assert_eq!(ml.len(), 0);
        }

        #[test]
        fn insert() {
            let ml = populated_member_list(4);
            assert_eq!(ml.len(), 4);
        }

        #[test]
        fn check_list() {
            let ml = populated_member_list(1000);
            let list_a = ml.check_list("foo");
            let list_b = ml.check_list("foo");
            assert!(list_a != list_b);
        }

        #[test]
        fn health_of() {
            let ml = populated_member_list(1);
            ml.with_members(|m| assert!(ml.check_health_of(m, Health::Alive)));
        }

        #[test]
        fn pingreq_targets() {
            let ml = populated_member_list(10);
            ml.with_member_iter(|mut i| {
                let from = i.nth(0).unwrap();
                let target = i.nth(1).unwrap();
                let mut counter: usize = 0;
                ml.with_pingreq_targets(from.get_id(), target.get_id(), |_m| counter += 1);
                assert_eq!(counter, PINGREQ_TARGETS);
            });
        }

        #[test]
        fn pingreq_targets_excludes_pinging_member() {
            let ml = populated_member_list(3);
            ml.with_member_iter(|mut i| {
                let from = i.nth(0).unwrap();
                let target = i.nth(1).unwrap();
                let mut excluded_appears: bool = false;
                ml.with_pingreq_targets(from.get_id(),
                                        target.get_id(),
                                        |m| if m.get_id() == from.get_id() {
                                            excluded_appears = true
                                        });
                assert_eq!(excluded_appears, false);
            });
        }

        #[test]
        fn pingreq_targets_excludes_target_member() {
            let ml = populated_member_list(3);
            ml.with_member_iter(|mut i| {
                let from = i.nth(0).unwrap();
                let target = i.nth(1).unwrap();
                let mut excluded_appears: bool = false;
                ml.with_pingreq_targets(from.get_id(),
                                        target.get_id(),
                                        |m| if m.get_id() == target.get_id() {
                                            excluded_appears = true
                                        });
                assert_eq!(excluded_appears, false);
            });
        }

        #[test]
        fn pingreq_targets_minimum_viable_pingreq_size_is_three() {
            let ml = populated_member_list(3);
            ml.with_member_iter(|mut i| {
                let from = i.nth(0).unwrap();
                let target = i.nth(1).unwrap();
                let mut counter: isize = 0;
                ml.with_pingreq_targets(from.get_id(), target.get_id(), |_m| counter += 1);
                assert_eq!(counter, 1);
            });
        }

        #[test]
        fn insert_no_member() {
            let ml = MemberList::new();
            let member = Member::new();
            let mcheck = member.clone();
            assert_eq!(ml.insert(member, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck, Health::Alive));
        }

        #[test]
        fn insert_existing_member_lower_incarnation() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let mut member_two = member_one.clone();
            member_two.set_incarnation(1);
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Alive), true);
            ml.with_member(mcheck_two.get_id(),
                           |m| assert_eq!(m.unwrap().get_incarnation(), 1));
        }

        #[test]
        fn insert_existing_member_higher_incarnation() {
            let ml = MemberList::new();
            let mut member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            member_one.set_incarnation(1);

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Alive), false);
            ml.with_member(mcheck_two.get_id(),
                           |m| assert_eq!(m.unwrap().get_incarnation(), 1));
        }

        #[test]
        fn insert_equal_incarnation_current_alive_new_alive() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Alive), false);
        }

        #[test]
        fn insert_equal_incarnation_current_alive_new_suspect() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Suspect), true);
            assert!(ml.check_health_of(&mcheck_two, Health::Suspect));
        }

        #[test]
        fn insert_equal_incarnation_current_alive_new_confirmed() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Confirmed), true);
            assert!(ml.check_health_of(&mcheck_two, Health::Confirmed));
        }

        #[test]
        fn insert_equal_incarnation_current_suspect_new_alive() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Suspect), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Suspect));

            assert_eq!(ml.insert(member_two, Health::Alive), false);
            assert!(ml.check_health_of(&mcheck_two, Health::Suspect));
        }

        #[test]
        fn insert_equal_incarnation_current_suspect_new_suspect() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Suspect), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Suspect));

            assert_eq!(ml.insert(member_two, Health::Suspect), false);
            assert!(ml.check_health_of(&mcheck_two, Health::Suspect));
        }

        #[test]
        fn insert_equal_incarnation_current_suspect_new_confirmed() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Suspect), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Suspect));

            assert_eq!(ml.insert(member_two, Health::Confirmed), true);
            assert!(ml.check_health_of(&mcheck_two, Health::Confirmed));
        }

        #[test]
        fn insert_equal_incarnation_current_confirmed_new_alive() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Confirmed), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Confirmed));

            assert_eq!(ml.insert(member_two, Health::Alive), false);
            assert!(ml.check_health_of(&mcheck_two, Health::Confirmed));
        }

        #[test]
        fn insert_equal_incarnation_current_confirmed_new_suspect() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Confirmed), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Confirmed));

            assert_eq!(ml.insert(member_two, Health::Suspect), false);
            assert!(ml.check_health_of(&mcheck_two, Health::Confirmed));
        }

        #[test]
        fn insert_equal_incarnation_current_confirmed_new_confirmed() {
            let ml = MemberList::new();
            let member_one = Member::new();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Confirmed), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Confirmed));

            assert_eq!(ml.insert(member_two, Health::Confirmed), false);
            assert!(ml.check_health_of(&mcheck_two, Health::Confirmed));
        }

    }
}
