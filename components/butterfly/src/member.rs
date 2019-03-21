//! Tracks membership. Contains both the `Member` struct and the `MemberList`.

use std::{collections::{hash_map,
                        HashMap},
          fmt,
          net::SocketAddr,
          num::ParseIntError,
          ops::Add,
          result,
          str::FromStr,
          sync::{atomic::{AtomicUsize,
                          Ordering},
                 RwLock}};

use habitat_core::util::ToI64;
use prometheus::IntGaugeVec;
use rand::{seq::{IteratorRandom,
                 SliceRandom},
           thread_rng};
use serde::{de,
            ser::{SerializeMap,
                  SerializeStruct},
            Deserialize,
            Deserializer,
            Serialize,
            Serializer};
use time::{Duration,
           SteadyTime};
use uuid::Uuid;

pub use crate::protocol::swim::Health;
use crate::{error::{Error,
                    Result},
            protocol::{self,
                       newscast,
                       swim as proto,
                       FromProto},
            rumor::{RumorKey,
                    RumorPayload,
                    RumorType}};

/// How many nodes do we target when we need to run PingReq.
const PINGREQ_TARGETS: usize = 5;

lazy_static! {
    static ref PEER_HEALTH_COUNT: IntGaugeVec =
        register_int_gauge_vec!("hab_butterfly_peer_health_total",
                                "Number of butterfly peers",
                                &["health"]).unwrap();
}

/// Wraps a `u64` to represent the "incarnation number" of a
/// `Member`. Incarnation numbers can only ever be incremented.
///
/// Note: we're intentionally deriving `Copy` to be able to treat this
/// like a "normal" numeric type.
#[derive(Clone, Debug, Ord, PartialEq, PartialOrd, Eq, Copy)]
pub struct Incarnation(u64);

impl Default for Incarnation {
    fn default() -> Self { Incarnation(0) }
}

impl From<u64> for Incarnation {
    fn from(num: u64) -> Self { Incarnation(num) }
}

impl Incarnation {
    pub fn to_u64(self) -> u64 { self.0 }

    pub fn to_i64(self) -> i64 { self.0.to_i64() }
}

impl fmt::Display for Incarnation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

impl Add<u64> for Incarnation {
    type Output = Incarnation;

    fn add(self, other: u64) -> Incarnation { Incarnation(self.0 + other) }
}

impl Serialize for Incarnation {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_u64(self.0)
    }
}

impl FromStr for Incarnation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let raw = s.parse::<u64>()?;
        Ok(Incarnation(raw))
    }
}

struct IncarnationVisitor;

impl<'de> de::Visitor<'de> for IncarnationVisitor {
    type Value = Incarnation;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a u64")
    }

    fn visit_u64<E>(self, v: u64) -> result::Result<Self::Value, E>
        where E: de::Error
    {
        Ok(Incarnation::from(v))
    }
}

impl<'de> Deserialize<'de> for Incarnation {
    fn deserialize<D>(deserializer: D) -> result::Result<Incarnation, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_u64(IncarnationVisitor)
    }
}

// This is a Uuid type turned to a string
pub type UuidSimple = String;

/// A member in the swim group. Passes most of its functionality along to the internal protobuf
/// representation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Member {
    pub id:          String,
    pub incarnation: Incarnation,
    pub address:     String,
    pub swim_port:   u16,
    pub gossip_port: u16,
    pub persistent:  bool,
    pub departed:    bool,
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
        Member { id:          Uuid::new_v4().to_simple_ref().to_string(),
                 incarnation: Incarnation::default(),
                 // TODO (CM): DANGER DANGER DANGER
                 // This is a lousy default, and suggests that the notion
                 // of a "default Member" doesn't make much sense.
                 //
                 // (Port numbers of 0 are also problematic.)
                 address:     String::default(),
                 swim_port:   0,
                 gossip_port: 0,
                 persistent:  false,
                 departed:    false, }
    }
}

impl From<Member> for RumorKey {
    fn from(member: Member) -> RumorKey { RumorKey::new(RumorType::Member, &member.id, "") }
}

impl<'a> From<&'a Member> for RumorKey {
    fn from(member: &'a Member) -> RumorKey { RumorKey::new(RumorType::Member, &member.id, "") }
}

impl<'a> From<&'a &'a Member> for RumorKey {
    fn from(member: &'a &'a Member) -> RumorKey { RumorKey::new(RumorType::Member, &member.id, "") }
}

impl From<Member> for proto::Member {
    fn from(value: Member) -> Self {
        proto::Member { id:          Some(value.id),
                        incarnation: Some(value.incarnation.to_u64()),
                        address:     Some(value.address),
                        swim_port:   Some(value.swim_port.into()),
                        gossip_port: Some(value.gossip_port.into()),
                        persistent:  Some(value.persistent),
                        departed:    Some(value.departed), }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub member: Member,
    pub health: Health,
}

impl Membership {
    /// See MemberList::insert
    fn newer_or_less_healthy_than(&self,
                                  other_incarnation: Incarnation,
                                  other_health: Health)
                                  -> bool {
        self.member.incarnation > other_incarnation
        || (self.member.incarnation == other_incarnation && self.health > other_health)
    }
}

impl protocol::Message<proto::Membership> for Membership {}

impl From<Membership> for proto::Membership {
    fn from(value: Membership) -> Self {
        proto::Membership { member: Some(value.member.into()),
                            health: Some(value.health as i32), }
    }
}

/// Since protobuf doesn't have support for 16-bit ints, we need to check that
/// we haven't received something illegal
fn as_port(x: i32) -> Option<u16> {
    const PORT_MIN: i32 = ::std::u16::MIN as i32;
    const PORT_MAX: i32 = ::std::u16::MAX as i32;

    match x {
        PORT_MIN..=PORT_MAX => Some(x as u16),
        _ => None,
    }
}

impl FromProto<proto::Member> for Member {
    fn from_proto(proto: proto::Member) -> Result<Self> {
        Ok(Member { id:          proto.id.ok_or(Error::ProtocolMismatch("id"))?,
                    incarnation: proto.incarnation
                                      .map_or_else(Incarnation::default, Incarnation::from),

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
                    address: proto.address.unwrap_or_default(),

                    swim_port:   proto.swim_port
                                      .and_then(as_port)
                                      .ok_or(Error::ProtocolMismatch("swim-port"))?,
                    gossip_port: proto.gossip_port
                                      .and_then(as_port)
                                      .ok_or(Error::ProtocolMismatch("gossip-port"))?,
                    persistent:  proto.persistent.unwrap_or(false),
                    departed:    proto.departed.unwrap_or(false), })
    }
}

impl FromProto<proto::Membership> for Membership {
    fn from_proto(proto: proto::Membership) -> Result<Self> {
        Ok(Membership { member: proto.member
                                     .ok_or(Error::ProtocolMismatch("member"))
                                     .and_then(Member::from_proto)?,
                        health: proto.health
                                     .and_then(Health::from_i32)
                                     .unwrap_or(Health::Alive), })
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

mod member_list {
    #[derive(Clone, Debug)]
    pub struct Entry {
        pub member:            super::Member,
        pub health:            super::Health,
        pub health_updated_at: super::SteadyTime,
    }
}

/// Tracks lists of members, their health, and how long they have been
/// suspect or confirmed.
#[derive(Debug)]
pub struct MemberList {
    entries:         RwLock<HashMap<UuidSimple, member_list::Entry>>,
    initial_members: RwLock<Vec<Member>>,
    update_counter:  AtomicUsize,
}

impl Serialize for MemberList {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("member_list", 4)?;

        // A hack to maintain backwards compatibility with the version
        // Of MemberList where members was a HashMap<UuidSimple, Member>
        // and health was a HashMap<UuidSimple, Health>
        let mut member_struct = HashMap::new();
        let mut health_struct = HashMap::new();
        for (id, member_list::Entry { member, health, .. }) in self.read_entries().iter() {
            member_struct.insert(id.clone(), member.clone());
            health_struct.insert(id.clone(), *health);
        }
        strukt.serialize_field("members", &member_struct)?;
        strukt.serialize_field("health", &health_struct)?;

        // TODO (CM): why does this use a different ordering than
        // get_update_counter (and why doesn't it just use get_update_counter?)
        let update_number = self.update_counter.load(Ordering::SeqCst);
        strukt.serialize_field("update_counter", &update_number)?;

        strukt.end()
    }
}

impl MemberList {
    /// Creates a new, empty, MemberList.
    pub fn new() -> MemberList {
        MemberList { entries:         RwLock::new(HashMap::new()),
                     initial_members: RwLock::new(Vec::new()),
                     update_counter:  AtomicUsize::new(0), }
    }

    fn read_entries(&self)
                    -> std::sync::RwLockReadGuard<'_, HashMap<UuidSimple, member_list::Entry>> {
        self.entries.read().expect("Members read lock")
    }

    fn write_entries(
        &self)
        -> std::sync::RwLockWriteGuard<'_, HashMap<UuidSimple, member_list::Entry>> {
        self.entries.write().expect("Members write lock")
    }

    fn initial_members_read(&self) -> std::sync::RwLockReadGuard<'_, Vec<Member>> {
        self.initial_members
            .read()
            .expect("Initial members read lock")
    }

    fn initial_members_write(&self) -> std::sync::RwLockWriteGuard<'_, Vec<Member>> {
        self.initial_members
            .write()
            .expect("Initial members write lock")
    }

    /// We don't care if this repeats - it just needs to be unique for any given two states, which
    /// it will be.
    fn increment_update_counter(&self) { self.update_counter.fetch_add(1, Ordering::Relaxed); }

    pub fn get_update_counter(&self) -> usize { self.update_counter.load(Ordering::Relaxed) }

    pub fn len_initial_members(&self) -> usize { self.initial_members_read().len() }

    pub fn add_initial_member(&self, member: Member) { self.initial_members_write().push(member); }

    pub fn set_initial_members(&self, members: Vec<Member>) {
        *self.initial_members_write() = members;
    }

    pub fn with_initial_members(&self, mut with_closure: impl FnMut(&Member)) {
        for member in self.initial_members_read().iter() {
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
    // TODO (CM): why don't we just insert a membership record here?
    pub fn insert(&self, incoming_member: Member, incoming_health: Health) -> bool {
        self.insert_membership(Membership { member: incoming_member,
                                            health: incoming_health, })
    }

    fn insert_membership(&self, incoming: Membership) -> bool {
        // Is this clone necessary, or can a key be a reference to a field contained in the value?
        // Maybe the members we store should not contain the ID to reduce the duplication?
        let modified = match self.write_entries().entry(incoming.member.id.clone()) {
            hash_map::Entry::Occupied(mut entry) => {
                let val = entry.get_mut();
                if incoming.newer_or_less_healthy_than(val.member.incarnation, val.health) {
                    *val = member_list::Entry { member:            incoming.member,
                                                health:            incoming.health,
                                                health_updated_at: SteadyTime::now(), };
                    true
                } else {
                    false
                }
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(member_list::Entry { member:            incoming.member,
                                                  health:            incoming.health,
                                                  health_updated_at: SteadyTime::now(), });
                true
            }
        };

        if modified {
            self.increment_update_counter();
            self.calculate_peer_health_metrics();
        }

        modified
    }

    pub fn set_departed(&self, member_id: &str) {
        if let Some(member_list::Entry { member, health, .. }) =
            self.write_entries().get_mut(member_id)
        {
            debug!("Setting health of {:?}, {} -> {}",
                   member,
                   health,
                   Health::Departed);
            *health = Health::Departed;
        } else {
            trace!("set_departed called on unknown member {}", member_id);
        }
    }

    fn calculate_peer_health_metrics(&self) {
        let mut health_counts: HashMap<Health, i64> = HashMap::new();

        for entry in self.read_entries().values() {
            *health_counts.entry(entry.health).or_insert(0) += 1;
        }

        for health in [Health::Alive,
                       Health::Suspect,
                       Health::Confirmed,
                       Health::Departed].iter()
        {
            PEER_HEALTH_COUNT.with_label_values(&[&health.to_string()])
                             .set(*health_counts.get(health).unwrap_or(&0));
        }
    }

    /// Returns the health of the member, if the member exists.
    pub fn health_of(&self, member: &Member) -> Option<Health> { self.health_of_by_id(&member.id) }

    /// Returns the health of the member, if the member exists.
    pub fn health_of_by_id(&self, member_id: &str) -> Option<Health> {
        self.read_entries()
            .get(member_id)
            .map(|member_list::Entry { health, .. }| *health)
    }

    /// Returns true if the member is alive, suspect, or persistent; used during the target
    /// selection phase of the outbound thread.
    pub fn pingable(&self, member: &Member) -> bool {
        if member.persistent {
            return true;
        }
        match self.health_of(member) {
            Some(Health::Alive) | Some(Health::Suspect) => true,
            _ => false,
        }
    }

    /// Returns true if we are pinging this member because they are persistent, but we think they
    /// are gone.
    pub fn persistent_and_confirmed(&self, member: &Member) -> bool {
        member.persistent && self.health_of(member) == Some(Health::Confirmed)
    }

    /// Returns a protobuf membership record for the given member id.
    pub fn membership_for(&self, member_id: &str) -> Option<Membership> {
        self.read_entries()
            .get(member_id)
            .map(|member_list::Entry { member, health, .. }| {
                Membership { member: member.clone(),
                             health: *health, }
            })
    }

    /// Returns the number of entries.
    pub fn len(&self) -> usize { self.read_entries().len() }

    pub fn is_empty(&self) -> bool { self.read_entries().is_empty() }

    /// A randomized list of members to check.
    pub fn check_list(&self, exclude_id: &str) -> Vec<Member> {
        let mut members: Vec<_> = self.read_entries()
                                      .values()
                                      .map(|member_list::Entry { member, .. }| member)
                                      .filter(|member| member.id != exclude_id)
                                      .cloned()
                                      .collect();
        members.shuffle(&mut thread_rng());
        members
    }

    /// Takes a function whose first argument is a member, and calls it for every pingreq target.
    pub fn with_pingreq_targets(&self,
                                sending_member_id: &str,
                                target_member_id: &str,
                                mut with_closure: impl FnMut(&Member)) {
        for member_list::Entry { member, .. } in
            self.read_entries()
                .values()
                .filter(|member_list::Entry { member, health, .. }| {
                    member.id != sending_member_id
                    && member.id != target_member_id
                    && *health == Health::Alive
                })
                .choose_multiple(&mut thread_rng(), PINGREQ_TARGETS)
        {
            with_closure(member);
        }
    }

    /// If an owned `Member` is required, use this. If a shared reference is
    /// good enough, use `with_member`.
    pub fn get_cloned(&self, member_id: &str) -> Option<Member> {
        self.read_entries()
            .get(member_id)
            .map(|member_list::Entry { member, .. }| member.clone())
    }

    /// Calls a function whose argument is a reference to a membership entry matching the given ID.
    pub fn with_member(&self, member_id: &str, mut with_closure: impl FnMut(Option<&Member>)) {
        with_closure(self.read_entries()
                         .get(member_id)
                         .map(|member_list::Entry { member, .. }| member));
    }

    /// Iterates over the member list, calling the function for each member.
    pub fn with_members(&self, mut with_closure: impl FnMut(&Member)) {
        for member_list::Entry { member, .. } in self.read_entries().values() {
            with_closure(member);
        }
    }

    // This could be Result<T> instead, but there's only the one caller now
    pub fn with_memberships(&self,
                            mut with_closure: impl FnMut(Membership) -> Result<u64>)
                            -> Result<u64> {
        let mut ok: Result<u64> = Ok(0);
        for membership in self.read_entries()
                              .values()
                              .map(|member_list::Entry { member, health, .. }| {
                                  Membership { member: member.clone(),
                                               health: *health, }
                              })
        {
            ok = Ok(with_closure(membership)?);
        }
        ok
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
        let precursor_health = match expiring_to {
            Health::Confirmed => Health::Suspect,
            Health::Departed => Health::Confirmed,
            other => panic!("Expiring to {} is invalid", other),
        };

        let expired: Vec<_> =
            self.write_entries()
                .iter_mut()
                .filter_map(|(id, v)| {
                    let member_list::Entry { health,
                                             health_updated_at,
                                             .. } = v;
                    if *health == precursor_health && now >= *health_updated_at + timeout {
                        *health = expiring_to;
                        *health_updated_at = now;
                        Some(id.clone())
                    } else {
                        None
                    }
                })
                .collect();

        if !expired.is_empty() {
            self.increment_update_counter();
        }

        expired
    }

    pub fn contains_member(&self, member_id: &str) -> bool {
        self.read_entries().contains_key(member_id)
    }
}

/// This proxy wraps a MemberList so that we can customize its serialization logic.
pub struct MemberListProxy<'a>(&'a MemberList);

impl<'a> MemberListProxy<'a> {
    pub fn new(m: &'a MemberList) -> Self { MemberListProxy(&m) }
}

impl<'a> Serialize for MemberListProxy<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let map = self.0.read_entries();

        let mut m = serializer.serialize_map(Some(map.len()))?;

        for (id, member_list::Entry { member, health, .. }) in map.iter() {
            m.serialize_entry(id, &MemberProxy::new(member, health))?;
        }

        m.end()
    }
}

/// This proxy wraps both a Member and Health, and presents them together, for use in the
/// supervisor's /butterfly HTTP API endpoint.
pub struct MemberProxy<'a>(&'a Member, &'a Health);

impl<'a> MemberProxy<'a> {
    pub fn new(m: &'a Member, h: &'a Health) -> Self { MemberProxy(&m, &h) }
}

impl<'a> Serialize for MemberProxy<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("member", 6)?;
        strukt.serialize_field("address", &self.0.address)?;
        strukt.serialize_field("gossip_port", &self.0.gossip_port)?;
        strukt.serialize_field("incarnation", &self.0.incarnation)?;
        strukt.serialize_field("persistent", &self.0.persistent)?;
        strukt.serialize_field("swim_port", &self.0.swim_port)?;
        strukt.serialize_field("health", &self.1)?;
        strukt.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl MemberList {
        // This is a remnant of when the MemberList::members entries were
        // simple Member structs. The tests that use this should be replaced,
        // but until then, this keeps them working.
        fn with_member_iter<T>(&self,
                               mut with_closure: impl FnMut(hash_map::Values<'_, String, Member>)
                                     -> T)
                               -> T {
            let mut member_map = HashMap::new();
            for (id, super::member_list::Entry { member, .. }) in self.read_entries().iter() {
                member_map.insert(id.clone(), member.clone());
            }
            with_closure(member_map.values())
        }
    }

    mod member {
        use crate::member::{Incarnation,
                            Member};

        // Sets the uuid to simple, and the incarnation to the default.
        #[test]
        fn new() {
            let member = Member::default();
            assert_eq!(member.id.len(), 32);
            assert_eq!(member.incarnation, Incarnation::default());
        }
    }

    mod membership {
        use crate::{member::{Health,
                             Member,
                             Membership},
                    protocol::Message};
        #[test]
        fn encode_decode_roundtrip() {
            let member = Member::default();
            let membership = Membership { member,
                                          health: Health::Suspect };

            let bytes = membership.clone()
                                  .write_to_bytes()
                                  .expect("Could not write membership to bytes!");
            let from_bytes =
                Membership::from_bytes(&bytes).expect("Could not decode membership from bytes!");

            assert_eq!(&membership.member, &from_bytes.member);
            assert_eq!(&membership.health, &from_bytes.health);
        }
    }

    mod member_list {
        use crate::member::{Health,
                            Member,
                            MemberList,
                            PINGREQ_TARGETS};

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
            assert!(ml.is_empty());
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
            ml.with_members(|m| assert_eq!(ml.health_of(m), Some(Health::Alive)));
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
            assert_eq!(ml.health_of(&mcheck), Some(Health::Alive));
        }

        /// Tests of MemberList::insert
        mod insert {
            use crate::member::{Health,
                                Incarnation,
                                Member,
                                MemberList};

            fn assert_cannot_insert_member_rumor_of_lower_incarnation(from_health: Health,
                                                                      to_health: Health)
            {
                let ml = MemberList::new();
                let initial_update_counter_value = ml.get_update_counter();
                let initial_incarnation = Incarnation::from(10); // just to pick a number

                let member = {
                    let mut m = Member::default();
                    m.incarnation = initial_incarnation;
                    m
                };

                assert!(ml.insert(member.clone(), from_health),
                        "Could not insert member into list initially");

                let update_counter_value_checkpoint_1 = ml.get_update_counter();
                assert_eq!(update_counter_value_checkpoint_1,
                           initial_update_counter_value + 1,
                           "Update counter should have incremented by one");

                assert_eq!(ml.health_of(&member),
                           Some(from_health),
                           "Member should have had health {:?}, but didn't",
                           from_health);

                let member_with_lower_incarnation = {
                    let mut m = member.clone();
                    m.incarnation = Incarnation::from(m.incarnation.to_u64() - 1);
                    m
                };

                assert!(!ml.insert(member_with_lower_incarnation, to_health),
                        "Inserting with {:?}->{:?} should be a no-op with a lower incarnation \
                         number",
                        from_health,
                        to_health);
                assert_eq!(ml.get_update_counter(),
                           update_counter_value_checkpoint_1,
                           "Update counter should not have been incremented after trying to \
                            insert a lower-incarnation-number rumor");
                assert_eq!(ml.health_of(&member),
                           Some(from_health),
                           "Member should have still have health {:?} following attempt to \
                            insert lower-incarnation-number rumor, but didn't",
                           from_health);
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

            fn assert_always_insert_member_rumor_of_higher_incarnation(from_health: Health,
                                                                       to_health: Health)
            {
                let ml = MemberList::new();
                let initial_update_counter_value = ml.get_update_counter();
                let initial_incarnation = Incarnation::from(10); // just to pick a number

                let member = {
                    let mut m = Member::default();
                    m.incarnation = initial_incarnation;
                    m
                };

                assert!(ml.insert(member.clone(), from_health),
                        "Could not insert member into list initially");

                let update_counter_value_checkpoint_1 = ml.get_update_counter();
                assert_eq!(update_counter_value_checkpoint_1,
                           initial_update_counter_value + 1,
                           "Update counter should have incremented by one");

                assert_eq!(ml.health_of(&member),
                           Some(from_health),
                           "Member should have had health {:?}, but didn't",
                           from_health);

                let member_with_higher_incarnation = {
                    let mut m = member.clone();
                    m.incarnation = m.incarnation + 1;
                    m
                };

                assert!(ml.insert(member_with_higher_incarnation, to_health),
                        "Inserting with {:?}->{:?} should be always work with a higher \
                         incarnation number",
                        from_health,
                        to_health);
                assert_eq!(ml.get_update_counter(),
                           update_counter_value_checkpoint_1 + 1,
                           "Update counter should increment by 1 when inserting a \
                            higher-incarnation-number rumor");
                assert_eq!(ml.health_of(&member),
                           Some(to_health),
                           "Member should have health {:?} following insertion of \
                            higher-incarnation-number rumor",
                           to_health);
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

            fn assert_only_insert_member_rumor_of_same_incarnation_if_health_is_worse(from_health: Health,
                                                                                      to_health: Health)
            {
                let ml = MemberList::new();
                let initial_update_counter_value = ml.get_update_counter();
                let initial_incarnation = Incarnation::from(10); // just to pick a number

                let member = {
                    let mut m = Member::default();
                    m.incarnation = initial_incarnation;
                    m
                };

                assert!(ml.insert(member.clone(), from_health),
                        "Could not insert member into list initially");

                let update_counter_value_checkpoint_1 = ml.get_update_counter();
                assert_eq!(update_counter_value_checkpoint_1,
                           initial_update_counter_value + 1,
                           "Update counter should have incremented by one");

                assert_eq!(ml.health_of(&member),
                           Some(from_health),
                           "Member should have had health {:?}, but didn't",
                           from_health);

                let member_with_same_incarnation = member.clone();

                if to_health > from_health {
                    assert!(ml.insert(member_with_same_incarnation, to_health),
                            "Inserting with {:?}->{:?} should work with an identical incarnation \
                             number",
                            from_health,
                            to_health);
                    assert_eq!(ml.get_update_counter(),
                               update_counter_value_checkpoint_1 + 1,
                               "Update counter should increment by 1 when inserting a \
                                same-incarnation-number rumor with worse health");
                    assert_eq!(ml.health_of(&member),
                               Some(to_health),
                               "Member should have health {:?} following insertion of \
                                same-incarnation-number rumor with worse health",
                               to_health);
                } else {
                    assert!(!ml.insert(member_with_same_incarnation, to_health),
                            "Inserting with {from:?}->{to:?} should never work with an identical \
                             incarnation number, because {to:?} is not \"worse than\" {from:?}",
                            from = from_health,
                            to = to_health);
                    assert_eq!(ml.get_update_counter(),
                               update_counter_value_checkpoint_1,
                               "Update counter should not increment when inserting a \
                                same-incarnation-number rumor without worse health");
                    assert_eq!(ml.health_of(&member),
                               Some(from_health),
                               "Member should still have health {:?} following insertion of \
                                same-incarnation-number rumor without worse health",
                               from_health);
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

            /// Tests that the transition from `from_health` to `to_health` for
            /// `insert` works properly.
            fn assert_insert_health_by_id_transition(from_health: Health, to_health: Health) {
                let ml = MemberList::new();
                let member_one = Member::default();

                assert!(ml.insert(member_one.clone(), from_health),
                        "Should be able to insert initial health of {:?} into empty MemberList",
                        from_health);
                assert_eq!(ml.health_of(&member_one)
                             .expect("Expected member to exist in health after initial insert, \
                                      but it didn't"),
                           from_health,
                           "Member should have initial health {:?}",
                           from_health);

                let update_counter_before = ml.get_update_counter();

                if from_health == to_health {
                    assert!(!ml.insert(member_one.clone(), to_health),
                            "Transitioning from {:?} to {:?} (i.e., no change) should be a no-op",
                            from_health,
                            to_health);
                    assert_eq!(ml.get_update_counter(),
                               update_counter_before,
                               "Transitioning from {:?} to {:?} (i.e., no change) should not \
                                increment update counter",
                               from_health,
                               to_health);
                    assert_eq!(ml.health_of(&member_one)
                                 .expect("Expected member to exist in health after update, but \
                                          it didn't"),
                               from_health,
                               "Member should have still have initial health {:?}",
                               from_health);
                } else if to_health > from_health {
                    assert!(ml.insert(member_one.clone(), to_health),
                            "Transitioning from {:?} to {:?} (i.e., worse health) should NOT be \
                             a no-op",
                            from_health,
                            to_health);
                    assert_eq!(ml.get_update_counter(),
                               update_counter_before + 1,
                               "Transitioning from {:?} to {:?} (i.e., different health) should \
                                increment update counter by one",
                               from_health,
                               to_health);
                    assert_eq!(ml.health_of(&member_one)
                                 .expect("Expected member to exist in health after update, but \
                                          it didn't"),
                               to_health,
                               "Member should have changed health from {:?} to {:?}",
                               from_health,
                               to_health);
                } else {
                    assert!(!ml.insert(member_one.clone(), to_health),
                            "Transitioning from {:?} to {:?} (i.e., no worse health) should be a \
                             no-op",
                            from_health,
                            to_health);
                    assert_eq!(ml.get_update_counter(),
                               update_counter_before,
                               "Transitioning from {:?} to {:?} (i.e., no worse health) should \
                                not increment update counter",
                               from_health,
                               to_health);
                    assert_eq!(ml.health_of(&member_one)
                                 .expect("Expected member to exist in health after update, but \
                                          it didn't"),
                               from_health,
                               "Member should have retained old health {:?}",
                               from_health);
                }
            }

            macro_rules! transition {
                // Unfortunately, Rust macros currently can't be used to generate
                // the name of a function, so we have to provide one :(
                ($fn_name:ident, $from:expr, $to:expr) => {
                    #[test]
                    fn $fn_name() { assert_insert_health_by_id_transition($from, $to); }
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
            use crate::member::{Health,
                                Member,
                                MemberList};
            use std::{thread,
                      time::Duration as StdDuration};
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
                        "An empty MemberList shouldn't have anything that's timing out to being \
                         Confirmed");

                assert!(ml.insert(member_one.clone(), Health::Alive));

                assert!(ml.members_expired_to_confirmed(small_timeout).is_empty(),
                        "Should be no newly Confirmed members when they're all Alive");

                assert!(ml.insert(member_one.clone(), Health::Suspect));

                assert!(ml.members_expired_to_confirmed(large_timeout).is_empty(),
                        "Nothing should have timed out to Confirmed with a large timeout");

                // Allow the Suspect to age
                thread::sleep(StdDuration::from_secs(small_seconds));

                let newly_confirmed = ml.members_expired_to_confirmed(small_timeout);
                assert!(newly_confirmed.contains(&member_one.id),
                        "Member should be newly Confirmed after timing out");

                assert_eq!(ml.health_of(&member_one),
                           Some(Health::Confirmed),
                           "Member should have a health of Confirmed after timing out");
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
                        "An empty MemberList shouldn't have anything that's timing out to being \
                         Departed");

                assert!(ml.insert(member_one.clone(), Health::Alive));
                assert!(ml.members_expired_to_departed(small_timeout).is_empty(),
                        "Should be no newly Departed members when they're all Alive");

                assert!(ml.insert(member_one.clone(), Health::Suspect));
                assert!(ml.members_expired_to_departed(small_timeout).is_empty(),
                        "Should be no newly Departed members when they're all Confirmed");

                assert!(ml.insert(member_one.clone(), Health::Confirmed));

                assert!(ml.members_expired_to_departed(small_timeout).is_empty(),
                        "Should be no newly Departed members when they're all Confirmed");

                assert!(ml.members_expired_to_departed(large_timeout).is_empty(),
                        "Nothing should have timed out to Departed with a large timeout");

                // Allow the Confirmed to age
                thread::sleep(StdDuration::from_secs(small_seconds));

                let newly_departed = ml.members_expired_to_departed(small_timeout);
                assert!(newly_departed.contains(&member_one.id),
                        "Member should be newly Departed after timing out");

                assert_eq!(ml.health_of(&member_one),
                           Some(Health::Departed),
                           "Member should have a health of Departed after timing out");
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
                assert!(newly_confirmed.contains(&member_1.id),
                        "Member 1 should be newly Confirmed after timing out");
                assert!(newly_confirmed.contains(&member_2.id),
                        "Member 2 should be newly Confirmed after timing out");
                assert!(!newly_confirmed.contains(&member_3.id),
                        "Member 3 should NOT be newly Confirmed, because it hasn't timed out yet");

                assert_eq!(ml.health_of(&member_1),
                           Some(Health::Confirmed),
                           "Member 1 should have a health of Confirmed after timing out");
                assert_eq!(ml.health_of(&member_2),
                           Some(Health::Confirmed),
                           "Member 2 should have a health of Confirmed after timing out");
                assert_eq!(ml.health_of(&member_3),
                           Some(Health::Suspect),
                           "Member 3 should still have a health of Suspect, because it hasn't \
                            timed out yet");
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
                assert!(newly_departed.contains(&member_1.id),
                        "Member 1 should be newly Departed after timing out");
                assert!(newly_departed.contains(&member_2.id),
                        "Member 2 should be newly Departed after timing out");
                assert!(!newly_departed.contains(&member_3.id),
                        "Member 3 should NOT be newly Departed, because it hasn't timed out yet");

                assert_eq!(ml.health_of(&member_1),
                           Some(Health::Departed),
                           "Member 1 should have a health of Departed after timing out");
                assert_eq!(ml.health_of(&member_2),
                           Some(Health::Departed),
                           "Member 2 should have a health of Departed after timing out");
                assert_eq!(ml.health_of(&member_3),
                           Some(Health::Confirmed),
                           "Member 3 should still have a health of Confirmed, because it hasn't \
                            timed out yet");
            }

        }
    }
}
