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

use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use std::str::FromStr;

use hcore::service::ServiceGroup;
use butterfly::rumor::service::Service as ServiceRumor;
use butterfly::rumor::election::{Election as ElectionRumor, Election_Status};
use butterfly::member::Member;

static LOGKEY: &'static str = "CE";

#[derive(Debug, PartialEq, Eq)]
pub struct CensusUpdate {
    service_counter: usize,
    election_counter: usize,
    membership_counter: usize,
}

impl CensusUpdate {
    pub fn new(service_counter: usize,
               election_counter: usize,
               membership_counter: usize)
               -> CensusUpdate {
        CensusUpdate {
            service_counter: service_counter,
            election_counter: election_counter,
            membership_counter: membership_counter,
        }
    }
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Default)]
pub struct CensusEntry {
    pub member_id: Option<String>,
    pub service: Option<String>,
    pub group: Option<String>,
    pub hostname: Option<String>,
    pub address: Option<String>,
    pub ip: Option<String>,
    pub port: Option<String>,
    pub exposes: Vec<String>,
    pub leader: Option<bool>,
    pub follower: Option<bool>,
    pub election_is_running: Option<bool>,
    pub election_is_no_quorum: Option<bool>,
    pub election_is_finished: Option<bool>,
    pub initialized: Option<bool>,
    pub alive: Option<bool>,
    pub suspect: Option<bool>,
    pub confirmed: Option<bool>,
    pub persistent: Option<bool>,
}

impl CensusEntry {
    pub fn get_service_group(&self) -> String {
        format!("{}.{}", self.get_service(), self.get_group())
    }

    pub fn get_member_id(&self) -> &str {
        match self.member_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    pub fn set_member_id(&mut self, value: String) {
        self.member_id = Some(value);
    }

    pub fn get_service(&self) -> &str {
        match self.service.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    pub fn set_service(&mut self, value: String) {
        self.service = Some(value);
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    pub fn set_group(&mut self, value: String) {
        self.group = Some(value);
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    pub fn set_hostname(&mut self, value: String) {
        self.hostname = Some(value);
    }

    pub fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    pub fn set_address(&mut self, value: String) {
        self.address = Some(value);
    }

    pub fn get_ip(&self) -> &str {
        match self.ip.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    pub fn set_ip(&mut self, value: String) {
        self.ip = Some(value);
    }

    pub fn get_port(&self) -> &str {
        match self.port.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    pub fn set_port(&mut self, value: String) {
        self.port = Some(value);
    }

    pub fn get_exposes(&self) -> &Vec<String> {
        &self.exposes
    }

    pub fn set_exposes(&mut self, value: Vec<String>) {
        self.exposes = value;
    }

    pub fn set_leader(&mut self, value: bool) {
        self.leader = Some(value);
    }

    pub fn get_leader(&self) -> bool {
        self.leader.unwrap_or(false)
    }

    pub fn set_follower(&mut self, value: bool) {
        self.follower = Some(value);
    }

    pub fn get_follower(&self) -> bool {
        self.follower.unwrap_or(false)
    }

    pub fn set_election_is_running(&mut self, value: bool) {
        self.election_is_running = Some(value);
    }

    pub fn get_election_is_running(&self) -> bool {
        self.election_is_running.unwrap_or(false)
    }

    pub fn set_election_is_no_quorum(&mut self, value: bool) {
        self.election_is_no_quorum = Some(value);
    }

    pub fn get_election_is_no_quorum(&self) -> bool {
        self.election_is_no_quorum.unwrap_or(false)
    }

    pub fn set_election_is_finished(&mut self, value: bool) {
        self.election_is_finished = Some(value);
    }

    pub fn get_election_is_finished(&self) -> bool {
        self.election_is_finished.unwrap_or(false)
    }

    pub fn set_initialized(&mut self, value: bool) {
        self.initialized = Some(value);
    }

    pub fn get_initialized(&self) -> bool {
        self.initialized.unwrap_or(false)
    }

    pub fn set_alive(&mut self, value: bool) {
        self.alive = Some(value);
    }

    pub fn get_alive(&self) -> bool {
        self.alive.unwrap_or(false)
    }

    pub fn set_suspect(&mut self, value: bool) {
        self.suspect = Some(value);
    }

    pub fn get_suspect(&self) -> bool {
        self.suspect.unwrap_or(false)
    }

    pub fn set_confirmed(&mut self, value: bool) {
        self.confirmed = Some(value);
    }

    pub fn get_confirmed(&self) -> bool {
        self.confirmed.unwrap_or(false)
    }

    pub fn set_persistent(&mut self, value: bool) {
        self.persistent = Some(value);
    }

    pub fn get_persistent(&self) -> bool {
        self.persistent.unwrap_or(false)
    }

    pub fn populate_from_service(&mut self, service_rumor: &ServiceRumor) {
        self.set_member_id(String::from(service_rumor.get_member_id()));
        let sg = match ServiceGroup::from_str(service_rumor.get_service_group()) {
            Ok(sg) => sg,
            Err(e) => {
                outputln!("Malformed service group; cannot populate configuration data. \
                           Aborting.: {}",
                          e);
                return;
            }
        };
        self.set_service(sg.service.clone());
        self.set_group(sg.group.clone());
        self.set_ip(String::from(service_rumor.get_ip()));
        self.set_hostname(String::from(service_rumor.get_hostname()));
        self.set_port(format!("{}", service_rumor.get_port()));
        self.set_exposes(service_rumor.get_exposes().iter().map(|p| format!("{}", p)).collect());
    }

    pub fn populate_from_member(&mut self, member: &Member) {
        self.set_member_id(String::from(member.get_id()));
        self.set_address(String::from(member.get_address()));
        self.set_persistent(true);
    }

    pub fn populate_from_election(&mut self, election: &ElectionRumor) {
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
}

#[derive(Debug)]
pub struct Census {
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
    pub fn new(member_id: String) -> Census {
        Census {
            population: HashMap::new(),
            member_id: member_id,
        }
    }

    pub fn me(&self) -> &CensusEntry {
        self.population.get(&self.member_id).unwrap()
    }

    pub fn get_leader(&self) -> Option<&CensusEntry> {
        self.population.values().find(|&ce| ce.get_leader())
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
}

//
#[derive(Debug)]
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
        let census =
            self.censuses.entry(census_entry.get_service_group()).or_insert(Census::new(member_id));
        let result = if census.contains_key(census_entry.get_member_id()) {
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
        if self.censuses.contains_key(election.get_service_group()) {
            // We just checked, so we're cool
            let census_entries = self.censuses.get_mut(election.get_service_group()).unwrap();
            for census_entry in census_entries.values_mut() {
                census_entry.populate_from_election(election);
            }
        }
    }

    pub fn populate_from_member(&mut self, member: &Member) {
        for (service_group, census) in self.censuses.iter_mut() {
            if census.contains_key(member.get_id()) {
                // We just checked, so its coolio
                let mut ce = census.get_mut(member.get_id()).unwrap();
                ce.populate_from_member(member);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod census_entry {
        use butterfly::rumor::service::Service;
        use butterfly::member::Member;
        use hcore::service::ServiceGroup;

        use manager::census::CensusEntry;

        #[test]
        fn default_is_empty() {
            let ce = CensusEntry::default();
            assert!(ce.member_id.is_none());
        }

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
            let service = Service::new("neurosis",
                                       ServiceGroup::new("times", "ofgrace", None),
                                       "foo.com",
                                       "162.42.150.33",
                                       vec![6060, 8080]);
            ce.populate_from_service(&service);
            assert_eq!(ce.get_member_id(), "neurosis");
            assert_eq!(ce.get_service(), "times");
            assert_eq!(ce.get_group(), "ofgrace");
            assert_eq!(ce.get_ip(), "162.42.150.33");
            assert_eq!(ce.get_port(), "6060");
            assert_eq!(ce.get_exposes(),
                       &vec![String::from("6060"), String::from("8080")]);
        }

        #[test]
        fn populate_from_member() {
            let mut ce = CensusEntry::default();
            let mut member = Member::new();
            member.set_address(String::from("162.42.150.33"));
            member.set_persistent(true);
            ce.populate_from_member(&member);
            assert_eq!(ce.get_member_id(), member.get_id());
            assert_eq!(ce.get_address(), member.get_address());
            assert_eq!(ce.get_persistent(), member.get_persistent());
        }
    }
}
