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

use std::str::FromStr;
use std::collections::HashMap;
use std::collections::BTreeMap;

use butterfly;
use butterfly::member::{MemberList, Member, Health};
use butterfly::rumor::service::Service as ServiceRumor;
use butterfly::rumor::service_file::ServiceFile as ServiceFileRumor;
use butterfly::rumor::service_config::ServiceConfig as ServiceConfigRumor;
use butterfly::rumor::election::Election as ElectionRumor;
use butterfly::rumor::election::ElectionUpdate as ElectionUpdateRumor;
use butterfly::rumor::RumorStore;
use butterfly::rumor::service::SysInfo;
use eventsrv::message::event::{CensusEntry as CensusEntryProto, PackageIdent as PackageIdentProto,
                               SysInfo as SysInfoProto};
use hcore;
use hcore::service::ServiceGroup;
use hcore::package::PackageIdent;
use toml;

static LOGKEY: &'static str = "CE";

pub type MemberId = String;
#[derive(Debug, Deserialize, Serialize)]
pub struct CensusRing {
    pub changed: bool,

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
    pub fn new<I: Into<MemberId>>(local_member_id: I) -> Self {
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

    pub fn update_from_rumors(&mut self,
                              service_rumors: &RumorStore<ServiceRumor>,
                              election_rumors: &RumorStore<ElectionRumor>,
                              election_update_rumors: &RumorStore<ElectionUpdateRumor>,
                              member_list: &MemberList,
                              service_config_rumors: &RumorStore<ServiceConfigRumor>,
                              service_file_rumors: &RumorStore<ServiceFileRumor>) {
        self.changed = false;
        self.update_from_service_store(service_rumors);
        self.update_from_election_store(election_rumors);
        self.update_from_election_update_store(election_update_rumors);
        self.update_from_member_list(member_list);
        self.update_from_service_config(service_config_rumors);
        self.update_from_service_files(service_file_rumors);
    }

    pub fn census_group_for(&self, sg: &ServiceGroup) -> Option<&CensusGroup> {
        self.census_groups.get(sg)
    }

    pub fn groups(&self) -> Vec<&CensusGroup> {
        self.census_groups.values().map(|cg| cg).collect()
    }

    fn update_from_service_store(&mut self,
                                 service_rumors: &RumorStore<ServiceRumor>) {
        if service_rumors.get_update_counter() <= self.last_service_counter {
            return;
        }
        self.changed = true;
        service_rumors.with_keys(|(service_group, rumors)| if let Ok(sg) =
            service_group_from_str(service_group) {
                                     let mut census_group =
                    self.census_groups
                    .entry(sg.clone())
                    .or_insert(CensusGroup::new(sg, &self.local_member_id));
                                     census_group.update_from_service_rumors(rumors);
                                 });
        self.last_service_counter = service_rumors.get_update_counter();
    }

    fn update_from_election_store(&mut self, election_rumors: &RumorStore<ElectionRumor>) {
        if election_rumors.get_update_counter() <= self.last_election_counter {
            return;
        }
        self.changed = true;
        election_rumors.with_keys(|(service_group, rumors)| {
            let election = rumors.get("election").unwrap();
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(census_group) = self.census_groups.get_mut(&sg) {
                    census_group.update_from_election_rumor(election);
                }
            }
        });
        self.last_election_counter = election_rumors.get_update_counter();
    }

    fn update_from_election_update_store(&mut self,
                                         election_update_rumors: &RumorStore<ElectionUpdateRumor>) {
        if election_update_rumors.get_update_counter() <= self.last_election_update_counter {
            return;
        }
        self.changed = true;
        election_update_rumors.with_keys(|(service_group, rumors)| {
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(census_group) = self.census_groups.get_mut(&sg) {
                    let election = rumors.get("election").unwrap();
                    census_group.update_from_election_update_rumor(election);
                }
            }
        });
        self.last_election_update_counter = election_update_rumors.get_update_counter();
    }

    fn update_from_member_list(&mut self, member_list: &MemberList) {
        if member_list.get_update_counter() <= self.last_membership_counter {
            return;
        }
        self.changed = true;
        member_list.with_members(|member| if let Some(census_member) =
            self.find_member_mut(&member.get_id().to_string()) {
                                     census_member.update_from_member(&member);
                                     if let Some(health) = member_list.health_of(member) {
                                         census_member.update_from_health(health);
                                     }
                                 });
        self.last_membership_counter = member_list.get_update_counter();
    }

    fn update_from_service_config(&mut self,
                                  service_config_rumors: &RumorStore<ServiceConfigRumor>) {
        if service_config_rumors.get_update_counter() <= self.last_service_config_counter {
            return;
        }
        self.changed = true;
        service_config_rumors.with_keys(|(service_group, rumors)| {
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(service_config) = rumors.get("service_config") {
                    if let Some(census_group) = self.census_groups.get_mut(&sg) {
                        census_group.update_from_service_config_rumor(service_config);
                    }
                }
            }
        });
        self.last_service_config_counter = service_config_rumors.get_update_counter();
    }

    fn update_from_service_files(&mut self, service_file_rumors: &RumorStore<ServiceFileRumor>) {
        if service_file_rumors.get_update_counter() <= self.last_service_file_counter {
            return;
        }
        self.changed = true;
        service_file_rumors.with_keys(|(service_group, rumors)| if let Ok(sg) =
            service_group_from_str(service_group) {
                                          let mut census_group =
                    self.census_groups
                    .entry(sg.clone())
                    .or_insert(CensusGroup::new(sg, &self.local_member_id));
                                          census_group.update_from_service_file_rumors(rumors);
                                      });
        self.last_service_file_counter = service_file_rumors.get_update_counter();
    }

    fn find_member_mut(&mut self, member_id: &MemberId) -> Option<&mut CensusMember> {
        for group in self.census_groups.values_mut() {
            if let Some(member) = group.find_member_mut(member_id) {
                return Some(member);
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum ElectionStatus {
    None,
    ElectionInProgress,
    ElectionNoQuorum,
    ElectionFinished,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ServiceFile {
    pub filename: String,
    pub incarnation: u64,
    pub body: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceConfig {
    pub incarnation: u64,
    pub value: toml::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CensusGroup {
    pub service_group: ServiceGroup,
    pub election_status: ElectionStatus,
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
            local_member_id: local_member_id.clone(),
            population: BTreeMap::new(),
            leader_id: None,
            update_leader_id: None,
            service_config: None,
            service_files: HashMap::new(),
            changed_service_files: Vec::new(),
        }
    }

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
        let alive_members: Vec<&CensusMember> = self.population
            .values()
            .filter(|cm| cm.alive.unwrap_or(false))
            .collect();
        if alive_members.len() <= 1 || self.me().is_none() {
            return None;
        }
        match alive_members
                  .iter()
                  .position(|cm| cm.member_id == self.me().unwrap().member_id) {
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
            let mut member = self.population
                .entry(member_id.to_string())
                .or_insert(CensusMember::default());
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
            butterfly::rumor::election::Election_Status::Running => {
                self.election_status = ElectionStatus::ElectionInProgress;
            }
            butterfly::rumor::election::Election_Status::NoQuorum => {
                self.election_status = ElectionStatus::ElectionNoQuorum;
            }
            butterfly::rumor::election::Election_Status::Finished => {
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
    }

    fn update_from_service_config_rumor(&mut self, service_config: &ServiceConfigRumor) {
        match service_config.config() {
            Ok(config) => {
                if self.service_config.is_none() ||
                   service_config.get_incarnation() >
                   self.service_config.as_ref().unwrap().incarnation {
                    self.service_config = Some(ServiceConfig {
                                                   incarnation: service_config.get_incarnation(),
                                                   value: config,
                                               });
                }
            }
            Err(err) => warn!("{}", err),
        }
    }

    fn update_from_service_file_rumors(&mut self,
                                       service_file_rumors: &HashMap<String, ServiceFileRumor>) {
        self.changed_service_files.clear();
        for (_m_id, service_file_rumor) in service_file_rumors.iter() {
            let filename = service_file_rumor.get_filename().to_string();
            let file = self.service_files
                .entry(filename.clone())
                .or_insert(ServiceFile::default());

            if service_file_rumor.get_incarnation() > file.incarnation {
                match service_file_rumor.body() {
                    Ok(body) => {
                        self.changed_service_files.push(filename.clone());
                        file.filename = filename.clone();
                        file.incarnation = service_file_rumor.get_incarnation();
                        file.body = body;
                    }
                    Err(e) => {
                        warn!("Cannot decrypt service file for {} {} {}: {}",
                              self.service_group,
                              service_file_rumor.get_filename(),
                              service_file_rumor.get_incarnation(),
                              e)
                    }
                }
            }
        }
    }

    fn find_member_mut(&mut self, member_id: &MemberId) -> Option<&mut CensusMember> {
        self.population.get_mut(member_id)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize,Default)]
pub struct CensusMember {
    pub member_id: MemberId,
    pub pkg: Option<PackageIdent>,

    service: String,
    group: String,
    org: Option<String>,
    cfg: toml::value::Table,
    sys: SysInfo,
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

impl CensusMember {
    pub fn as_protobuf(&self) -> CensusEntryProto {
        let mut cep = CensusEntryProto::new();
        cep.set_member_id(self.member_id.clone());
        cep.set_service(self.service.clone());
        cep.set_group(self.group.clone());
        cep.set_org(self.org.as_ref().unwrap_or(&"".to_string()).clone());

        let cfg_str = toml::to_string(&self.cfg).unwrap();
        cep.set_cfg(cfg_str.into_bytes());

        let mut sys_info = SysInfoProto::new();
        sys_info.set_ip(self.sys.ip.clone());
        sys_info.set_hostname(self.sys.hostname.clone());
        sys_info.set_gossip_ip(self.sys.gossip_ip.clone());
        sys_info.set_gossip_port(self.sys.gossip_port.clone());
        sys_info.set_http_gateway_ip(self.sys.http_gateway_ip.clone());
        sys_info.set_http_gateway_port(self.sys.http_gateway_port.clone());
        cep.set_sys(sys_info);

        if self.pkg.is_some() {
            let pkg = self.pkg.clone().unwrap();
            let mut pkg_ident = PackageIdentProto::new();
            pkg_ident.set_origin(pkg.origin);
            pkg_ident.set_name(pkg.name);
            pkg_ident.set_version(pkg.version.unwrap_or(String::new()));
            pkg_ident.set_release(pkg.release.unwrap_or(String::new()));
            cep.set_pkg(pkg_ident);
        }

        cep.set_leader(self.leader.unwrap_or(false));
        cep.set_follower(self.follower.unwrap_or(false));
        cep.set_update_leader(self.update_leader.unwrap_or(false));
        cep.set_update_follower(self.update_follower.unwrap_or(false));
        cep.set_election_is_running(self.election_is_running.unwrap_or(false));
        cep.set_election_is_no_quorum(self.election_is_no_quorum.unwrap_or(false));
        cep.set_election_is_finished(self.election_is_finished.unwrap_or(false));
        cep.set_update_election_is_running(self.update_election_is_running.unwrap_or(false));
        cep.set_update_election_is_no_quorum(self.update_election_is_no_quorum.unwrap_or(false));
        cep.set_update_election_is_finished(self.update_election_is_finished.unwrap_or(false));
        cep.set_initialized(self.initialized.unwrap_or(false));
        cep.set_alive(self.alive.unwrap_or(false));
        cep.set_suspect(self.suspect.unwrap_or(false));
        cep.set_confirmed(self.confirmed.unwrap_or(false));
        cep.set_persistent(self.persistent.unwrap_or(false));
        cep
    }

    fn update_from_service_rumor(&mut self, sg: &ServiceGroup, rumor: &ServiceRumor) {
        self.member_id = String::from(rumor.get_member_id());
        self.service = sg.service().to_string();
        self.group = sg.group().to_string();
        if let Some(org) = sg.org() {
            self.org = Some(org.to_string());
        }
        match PackageIdent::from_str(rumor.get_pkg()) {
            Ok(ident) => self.pkg = Some(ident),
            Err(err) => warn!("Received a bad package ident from gossip data, err={}", err),
        };
        self.cfg = toml::from_slice(rumor.get_cfg()).unwrap_or(toml::value::Table::default());
        self.sys = toml::from_slice(rumor.get_sys()).unwrap_or(SysInfo::default());
    }

    fn update_from_election_rumor(&mut self, election: &ElectionRumor) -> bool {
        match election.get_status() {
            butterfly::rumor::election::Election_Status::Running => {
                self.leader = Some(false);
                self.follower = Some(false);
                self.election_is_running = Some(true);
                self.election_is_no_quorum = Some(false);
                self.election_is_finished = Some(false);
            }
            butterfly::rumor::election::Election_Status::NoQuorum => {
                self.leader = Some(false);
                self.follower = Some(false);
                self.election_is_running = Some(false);
                self.election_is_no_quorum = Some(true);
                self.election_is_finished = Some(false);
            }
            butterfly::rumor::election::Election_Status::Finished => {
                self.election_is_running = Some(false);
                self.election_is_no_quorum = Some(false);
                self.election_is_finished = Some(true);
                if self.member_id == election.get_member_id() {
                    self.leader = Some(true);
                    self.follower = Some(false);
                    return true;
                } else {
                    self.leader = Some(false);
                    self.follower = Some(true);
                }
            }
        }
        false
    }

    fn update_from_election_update_rumor(&mut self, election: &ElectionUpdateRumor) -> bool {
        match election.get_status() {
            butterfly::rumor::election::Election_Status::Running => {
                self.update_leader = Some(false);
                self.update_follower = Some(false);
                self.update_election_is_running = Some(true);
                self.update_election_is_no_quorum = Some(false);
                self.update_election_is_finished = Some(false);
            }
            butterfly::rumor::election::Election_Status::NoQuorum => {
                self.update_leader = Some(false);
                self.update_follower = Some(false);
                self.update_election_is_running = Some(false);
                self.update_election_is_no_quorum = Some(true);
                self.update_election_is_finished = Some(false);
            }
            butterfly::rumor::election::Election_Status::Finished => {
                self.update_election_is_running = Some(false);
                self.update_election_is_no_quorum = Some(false);
                self.update_election_is_finished = Some(true);
                if self.member_id == election.get_member_id() {
                    self.update_leader = Some(true);
                    self.update_follower = Some(false);
                    return true;
                } else {
                    self.update_leader = Some(false);
                    self.update_follower = Some(true);
                }
            }
        }
        false
    }

    fn update_from_member(&mut self, member: &Member) {
        self.sys.gossip_ip = member.get_address().to_string();
        self.sys.gossip_port = member.get_gossip_port().to_string();
        self.persistent = Some(true);
    }

    fn update_from_health(&mut self, health: Health) {
        match health {
            Health::Alive => {
                self.alive = Some(true);
                self.suspect = Some(false);
                self.confirmed = Some(false);
            }
            Health::Suspect => {
                self.alive = Some(false);
                self.suspect = Some(true);
                self.confirmed = Some(false);
            }
            Health::Confirmed => {
                self.alive = Some(false);
                self.suspect = Some(false);
                self.confirmed = Some(true);
            }
        }
    }
}

fn service_group_from_str(sg: &str) -> Result<ServiceGroup, hcore::Error> {
    ServiceGroup::from_str(sg)
            .map_err(|e| {
                         outputln!("Malformed service group; cannot populate configuration data. \
                           Aborting.: {}",
                                   e);
                         e
                     })
}

#[cfg(test)]
mod tests {
    mod census {
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
            let sys_info = SysInfo {
                ip: "1.2.3.4".to_string(),
                hostname: "hostname".to_string(),
                gossip_ip: "0.0.0.0".to_string(),
                gossip_port: "7777".to_string(),
                http_gateway_ip: "0.0.0.0".to_string(),
                http_gateway_port: "9631".to_string(),
            };
            let pg_id = PackageIdent::new("starkandwayne",
                                          "shield",
                                          Some("0.10.4"),
                                          Some("20170419115548"));
            let sg_one = ServiceGroup::new("shield", "one", None).unwrap();

            let service_store: RumorStore<ServiceRumor> = RumorStore::default();
            let service_one =
                ServiceRumor::new("member-a".to_string(), &pg_id, &sg_one, &sys_info, None);
            let sg_two = ServiceGroup::new("shield", "two", None).unwrap();
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
            ring.update_from_rumors(&service_store,
                                    &election_store,
                                    &election_update_store,
                                    &member_list,
                                    &service_config_store,
                                    &service_file_store);
            let census_group_one = ring.census_group_for(&sg_one).unwrap();
            assert_eq!(census_group_one.me(), None);
            assert_eq!(census_group_one.leader().unwrap().member_id, "member-a");
            assert_eq!(census_group_one.update_leader(), None);

            let census_group_two = ring.census_group_for(&sg_two).unwrap();
            assert_eq!(census_group_two.me().unwrap().member_id,
                       "member-b".to_string());
            assert_eq!(census_group_two.update_leader().unwrap().member_id,
                       "member-b".to_string());

            let members = census_group_two.members();
            assert_eq!(members[0].member_id, "member-a");
            assert_eq!(members[1].member_id, "member-b");
        }
    }
}
