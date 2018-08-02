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
use std::iter::IntoIterator;
use std::net::SocketAddr;
use std::ops::Deref;
use std::result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use bytes::BytesMut;
use prost::Message as ProstMessage;
use rand::{thread_rng, Rng};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use time::SteadyTime;
use uuid::Uuid;

use error::{Error, Result};
pub use protocol::swim::Health;
use protocol::{newscast, swim as proto, FromProto};
use rumor::{RumorEnvelope, RumorKey, RumorKind, RumorPayload, RumorType};

/// How many nodes do we target when we need to run PingReq.
const PINGREQ_TARGETS: usize = 5;

// This is a Uuid type turned to a string
pub type UuidSimple = String;

/// A member in the swim group. Passes most of its functionality along to the internal protobuf
/// representation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub incarnation: u64,
    pub address: String,
    pub swim_port: i32,
    pub gossip_port: i32,
    pub persistent: bool,
    pub departed: bool,
}

impl Member {
    /// Returns the socket address of this member.
    ///
    /// # Panics
    ///
    /// This function panics if the address is un-parseable. In practice, it shouldn't be
    /// un-parseable, since its set from the inbound socket directly.
    pub fn swim_socket_address(&self) -> SocketAddr {
        let address_str = format!("{}:{}", self.address, self.swim_port);
        match address_str.parse() {
            Ok(addr) => addr,
            Err(e) => {
                panic!("Cannot parse member {:?} address: {}", self, e);
            }
        }
    }
}

impl Default for Member {
    fn default() -> Self {
        Member {
            id: Uuid::new_v4().simple().to_string(),
            incarnation: 0,
            address: String::default(),
            swim_port: 0,
            gossip_port: 0,
            persistent: false,
            departed: false,
        }
    }
}

impl From<Member> for RumorKey {
    fn from(member: Member) -> RumorKey {
        RumorKey::new(RumorType::Member, &member.id, "")
    }
}

impl<'a> From<&'a Member> for RumorKey {
    fn from(member: &'a Member) -> RumorKey {
        RumorKey::new(RumorType::Member, &member.id, "")
    }
}

impl<'a> From<&'a &'a Member> for RumorKey {
    fn from(member: &'a &'a Member) -> RumorKey {
        RumorKey::new(RumorType::Member, &member.id, "")
    }
}

impl From<Member> for proto::Member {
    fn from(value: Member) -> Self {
        proto::Member {
            id: Some(value.id),
            incarnation: Some(value.incarnation),
            address: Some(value.address),
            swim_port: Some(value.swim_port),
            gossip_port: Some(value.gossip_port),
            persistent: Some(value.persistent),
            departed: Some(value.departed),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub member: Member,
    pub health: Health,
}

impl Membership {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let rumor = RumorEnvelope::decode(bytes)?;
        match rumor.kind {
            RumorKind::Membership(payload) => Ok(payload),
            _ => panic!("from-bytes member"),
        }
    }

    pub fn write_to_bytes(self) -> Result<Vec<u8>> {
        let rumor: proto::Membership = self.into();
        let mut buf = BytesMut::with_capacity(rumor.encoded_len());
        rumor.encode(&mut buf)?;
        Ok(buf.to_vec())
    }
}

impl From<Membership> for proto::Membership {
    fn from(value: Membership) -> Self {
        proto::Membership {
            member: Some(value.member.into()),
            health: Some(value.health as i32),
        }
    }
}

impl FromProto<proto::Member> for Member {
    fn from_proto(proto: proto::Member) -> Result<Self> {
        Ok(Member {
            id: proto.id.ok_or(Error::ProtocolMismatch("id"))?,
            incarnation: proto.incarnation.unwrap_or(0),
            address: proto.address.ok_or(Error::ProtocolMismatch("address"))?,
            swim_port: proto.swim_port.ok_or(Error::ProtocolMismatch("swim-port"))?,
            gossip_port: proto
                .gossip_port
                .ok_or(Error::ProtocolMismatch("gossip-port"))?,
            persistent: proto.persistent.unwrap_or(false),
            departed: proto.departed.unwrap_or(false),
        })
    }
}

impl FromProto<proto::Membership> for Membership {
    fn from_proto(proto: proto::Membership) -> Result<Self> {
        Ok(Membership {
            member: proto
                .member
                .ok_or(Error::ProtocolMismatch("member"))
                .and_then(Member::from_proto)?,
            health: proto
                .health
                .and_then(Health::from_i32)
                .unwrap_or(Health::Alive),
        })
    }
}

impl FromProto<newscast::Rumor> for Membership {
    fn from_proto(proto: newscast::Rumor) -> Result<Self> {
        match proto.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::Member(membership) => Membership::from_proto(membership),
            _ => panic!("from-proto payload"),
        }
    }
}

/// Tracks lists of members, their health, and how long they have been suspect.
#[derive(Debug, Clone)]
pub struct MemberList {
    pub members: Arc<RwLock<HashMap<UuidSimple, Member>>>,
    pub health: Arc<RwLock<HashMap<UuidSimple, Health>>>,
    suspect: Arc<RwLock<HashMap<UuidSimple, SteadyTime>>>,
    depart: Arc<RwLock<HashMap<UuidSimple, SteadyTime>>>,
    initial_members: Arc<RwLock<Vec<Member>>>,
    update_counter: Arc<AtomicUsize>,
}

impl Serialize for MemberList {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("member_list", 4)?;
        {
            let member_struct = self.members.read().expect("Member lock is poisoned");
            strukt.serialize_field("members", &*member_struct)?;
        }
        {
            let health_struct = self.health.read().expect("Health lock is poisoned");
            strukt.serialize_field("health", &*health_struct)?;
        }
        {
            let update_number = self.update_counter.load(Ordering::SeqCst);
            strukt.serialize_field("update_counter", &update_number)?;
        }
        strukt.end()
    }
}

impl MemberList {
    /// Creates a new, empty, MemberList.
    pub fn new() -> MemberList {
        MemberList {
            members: Arc::new(RwLock::new(HashMap::new())),
            health: Arc::new(RwLock::new(HashMap::new())),
            suspect: Arc::new(RwLock::new(HashMap::new())),
            depart: Arc::new(RwLock::new(HashMap::new())),
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

    pub fn len_initial_members(&self) -> usize {
        let im = self
            .initial_members
            .read()
            .expect("Initial members lock is poisoned");
        im.len()
    }

    pub fn add_initial_member(&self, member: Member) {
        let mut im = self
            .initial_members
            .write()
            .expect("Initial members lock is poisoned");
        im.push(member);
    }

    pub fn set_initial_members(&self, members: Vec<Member>) {
        let mut im = self
            .initial_members
            .write()
            .expect("Initial members lock is poisoned");
        im.clear();
        im.extend(members);
    }

    pub fn with_initial_members<F>(&self, mut with_closure: F) -> ()
    where
        F: FnMut(&Member),
    {
        let im = self
            .initial_members
            .read()
            .expect("Initial members lock is poisoned");
        for member in im.iter() {
            with_closure(member);
        }
    }

    /// Inserts a member into the member list with the given health.
    pub fn insert(&self, member: Member, health: Health) -> bool {
        let share_rumor: bool;
        let mut start_suspicion: bool = false;
        let mut stop_suspicion: bool = false;
        let mut stop_departure: bool = false;

        // If we have an existing member record..
        if let Some(current_member) = self
            .members
            .read()
            .expect("Member List read lock poisoned")
            .get(&member.id)
        {
            // If my incarnation is newer than the member we are being asked
            // to insert, we want to prefer our member, health and all.
            if current_member.incarnation > member.incarnation {
                share_rumor = false;
            // If the new rumor has a higher incarnation than our status, we want
            // to prefer it.
            } else if member.incarnation > current_member.incarnation {
                share_rumor = true;
                if health == Health::Confirmed {
                    stop_suspicion = true;
                }
                if health == Health::Departed {
                    stop_suspicion = true;
                    stop_departure = true;
                }
            } else {
                // We know we have a health if we have a record
                let hl = self.health.read().expect("Health lock is poisoned");
                let current_health = hl.get(&current_member.id).expect(
                    "No health for a membership record should be impossible; did you use insert?",
                );
                // If currently healthy and the rumor is suspicion, then we are now suspicious.
                if *current_health == Health::Alive && health == Health::Suspect {
                    start_suspicion = true;
                    share_rumor = true;
                // If currently healthy and the rumor is confirmation, then we are now confirmed
                } else if *current_health == Health::Alive && health == Health::Confirmed {
                    share_rumor = true;
                // If currently healthy and the rumor is departed, then we are now departed
                } else if *current_health == Health::Alive && health == Health::Departed {
                    share_rumor = true;
                // If we are both alive, then nothing to see here.
                } else if *current_health == Health::Alive && health == Health::Alive {
                    share_rumor = false;
                // If currently suspicious and the rumor is alive, then we are still suspicious
                } else if *current_health == Health::Suspect && health == Health::Alive {
                    share_rumor = false;
                // If currently suspicious and the rumor is suspicion, then nothing to see here.
                } else if *current_health == Health::Suspect && health == Health::Suspect {
                    share_rumor = false;
                // If currently suspicious and the rumor is confirmation, then we are now
                // confirmed
                } else if *current_health == Health::Suspect && health == Health::Confirmed {
                    stop_suspicion = true;
                    share_rumor = true;
                // If currently suspicious and the rumor is departed, then we are now
                // departed
                } else if *current_health == Health::Suspect && health == Health::Departed {
                    stop_suspicion = true;
                    share_rumor = true;
                // If we are confirmed, and the rumor is departed, we accept the departure
                } else if *current_health == Health::Confirmed && health == Health::Departed {
                    share_rumor = true;
                    stop_departure = true;
                // When we are currently confirmed or departed, we stay that way until something
                // with a higher incarnation changes our mind. (except for the above case)
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
                .insert(member.id.clone(), health);
            if start_suspicion == true {
                self.suspect
                    .write()
                    .expect("Suspect lock is poisoned")
                    .insert(member.id.clone(), SteadyTime::now());
            }
            if stop_suspicion == true {
                self.suspect
                    .write()
                    .expect("Suspect lock is poisoned")
                    .remove(&member.id);
            }
            if stop_departure == true {
                self.depart
                    .write()
                    .expect("Departure lock is poisoned")
                    .remove(&member.id);
            }
            self.members
                .write()
                .expect("Member list lock is poisoned")
                .insert(member.id.clone(), member);
        }
        share_rumor
    }

    /// Returns the health of the member, if the member exists.
    pub fn health_of(&self, member: &Member) -> Option<Health> {
        match self
            .health
            .read()
            .expect("Health lock is poisoned")
            .get(&member.id)
        {
            Some(health) => Some(*health),
            None => None,
        }
    }

    /// Returns the health of the member, if the member exists.
    pub fn health_of_by_id(&self, member_id: &str) -> Option<Health> {
        match self
            .health
            .read()
            .expect("Health lock is poisoned")
            .get(member_id)
        {
            Some(health) => Some(*health),
            None => None,
        }
    }

    pub fn check_in_voting_population_by_id(&self, member_id: &str) -> bool {
        match self
            .health
            .read()
            .expect("Health lock is poisoned")
            .get(member_id)
        {
            Some(&Health::Alive) | Some(&Health::Suspect) | Some(&Health::Confirmed) => true,
            Some(&Health::Departed) => false,
            None => false,
        }
    }

    /// Returns true if the members health is the same as `health`. False otherwise.
    pub fn check_health_of(&self, member: &Member, health: Health) -> bool {
        match self
            .health
            .read()
            .expect("Health lock is poisoned")
            .get(&member.id)
        {
            Some(real_health) if *real_health == health => true,
            Some(_) => false,
            None => false,
        }
    }

    /// Returns true if the members health is the same as `health`. False otherwise.
    pub fn check_health_of_by_id(&self, member_id: &str, health: Health) -> bool {
        match self
            .health
            .read()
            .expect("Health lock is poisoned")
            .get(member_id)
        {
            Some(real_health) if *real_health == health => true,
            Some(_) => false,
            None => false,
        }
    }

    /// Returns true if the member is alive, suspect, or persistent; used during the target
    /// selection phase of the outbound thread.
    pub fn pingable(&self, member: &Member) -> bool {
        if member.persistent {
            return true;
        }
        self.check_health_of(member, Health::Alive) || self.check_health_of(member, Health::Suspect)
    }

    /// Returns true if we are pinging this member because they are persistent, but we think they
    /// are gone.
    pub fn persistent_and_confirmed(&self, member: &Member) -> bool {
        member.persistent && self.check_health_of(member, Health::Confirmed)
    }

    /// Updates the health of a member without touching the member itself. Returns true if the
    /// health changed, false otherwise.
    pub fn insert_health_by_id(&self, member_id: &str, health: Health) -> bool {
        if let Some(current_health) = self
            .health
            .read()
            .expect("Health read lock is poisoned")
            .get(member_id)
        {
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
        self.insert_health_by_id(&member.id, health)
    }

    /// Returns a protobuf membership record for the given member id.
    pub fn membership_for(&self, member_id: &str) -> Option<Membership> {
        let mhealth = match self
            .health
            .read()
            .expect("Health lock is poisoned")
            .get(member_id)
        {
            Some(health) => *health,
            None => return None,
        };
        let ml = self.members.read().expect("Member list lock is poisoned");
        match ml.get(member_id) {
            Some(member) => Some(Membership {
                health: mhealth,
                member: member.clone(),
            }),
            None => None,
        }
    }

    /// Returns the number of members.
    pub fn len(&self) -> usize {
        self.members
            .read()
            .expect("Member list lock is poisoned")
            .len()
    }

    /// A randomized list of members to check.
    pub fn check_list(&self, exclude_id: &str) -> Vec<Member> {
        let mut members: Vec<Member> = self
            .members
            .read()
            .expect("Member list lock is poisoned")
            .values()
            .filter(|v| v.id != exclude_id)
            .map(|v| v.clone())
            .collect();
        let mut rng = thread_rng();
        rng.shuffle(&mut members);
        members
    }

    /// Takes a function whose first argument is a member, and calls it for every pingreq target.
    pub fn with_pingreq_targets<F>(
        &self,
        sending_member_id: &str,
        target_member_id: &str,
        mut with_closure: F,
    ) -> ()
    where
        F: FnMut(Member) -> (),
    {
        // This will lead to nested read locks if you don't deal with making a copy
        let mut members: Vec<Member> = {
            let ml = self.members.read().expect("Member list lock is poisoned");
            ml.values().map(|v| v.clone()).collect()
        };
        let mut rng = thread_rng();
        rng.shuffle(&mut members);
        for member in members
            .into_iter()
            .filter(|m| {
                m.id != sending_member_id
                    && m.id != target_member_id
                    && self.check_health_of_by_id(&m.id, Health::Alive)
            })
            .take(PINGREQ_TARGETS)
        {
            with_closure(member);
        }
    }

    /// Takes a function whose argument is a `HashMap::Values` iterator, with the ID and Membership
    /// entry.
    pub fn with_member_iter<F>(&self, mut with_closure: F) -> ()
    where
        F: FnMut(hash_map::Values<String, Member>) -> (),
    {
        with_closure(
            self.members
                .read()
                .expect("Member list lock is poisoned")
                .values(),
        );
    }

    /// Takes a function whose argument is a reference to the member list hashmap.
    pub fn with_member_list<F>(&self, mut with_closure: F) -> ()
    where
        F: FnMut(&HashMap<String, Member>) -> (),
    {
        with_closure(
            self.members
                .read()
                .expect("Member list lock is poisoned")
                .deref(),
        );
    }

    /// Calls a function whose argument is a reference to a membership entry matching the given ID.
    pub fn with_member<F>(&self, member_id: &str, mut with_closure: F) -> ()
    where
        F: FnMut(Option<&Member>) -> (),
    {
        let ml = self.members.read().expect("Member list lock poisoned");
        let member = ml.get(member_id);
        with_closure(member);
    }

    /// Iterates over the member list, calling the function for each member.
    pub fn with_members<F>(&self, mut with_closure: F) -> ()
    where
        F: FnMut(&Member) -> (),
    {
        for member in self
            .members
            .read()
            .expect("Member list lock is poisoned")
            .values()
        {
            with_closure(member);
        }
    }

    /// Iterates over every suspected membership entry, calling the given closure.
    pub fn with_suspects<F>(&self, mut with_closure: F) -> ()
    where
        F: FnMut((&str, &SteadyTime)) -> (),
    {
        for (id, suspect) in self
            .suspect
            .read()
            .expect("Suspect list lock is poisoned")
            .iter()
        {
            with_closure((id, suspect));
        }
    }

    /// Iterates over every suspected membership entry, calling the given closure.
    pub fn with_departures<F>(&self, mut with_closure: F) -> ()
    where
        F: FnMut((&str, &SteadyTime)) -> (),
    {
        for (id, departure_time) in self
            .depart
            .read()
            .expect("Departure list lock is poisoned")
            .iter()
        {
            with_closure((id, departure_time));
        }
    }

    /// Expires a member from the suspect list.
    pub fn expire(&self, member_id: &str) {
        let mut suspects = self.suspect.write().expect("Suspect list lock is poisoned");
        suspects.remove(member_id);
    }

    /// Sets a departure time for a member who has been confirmed
    pub fn depart(&self, member_id: &str) {
        let mut depart = self
            .depart
            .write()
            .expect("Departure list lock is poisoned");
        depart.insert(member_id.to_string(), SteadyTime::now());
    }

    /// Removes a member from the departure list
    pub fn depart_remove(&self, member_id: &str) {
        let mut depart = self
            .depart
            .write()
            .expect("Departure list lock is poisoned");
        depart.remove(member_id);
    }

    pub fn contains_member(&self, member_id: &str) -> bool {
        self.members
            .read()
            .expect("Member list lock is poisoned")
            .contains_key(member_id)
    }
}

#[cfg(test)]
mod tests {
    mod member {
        use member::Member;

        // Sets the uuid to simple, and the incarnation to zero.
        #[test]
        fn new() {
            let member = Member::default();
            assert_eq!(member.id.len(), 32);
            assert_eq!(member.incarnation, 0);
        }
    }

    mod member_list {
        use member::{Health, Member, MemberList, PINGREQ_TARGETS};

        fn populated_member_list(size: u64) -> MemberList {
            let ml = MemberList::new();
            for _x in 0..size {
                let m = Member::default();
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
                ml.with_pingreq_targets(&from.id, &target.id, |_m| counter += 1);
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
                ml.with_pingreq_targets(&from.id, &target.id, |m| {
                    if m.id == from.id {
                        excluded_appears = true
                    }
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
                ml.with_pingreq_targets(&from.id, &target.id, |m| {
                    if m.id == target.id {
                        excluded_appears = true
                    }
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
                ml.with_pingreq_targets(&from.id, &target.id, |_m| counter += 1);
                assert_eq!(counter, 1);
            });
        }

        #[test]
        fn insert_no_member() {
            let ml = MemberList::new();
            let member = Member::default();
            let mcheck = member.clone();
            assert_eq!(ml.insert(member, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck, Health::Alive));
        }

        #[test]
        fn insert_existing_member_lower_incarnation() {
            let ml = MemberList::new();
            let member_one = Member::default();
            let mcheck_one = member_one.clone();
            let mut member_two = member_one.clone();
            member_two.incarnation = 1;
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Alive), true);
            ml.with_member(&mcheck_two.id, |m| assert_eq!(m.unwrap().incarnation, 1));
        }

        #[test]
        fn insert_existing_member_higher_incarnation() {
            let ml = MemberList::new();
            let mut member_one = Member::default();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            member_one.incarnation = 1;

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Alive), false);
            ml.with_member(&mcheck_two.id, |m| assert_eq!(m.unwrap().incarnation, 1));
        }

        #[test]
        fn insert_equal_incarnation_current_alive_new_alive() {
            let ml = MemberList::new();
            let member_one = Member::default();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Alive), false);
        }

        #[test]
        fn insert_equal_incarnation_current_alive_new_suspect() {
            let ml = MemberList::new();
            let member_one = Member::default();
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
            let member_one = Member::default();
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
            let member_one = Member::default();
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
            let member_one = Member::default();
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
            let member_one = Member::default();
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
            let member_one = Member::default();
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
            let member_one = Member::default();
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
            let member_one = Member::default();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Confirmed), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Confirmed));

            assert_eq!(ml.insert(member_two, Health::Confirmed), false);
            assert!(ml.check_health_of(&mcheck_two, Health::Confirmed));
        }

        #[test]
        fn insert_equal_incarnation_current_alive_new_departed() {
            let ml = MemberList::new();
            let member_one = Member::default();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Alive), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Alive));

            assert_eq!(ml.insert(member_two, Health::Departed), true);
            assert!(ml.check_health_of(&mcheck_two, Health::Departed));
        }

        #[test]
        fn insert_equal_incarnation_current_suspect_new_departed() {
            let ml = MemberList::new();
            let member_one = Member::default();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Suspect), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Suspect));

            assert_eq!(ml.insert(member_two, Health::Departed), true);
            assert!(ml.check_health_of(&mcheck_two, Health::Departed));
        }

        #[test]
        fn insert_equal_incarnation_current_confirmed_new_departed() {
            let ml = MemberList::new();
            let member_one = Member::default();
            let mcheck_one = member_one.clone();
            let member_two = member_one.clone();
            let mcheck_two = member_two.clone();

            assert_eq!(ml.insert(member_one, Health::Confirmed), true);
            assert!(ml.check_health_of(&mcheck_one, Health::Confirmed));

            assert_eq!(ml.insert(member_two, Health::Departed), true);
            assert!(ml.check_health_of(&mcheck_two, Health::Departed));
        }

    }
}
