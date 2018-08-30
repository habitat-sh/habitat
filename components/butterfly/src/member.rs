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

use std::cmp;
use std::collections::{hash_map, HashMap};
use std::iter::IntoIterator;
use std::net::SocketAddr;
use std::result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use bytes::BytesMut;
use prost::Message as ProstMessage;
use rand::{thread_rng, Rng};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use time::{Duration, SteadyTime};
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
            // TODO (CM): DANGER DANGER DANGER
            // This is a lousy default, and suggests that the notion
            // of a "default Member" doesn't make much sense.
            //
            // (Port numbers of 0 are also problematic.)
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

            // This hurts so bad...
            //
            // Our "Member" protobuf is currently serving two
            // purposes. One is here, serving as the return address of
            // a Supervisor for a message. Another is serving as a
            // record of a known member of the Supervisor ring; this
            // data is piggy-backed on our core SWIM messages as a way
            // of introducing new members to existing network members.
            //
            // The thing is, depending on which case the Member struct
            // is being used for, it may or may not have an "address"
            // field. If it's as the return address, it's actually
            // getting the address from the networking layer; the
            // sending Supervisor doesn't actually have that
            // information.
            //
            // On the other hand, if it's an actual membership record,
            // then it _will_ have an address, which will ultimately
            // have been resolved at some point in the past by the
            // aforementioned method of relying on the networking
            // layer.
            //
            // The Prost migration introduced validation that wasn't
            // taking this into account; it assumed that we would
            // _always_ have a network address. This cause it to
            // essentially reject any messages from 0.59.0 (and
            // before) Supervisors, because they had no such
            // validation, and never set any value for a return
            // address.
            //
            // It was able to work with Supervisors _after_ the Prost
            // migration because we default to setting an empty string
            // for the address. This is arguably NOT the right thing
            // to do, since a value of `Some("")` is more dangerous than
            // a value of `None`. We ultimately need to either _not_
            // generate meaningless default values, or tease apart the
            // two uses of our Member protobuf, or both.
            address: proto.address.unwrap_or("".to_string()),

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

/// Tracks lists of members, their health, and how long they have been
/// suspect or confirmed.
#[derive(Debug, Clone)]
pub struct MemberList {
    pub members: Arc<RwLock<HashMap<UuidSimple, Member>>>,
    pub health: Arc<RwLock<HashMap<UuidSimple, Health>>>,
    /// Records timestamps of when Members are marked `Suspect`. This
    /// supports automatically transitioning them to `Confirmed` after
    /// an appropriate amount of time.
    aging_suspects: Arc<RwLock<HashMap<UuidSimple, SteadyTime>>>,
    /// Records timestamps of when Members are marked
    /// `Confirmed`. This supports automatically transitioning them to
    /// `Departed` after an appropriate amount of time.
    aging_confirmed: Arc<RwLock<HashMap<UuidSimple, SteadyTime>>>,
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
            // TODO (CM): why does this use a different ordering than
            // get_update_counter (and why doesn't it just use get_update_counter?)
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
            aging_suspects: Arc::new(RwLock::new(HashMap::new())),
            aging_confirmed: Arc::new(RwLock::new(HashMap::new())),
            initial_members: Arc::new(RwLock::new(Vec::new())),
            update_counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Increment the update counter for this store.
    ///
    /// We don't care if this repeats - it just needs to be unique for any given two states, which
    /// it will be.
    fn increment_update_counter(&self) {
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

    /// Inserts a member into the member list with the given health,
    /// but only if the criteria for insertion are met. Returns `true`
    /// if the rumor information was actually accepted, and `false`
    /// otherwise.
    ///
    /// There are a few rules governing how we choose to accept
    /// Membership rumors.
    ///
    /// First, if we have absolutely no record of having seen
    /// `incoming_member` before, we'll accept the rumor without
    /// question.
    ///
    /// On the other hand, if we _have_ seen `incoming_member` we need
    /// to compare the incoming information to what we currently have
    /// before we decide whether to accept the new information.
    ///
    /// If the incarnation number of the `incoming_member` is lower
    /// than that of the rumor we already have, then we reject
    /// it. Incarnation numbers for Members are only ever incremented
    /// by that Member itself, so the fact that we already have one
    /// that is higher means that we have more up-to-date information.
    ///
    /// Similarly, if the incoming incarnation number is greater than
    /// what we have, we'll accept it as more up-to-date information.
    ///
    /// If the incarnation numbers match, we need to look at the
    /// health to determine if we accept the rumor.
    ///
    /// We only accept the incoming rumor if its health is strictly
    /// "worse than" the health we currently have for the member.
    ///
    /// Alternatively, you can think of "worse than" as "greater
    /// than", given this ordering of Health states (this is governed
    /// by the `PartialOrd` and `Ord` trait implementations on `Health`):
    ///
    /// Alive < Suspect < Confirmed < Departed
    ///
    /// For example, if we think that "Supervisor X (at incarnation 1)
    /// is Alive", but the rumor is telling us that "Supervisor X (at
    /// incarnation 1) is Suspect", that means that whoever we've
    /// received this rumor from is having trouble contacting
    /// Supervisor X. We should accept this rumor and propagate it to
    /// ensure that a) information about degraded connectivity makes
    /// it around the network, and b) the odds that Supervisor X will
    /// receive this rumor increase, allowing it to refute this (if
    /// indeed Supervisor X is still around.)
    ///
    /// If we were to just accept the rumor regardless of what the
    /// health was, we could basically start "arguing" across the
    /// network; one Supervisor thinks X is Alive, another thinks it's
    /// Suspect, and we just keep flip-flopping between the two
    /// without any sort of resolution.
    ///
    /// Below is the truth table that illustrates this. "Current Health"
    /// is down the left side, while "Incoming Health" is across the
    /// top. We only propagate when Incoming is "worse than" Current:
    ///
    /// |           | Alive | Suspect   | Confirmed | Departed  |
    /// |-----------+-------+-----------+-----------+-----------|
    /// | Alive     |       | propagate | propagate | propagate |
    /// | Suspect   |       |           | propagate | propagate |
    /// | Confirmed |       |           |           | propagate |
    /// | Departed  |       |           |           |           |
    ///
    // TODO (CM): why don't we just insert a membership record here?
    pub fn insert(&self, incoming_member: Member, incoming_health: Health) -> bool {
        let accept_and_propagate_rumor = match self
            .members
            .read()
            .expect("Member List read lock poisoned")
            .get(&incoming_member.id)
        {
            None => true,
            Some(existing_member) => {
                match existing_member
                    .incarnation
                    .cmp(&incoming_member.incarnation)
                {
                    cmp::Ordering::Greater => false,
                    cmp::Ordering::Less => true,
                    cmp::Ordering::Equal => {
                        // We know we have a health if we have a
                        // record.
                        //
                        // (Doing this in two operations is necessary
                        // due to the lock and lifetimes.)
                        let health_lock = self.health.read().expect("Health lock is poisoned");
                        let existing_health = health_lock.get(&existing_member.id).expect(
                            "No health for a membership record should be impossible; did you use insert?",
                        );

                        incoming_health > *existing_health
                    }
                }
            }
        };

        if accept_and_propagate_rumor {
            let member_id = incoming_member.id.clone();
            self.members
                .write()
                .expect("Member list lock is poisoned")
                .insert(incoming_member.id.clone(), incoming_member);
            let updated = self.insert_health_by_id(&member_id, incoming_health);
            if !updated {
                // TODO (CM): clean this logic up better

                // insert_health_by_id calls increment_update_counter,
                // but depending on the current state and the incoming
                // rumor, it may not actually change the health
                // recorded. In that case, we need to ensure that we
                // increment the update counter to reflect that this
                // rumor was accepted and should be propagated
                self.increment_update_counter();
            }
        }

        accept_and_propagate_rumor
    }

    // TODO (CM): Should there be an invariant that you must refer to
    // an existing member in order to run this?

    /// Updates the health of a member without touching the member itself. Returns true if the
    /// health changed, false otherwise. As health transitions from
    /// state to state, appropriate bookkeeping is done to ensure that
    /// Suspect and Confirmed rumors can properly time out to
    /// Confirmed and Departed rumors, respectively.
    pub fn insert_health_by_id(&self, member_id: &str, health: Health) -> bool {
        // If we already have a health record for this member, then we
        // need to check what that health is.
        if let Some(current_health) = self
            .health
            .read()
            .expect("Health read lock is poisoned")
            .get(member_id)
        {
            if *current_health == health {
                // Health did not change; there's nothing else to do,
                // so bail out.
                return false;
            }

            // If we are transitioning away from Suspect or Confirmed,
            // we should stop timing how long we have been in that state.
            match *current_health {
                Health::Suspect => Some(&self.aging_suspects),
                Health::Confirmed => Some(&self.aging_confirmed),
                _ => None,
            }.and_then(|map| {
                map.write()
                    .expect("aging lock is poisoned")
                    .remove(member_id)
            });
        }

        // Whether we have seen this member before or not, we now need
        // to record the time it entered into the Suspect or Confirmed
        // states.
        match health {
            Health::Suspect => Some(&self.aging_suspects),
            Health::Confirmed => Some(&self.aging_confirmed),
            _ => None,
        }.and_then(|map| {
            map.write()
                .expect("aging lock is poisoned")
                .insert(member_id.to_string(), SteadyTime::now())
        });

        // Finally, we record the new health.
        self.health
            .write()
            .expect("Health write lock is poisoned")
            .insert(String::from(member_id), health);

        self.increment_update_counter();
        true
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

    // TODO (CM): accept AsRef<MemberId> here (and implement that on
    // Member, as well as creating a MemberId type)... this would
    // allow us to consolidate check_health_of and check_health_of_by_id

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

    /// Query the list of aging Suspect members to find those which
    /// have now expired to Confirmed. Health is updated
    /// appropriately, and a list of newly-Confirmed Member IDs is
    /// returned.
    pub fn members_expired_to_confirmed(&self, timeout: Duration) -> Vec<String> {
        self.members_expired_to(Health::Confirmed, timeout)
    }

    /// Query the list of aging Confirmed members to find those which
    /// have now expired to Departed. Health is updated appropriately,
    /// and a list of newly-Departed Member IDs is returned.
    pub fn members_expired_to_departed(&self, timeout: Duration) -> Vec<String> {
        self.members_expired_to(Health::Departed, timeout)
    }

    /// Return the member IDs of all members that have "timed out" to
    /// the `expiring_to` `Health`.
    ///
    /// For instance,
    ///
    ///   members_expired_to(Health::Departed, timeout)
    ///
    /// will return the IDs of those members that have been
    /// `Confirmed` for longer than the given `timeout`.
    ///
    /// The newly-updated health status is recorded properly.
    // TODO (CM): Better return type than Vec<String>
    fn members_expired_to(&self, expiring_to: Health, timeout: Duration) -> Vec<String> {
        let now = SteadyTime::now();
        let mut expired = Vec::new();

        let population = match expiring_to {
            Health::Confirmed => &self.aging_suspects,
            Health::Departed => &self.aging_confirmed,
            _ => {
                // Note: this shouldn't ever be called
                return expired;
            }
        };

        population.write().expect("aging lock is poisoned").retain(
            |ref member_id, ref starting_timestamp| {
                if now >= **starting_timestamp + timeout {
                    expired.push(member_id.to_string());
                    false
                } else {
                    true
                }
            },
        );

        for id in expired.iter() {
            self.insert_health_by_id(id, expiring_to);
        }
        expired
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
        fn insert_several_members() {
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

        /// Tests of MemberList::insert
        mod insert {
            use member::{Health, Member, MemberList};

            fn assert_cannot_insert_member_rumor_of_lower_incarnation(
                from_health: Health,
                to_health: Health,
            ) {
                let ml = MemberList::new();
                let initial_update_counter_value = ml.get_update_counter();
                let initial_incarnation = 10; // just to pick a number

                let member = {
                    let mut m = Member::default();
                    m.incarnation = initial_incarnation;
                    m
                };

                assert!(
                    ml.insert(member.clone(), from_health.clone()),
                    "Could not insert member into list initially"
                );

                let update_counter_value_checkpoint_1 = ml.get_update_counter();
                assert_eq!(
                    update_counter_value_checkpoint_1,
                    initial_update_counter_value + 1,
                    "Update counter should have incremented by one"
                );

                assert!(
                    ml.check_health_of(&member, from_health.clone()),
                    "Member should have had health {:?}, but didn't",
                    from_health
                );

                let member_with_lower_incarnation = {
                    let mut m = member.clone();
                    m.incarnation = m.incarnation - 1;
                    m
                };

                assert!(
                    !ml.insert(member_with_lower_incarnation, to_health),
                    "Inserting with {:?}->{:?} should be a no-op with a lower incarnation number",
                    from_health,
                    to_health
                );
                assert_eq!(
                ml.get_update_counter(),
                update_counter_value_checkpoint_1,
                "Update counter should not have been incremented after trying to insert a lower-incarnation-number rumor"
            );
                assert!(
                ml.check_health_of(&member, from_health.clone()),
                "Member should have still have health {:?} following attempt to insert lower-incarnation-number rumor, but didn't",
                from_health
            );
            }

            macro_rules! lower_incarnation {
                // Unfortunately, Rust macros currently can't be used to generate
                // the name of a function, so we have to provide one :(
                ($fn_name:ident, $from:expr, $to:expr) => {
                    #[test]
                    fn $fn_name() {
                        assert_cannot_insert_member_rumor_of_lower_incarnation($from, $to);
                    }
                };
            }

            lower_incarnation!(lower_a_to_a, Health::Alive, Health::Alive);
            lower_incarnation!(lower_a_to_s, Health::Alive, Health::Suspect);
            lower_incarnation!(lower_a_to_c, Health::Alive, Health::Confirmed);
            lower_incarnation!(lower_a_to_d, Health::Alive, Health::Departed);

            lower_incarnation!(lower_s_to_a, Health::Suspect, Health::Alive);
            lower_incarnation!(lower_s_to_s, Health::Suspect, Health::Suspect);
            lower_incarnation!(lower_s_to_c, Health::Suspect, Health::Confirmed);
            lower_incarnation!(lower_s_to_d, Health::Suspect, Health::Departed);

            lower_incarnation!(lower_c_to_a, Health::Confirmed, Health::Alive);
            lower_incarnation!(lower_c_to_s, Health::Confirmed, Health::Suspect);
            lower_incarnation!(lower_c_to_c, Health::Confirmed, Health::Confirmed);
            lower_incarnation!(lower_c_to_d, Health::Confirmed, Health::Departed);

            lower_incarnation!(lower_d_to_a, Health::Departed, Health::Alive);
            lower_incarnation!(lower_d_to_s, Health::Departed, Health::Suspect);
            lower_incarnation!(lower_d_to_c, Health::Departed, Health::Confirmed);
            lower_incarnation!(lower_d_to_d, Health::Departed, Health::Departed);

            fn assert_always_insert_member_rumor_of_higher_incarnation(
                from_health: Health,
                to_health: Health,
            ) {
                let ml = MemberList::new();
                let initial_update_counter_value = ml.get_update_counter();
                let initial_incarnation = 10; // just to pick a number

                let member = {
                    let mut m = Member::default();
                    m.incarnation = initial_incarnation;
                    m
                };

                assert!(
                    ml.insert(member.clone(), from_health.clone()),
                    "Could not insert member into list initially"
                );

                let update_counter_value_checkpoint_1 = ml.get_update_counter();
                assert_eq!(
                    update_counter_value_checkpoint_1,
                    initial_update_counter_value + 1,
                    "Update counter should have incremented by one"
                );

                assert!(
                    ml.check_health_of(&member, from_health.clone()),
                    "Member should have had health {:?}, but didn't",
                    from_health
                );

                let member_with_higher_incarnation = {
                    let mut m = member.clone();
                    m.incarnation = m.incarnation + 1;
                    m
                };

                assert!(
                ml.insert(member_with_higher_incarnation, to_health),
                "Inserting with {:?}->{:?} should be always work with a higher incarnation number",
                from_health,
                to_health
            );
                assert_eq!(
                ml.get_update_counter(),
                update_counter_value_checkpoint_1 + 1,
                "Update counter should increment by 1 when inserting a higher-incarnation-number rumor"
            );
                assert!(
                ml.check_health_of(&member, to_health.clone()),
                "Member should have health {:?} following insertion of higher-incarnation-number rumor",
                to_health
            );
            }

            macro_rules! higher_incarnation {
                // Unfortunately, Rust macros currently can't be used to generate
                // the name of a function, so we have to provide one :(
                ($fn_name:ident, $from:expr, $to:expr) => {
                    #[test]
                    fn $fn_name() {
                        assert_always_insert_member_rumor_of_higher_incarnation($from, $to);
                    }
                };
            }

            higher_incarnation!(higher_a_to_a, Health::Alive, Health::Alive);
            higher_incarnation!(higher_a_to_s, Health::Alive, Health::Suspect);
            higher_incarnation!(higher_a_to_c, Health::Alive, Health::Confirmed);
            higher_incarnation!(higher_a_to_d, Health::Alive, Health::Departed);

            higher_incarnation!(higher_s_to_a, Health::Suspect, Health::Alive);
            higher_incarnation!(higher_s_to_s, Health::Suspect, Health::Suspect);
            higher_incarnation!(higher_s_to_c, Health::Suspect, Health::Confirmed);
            higher_incarnation!(higher_s_to_d, Health::Suspect, Health::Departed);

            higher_incarnation!(higher_c_to_a, Health::Confirmed, Health::Alive);
            higher_incarnation!(higher_c_to_s, Health::Confirmed, Health::Suspect);
            higher_incarnation!(higher_c_to_c, Health::Confirmed, Health::Confirmed);
            higher_incarnation!(higher_c_to_d, Health::Confirmed, Health::Departed);

            higher_incarnation!(higher_d_to_a, Health::Departed, Health::Alive);
            higher_incarnation!(higher_d_to_s, Health::Departed, Health::Suspect);
            higher_incarnation!(higher_d_to_c, Health::Departed, Health::Confirmed);
            higher_incarnation!(higher_d_to_d, Health::Departed, Health::Departed);

            fn assert_only_insert_member_rumor_of_same_incarnation_if_health_is_worse(
                from_health: Health,
                to_health: Health,
            ) {
                let ml = MemberList::new();
                let initial_update_counter_value = ml.get_update_counter();
                let initial_incarnation = 10; // just to pick a number

                let member = {
                    let mut m = Member::default();
                    m.incarnation = initial_incarnation;
                    m
                };

                assert!(
                    ml.insert(member.clone(), from_health.clone()),
                    "Could not insert member into list initially"
                );

                let update_counter_value_checkpoint_1 = ml.get_update_counter();
                assert_eq!(
                    update_counter_value_checkpoint_1,
                    initial_update_counter_value + 1,
                    "Update counter should have incremented by one"
                );

                assert!(
                    ml.check_health_of(&member, from_health.clone()),
                    "Member should have had health {:?}, but didn't",
                    from_health
                );

                let member_with_same_incarnation = member.clone();

                if to_health > from_health {
                    assert!(
                    ml.insert(member_with_same_incarnation, to_health),
                    "Inserting with {:?}->{:?} should work with an identical incarnation number",
                    from_health,
                    to_health
                );
                    assert_eq!(
                    ml.get_update_counter(),
                    update_counter_value_checkpoint_1 + 1,
                    "Update counter should increment by 1 when inserting a same-incarnation-number rumor with worse health"
                );
                    assert!(
                    ml.check_health_of(&member, to_health.clone()),
                    "Member should have health {:?} following insertion of same-incarnation-number rumor with worse health",
                    to_health
                );
                } else {
                    assert!(
                    !ml.insert(member_with_same_incarnation, to_health),
                    "Inserting with {from:?}->{to:?} should never work with an identical incarnation number, because {to:?} is not \"worse than\" {from:?}",
                    from=from_health,
                    to=to_health
                );
                    assert_eq!(
                    ml.get_update_counter(),
                    update_counter_value_checkpoint_1,
                    "Update counter should not increment when inserting a same-incarnation-number rumor without worse health"
                );
                    assert!(
                    ml.check_health_of(&member, from_health.clone()),
                    "Member should still have health {:?} following insertion of same-incarnation-number rumor without worse health",
                    from_health
                );
                }
            }

            macro_rules! same_incarnation {
                // Unfortunately, Rust macros currently can't be used to generate
                // the name of a function, so we have to provide one :(
                ($fn_name:ident, $from:expr, $to:expr) => {
                    #[test]
                    fn $fn_name() {
                        assert_only_insert_member_rumor_of_same_incarnation_if_health_is_worse(
                            $from, $to,
                        );
                    }
                };
            }

            same_incarnation!(same_a_to_a, Health::Alive, Health::Alive);
            same_incarnation!(same_a_to_s, Health::Alive, Health::Suspect);
            same_incarnation!(same_a_to_c, Health::Alive, Health::Confirmed);
            same_incarnation!(same_a_to_d, Health::Alive, Health::Departed);

            same_incarnation!(same_s_to_a, Health::Suspect, Health::Alive);
            same_incarnation!(same_s_to_s, Health::Suspect, Health::Suspect);
            same_incarnation!(same_s_to_c, Health::Suspect, Health::Confirmed);
            same_incarnation!(same_s_to_d, Health::Suspect, Health::Departed);

            same_incarnation!(same_c_to_a, Health::Confirmed, Health::Alive);
            same_incarnation!(same_c_to_s, Health::Confirmed, Health::Suspect);
            same_incarnation!(same_c_to_c, Health::Confirmed, Health::Confirmed);
            same_incarnation!(same_c_to_d, Health::Confirmed, Health::Departed);

            same_incarnation!(same_d_to_a, Health::Departed, Health::Alive);
            same_incarnation!(same_d_to_s, Health::Departed, Health::Suspect);
            same_incarnation!(same_d_to_c, Health::Departed, Health::Confirmed);
            same_incarnation!(same_d_to_d, Health::Departed, Health::Departed);
        }

        /// Tests of MemberList::insert_health_by_id
        mod insert_health_by_id {
            use member::{Health, Member, MemberList};

            /// Tests that the transition from `from_health` to `to_health` for
            /// `insert_health_by_id` works properly.
            fn assert_insert_health_by_id_transition(from_health: Health, to_health: Health) {
                let ml = MemberList::new();
                let member_one = Member::default();

                assert!(
                    ml.insert_health_by_id(&member_one.id, from_health.clone()),
                    "Should be able to insert initial health of {:?} into empty MemberList",
                    from_health
                );
                assert_eq!(
                    ml.health_of(&member_one).expect(
                        "Expected member to exist in health after initial insert, but it didn't"
                    ),
                    from_health,
                    "Member should have initial health {:?}",
                    from_health
                );

                let update_counter_before = ml.get_update_counter();

                {
                    let aging_suspects = ml.aging_suspects.read().unwrap();
                    match from_health {
                        Health::Suspect => {
                            assert!(
                                aging_suspects.contains_key(&member_one.id),
                                "{:?} member should be an aging Suspect initially",
                                from_health
                            );
                        }
                        _ => {
                            assert!(
                                !aging_suspects.contains_key(&member_one.id),
                                "{:?} member should not be an aging Suspect initially",
                                from_health
                            );
                        }
                    }
                }

                {
                    let aging_confirmed = ml.aging_confirmed.read().unwrap();
                    match from_health {
                        Health::Confirmed => {
                            assert!(
                                aging_confirmed.contains_key(&member_one.id),
                                "{:?} member should be an aging Confirmed initially",
                                from_health
                            );
                        }
                        _ => {
                            assert!(
                                !aging_confirmed.contains_key(&member_one.id),
                                "{:?} member should not be an aging Confirmed initially",
                                from_health
                            );
                        }
                    }
                }

                if from_health == to_health {
                    assert!(
                        !ml.insert_health_by_id(&member_one.id, to_health.clone()),
                        "Transitioning from {:?} to {:?} (i.e., no change) should be a no-op",
                        from_health,
                        to_health
                    );
                    assert_eq!(ml.get_update_counter(), update_counter_before,
                           "Transitioning from {:?} to {:?} (i.e., no change) should not increment update counter",
                           from_health,
                           to_health);
                    assert_eq!(
                        ml.health_of(&member_one).expect(
                            "Expected member to exist in health after update, but it didn't"
                        ),
                        from_health,
                        "Member should have still have initial health {:?}",
                        from_health
                    );
                } else {
                    assert!(
                    ml.insert_health_by_id(&member_one.id, to_health.clone()),
                    "Transitioning from {:?} to {:?} (i.e., different health) should NOT be a no-op",
                    from_health,
                    to_health
                );
                    assert_eq!(ml.get_update_counter(), update_counter_before + 1,
                           "Transitioning from {:?} to {:?} (i.e., different health) should increment update counter by one",
                           from_health, to_health
                );
                    assert_eq!(
                        ml.health_of(&member_one).expect(
                            "Expected member to exist in health after update, but it didn't"
                        ),
                        to_health,
                        "Member should have changed health from {:?} to {:?}",
                        from_health,
                        to_health
                    );
                }

                {
                    let aging_suspects = ml.aging_suspects.read().unwrap();
                    match to_health {
                        Health::Suspect => {
                            assert!(
                                aging_suspects.contains_key(&member_one.id),
                                "{:?} member should be an aging Suspect after update to {:?}",
                                from_health,
                                to_health
                            );
                        }
                        _ => {
                            assert!(
                                !aging_suspects.contains_key(&member_one.id),
                                "{:?} member should not be an aging Suspect after update to {:?}",
                                from_health,
                                to_health
                            );
                        }
                    }
                }

                {
                    let aging_confirmed = ml.aging_confirmed.read().unwrap();
                    match to_health {
                        Health::Confirmed => {
                            assert!(
                                aging_confirmed.contains_key(&member_one.id),
                                "{:?} member should be an aging Confirmed after update to {:?}",
                                from_health,
                                to_health
                            );
                        }
                        _ => {
                            assert!(
                                !aging_confirmed.contains_key(&member_one.id),
                                "{:?} member should not be an aging Confirmed after update to {:?}",
                                from_health,
                                to_health
                            );
                        }
                    }
                }
            }

            macro_rules! transition {
                // Unfortunately, Rust macros currently can't be used to generate
                // the name of a function, so we have to provide one :(
                ($fn_name:ident, $from:expr, $to:expr) => {
                    #[test]
                    fn $fn_name() {
                        assert_insert_health_by_id_transition($from, $to);
                    }
                };
            }

            transition!(a_to_a, Health::Alive, Health::Alive);
            transition!(a_to_s, Health::Alive, Health::Suspect);
            transition!(a_to_c, Health::Alive, Health::Confirmed);
            transition!(a_to_d, Health::Alive, Health::Departed);

            transition!(s_to_a, Health::Suspect, Health::Alive);
            transition!(s_to_s, Health::Suspect, Health::Suspect);
            transition!(s_to_c, Health::Suspect, Health::Confirmed);
            transition!(s_to_d, Health::Suspect, Health::Departed);

            transition!(c_to_a, Health::Confirmed, Health::Alive);
            transition!(c_to_s, Health::Confirmed, Health::Suspect);
            transition!(c_to_c, Health::Confirmed, Health::Confirmed);
            transition!(c_to_d, Health::Confirmed, Health::Departed);

            transition!(d_to_a, Health::Departed, Health::Alive);
            transition!(d_to_s, Health::Departed, Health::Suspect);
            transition!(d_to_c, Health::Departed, Health::Confirmed);
            transition!(d_to_d, Health::Departed, Health::Departed);
        }

        /// Testing of
        ///
        /// - MemberList::members_expired_to_confirmed
        /// - MemberList::members_expired_to_departed
        mod timed_expiration {
            use member::{Health, Member, MemberList};
            use std::thread;
            use std::time::Duration as StdDuration;
            use time::Duration;

            #[test]
            fn timing_out_from_suspect_to_confirmed() {
                let ml = MemberList::new();
                let member_one = Member::default();
                let small_seconds = 1;
                let large_seconds = 100_000;

                // TODO (CM): OMG, use only one kind of Duration, pleeeeeeeease
                let small_timeout =
                    Duration::from_std(StdDuration::from_secs(small_seconds)).unwrap();
                let large_timeout =
                    Duration::from_std(StdDuration::from_secs(large_seconds)).unwrap();

                assert!(ml.members_expired_to_confirmed(small_timeout).is_empty(),
                        "An empty MemberList shouldn't have anything that's timing out to being Confirmed");

                assert!(ml.insert(member_one.clone(), Health::Alive));

                {
                    let aging_suspects = ml.aging_suspects.read().unwrap();
                    assert!(
                        aging_suspects.is_empty(),
                        "Member should not be considered an aging Suspect if it's Alive"
                    );
                }

                assert!(
                    ml.members_expired_to_confirmed(small_timeout).is_empty(),
                    "Should be no newly Confirmed members when they're all Alive"
                );

                assert!(ml.insert_health_by_id(&member_one.id, Health::Suspect));

                {
                    let aging_suspects = ml.aging_suspects.read().unwrap();
                    assert!(
                        aging_suspects.contains_key(&member_one.id),
                        "Member should be in aging_suspects after transitioning to Suspect"
                    );
                }

                assert!(
                    ml.members_expired_to_confirmed(large_timeout).is_empty(),
                    "Nothing should have timed out to Confirmed with a large timeout"
                );

                // Allow the Suspect to age
                thread::sleep(StdDuration::from_secs(small_seconds));

                let newly_confirmed = ml.members_expired_to_confirmed(small_timeout);
                assert!(
                    newly_confirmed.contains(&member_one.id),
                    "Member should be newly Confirmed after timing out"
                );

                assert!(
                    ml.check_health_of(&member_one, Health::Confirmed),
                    "Member should have a health of Confirmed after timing out"
                );

                {
                    let aging_suspects = ml.aging_suspects.read().unwrap();
                    assert!(
                        !aging_suspects.contains_key(&member_one.id),
                        "Member should no longer be considered an aging Suspect after timing out to Confirmed"
                    );
                }

                {
                    let aging_confirmed = ml.aging_confirmed.read().unwrap();
                    assert!(
                        aging_confirmed.contains_key(&member_one.id),
                        "Member should be considered an aging Confirmed after timing out to Confirmed"
                    );
                }
            }

            #[test]
            fn timing_out_from_confirmed_to_departed() {
                let ml = MemberList::new();
                let member_one = Member::default();
                let small_seconds = 1;
                let large_seconds = 100_000;

                // TODO (CM): OMG, use only one kind of Duration, pleeeeeeeease
                let small_timeout =
                    Duration::from_std(StdDuration::from_secs(small_seconds)).unwrap();
                let large_timeout =
                    Duration::from_std(StdDuration::from_secs(large_seconds)).unwrap();

                assert!(ml.members_expired_to_departed(small_timeout).is_empty(),
                        "An empty MemberList shouldn't have anything that's timing out to being Departed");

                assert!(ml.insert(member_one.clone(), Health::Alive));
                {
                    let aging_confirmed = ml.aging_confirmed.read().unwrap();
                    assert!(
                        aging_confirmed.is_empty(),
                        "Member should not be considered an aging Confirmed if it's Alive"
                    );
                }
                assert!(
                    ml.members_expired_to_departed(small_timeout).is_empty(),
                    "Should be no newly Departed members when they're all Alive"
                );

                assert!(ml.insert_health_by_id(&member_one.id, Health::Suspect));
                {
                    let aging_confirmed = ml.aging_confirmed.read().unwrap();
                    assert!(
                        aging_confirmed.is_empty(),
                        "Member should not be considered an aging Confirmed if it's Suspect"
                    );
                }
                assert!(
                    ml.members_expired_to_departed(small_timeout).is_empty(),
                    "Should be no newly Departed members when they're all Confirmed"
                );

                assert!(ml.insert_health_by_id(&member_one.id, Health::Confirmed));
                {
                    let aging_confirmed = ml.aging_confirmed.read().unwrap();
                    assert!(
                        aging_confirmed.contains_key(&member_one.id),
                        "Member should be in aging_confirmed after transitioning to Confirmed"
                    );
                }

                assert!(
                    ml.members_expired_to_departed(small_timeout).is_empty(),
                    "Should be no newly Departed members when they're all Confirmed"
                );

                assert!(
                    ml.members_expired_to_departed(large_timeout).is_empty(),
                    "Nothing should have timed out to Departed with a large timeout"
                );

                // Allow the Confirmed to age
                thread::sleep(StdDuration::from_secs(small_seconds));

                let newly_departed = ml.members_expired_to_departed(small_timeout);
                assert!(
                    newly_departed.contains(&member_one.id),
                    "Member should be newly Departed after timing out"
                );

                assert!(
                    ml.check_health_of(&member_one, Health::Departed),
                    "Member should have a health of Departed after timing out"
                );
                {
                    let aging_confirmed = ml.aging_confirmed.read().unwrap();
                    assert!(
                        !aging_confirmed.contains_key(&member_one.id),
                        "Member should no longer be considered an aging Confirmed after timing out to Departed"
                    );
                }
            }

            #[test]
            fn suspect_timeout_is_appropriately_selective() {
                let ml = MemberList::new();
                let member_1 = Member::default();
                let member_2 = Member::default();
                let member_3 = Member::default();

                assert!(ml.insert(member_1.clone(), Health::Suspect));
                thread::sleep(StdDuration::from_secs(1));
                assert!(ml.insert(member_2.clone(), Health::Suspect));
                thread::sleep(StdDuration::from_secs(2)); // Give us a bit of padding
                assert!(ml.insert(member_3.clone(), Health::Suspect));

                let timeout = Duration::from_std(StdDuration::from_secs(2)).unwrap();

                let newly_confirmed = ml.members_expired_to_confirmed(timeout);
                assert!(
                    newly_confirmed.contains(&member_1.id),
                    "Member 1 should be newly Confirmed after timing out"
                );
                assert!(
                    newly_confirmed.contains(&member_2.id),
                    "Member 2 should be newly Confirmed after timing out"
                );
                assert!(
                    !newly_confirmed.contains(&member_3.id),
                    "Member 3 should NOT be newly Confirmed, because it hasn't timed out yet"
                );

                assert!(
                    ml.check_health_of(&member_1, Health::Confirmed),
                    "Member 1 should have a health of Confirmed after timing out"
                );
                assert!(
                    ml.check_health_of(&member_2, Health::Confirmed),
                    "Member 2 should have a health of Confirmed after timing out"
                );
                assert!(
                    ml.check_health_of(&member_3, Health::Suspect),
                    "Member 3 should still have a health of Suspect, because it hasn't timed out yet"
                );
            }

            #[test]
            fn confirmed_timeout_is_appropriately_selective() {
                let ml = MemberList::new();
                let member_1 = Member::default();
                let member_2 = Member::default();
                let member_3 = Member::default();

                assert!(ml.insert(member_1.clone(), Health::Confirmed));
                thread::sleep(StdDuration::from_secs(1));
                assert!(ml.insert(member_2.clone(), Health::Confirmed));
                thread::sleep(StdDuration::from_secs(2)); // Give us a bit of padding
                assert!(ml.insert(member_3.clone(), Health::Confirmed));

                let timeout = Duration::from_std(StdDuration::from_secs(2)).unwrap();

                let newly_departed = ml.members_expired_to_departed(timeout);
                assert!(
                    newly_departed.contains(&member_1.id),
                    "Member 1 should be newly Departed after timing out"
                );
                assert!(
                    newly_departed.contains(&member_2.id),
                    "Member 2 should be newly Departed after timing out"
                );
                assert!(
                    !newly_departed.contains(&member_3.id),
                    "Member 3 should NOT be newly Departed, because it hasn't timed out yet"
                );

                assert!(
                    ml.check_health_of(&member_1, Health::Departed),
                    "Member 1 should have a health of Departed after timing out"
                );
                assert!(
                    ml.check_health_of(&member_2, Health::Departed),
                    "Member 2 should have a health of Departed after timing out"
                );
                assert!(
                    ml.check_health_of(&member_3, Health::Confirmed),
                    "Member 3 should still have a health of Confirmed, because it hasn't timed out yet"
                );
            }

        }
    }
}
