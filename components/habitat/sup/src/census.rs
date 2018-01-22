// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::str::FromStr;

use butterfly::member::{MemberList, Member, Health};
use butterfly::rumor::RumorStore;
use butterfly::rumor::service::Service as ServiceRumor;
use butterfly::rumor::service_file::ServiceFile as ServiceFileRumor;
use butterfly::rumor::service_config::ServiceConfig as ServiceConfigRumor;
use butterfly::rumor::election::Election as ElectionRumor;
use butterfly::rumor::election::Election_Status as ElectionStatusRumor;
use butterfly::rumor::election::ElectionUpdate as ElectionUpdateRumor;
use butterfly::rumor::service::SysInfo;
use hcore;
use hcore::service::ServiceGroup;
use hcore::package::PackageIdent;
use toml;

use error::{Error, SupError};

static LOGKEY: &'static str = "CE";

type MemberId = String;

#[derive(Debug, Serialize)]
pub struct CensusRing {
    changed: bool,

    census_groups: HashMap<ServiceGroup, CensusGroup>,
    local_member_id: MemberId,
    last_service_counter: usize,
    last_election_counter: usize,
    last_election_update_counter: usize,
    last_membership_counter: usize,
    last_service_config_counter: usize,
    last_service_file_counter: usize,
}

impl CensusRing {
    /// Indicates whether the census has changed since the last time
    /// we looked at rumors.
    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn new<I>(local_member_id: I) -> Self
    where
        I: Into<MemberId>,
    {
        CensusRing {
            changed: false,
            census_groups: HashMap::new(),
            local_member_id: local_member_id.into(),
            last_service_counter: 0,
            last_election_counter: 0,
            last_election_update_counter: 0,
            last_membership_counter: 0,
            last_service_config_counter: 0,
            last_service_file_counter: 0,
        }
    }

    pub fn update_from_rumors(
        &mut self,
        service_rumors: &RumorStore<ServiceRumor>,
        election_rumors: &RumorStore<ElectionRumor>,
        election_update_rumors: &RumorStore<ElectionUpdateRumor>,
        member_list: &MemberList,
        service_config_rumors: &RumorStore<ServiceConfigRumor>,
        service_file_rumors: &RumorStore<ServiceFileRumor>,
    ) {
        // If ANY new rumor, of any type, has been received,
        // reconstruct the entire census state to ensure consistency
        if (service_rumors.get_update_counter() > self.last_service_counter) ||
            (member_list.get_update_counter() > self.last_membership_counter) ||
            (election_rumors.get_update_counter() > self.last_election_counter) ||
            (election_update_rumors.get_update_counter() > self.last_election_update_counter) ||
            (service_config_rumors.get_update_counter() > self.last_service_config_counter) ||
            (service_file_rumors.get_update_counter() > self.last_service_file_counter)
        {

            self.changed = true;

            self.populate_census(service_rumors, member_list);
            self.update_from_election_store(election_rumors);
            self.update_from_election_update_store(election_update_rumors);
            self.update_from_service_config(service_config_rumors);
            self.update_from_service_files(service_file_rumors);

            // Update our counters to reflect current state.
            self.last_membership_counter = member_list.get_update_counter();
            self.last_service_counter = service_rumors.get_update_counter();
            self.last_election_counter = election_rumors.get_update_counter();
            self.last_election_update_counter = election_update_rumors.get_update_counter();
            self.last_service_config_counter = service_config_rumors.get_update_counter();
            self.last_service_file_counter = service_file_rumors.get_update_counter();

        } else {
            self.changed = false;
        }
    }

    pub fn census_group_for(&self, sg: &ServiceGroup) -> Option<&CensusGroup> {
        self.census_groups.get(sg)
    }

    pub fn groups(&self) -> Vec<&CensusGroup> {
        self.census_groups.values().map(|cg| cg).collect()
    }

    /// Populates the census from `ServiceRumor`s and Butterfly-level
    /// membership lists.
    ///
    /// (Butterfly provides the health, the ServiceRumors provide the
    /// rest).
    fn populate_census(
        &mut self,
        service_rumors: &RumorStore<ServiceRumor>,
        member_list: &MemberList,
    ) {
        // Populate our census; new groups are created here, as are
        // new members of those groups.
        //
        // NOTE: In the current implementation, these members have an
        // indeterminate health status until we process the contents
        // of `member_list`. In the future, it would be nice to
        // incorporate the member list into
        // `census_group.update_from_service_rumors`, where new census
        // members are created, so there would be no time that there
        // is an indeterminate health anywhere.
        service_rumors.with_keys(|(service_group, rumors)| if let Ok(sg) =
            service_group_from_str(service_group)
        {
            let census_group = self.census_groups.entry(sg.clone()).or_insert(
                CensusGroup::new(
                    sg,
                    &self.local_member_id,
                ),
            );
            census_group.update_from_service_rumors(rumors);
        });

        member_list.with_members(|member| {
            let health = member_list.health_of(&member).unwrap();
            for group in self.census_groups.values_mut() {
                if let Some(census_member) = group.find_member_mut(member.get_id()) {
                    census_member.update_from_member(&member);
                    census_member.update_from_health(health);
                }
            }
        });
    }

    fn update_from_election_store(&mut self, election_rumors: &RumorStore<ElectionRumor>) {
        election_rumors.with_keys(|(service_group, rumors)| {
            let election = rumors.get("election").unwrap();
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(census_group) = self.census_groups.get_mut(&sg) {
                    census_group.update_from_election_rumor(election);
                }
            }
        });
    }

    fn update_from_election_update_store(
        &mut self,
        election_update_rumors: &RumorStore<ElectionUpdateRumor>,
    ) {
        election_update_rumors.with_keys(|(service_group, rumors)| if let Ok(sg) =
            service_group_from_str(service_group)
        {
            if let Some(census_group) = self.census_groups.get_mut(&sg) {
                let election = rumors.get("election").unwrap();
                census_group.update_from_election_update_rumor(election);
            }
        });
    }

    fn update_from_service_config(
        &mut self,
        service_config_rumors: &RumorStore<ServiceConfigRumor>,
    ) {
        service_config_rumors.with_keys(|(service_group, rumors)| if let Ok(sg) =
            service_group_from_str(service_group)
        {
            if let Some(service_config) = rumors.get("service_config") {
                if let Some(census_group) = self.census_groups.get_mut(&sg) {
                    census_group.update_from_service_config_rumor(service_config);
                }
            }
        });
    }

    fn update_from_service_files(&mut self, service_file_rumors: &RumorStore<ServiceFileRumor>) {
        service_file_rumors.with_keys(|(service_group, rumors)| if let Ok(sg) =
            service_group_from_str(service_group)
        {
            let census_group = self.census_groups.entry(sg.clone()).or_insert(
                CensusGroup::new(
                    sg,
                    &self.local_member_id,
                ),
            );
            census_group.update_from_service_file_rumors(rumors);
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ElectionStatus {
    None,
    ElectionInProgress,
    ElectionNoQuorum,
    ElectionFinished,
}

impl Default for ElectionStatus {
    fn default() -> ElectionStatus {
        ElectionStatus::None
    }
}

impl fmt::Display for ElectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            ElectionStatus::ElectionInProgress => "in-progress",
            ElectionStatus::ElectionNoQuorum => "no-quorum",
            ElectionStatus::ElectionFinished => "finished",
            ElectionStatus::None => "none",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for ElectionStatus {
    type Err = SupError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "in-progress" => Ok(ElectionStatus::ElectionInProgress),
            "no-quorum" => Ok(ElectionStatus::ElectionNoQuorum),
            "finished" => Ok(ElectionStatus::ElectionFinished),
            "none" => Ok(ElectionStatus::None),
            _ => Err(sup_error!(Error::BadElectionStatus(value.to_string()))),
        }
    }
}

impl From<ElectionStatusRumor> for ElectionStatus {
    fn from(val: ElectionStatusRumor) -> ElectionStatus {
        match val {
            ElectionStatusRumor::Running => ElectionStatus::ElectionInProgress,
            ElectionStatusRumor::NoQuorum => ElectionStatus::ElectionNoQuorum,
            ElectionStatusRumor::Finished => ElectionStatus::ElectionFinished,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ServiceFile {
    pub filename: String,
    pub incarnation: u64,
    pub body: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct ServiceConfig {
    pub incarnation: u64,
    pub value: toml::Value,
}

#[derive(Debug, Serialize)]
pub struct CensusGroup {
    pub service_group: ServiceGroup,
    pub election_status: ElectionStatus,
    pub update_election_status: ElectionStatus,
    pub leader_id: Option<MemberId>,
    pub service_config: Option<ServiceConfig>,

    local_member_id: MemberId,
    population: BTreeMap<MemberId, CensusMember>,
    update_leader_id: Option<MemberId>,
    changed_service_files: Vec<String>,
    service_files: HashMap<String, ServiceFile>,
}

impl CensusGroup {
    fn new(sg: ServiceGroup, local_member_id: &MemberId) -> Self {
        CensusGroup {
            service_group: sg,
            election_status: ElectionStatus::None,
            update_election_status: ElectionStatus::None,
            local_member_id: local_member_id.clone(),
            population: BTreeMap::new(),
            leader_id: None,
            update_leader_id: None,
            service_config: None,
            service_files: HashMap::new(),
            changed_service_files: Vec::new(),
        }
    }

    /// Returns the census member in the census ring for the running Supervisor.
    pub fn me(&self) -> Option<&CensusMember> {
        self.population.get(&self.local_member_id)
    }

    pub fn leader(&self) -> Option<&CensusMember> {
        match self.leader_id {
            Some(ref id) => self.population.get(id),
            None => None,
        }
    }

    pub fn update_leader(&self) -> Option<&CensusMember> {
        match self.update_leader_id {
            Some(ref id) => self.population.get(id),
            None => None,
        }
    }

    /// Returns a list of all members in the census ring.
    pub fn members(&self) -> Vec<&CensusMember> {
        self.population.values().map(|cm| cm).collect()
    }

    pub fn changed_service_files(&self) -> Vec<&ServiceFile> {
        self.changed_service_files
            .iter()
            .map(|f| self.service_files.get(f).unwrap())
            .collect()
    }

    /// Return previous alive peer, the peer to your left in the ordered members list, or None if
    /// you have no alive peers.
    pub fn previous_peer(&self) -> Option<&CensusMember> {
        let alive_members: Vec<&CensusMember> =
            self.population.values().filter(|cm| cm.alive).collect();
        if alive_members.len() <= 1 || self.me().is_none() {
            return None;
        }
        match alive_members.iter().position(|cm| {
            cm.member_id == self.me().unwrap().member_id
        }) {
            Some(idx) => {
                if idx <= 0 {
                    Some(alive_members[alive_members.len() - 1])
                } else {
                    Some(alive_members[idx - 1])
                }
            }
            None => None,
        }
    }

    fn update_from_service_rumors(&mut self, rumors: &HashMap<String, ServiceRumor>) {
        for (member_id, service_rumor) in rumors.iter() {
            // Yeah - we are ourself - we're alive.
            let is_self = member_id == &self.local_member_id;
            let member = self.population
                .entry(member_id.to_string())
                .or_insert_with(|| {
                    let mut new_member = CensusMember::default();
                    new_member.alive = is_self;
                    new_member
                });
            member.update_from_service_rumor(&self.service_group, service_rumor);
        }
    }

    fn update_from_election_rumor(&mut self, election: &ElectionRumor) {
        self.leader_id = None;
        for census_member in self.population.values_mut() {
            if census_member.update_from_election_rumor(election) {
                self.leader_id = Some(census_member.member_id.clone());
            }
        }
        match election.get_status() {
            ElectionStatusRumor::Running => {
                self.election_status = ElectionStatus::ElectionInProgress;
            }
            ElectionStatusRumor::NoQuorum => {
                self.election_status = ElectionStatus::ElectionNoQuorum;
            }
            ElectionStatusRumor::Finished => {
                self.election_status = ElectionStatus::ElectionFinished;
            }
        }
    }

    fn update_from_election_update_rumor(&mut self, election: &ElectionUpdateRumor) {
        self.update_leader_id = None;
        for census_member in self.population.values_mut() {
            if census_member.update_from_election_update_rumor(election) {
                self.update_leader_id = Some(census_member.member_id.clone());
            }
        }
        match election.get_status() {
            ElectionStatusRumor::Running => {
                self.update_election_status = ElectionStatus::ElectionInProgress;
            }
            ElectionStatusRumor::NoQuorum => {
                self.update_election_status = ElectionStatus::ElectionNoQuorum;
            }
            ElectionStatusRumor::Finished => {
                self.update_election_status = ElectionStatus::ElectionFinished;
            }
        }
    }

    fn update_from_service_config_rumor(&mut self, service_config: &ServiceConfigRumor) {
        match service_config.config() {
            Ok(config) => {
                if self.service_config.is_none() ||
                    service_config.get_incarnation() >
                        self.service_config.as_ref().unwrap().incarnation
                {
                    self.service_config = Some(ServiceConfig {
                        incarnation: service_config.get_incarnation(),
                        value: config,
                    });
                }
            }
            Err(err) => warn!("{}", err),
        }
    }

    fn update_from_service_file_rumors(
        &mut self,
        service_file_rumors: &HashMap<String, ServiceFileRumor>,
    ) {
        self.changed_service_files.clear();
        for (_m_id, service_file_rumor) in service_file_rumors.iter() {
            let filename = service_file_rumor.get_filename().to_string();
            let file = self.service_files.entry(filename.clone()).or_insert(
                ServiceFile::default(),
            );

            if service_file_rumor.get_incarnation() > file.incarnation {
                match service_file_rumor.body() {
                    Ok(body) => {
                        self.changed_service_files.push(filename.clone());
                        file.filename = filename.clone();
                        file.incarnation = service_file_rumor.get_incarnation();
                        file.body = body;
                    }
                    Err(e) => {
                        warn!(
                            "Cannot decrypt service file for {} {} {}: {}",
                            self.service_group,
                            service_file_rumor.get_filename(),
                            service_file_rumor.get_incarnation(),
                            e
                        )
                    }
                }
            }
        }
    }

    fn find_member_mut(&mut self, member_id: &str) -> Option<&mut CensusMember> {
        self.population.get_mut(member_id)
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CensusMember {
    pub member_id: MemberId,
    pub pkg: Option<PackageIdent>,
    pub application: Option<String>,
    pub environment: Option<String>,
    pub service: String,
    pub group: String,
    pub org: Option<String>,
    pub persistent: bool,
    pub leader: bool,
    pub follower: bool,
    pub update_leader: bool,
    pub update_follower: bool,
    pub election_is_running: bool,
    pub election_is_no_quorum: bool,
    pub election_is_finished: bool,
    pub update_election_is_running: bool,
    pub update_election_is_no_quorum: bool,
    pub update_election_is_finished: bool,
    pub sys: SysInfo,
    alive: bool,
    suspect: bool,
    confirmed: bool,
    departed: bool,
    // Maps must be represented last in a serializable struct for the current version of the toml
    // crate. Additionally, this deserialization method is required to correct any ordering issues
    // with the table being serialized - https://docs.rs/toml/0.4.0/toml/ser/fn.tables_last.html
    #[serde(serialize_with = "toml::ser::tables_last")]
    pub cfg: toml::value::Table,
}

impl CensusMember {
    fn update_from_service_rumor(&mut self, sg: &ServiceGroup, rumor: &ServiceRumor) {
        self.member_id = String::from(rumor.get_member_id());
        self.service = sg.service().to_string();
        self.group = sg.group().to_string();
        if let Some(org) = sg.org() {
            self.org = Some(org.to_string());
        }
        if let Some(appenv) = sg.application_environment() {
            self.application = Some(appenv.application().to_string());
            self.environment = Some(appenv.environment().to_string());
        }
        match PackageIdent::from_str(rumor.get_pkg()) {
            Ok(ident) => self.pkg = Some(ident),
            Err(err) => warn!("Received a bad package ident from gossip data, err={}", err),
        };
        self.sys = rumor.get_sys().clone().into();
        self.cfg = toml::from_slice(rumor.get_cfg()).unwrap_or(toml::value::Table::default());
    }

    fn update_from_election_rumor(&mut self, election: &ElectionRumor) -> bool {
        self.election_is_running = election.get_status() == ElectionStatusRumor::Running;
        self.election_is_no_quorum = election.get_status() == ElectionStatusRumor::NoQuorum;
        self.election_is_finished = election.get_status() == ElectionStatusRumor::Finished;
        if self.election_is_finished {
            if self.member_id == election.get_member_id() {
                self.leader = true;
                self.follower = false;
            } else {
                self.leader = false;
                self.follower = true;
            }
        }
        self.leader
    }

    fn update_from_election_update_rumor(&mut self, election: &ElectionUpdateRumor) -> bool {
        self.update_election_is_running = election.get_status() == ElectionStatusRumor::Running;
        self.update_election_is_no_quorum = election.get_status() == ElectionStatusRumor::NoQuorum;
        self.update_election_is_finished = election.get_status() == ElectionStatusRumor::Finished;
        if self.update_election_is_finished {
            if self.member_id == election.get_member_id() {
                self.update_leader = true;
                self.update_follower = false;
            } else {
                self.update_leader = false;
                self.update_follower = true;
            }
        }
        self.update_leader
    }

    fn update_from_member(&mut self, member: &Member) {
        self.sys.set_gossip_ip(member.get_address().to_string());
        self.sys.set_gossip_port(member.get_gossip_port() as u32);
        self.persistent = true;
    }

    fn update_from_health(&mut self, health: Health) {
        self.alive = false;
        self.suspect = false;
        self.confirmed = false;
        self.departed = false;
        match health {
            Health::Alive => self.alive = true,
            Health::Suspect => self.suspect = true,
            Health::Confirmed => self.confirmed = true,
            Health::Departed => self.departed = true,
        }
    }

    /// Is this member currently considered to be alive or not?
    pub fn alive(&self) -> bool {
        self.alive
    }
}

fn service_group_from_str(sg: &str) -> Result<ServiceGroup, hcore::Error> {
    ServiceGroup::from_str(sg).map_err(|e| {
        outputln!(
            "Malformed service group; cannot populate configuration data. \
                           Aborting.: {}",
            e
        );
        e
    })
}

#[cfg(test)]
mod tests {
    use hcore::package::ident::PackageIdent;
    use hcore::service::ServiceGroup;
    use butterfly::member::MemberList;
    use butterfly::rumor::service::Service as ServiceRumor;
    use butterfly::rumor::service_config::ServiceConfig as ServiceConfigRumor;
    use butterfly::rumor::service_file::ServiceFile as ServiceFileRumor;
    use butterfly::rumor::election::Election as ElectionRumor;
    use butterfly::rumor::election::ElectionUpdate as ElectionUpdateRumor;
    use butterfly::rumor::service::SysInfo;
    use butterfly::rumor::RumorStore;
    use census::CensusRing;

    #[test]
    fn update_from_rumors() {
        let mut sys_info = SysInfo::new();
        sys_info.set_ip("1.2.3.4".to_string());
        sys_info.set_hostname("hostname".to_string());
        sys_info.set_gossip_ip("0.0.0.0".to_string());
        sys_info.set_gossip_port(7777);
        sys_info.set_http_gateway_ip("0.0.0.0".to_string());
        sys_info.set_http_gateway_port(9631);
        let pg_id = PackageIdent::new(
            "starkandwayne",
            "shield",
            Some("0.10.4"),
            Some("20170419115548"),
        );
        let sg_one = ServiceGroup::new(None, "shield", "one", None).unwrap();

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let service_one =
            ServiceRumor::new("member-a".to_string(), &pg_id, &sg_one, &sys_info, None);
        let sg_two = ServiceGroup::new(None, "shield", "two", None).unwrap();
        let service_two =
            ServiceRumor::new("member-b".to_string(), &pg_id, &sg_two, &sys_info, None);
        let service_three =
            ServiceRumor::new("member-a".to_string(), &pg_id, &sg_two, &sys_info, None);

        service_store.insert(service_one);
        service_store.insert(service_two);
        service_store.insert(service_three);

        let election_store: RumorStore<ElectionRumor> = RumorStore::default();
        let mut election = ElectionRumor::new("member-a", sg_one.clone(), 10);
        election.finish();
        election_store.insert(election);

        let election_update_store: RumorStore<ElectionUpdateRumor> = RumorStore::default();
        let mut election_update = ElectionUpdateRumor::new("member-b", sg_two.clone(), 10);
        election_update.finish();
        election_update_store.insert(election_update);

        let member_list = MemberList::new();

        let service_config_store: RumorStore<ServiceConfigRumor> = RumorStore::default();
        let service_file_store: RumorStore<ServiceFileRumor> = RumorStore::default();
        let mut ring = CensusRing::new("member-b".to_string());
        ring.update_from_rumors(
            &service_store,
            &election_store,
            &election_update_store,
            &member_list,
            &service_config_store,
            &service_file_store,
        );
        let census_group_one = ring.census_group_for(&sg_one).unwrap();
        assert!(census_group_one.me().is_none());
        assert_eq!(census_group_one.leader().unwrap().member_id, "member-a");
        assert!(census_group_one.update_leader().is_none());

        let census_group_two = ring.census_group_for(&sg_two).unwrap();
        assert_eq!(
            census_group_two.me().unwrap().member_id,
            "member-b".to_string()
        );
        assert_eq!(
            census_group_two.update_leader().unwrap().member_id,
            "member-b".to_string()
        );

        let members = census_group_two.members();
        assert_eq!(members[0].member_id, "member-a");
        assert_eq!(members[1].member_id, "member-b");
    }
}
