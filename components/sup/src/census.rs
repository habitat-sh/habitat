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

//! The Census is the core of our service discovery mechanism. It keeps track of every supervisor
//! in our group, and handles reading, writing, and serializing it with the discovery backend
//! (etcd.) It has 4 main components:
//!
//! * CensusEntry: a given supervisors entry in the census.
//! * CensusEntryActor: a GenServer responsible for serializing our Census Entry to the backend
//! * Census: The complete list of all supervisors, plus functions for analyzing the data, and
//!   updating the census.
//! * CensusActor: a GenServer responsible for reading the global census from the backend
//!
//! Think of each supervisor in the system as a 'CensusEntry'; taken together, they form a
//! 'Census'. Operations to discover or mutate the state of the Census happen through algorithms
//! that arrive at the same conclusion given the same inputs.
//!
//! An example is leader election; it's handled here by having a consistent (and simple) algorithm
//! for selecting a leader deterministically for the group. We rely on the eventual consistency of
//! every supervisors CensusEntry to elect a new leader in a reasonable amount of time.

use std::collections::{HashMap, BTreeMap};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use toml;
use uuid::Uuid;

use gossip::member::{MemberId, MemberList, Health};
use gossip::lamport_clock::LamportClock;
use error::{Error, Result};
use util;

static LOGKEY: &'static str = "CN";
pub static MIN_QUORUM: usize = 3;

pub type CensusEntryId = Uuid;

/// A CensusEntry. Manages all the data about a given member of the census.
#[derive(Debug, Clone, RustcDecodable, RustcEncodable, Eq)]
pub struct CensusEntry {
    pub id: CensusEntryId,
    pub member_id: MemberId,
    pub hostname: String,
    pub ip: String,
    suitability: u64,
    pub port: Option<String>,
    pub exposes: Option<Vec<String>>,
    pub leader: bool,
    pub follower: bool,
    pub data_init: bool,
    pub vote: Option<String>,
    pub election: Option<bool>,
    pub needs_write: Option<bool>,
    pub initialized: bool,
    keep_me: bool,
    pub service: String,
    pub group: String,
    pub alive: bool,
    pub suspect: bool,
    pub confirmed: bool,
    pub detached: bool,
    pub incarnation: LamportClock,
}

impl CensusEntry {
    /// Create a new CensusEntry for this supervisor.
    pub fn new<S>(service: S, group: S, member_id: MemberId) -> CensusEntry
        where S: Into<String>
    {
        CensusEntry {
            id: Uuid::new_v4(),
            member_id: member_id,
            hostname: util::sys::hostname().unwrap_or(String::from("unknown")),
            ip: util::sys::ip().map(|s| s.to_string()).unwrap_or("127.0.0.1".to_string()),
            suitability: 0,
            port: None,
            exposes: None,
            leader: false,
            follower: false,
            data_init: false,
            vote: None,
            election: None,
            needs_write: None,
            initialized: false,
            keep_me: true,
            alive: true,
            suspect: false,
            confirmed: false,
            detached: false,
            service: service.into(),
            group: group.into(),
            incarnation: LamportClock::new(),
        }
    }

    pub fn needs_write(&self) -> bool {
        self.needs_write.is_some()
    }

    /// Set our suitability number. This is an arbitrary determination of our 'suitability' to a
    /// task; most likely, being the leader in an election.
    pub fn suitability(&mut self, suitability: u64) {
        self.suitability = suitability;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set a port number; often used as the default for watches
    pub fn port(&mut self, port: Option<String>) {
        self.port = port;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set an array of port numbers we expose.
    pub fn exposes(&mut self, exposes: Option<Vec<String>>) {
        self.exposes = exposes;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set our status at the leader.
    pub fn leader(&mut self, leader: bool) {
        if self.leader != leader {
            self.leader = leader;
            self.incarnation.increment();
            self.needs_write = Some(true);
        }
    }

    /// Set our status as a follower.
    pub fn follower(&mut self, follower: bool) {
        if self.follower != follower {
            self.follower = follower;
            self.incarnation.increment();
            self.needs_write = Some(true);
        }
    }

    /// Set our application initialization status to true.
    pub fn initialized(&mut self) {
        self.initialized = true;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set our status on having initialzied data.
    pub fn data_init(&mut self, data_init: bool) {
        if self.data_init != data_init {
            self.data_init = data_init;
            self.incarnation.increment();
            self.needs_write = Some(true);
        }
    }

    /// Set our vote.
    pub fn vote(&mut self, vote: Option<String>) {
        self.vote = vote;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Are we in an election?
    pub fn election(&mut self, election: Option<bool>) {
        self.election = election;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set us to alive.
    pub fn set_alive(&mut self) {
        self.alive = true;
        self.suspect = false;
        self.confirmed = false;
        self.detached = false;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set our suspectness.
    pub fn set_suspect(&mut self) {
        self.alive = false;
        self.suspect = true;
        self.confirmed = false;
        self.detached = false;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set our confirmedness.
    pub fn set_confirmed(&mut self) {
        self.alive = false;
        self.suspect = false;
        self.confirmed = true;
        self.detached = false;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Set our detachedness.
    pub fn set_detached(&mut self) {
        self.alive = false;
        self.suspect = false;
        self.confirmed = false;
        self.detached = true;
        self.incarnation.increment();
        self.needs_write = Some(true);
    }

    /// Return the string we use for this CensusEntry when it is a candidate in an election.
    pub fn candidate_string(&self) -> String {
        format!("{}", self.id)
    }

    /// Return the service.group string
    pub fn service_group(&self) -> String {
        format!("{}.{}", self.service, self.group)
    }

    /// Update this entry from another entry. If the other side has a higher incarnation, take it
    /// as the new you.
    pub fn update_via(&mut self, other_ce: CensusEntry) -> bool {
        if other_ce.incarnation > self.incarnation {
            *self = other_ce;
            true
        } else {
            false
        }
    }

    pub fn written(&mut self) {
        self.needs_write = None;
    }
}

impl PartialEq for CensusEntry {
    // We are equal, but we don't care about some fields.
    fn eq(&self, other: &CensusEntry) -> bool {
        if self.id != other.id {
            false
        } else if self.incarnation != other.incarnation {
            false
        } else if self.hostname != other.hostname {
            false
        } else if self.ip != other.ip {
            false
        } else if self.suitability != other.suitability {
            false
        } else if self.port != other.port {
            false
        } else if self.exposes != other.exposes {
            false
        } else if self.leader != other.leader {
            false
        } else if self.follower != other.follower {
            false
        } else if self.data_init != other.data_init {
            false
        } else if self.vote != other.vote {
            false
        } else if self.election != other.election {
            false
        } else if self.service != other.service {
            false
        } else if self.group != other.group {
            false
        } else {
            true
        }
    }
}

impl fmt::Display for CensusEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.service_group(), self.id)
    }
}

/// A simple map of Census Entries; used for decoding toml data
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct CensusMap {
    pub census: BTreeMap<String, CensusEntry>,
}

/// A census!
///
/// Keeps a population of CensusEntries, and allows you to interrogate their global state.
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Census {
    /// The uuid of the current supervisor
    me: Uuid,
    /// The total population of CensusEntries
    population: HashMap<Uuid, CensusEntry>,
    /// Whether we are currently in an event
    pub in_event: bool,
    pub service: String,
    pub group: String,
}

impl Census {
    /// Creates a new Census. Takes a CensusEntry for the current supervisor.
    pub fn new(ce: CensusEntry) -> Census {
        let my_id = ce.id.clone();
        let service = ce.service.clone();
        let group = ce.group.clone();
        let mut hm = HashMap::new();
        hm.insert(my_id, ce);
        Census {
            me: my_id,
            population: hm,
            in_event: false,
            service: service,
            group: group,
        }
    }

    pub fn needs_write(&self) -> bool {
        self.population.iter().any(|(_id, ce)| ce.needs_write())
    }

    pub fn written(&mut self) {
        for (_id, mut ce) in self.population.iter_mut() {
            ce.written();
        }
    }

    pub fn service_group(&self) -> String {
        format!("{}.{}", self.service, self.group)
    }

    pub fn get(&self, id: &CensusEntryId) -> Option<&CensusEntry> {
        self.population.get(id)
    }

    pub fn get_mut(&mut self, id: &CensusEntryId) -> Option<&mut CensusEntry> {
        self.population.get_mut(id)
    }

    /// A reference to the current supervisors entry in the census.
    ///
    /// # Failures
    ///
    /// * If the entry doesn't exist
    pub fn me(&self) -> &CensusEntry {
        self.population
            .get(&self.me)
            .unwrap()
    }

    /// A mutable reference to the current supervisors entry in the census.
    ///
    /// # Failures
    ///
    /// * If the entry doesn't exist
    pub fn me_mut(&mut self) -> &mut CensusEntry {
        self.population
            .get_mut(&self.me)
            .unwrap()
    }

    /// Add an entry to the census
    pub fn add(&mut self, ce: CensusEntry) {
        self.population.insert(ce.id, ce);
    }

    /// Set whether we are in an event
    pub fn in_event(&mut self, status: bool) {
        self.in_event = status;
    }

    /// Given a toml string of our census, update the internal representation of the census.
    ///
    /// # Failures
    ///
    /// * If we cannot parse the toml
    pub fn update(&mut self, census_string: &str) -> Result<()> {
        let mut toml_parser = toml::Parser::new(census_string);
        let toml = try!(toml_parser.parse()
            .ok_or(sup_error!(Error::TomlParser(toml_parser.errors))));
        let toml_value = toml::Value::Table(toml);
        let census_map: CensusMap = toml::decode(toml_value).unwrap();
        let current_uuids: Vec<Uuid> = self.population.keys().map(|&x| x.clone()).collect();
        let mut new_uuids: Vec<Uuid> = Vec::new();

        for new_entry in census_map.census.values() {
            let update = match self.population.get(&new_entry.id) {
                Some(current_entry) => {
                    if current_entry.id == self.me {
                        false
                    } else if current_entry == new_entry {
                        false
                    } else {
                        true
                    }
                }
                None => true,
            };
            if update {
                debug!("updating {:#?}", new_entry);
                self.population.insert(new_entry.id.clone(), new_entry.clone());
            }
            new_uuids.push(new_entry.id.clone());
        }

        for uuid in current_uuids.iter().filter(|&x| !new_uuids.contains(x)) {
            if *uuid != self.me {
                self.population.remove(&uuid);
            }
        }

        Ok(())
    }

    /// Turn the current census into a toml string, to be used when we render the configuration
    /// files.
    ///
    /// # Failures
    ///
    /// * If we cannot parse a Uuid
    pub fn to_toml(&self) -> Result<String> {
        let mut top = toml::Table::new();
        let mut census = toml::Table::new();
        census.insert("service".to_string(),
                      toml::Value::String(self.service.clone()));
        census.insert("group".to_string(), toml::Value::String(self.group.clone()));
        let mut members = toml::Array::new();
        let mut sorted_keys: Vec<_> = self.population
            .keys()
            .map(|&x| x.simple().to_string())
            .collect();
        sorted_keys.sort();
        for key in sorted_keys {
            let uuid_key = try!(Uuid::parse_str(&key));
            let value = self.population.get(&uuid_key).unwrap();
            members.push(toml::encode(value));
        }

        census.insert("members".to_string(), toml::Value::Array(members));

        let me = self.me();
        census.insert("me".to_string(), toml::encode(me));

        match self.get_leader() {
            Some(leader) => {
                census.insert("leader".to_string(), toml::encode(leader));
            }
            None => {}
        }

        top.insert("census".to_string(), toml::Value::Table(census));

        Ok(toml::encode_str(&top))
    }

    /// Have all members of the census initialized their data?
    pub fn dataset_initialized(&self) -> bool {
        let count = self.population
            .values()
            .filter(|&ce| { if ce.data_init { true } else { false } })
            .count();
        if count > 0 { true } else { false }
    }

    /// Is there a living leader in the census? Returns that entry.
    pub fn get_leader(&self) -> Option<&CensusEntry> {
        self.population
            .values()
            .find(|&ce| { if ce.leader && ce.alive { true } else { false } })
    }

    /// Is there an alive leader in the census?
    pub fn has_leader(&self) -> bool {
        let count = self.population
            .values()
            .filter(|&ce| { if ce.leader && ce.alive { true } else { false } })
            .count();
        if count > 0 { true } else { false }
    }

    /// Is there one leader, and everyone alive is a follower?
    pub fn has_all_followers(&self) -> bool {
        let size = self.population.values().filter(|&ce| ce.alive).count() - 1;

        let count = self.population
            .values()
            .filter(|&ce| { if ce.follower && ce.alive { true } else { false } })
            .count();
        if count == size { true } else { false }
    }

    /// Decide who we should vote for, and return their CensusEntry.
    ///
    /// * Choose the node with the highest `suitability` number
    /// * If all those are equal, choose the node whose `id` field sorts first lexicographically
    pub fn determine_vote(&self) -> &CensusEntry {
        let acc: Option<&CensusEntry> = None;
        let vote: &CensusEntry = self.population
            .values()
            .filter(|ce| ce.alive)
            .fold(acc, |acc, ref rce| {
                match acc {
                    Some(lce) => {
                        if rce.suitability > lce.suitability {
                            Some(rce)
                        } else if lce.suitability == rce.suitability {
                            if rce.id.simple().to_string() > lce.id.simple().to_string() {
                                Some(rce)
                            } else {
                                Some(lce)
                            }
                        } else {
                            Some(lce)
                        }
                    }
                    None => Some(rce),
                }
            })
            .unwrap();
        vote
    }

    /// Voting is finished, and we return the winner, if:
    ///
    /// * All entries in the census are in an election
    /// * They have all cast their vote
    /// * Everyone votes for the same CensusEntry
    pub fn voting_finished(&self) -> Option<&CensusEntry> {
        let all_in_election = self.population
            .values()
            .filter(|ref ce| ce.alive)
            .all(|ref ce| {
                match ce.election {
                    Some(true) => true,
                    Some(false) => false,
                    None => false,
                }
            });
        if all_in_election == false {
            debug!("Not all in election: {:#?}", self);
            return None;
        };

        let all_voted = self.population
            .values()
            .filter(|ref ce| ce.alive)
            .all(|ref ce| {
                match ce.vote {
                    Some(_) => true,
                    None => false,
                }
            });
        if all_voted == false {
            debug!("Not everyone has voted: {:#?}", self);
            return None;
        };

        let ce = self.me();
        let my_vote = ce.clone().vote.unwrap();

        for (_lid, lce) in self.population.iter().filter(|&(_id, ce)| ce.alive) {
            match lce.vote {
                Some(ref their_vote) => {
                    if my_vote != *their_vote {
                        debug!("We do not all agree: {:#?} vs {:#?}", my_vote, their_vote);
                        return None;
                    }
                }
                None => {
                    debug!("Citizen {:#?} has not voted yet", lce);
                    return None;
                }
            }
        }

        self.population.get(&Uuid::parse_str(&my_vote).unwrap())
    }

    pub fn total_population(&self) -> usize {
        self.population.len()
    }

    pub fn alive_population(&self) -> usize {
        self.population
            .iter()
            .filter(|&(_id, ce)| ce.alive)
            .count()
    }

    pub fn minimum_quorum(&self) -> bool {
        let total_population = self.population.len();
        total_population >= MIN_QUORUM
    }

    pub fn has_quorum(&self) -> bool {
        let total_population = self.total_population();
        if total_population % 2 == 0 {
            warn!("This census has an even population. If half the membership fails, quorum will \
                   never be met, and no leader will be elected. Add another instance to the \
                   service group!");
        }
        let alive_population = self.alive_population() as f32;
        let total_pop = total_population as f32;
        let percent_alive: usize = ((alive_population.round() / total_pop.round()) * 100.0)
            .round() as usize;
        if percent_alive > 50 { true } else { false }
    }

    pub fn no_leaders_allowed(&mut self) {
        for (_id, ce) in self.population.iter_mut() {
            ce.leader = false;
        }
    }
}

impl Deref for Census {
    type Target = HashMap<Uuid, CensusEntry>;

    fn deref(&self) -> &HashMap<Uuid, CensusEntry> {
        &self.population
    }
}

impl DerefMut for Census {
    fn deref_mut(&mut self) -> &mut HashMap<Uuid, CensusEntry> {
        &mut self.population
    }
}


#[derive(Debug, RustcEncodable)]
pub struct CensusList {
    local_census: String,
    // I'm sorry about this. This really is the plural of census. What can you do.
    // String here == service.group
    censuses: HashMap<String, Census>,
}

impl CensusList {
    pub fn new(local_census: Census) -> CensusList {
        let mut cl = CensusList {
            censuses: HashMap::new(),
            local_census: local_census.service_group(),
        };
        cl.insert(local_census);
        cl
    }

    pub fn me(&self) -> &CensusEntry {
        self.local_census().me()
    }

    pub fn me_mut(&mut self) -> &mut CensusEntry {
        self.local_census_mut().me_mut()
    }

    pub fn local_census(&self) -> &Census {
        self.censuses.get(&self.local_census).unwrap()
    }

    pub fn local_census_mut(&mut self) -> &mut Census {
        self.censuses.get_mut(&self.local_census).unwrap()
    }

    pub fn insert(&mut self, census: Census) {
        self.censuses.insert(census.service_group(), census);
    }

    pub fn insert_entry(&mut self, ce: CensusEntry) {
        if let Some(mut census) = self.censuses.get_mut(&ce.service_group()) {
            census.add(ce);
            return;
        }

        let census = Census::new(ce);
        self.insert(census);
    }

    pub fn get_mut(&mut self,
                   census_entry_id: &CensusEntryId,
                   service_group: &str)
                   -> Option<&mut CensusEntry> {
        match self.censuses.get_mut(service_group) {
            Some(mut census) => census.get_mut(census_entry_id),
            None => None,
        }
    }

    pub fn process(&mut self, mut remote_ce: CensusEntry) -> bool {
        remote_ce.needs_write = Some(true);
        if let Some(mut current_ce) = self.get_mut(&remote_ce.id, &remote_ce.service_group()) {
            return current_ce.update_via(remote_ce);
        }
        self.insert_entry(remote_ce);
        return true;
    }

    pub fn written(&mut self) {
        for (_sg, mut census) in self.censuses.iter_mut() {
            census.written();
        }
    }

    pub fn needs_write(&self) -> bool {
        for (_sg, census) in self.censuses.iter() {
            if census.needs_write() {
                return true;
            }
        }
        return false;
    }
}

impl Deref for CensusList {
    type Target = HashMap<String, Census>;

    fn deref(&self) -> &HashMap<String, Census> {
        &self.censuses
    }
}

impl DerefMut for CensusList {
    fn deref_mut(&mut self) -> &mut HashMap<String, Census> {
        &mut self.censuses
    }
}

pub fn start_health_adjuster(census_list: Arc<RwLock<CensusList>>,
                             member_list: Arc<RwLock<MemberList>>) {
    outputln!("Starting census health adjuster");
    let cl1 = census_list.clone();
    let ml1 = member_list.clone();
    let _t = thread::Builder::new().name("health_adjuster".to_string()).spawn(move || {
        loop {
            {
                let mut cl = cl1.write().unwrap();
                for (_service_group, mut census) in cl.iter_mut() {
                    for (_census_entry_id, mut census_entry) in census.iter_mut() {
                        let ml = ml1.read().unwrap();
                        if let Some(member) = ml.get(&census_entry.member_id) {
                            match member.health {
                                Health::Alive => {
                                    if census_entry.alive == false {
                                        census_entry.set_alive();
                                    }
                                }
                                Health::Suspect => {
                                    if census_entry.suspect == false {
                                        census_entry.set_suspect();
                                    }
                                }
                                Health::Confirmed => {
                                    if census_entry.confirmed == false {
                                        census_entry.set_confirmed();
                                    }
                                }
                            }
                        } else {
                            if census_entry.detached == false {
                                census_entry.set_detached();
                            }
                        }
                    }
                }
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });
}

#[cfg(test)]
mod test {
    mod census {
        use gossip::member::MemberId;
        use census::{Census, CensusEntry};

        fn generate_ce() -> CensusEntry {
            CensusEntry::new("soup", "unit", MemberId::new_v4())
        }

        fn generate_census() -> Census {
            Census::new(generate_ce())
        }

        fn add_entries(census: &mut Census, count: usize) {
            for _x in 0..count {
                census.add(generate_ce());
            }
        }

        fn confirm_entries(census: &mut Census, count: usize) {
            let me = census.me.clone();
            for (_id, mut ce) in census.iter_mut()
                .filter(|&(id, ref _ce)| *id != me)
                .take(count) {
                ce.set_confirmed();
            }
        }

        fn elect_an_entry(census: &mut Census) {
            let (_id, mut ce) = census.population.iter_mut().next().unwrap();
            ce.leader(true);
            ce.follower(false);
        }

        fn fail_the_leader(census: &mut Census) {
            let (_id, mut leader) = census.population
                .iter_mut()
                .find(|&(_id, ref ce)| ce.leader)
                .unwrap();
            leader.set_confirmed();
        }

        #[test]
        fn has_quorum() {
            let mut census = generate_census();
            add_entries(&mut census, 10);
            assert_eq!(census.has_quorum(), true);
            confirm_entries(&mut census, 6);
            assert_eq!(census.has_quorum(), false);
        }

        #[test]
        fn has_quorum_even_population_split_brain_is_false() {
            let mut census = generate_census();
            add_entries(&mut census, 9);
            confirm_entries(&mut census, 5);
            assert_eq!(census.has_quorum(), false);
        }

        #[test]
        fn minimum_quorum() {
            let mut census = generate_census();
            add_entries(&mut census, 1);
            assert_eq!(census.minimum_quorum(), false);
            add_entries(&mut census, 1);
            assert_eq!(census.minimum_quorum(), true);
            add_entries(&mut census, 10);
            assert_eq!(census.minimum_quorum(), true);
        }

        #[test]
        fn has_leader() {
            let mut census = generate_census();
            add_entries(&mut census, 10);
            elect_an_entry(&mut census);

            // We have a leader, and it is alive
            assert_eq!(census.has_leader(), true);

            // We have a leader, but it is confirmed dead
            fail_the_leader(&mut census);
            assert_eq!(census.has_leader(), false);
        }
    }
}
