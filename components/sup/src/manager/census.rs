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

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::str::{self, FromStr};

use hcore::package::PackageIdent;
use hcore::service::ServiceGroup;
use butterfly;
use butterfly::rumor::{Service as ServiceRumor, Election as ElectionRumor};
use butterfly::rumor::election::Election_Status;
use butterfly::rumor::service::SysInfo;
use butterfly::member::{Member, Health};
use toml;

static LOGKEY: &'static str = "CE";

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CensusUpdate {
    pub service_counter: usize,
    service_config_counter: usize,
    election_counter: usize,
    election_update_counter: usize,
    membership_counter: usize,
}

impl CensusUpdate {
    pub fn new(butterfly: &butterfly::Server) -> CensusUpdate {
        CensusUpdate {
            service_counter: butterfly.service_store.get_update_counter(),
            service_config_counter: butterfly.service_config_store.get_update_counter(),
            election_counter: butterfly.election_store.get_update_counter(),
            election_update_counter: butterfly.update_store.get_update_counter(),
            membership_counter: butterfly.member_list.get_update_counter(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ElectionStatus {
    None,
    ElectionInProgress,
    ElectionNoQuorum,
    ElectionFinished,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
pub struct CensusEntry {
    member_id: String,
    service: String,
    group: String,
    org: Option<String>,
    cfg: toml::value::Table,
    sys: SysInfo,
    pub pkg: Option<PackageIdent>,
    leader: Option<bool>,
    follower: Option<bool>,
    update_leader: Option<bool>,
    update_follower: Option<bool>,
    election_is_running: Option<bool>,
    election_is_no_quorum: Option<bool>,
    election_is_finished: Option<bool>,
    update_election_is_running: Option<bool>,
    update_election_is_no_quorum: Option<bool>,
    update_election_is_finished: Option<bool>,
    initialized: Option<bool>,
    alive: Option<bool>,
    suspect: Option<bool>,
    confirmed: Option<bool>,
    persistent: Option<bool>,
}

impl CensusEntry {
    fn get_service_group(&self) -> String {
        if self.org.is_some() {
            format!("{}.{}@{}",
                    self.get_service(),
                    self.get_group(),
                    self.get_org())
        } else {
            format!("{}.{}", self.get_service(), self.get_group())
        }
    }

    pub fn get_member_id(&self) -> &str {
        &self.member_id
    }

    fn set_member_id(&mut self, value: String) {
        self.member_id = value
    }

    fn get_service(&self) -> &str {
        &self.service
    }

    fn set_service(&mut self, value: String) {
        self.service = value;
    }

    pub fn get_group(&self) -> &str {
        &self.group
    }

    fn set_group(&mut self, value: String) {
        self.group = value;
    }

    fn get_org(&self) -> &str {
        match self.org.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn set_org(&mut self, value: String) {
        self.org = Some(value);
    }

    #[allow(dead_code)] // only used in tests
    fn get_pkg(&self) -> &PackageIdent {
        self.pkg.as_ref().unwrap()
    }

    fn set_pkg(&mut self, value: PackageIdent) {
        self.pkg = Some(value);
    }

    fn set_leader(&mut self, value: bool) {
        self.leader = Some(value);
    }

    fn get_leader(&self) -> bool {
        self.leader.unwrap_or(false)
    }

    fn set_follower(&mut self, value: bool) {
        self.follower = Some(value);
    }

    #[allow(dead_code)] // only used in tests
    fn get_follower(&self) -> bool {
        self.follower.unwrap_or(false)
    }

    fn set_update_leader(&mut self, value: bool) {
        self.update_leader = Some(value);
    }

    pub fn get_update_leader(&self) -> bool {
        self.update_leader.unwrap_or(false)
    }

    fn set_update_follower(&mut self, value: bool) {
        self.update_follower = Some(value);
    }

    fn set_election_is_running(&mut self, value: bool) {
        self.election_is_running = Some(value);
    }

    fn get_election_is_running(&self) -> bool {
        self.election_is_running.unwrap_or(false)
    }

    fn set_election_is_no_quorum(&mut self, value: bool) {
        self.election_is_no_quorum = Some(value);
    }

    fn get_election_is_no_quorum(&self) -> bool {
        self.election_is_no_quorum.unwrap_or(false)
    }

    fn set_election_is_finished(&mut self, value: bool) {
        self.election_is_finished = Some(value);
    }

    fn get_election_is_finished(&self) -> bool {
        self.election_is_finished.unwrap_or(false)
    }

    fn set_update_election_is_running(&mut self, value: bool) {
        self.update_election_is_running = Some(value);
    }

    fn set_update_election_is_no_quorum(&mut self, value: bool) {
        self.update_election_is_no_quorum = Some(value);
    }

    fn set_update_election_is_finished(&mut self, value: bool) {
        self.update_election_is_finished = Some(value);
    }

    pub fn get_election_status(&self) -> ElectionStatus {
        if self.get_election_is_running() {
            ElectionStatus::ElectionInProgress
        } else if self.get_election_is_no_quorum() {
            ElectionStatus::ElectionNoQuorum
        } else if self.get_election_is_finished() {
            ElectionStatus::ElectionFinished
        } else {
            ElectionStatus::None
        }
    }

    fn set_alive(&mut self, value: bool) {
        self.alive = Some(value);
    }

    fn get_alive(&self) -> bool {
        self.alive.unwrap_or(false)
    }

    fn set_suspect(&mut self, value: bool) {
        self.suspect = Some(value);
    }

    fn set_confirmed(&mut self, value: bool) {
        self.confirmed = Some(value);
    }

    fn set_persistent(&mut self, value: bool) {
        self.persistent = Some(value);
    }

    #[allow(dead_code)] // only used in tests
    fn get_persistent(&self) -> bool {
        self.persistent.unwrap_or(false)
    }

    pub fn populate_from_service(&mut self, rumor: &ServiceRumor) {
        self.set_member_id(String::from(rumor.get_member_id()));
        let sg = match ServiceGroup::from_str(rumor.get_service_group()) {
            Ok(sg) => sg,
            Err(e) => {
                outputln!("Malformed service group; cannot populate configuration data. \
                           Aborting.: {}",
                          e);
                return;
            }
        };
        self.set_service(sg.service().to_string());
        self.set_group(sg.group().to_string());
        if let Some(org) = sg.org() {
            self.set_org(org.to_string());
        }
        match PackageIdent::from_str(rumor.get_pkg()) {
            Ok(ident) => self.set_pkg(ident),
            Err(err) => warn!("Received a bad package ident from gossip data, err={}", err),
        }
        self.cfg = toml::from_slice(rumor.get_cfg()).unwrap_or(toml::value::Table::default());
        self.sys = toml::from_slice(rumor.get_sys()).unwrap_or(SysInfo::default());
    }

    fn populate_from_member(&mut self, member: &Member) {
        self.set_member_id(String::from(member.get_id()));
        self.sys.gossip_ip = member.get_address().to_string();
        self.sys.gossip_port = member.get_gossip_port().to_string();
        self.set_persistent(true);
    }

    fn populate_from_health(&mut self, health: Health) {
        match health {
            Health::Alive => {
                self.set_alive(true);
                self.set_suspect(false);
                self.set_confirmed(false);
            }
            Health::Suspect => {
                self.set_alive(false);
                self.set_suspect(true);
                self.set_confirmed(false);
            }
            Health::Confirmed => {
                self.set_alive(false);
                self.set_suspect(false);
                self.set_confirmed(true);
            }
        }
    }

    fn populate_from_election(&mut self, election: &ElectionRumor) {
        match election.get_status() {
            Election_Status::Running => {
                self.set_leader(false);
                self.set_follower(false);
                self.set_election_is_running(true);
                self.set_election_is_no_quorum(false);
                self.set_election_is_finished(false);
            }
            Election_Status::NoQuorum => {
                self.set_leader(false);
                self.set_follower(false);
                self.set_election_is_running(false);
                self.set_election_is_no_quorum(true);
                self.set_election_is_finished(false);
            }
            Election_Status::Finished => {
                if self.get_member_id() == election.get_member_id() {
                    self.set_leader(true);
                    self.set_follower(false);
                } else {
                    self.set_leader(false);
                    self.set_follower(true);
                }
                self.set_election_is_running(false);
                self.set_election_is_no_quorum(false);
                self.set_election_is_finished(true);
            }
        }
    }

    fn populate_from_update_election(&mut self, election: &ElectionRumor) {
        match election.get_status() {
            Election_Status::Running => {
                self.set_update_leader(false);
                self.set_update_follower(false);
                self.set_update_election_is_running(true);
                self.set_update_election_is_no_quorum(false);
                self.set_update_election_is_finished(false);
            }
            Election_Status::NoQuorum => {
                self.set_update_leader(false);
                self.set_update_follower(false);
                self.set_update_election_is_running(false);
                self.set_update_election_is_no_quorum(true);
                self.set_update_election_is_finished(false);
            }
            Election_Status::Finished => {
                if self.get_member_id() == election.get_member_id() {
                    self.set_update_leader(true);
                    self.set_update_follower(false);
                } else {
                    self.set_update_leader(false);
                    self.set_update_follower(true);
                }
                self.set_update_election_is_running(false);
                self.set_update_election_is_no_quorum(false);
                self.set_update_election_is_finished(true);
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Census {
    // JW TODO: This needs to become an Ordered HashMap keyed on member_id. This will reduce our
    // allocations when ordering the population to determine who should update next in a rolling
    // update strategy. For now, we allocate a new vector every server tick by the members() and
    // members_ordered() functions.
    population: HashMap<String, CensusEntry>,
    member_id: String,
}

impl Deref for Census {
    type Target = HashMap<String, CensusEntry>;

    fn deref(&self) -> &HashMap<String, CensusEntry> {
        &self.population
    }
}

impl DerefMut for Census {
    fn deref_mut(&mut self) -> &mut HashMap<String, CensusEntry> {
        &mut self.population
    }
}

impl Census {
    fn new(member_id: String) -> Census {
        Census {
            population: HashMap::new(),
            member_id: member_id,
        }
    }

    pub fn me(&self) -> Option<&CensusEntry> {
        self.population.get(&self.member_id)
    }

    /// Return all alive members.
    fn alive_members(&self) -> Vec<&CensusEntry> {
        self.population
            .values()
            .filter(|ce| ce.get_alive())
            .collect()
    }

    /// Return all alive members ordered by member_id.
    fn alive_members_ordered(&self) -> Vec<&CensusEntry> {
        let mut members = self.alive_members();
        members.sort_by(|a, b| a.member_id.cmp(&b.member_id));
        members
    }

    /// Return all members.
    fn members(&self) -> Vec<&CensusEntry> {
        self.population.values().map(|ce| ce).collect()
    }

    /// Return all members ordered by member_id.
    pub fn members_ordered(&self) -> Vec<&CensusEntry> {
        let mut members = self.members();
        members.sort_by(|a, b| a.member_id.cmp(&b.member_id));
        members
    }

    pub fn get_leader(&self) -> Option<&CensusEntry> {
        self.population.values().find(|&ce| ce.get_leader())
    }

    /// Return the leader of the currently running update election or None if there is no leader.
    pub fn get_update_leader(&self) -> Option<&CensusEntry> {
        self.population
            .values()
            .find(|&ce| ce.get_update_leader())
    }

    pub fn get_service_group(&self) -> String {
        // We know we have one, because otherwise the census wouldn't exist
        let entry = self.population.values().nth(0).unwrap();
        entry.get_service_group()
    }

    pub fn get_group(&self) -> &str {
        let entry = self.population.values().nth(0).unwrap();
        entry.get_group()
    }

    pub fn get_service(&self) -> &str {
        let entry = self.population.values().nth(0).unwrap();
        entry.get_service()
    }

    /// Return previous alive peer, the peer to your left in the ordered members list, or None if
    /// you have no alive peers.
    pub fn previous_peer(&self) -> Option<&CensusEntry> {
        let members = self.alive_members_ordered();
        if members.len() <= 1 || self.me().is_none() {
            return None;
        }
        match members
                  .iter()
                  .position(|ce| ce.member_id == self.me().unwrap().member_id) {
            Some(idx) => {
                if idx <= 0 {
                    Some(members[members.len() - 1])
                } else {
                    Some(members[idx - 1])
                }
            }
            None => None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CensusList {
    censuses: HashMap<String, Census>,
}

impl Deref for CensusList {
    type Target = HashMap<String, Census>;

    fn deref(&self) -> &HashMap<String, Census> {
        &self.censuses
    }
}

impl CensusList {
    pub fn new() -> CensusList {
        CensusList { censuses: HashMap::new() }
    }

    pub fn insert(&mut self, member_id: String, census_entry: CensusEntry) {
        let census = self.censuses
            .entry(census_entry.get_service_group())
            .or_insert(Census::new(member_id));
        if census.contains_key(census_entry.get_member_id()) {
            let entry = census.get_mut(census_entry.get_member_id()).unwrap();
            *entry = census_entry;
        } else {
            census.insert(String::from(census_entry.get_member_id()), census_entry);
        };
    }

    pub fn get(&self, service_group: &str) -> Option<&Census> {
        self.censuses.get(service_group)
    }

    pub fn populate_from_election(&mut self, election: &ElectionRumor) {
        if let Some(census_entries) = self.censuses.get_mut(election.get_service_group()) {
            for census_entry in census_entries.values_mut() {
                census_entry.populate_from_election(election);
            }
        }
    }

    pub fn populate_from_update_election(&mut self, election: &ElectionRumor) {
        if let Some(census_entries) = self.censuses.get_mut(election.get_service_group()) {
            for census_entry in census_entries.values_mut() {
                census_entry.populate_from_update_election(election);
            }
        }
    }

    pub fn populate_from_member(&mut self, member: &Member) {
        for (_service_group, census) in self.censuses.iter_mut() {
            if let Some(ce) = census.get_mut(member.get_id()) {
                ce.populate_from_member(member);
            }
        }
    }

    pub fn populate_from_health(&mut self, member: &Member, health: Health) {
        for (_service_group, census) in self.censuses.iter_mut() {
            if let Some(ce) = census.get_mut(member.get_id()) {
                ce.populate_from_health(health);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod census_entry {
        use std::str::FromStr;

        use butterfly::rumor::service::{Service, SysInfo};
        use butterfly::member::Member;
        use hcore::service::ServiceGroup;
        use hcore::package::ident::PackageIdent;

        use manager::census::CensusEntry;

        #[test]
        fn member_id() {
            let mut ce = CensusEntry::default();
            assert_eq!(ce.get_member_id(), "");
            ce.set_member_id(String::from("neurosis"));
            assert_eq!(ce.get_member_id(), "neurosis");
        }

        #[test]
        fn populate_from_service_rumor() {
            let mut ce = CensusEntry::default();
            let ident = PackageIdent::from_str("core/overwatch/1.2.3/20161208121212").unwrap();
            let sg = ServiceGroup::new("overwatch", "times", Some("ofgrace")).unwrap();
            let service = Service::new("neurosis".to_string(),
                                       &ident,
                                       &sg,
                                       &SysInfo::default(),
                                       None);
            ce.populate_from_service(&service);
            assert_eq!(ce.get_member_id(), "neurosis");
            assert_eq!(ce.get_service(), "overwatch");
            assert_eq!(ce.get_group(), "times");
            assert_eq!(ce.get_org(), "ofgrace");
            assert_eq!(ce.get_pkg(), &ident);
        }

        #[test]
        fn populate_from_member() {
            let mut ce = CensusEntry::default();
            let mut member = Member::default();
            member.set_address(String::from("162.42.150.33"));
            member.set_persistent(true);
            ce.populate_from_member(&member);
            assert_eq!(ce.get_member_id(), member.get_id());
            assert_eq!(ce.sys.gossip_ip, member.get_address());
            assert_eq!(ce.get_persistent(), member.get_persistent());
        }
    }
}
