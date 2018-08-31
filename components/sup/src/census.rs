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

use std::collections::{btree_map::Entry, BTreeMap, HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

use butterfly::member::{Health, MemberList};
use butterfly::message::BfUuid;
use butterfly::rumor::election::Election as ElectionRumor;
use butterfly::rumor::election::ElectionStatus as ElectionStatusRumor;
use butterfly::rumor::election::ElectionUpdate as ElectionUpdateRumor;
use butterfly::rumor::service::Service as ServiceRumor;
use butterfly::rumor::service::SysInfo;
use butterfly::rumor::service_config::ServiceConfig as ServiceConfigRumor;
use butterfly::rumor::service_file::ServiceFile as ServiceFileRumor;
use butterfly::rumor::RumorStore;
use butterfly::zone::{Reachable, ZoneAddress, ZoneList};
use hcore;
use hcore::package::PackageIdent;
use hcore::service::ServiceGroup;
use toml;

use error::{Error, SupError};

static LOGKEY: &'static str = "CE";

pub type MemberId = String;

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
    last_zone_list_counter: usize,
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
            last_zone_list_counter: 0,
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
        zone_list: &ZoneList,
    ) {
        // If ANY new rumor, of any type, has been received,
        // reconstruct the entire census state to ensure consistency
        if (service_rumors.get_update_counter() > self.last_service_counter)
            || (member_list.get_update_counter() > self.last_membership_counter)
            || (election_rumors.get_update_counter() > self.last_election_counter)
            || (election_update_rumors.get_update_counter() > self.last_election_update_counter)
            || (service_config_rumors.get_update_counter() > self.last_service_config_counter)
            || (service_file_rumors.get_update_counter() > self.last_service_file_counter)
            || (zone_list.get_update_counter() > self.last_zone_list_counter)
        {
            self.changed = true;

            self.populate_census(service_rumors, member_list, zone_list);
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
            self.last_zone_list_counter = zone_list.get_update_counter();
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
        zone_list: &ZoneList,
    ) {
        let our_zone_id = member_list
            .members
            .read()
            .expect("Member list lock poisoned")
            .get(&self.local_member_id)
            .map(|m| m.zone_id)
            .unwrap_or_else(|| BfUuid::nil());
        let updates_for_local_only = our_zone_id.is_nil();

        // Populate our census; new groups are created here, as are
        // new members of those groups.
        service_rumors.with_keys(|(service_group, rumors)| {
            if let Ok(sg) = service_group_from_str(service_group) {
                let census_group = self
                    .census_groups
                    .entry(sg.clone())
                    .or_insert(CensusGroup::new(sg, &self.local_member_id));
                census_group.update_from_service_rumors_members_and_zones(
                    rumors,
                    member_list,
                    zone_list,
                    our_zone_id,
                    updates_for_local_only,
                );
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
        election_update_rumors.with_keys(|(service_group, rumors)| {
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(census_group) = self.census_groups.get_mut(&sg) {
                    let election = rumors.get("election").unwrap();
                    census_group.update_from_election_update_rumor(election);
                }
            }
        });
    }

    fn update_from_service_config(
        &mut self,
        service_config_rumors: &RumorStore<ServiceConfigRumor>,
    ) {
        service_config_rumors.with_keys(|(service_group, rumors)| {
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(service_config) = rumors.get("service_config") {
                    if let Some(census_group) = self.census_groups.get_mut(&sg) {
                        census_group.update_from_service_config_rumor(service_config);
                    }
                }
            }
        });
    }

    fn update_from_service_files(&mut self, service_file_rumors: &RumorStore<ServiceFileRumor>) {
        service_file_rumors.with_keys(|(service_group, rumors)| {
            if let Ok(sg) = service_group_from_str(service_group) {
                let census_group = self
                    .census_groups
                    .entry(sg.clone())
                    .or_insert(CensusGroup::new(sg, &self.local_member_id));
                census_group.update_from_service_file_rumors(rumors);
            }
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
    pub value: toml::value::Table,
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

    /// Same as `members`, but only returns members that are either
    /// alive or suspect, i.e., nothing that is confirmed dead or
    /// departed. These are the members that we'll reasonably be
    /// interacting with at runtime.
    pub fn active_members(&self) -> Vec<&CensusMember> {
        self.population
            .values()
            .filter(|cm| cm.alive() || cm.suspect())
            .collect()
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
            self.population.values().filter(|cm| cm.alive()).collect();
        if alive_members.len() <= 1 || self.me().is_none() {
            return None;
        }
        match alive_members
            .iter()
            .position(|cm| cm.member_id == self.me().unwrap().member_id)
        {
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

    fn update_from_service_rumors_members_and_zones(
        &mut self,
        rumors: &HashMap<String, ServiceRumor>,
        member_list: &MemberList,
        zone_list: &ZoneList,
        our_zone_uuid: BfUuid,
        updates_for_local_only: bool,
    ) {
        if updates_for_local_only {
            self.update_from_service_rumors_members_and_zones_local_only(rumors);
        } else {
            self.update_from_service_rumors_members_and_zones_all(
                rumors,
                member_list,
                zone_list,
                our_zone_uuid,
            );
        }
    }

    fn update_from_service_rumors_members_and_zones_local_only(
        &mut self,
        rumors: &HashMap<String, ServiceRumor>,
    ) {
        for (member_id, service_rumor) in rumors.iter() {
            if *member_id != self.local_member_id {
                self.population.remove(member_id);
                continue;
            }
            let zone_address = None;
            let named_ports = None;
            let health = Health::Alive;

            match self.population.entry(member_id.to_string()) {
                Entry::Occupied(oe) => {
                    oe.into_mut().update_from_service_rumor(
                        &self.service_group,
                        service_rumor,
                        zone_address,
                        named_ports,
                        health,
                    );
                }
                Entry::Vacant(ve) => {
                    let mut new_member = CensusMember::default();

                    new_member.update_from_service_rumor(
                        &self.service_group,
                        service_rumor,
                        zone_address,
                        named_ports,
                        health,
                    );
                    ve.insert(new_member);
                }
            }
        }
    }

    fn update_from_service_rumors_members_and_zones_all(
        &mut self,
        rumors: &HashMap<String, ServiceRumor>,
        member_list: &MemberList,
        zone_list: &ZoneList,
        our_zone_id: BfUuid,
    ) {
        let members = member_list
            .members
            .read()
            .expect("Members lock is poisoned");
        let our_member = match members.get(&self.local_member_id) {
            Some(member) => member,
            None => {
                outputln!("Skippping updating census from service rumors - no our member");
                // TODO: eh? no our member?
                return;
            }
        };

        for (member_id, service_rumor) in rumors.iter() {
            let service_member = match members.get(member_id) {
                Some(member) => member,
                None => {
                    outputln!("Skippping updating census from a service rumor - no service member");
                    self.population.remove(member_id);
                    continue;
                }
            };
            let service_member_zone_id = service_member.zone_id;
            if service_member_zone_id.is_nil() {
                outputln!("Skippping updating census from a service rumor - invalid or nil zone");
                self.population.remove(member_id);
                continue;
            }
            let reachable = zone_list.directly_reachable(
                our_zone_id,
                service_member_zone_id,
                &our_member.additional_addresses,
                &service_member.additional_addresses,
            );
            let zone_address = match reachable {
                Reachable::Yes => None,
                Reachable::ThroughOtherZone(zone_id) => {
                    let mut zone_address_for_zone = None;

                    for zone_address in service_member.additional_addresses.iter() {
                        if zone_address.zone_id == zone_id {
                            zone_address_for_zone = Some(zone_address);
                            break;
                        }
                    }

                    if zone_address_for_zone.is_none() {
                        self.population.remove(member_id);
                        outputln!("Skippping updating census from a service rumor - no zone address for reachable zone");
                        continue;
                    }

                    if zone_address_for_zone.as_ref().unwrap().address.is_none() {
                        self.population.remove(member_id);
                        outputln!("Skippping updating census from a service rumor - no zone address for reachable zone");
                        continue;
                    }

                    zone_address_for_zone
                }
                Reachable::No => {
                    outputln!(
                        "Skippping updating census from a service rumor - service is unreachable"
                    );
                    self.population.remove(member_id);
                    continue;
                }
            };
            let named_ports = {
                if let Some(ref za) = zone_address {
                    let our_named_ports = service_rumor.tagged_ports.get(&za.tag);

                    if our_named_ports.is_none() {
                        outputln!("Skippping updating census from a service rumor - no ports under expected tag");
                        self.population.remove(member_id);
                        continue;
                    }

                    our_named_ports
                } else {
                    None
                }
            };
            // krnowak: other code was using unwrap, so I assume it's
            // fine to use it here. Hopefully it won't panic.
            let health = member_list.health_of(&service_member).unwrap();

            match self.population.entry(member_id.to_string()) {
                Entry::Occupied(oe) => {
                    oe.into_mut().update_from_service_rumor(
                        &self.service_group,
                        service_rumor,
                        zone_address,
                        named_ports,
                        health,
                    );
                }
                Entry::Vacant(ve) => {
                    let mut new_member = CensusMember::default();

                    new_member.update_from_service_rumor(
                        &self.service_group,
                        service_rumor,
                        zone_address,
                        named_ports,
                        health,
                    );
                    ve.insert(new_member);
                }
            }
        }
    }

    fn update_from_election_rumor(&mut self, election: &ElectionRumor) {
        self.leader_id = None;
        for census_member in self.population.values_mut() {
            if census_member.update_from_election_rumor(election) {
                self.leader_id = Some(census_member.member_id.clone());
            }
        }
        match election.status {
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
        match election.status {
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
                if self.service_config.is_none()
                    || service_config.incarnation
                        > self.service_config.as_ref().unwrap().incarnation
                {
                    self.service_config = Some(ServiceConfig {
                        incarnation: service_config.incarnation,
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
            let filename = service_file_rumor.filename.to_string();
            let file = self
                .service_files
                .entry(filename.clone())
                .or_insert(ServiceFile::default());

            if service_file_rumor.incarnation > file.incarnation {
                match service_file_rumor.body() {
                    Ok(body) => {
                        self.changed_service_files.push(filename.clone());
                        file.filename = filename.clone();
                        file.incarnation = service_file_rumor.incarnation;
                        file.body = body;
                    }
                    Err(e) => warn!(
                        "Cannot decrypt service file for {} {} {}: {}",
                        self.service_group,
                        service_file_rumor.filename,
                        service_file_rumor.incarnation,
                        e
                    ),
                }
            }
        }
    }

    /// Determine what configuration keys the group as a whole
    /// exports. Returns a set of the top-level exported keys.
    ///
    /// This implementation is a righteous hack to cover the fact that
    /// there is not yet a centralized view of what a "group" actually
    /// exports! There has been some talk of having a "leader" role in
    /// all topologies, in which case we could just ask the leader
    /// what the group exports. Until that time, the best we can do is
    /// ask an active member what *they* export (if there is a leader,
    /// though, we'll just ask them).
    pub fn group_exports<'a>(&'a self) -> Result<HashSet<&'a String>, SupError> {
        self.leader()
            .or_else(|| self.active_members().first().map(|m| *m))
            .ok_or(sup_error!(Error::NoActiveMembers(
                self.service_group.clone()
            )))
            .map(|m| m.cfg.keys().collect())
    }
}
// NOTE: This is exposed to users in templates. Any public member is
// accessible to users, so change this interface with care.
//
// User-facing documentation is available at
// https://www.habitat.sh/docs/reference/#template-data; update that
// as required.
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
    fn update_from_service_rumor(
        &mut self,
        sg: &ServiceGroup,
        rumor: &ServiceRumor,
        zone_address: Option<&ZoneAddress>,
        named_ports: Option<&HashMap<String, u16>>,
        health: Health,
    ) {
        self.member_id = rumor.member_id.to_string();
        self.service = sg.service().to_string();
        self.group = sg.group().to_string();
        if let Some(org) = sg.org() {
            self.org = Some(org.to_string());
        }
        if let Some(appenv) = sg.application_environment() {
            self.application = Some(appenv.application().to_string());
            self.environment = Some(appenv.environment().to_string());
        }
        match PackageIdent::from_str(&rumor.pkg) {
            Ok(ident) => self.pkg = Some(ident),
            Err(err) => warn!("Received a bad package ident from gossip data, err={}", err),
        };
        self.setup_sys(rumor, &zone_address);
        self.setup_cfg(rumor, &named_ports);
        self.persistent = true;
        self.update_from_health(health);
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

    fn setup_sys(&mut self, rumor: &ServiceRumor, zone_address: &Option<&ZoneAddress>) {
        self.sys = rumor.sys.clone().into();
        if let Some(ref za) = zone_address {
            // TODO(krnowak): We won't get here if zone address has no
            // address, but it would certainly be better if we had a
            // data structure with a String for address instead of
            // Option<String>.
            self.sys.ip = za.address.clone().unwrap();
            // TODO(krnowak): What about hostname? Clear it?
            self.sys.gossip_ip = za.address.clone().unwrap();
            self.sys.gossip_port = za.gossip_port as u32;
            // TODO(krnowak): What about http/ctl gateway addresses and ports?
        }
    }

    fn setup_cfg(&mut self, rumor: &ServiceRumor, named_ports: &Option<&HashMap<String, u16>>) {
        self.cfg = toml::from_slice(&rumor.cfg).unwrap_or(toml::value::Table::default());
        if let Some(np) = named_ports {
            for (name, port) in np.iter() {
                if let Some(ref mut value) = self.cfg.get_mut(name) {
                    match value {
                        toml::value::Value::String(ref mut s) => {
                            *s = format!("{}", *port);
                        }
                        toml::value::Value::Integer(ref mut i) => {
                            *i = *port as i64;
                        }
                        _ => {
                            // TODO(krnowak): Warn?
                        }
                    }
                }
            }
        }
    }

    fn update_from_election_rumor(&mut self, election: &ElectionRumor) -> bool {
        self.election_is_running = election.status == ElectionStatusRumor::Running;
        self.election_is_no_quorum = election.status == ElectionStatusRumor::NoQuorum;
        self.election_is_finished = election.status == ElectionStatusRumor::Finished;
        if self.election_is_finished {
            if self.member_id == election.member_id {
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
        self.update_election_is_running = election.status == ElectionStatusRumor::Running;
        self.update_election_is_no_quorum = election.status == ElectionStatusRumor::NoQuorum;
        self.update_election_is_finished = election.status == ElectionStatusRumor::Finished;
        if self.update_election_is_finished {
            if self.member_id == election.member_id {
                self.update_leader = true;
                self.update_follower = false;
            } else {
                self.update_leader = false;
                self.update_follower = true;
            }
        }
        self.update_leader
    }

    /// Is this member currently considered to be alive or not?
    pub fn alive(&self) -> bool {
        self.alive
    }

    pub fn suspect(&self) -> bool {
        self.suspect
    }

    pub fn confirmed(&self) -> bool {
        self.confirmed
    }

    pub fn departed(&self) -> bool {
        self.departed
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
    use std::collections::HashMap;

    use super::*;
    use butterfly::member::{Health, Member, MemberList};
    use butterfly::rumor::election::Election as ElectionRumor;
    use butterfly::rumor::election::ElectionUpdate as ElectionUpdateRumor;
    use butterfly::rumor::service::Service as ServiceRumor;
    use butterfly::rumor::service::SysInfo;
    use butterfly::rumor::service_config::ServiceConfig as ServiceConfigRumor;
    use butterfly::rumor::service_file::ServiceFile as ServiceFileRumor;
    use butterfly::rumor::RumorStore;
    use butterfly::zone::{Zone, ZoneList};
    use hcore::package::ident::PackageIdent;
    use hcore::service::ServiceGroup;

    use toml;

    #[test]
    fn update_from_rumors() {
        let mut sys_info = SysInfo::default();
        sys_info.ip = "1.2.3.4".to_string();
        sys_info.hostname = "hostname".to_string();
        sys_info.gossip_ip = "0.0.0.0".to_string();
        sys_info.gossip_port = 7777;
        sys_info.http_gateway_ip = "0.0.0.0".to_string();
        sys_info.http_gateway_port = 9631;
        let pg_id = PackageIdent::new(
            "starkandwayne",
            "shield",
            Some("0.10.4"),
            Some("20170419115548"),
        );
        let sg_one = ServiceGroup::new(None, "shield", "one", None).unwrap();

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let service_one = ServiceRumor::new(
            "member-a".to_string(),
            &pg_id,
            sg_one.clone(),
            sys_info.clone(),
            None,
            HashMap::new(),
        );
        let sg_two = ServiceGroup::new(None, "shield", "two", None).unwrap();
        let service_two = ServiceRumor::new(
            "member-b".to_string(),
            &pg_id,
            sg_two.clone(),
            sys_info.clone(),
            None,
            HashMap::new(),
        );
        let service_three = ServiceRumor::new(
            "member-a".to_string(),
            &pg_id,
            sg_two.clone(),
            sys_info.clone(),
            None,
            HashMap::new(),
        );

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
        let mut zone_list = ZoneList::new();
        let zone_uuid = BfUuid::generate();
        let zone = Zone::new(zone_uuid, "member-a".to_string());
        let mut member_a = Member::default();

        member_a.id = "member-a".to_string();
        member_a.zone_id = zone_uuid;;

        let mut member_b = Member::default();

        member_b.id = "member-b".to_string();
        member_b.zone_id = zone_uuid;

        member_list.insert(member_a, Health::Alive);
        member_list.insert(member_b, Health::Alive);
        zone_list.insert(zone);

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
            &zone_list,
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

    #[test]
    fn update_from_rumors_from_different_zone() {
        let sys_info_inside = SysInfo {
            ip: "4.3.2.1".to_string(),
            hostname: "inside".to_string(),
            gossip_ip: "0.0.0.0".to_string(),
            gossip_port: 7777,
            http_gateway_ip: "0.0.0.0".to_string(),
            http_gateway_port: 9631,
            ctl_gateway_ip: "0.0.0.0".to_string(),
            ctl_gateway_port: 0,
        };
        let pg_id = PackageIdent::new(
            "starkandwayne",
            "shield",
            Some("0.10.4"),
            Some("20170419115548"),
        );
        let sg = ServiceGroup::new(None, "shield", "one", None).unwrap();
        let mut tagged_ports = HashMap::new();
        let mut named_ports = HashMap::new();

        named_ports.insert("port".to_string(), 12345);
        named_ports.insert("ssl-port".to_string(), 54321);
        tagged_ports.insert("tag".to_string(), named_ports);

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let mut cfg = BTreeMap::new();

        cfg.insert("port".to_string(), toml::value::Value::Integer(1234));
        cfg.insert(
            "ssl-port".to_string(),
            toml::value::Value::String("4321".to_string()),
        );
        cfg.insert("foo".to_string(), toml::value::Value::Boolean(true));

        let service_inside = ServiceRumor::new(
            "member-inside".to_string(),
            &pg_id,
            sg.clone(),
            sys_info_inside,
            Some(&cfg),
            tagged_ports,
        );

        service_store.insert(service_inside);

        let election_store: RumorStore<ElectionRumor> = RumorStore::default();
        let mut election = ElectionRumor::new("member-inside", sg.clone(), 10);
        election.finish();
        election_store.insert(election);

        let election_update_store: RumorStore<ElectionUpdateRumor> = RumorStore::default();
        let mut election_update = ElectionUpdateRumor::new("member-inside", sg.clone(), 10);
        election_update.finish();
        election_update_store.insert(election_update);

        let member_list = MemberList::new();
        let mut zone_list = ZoneList::new();
        let zone_uuid_outside = BfUuid::generate();
        let zone_uuid_inside = BfUuid::generate();
        let mut zone_outside = Zone::new(zone_uuid_outside, "member-outside".to_string());
        let mut zone_inside = Zone::new(zone_uuid_inside, "member-inside".to_string());

        zone_outside.child_zone_ids.push(zone_uuid_inside);
        zone_inside.parent_zone_id = Some(zone_uuid_outside);

        let mut member_outside = Member::default();

        member_outside.id = "member-outside".to_string();
        member_outside.zone_id = zone_uuid_outside;

        let mut member_inside = Member::default();
        let zone_address = ZoneAddress {
            zone_id: zone_uuid_outside,
            address: Some("1.2.3.4".to_string()),
            swim_port: 33333,
            gossip_port: 44444,
            tag: "tag".to_string(),
        };

        member_inside.id = "member-inside".to_string();
        member_inside.zone_id = zone_uuid_inside;
        member_inside.additional_addresses.push(zone_address);

        member_list.insert(member_outside, Health::Alive);
        member_list.insert(member_inside, Health::Alive);
        zone_list.insert(zone_outside);
        zone_list.insert(zone_inside);

        let service_config_store: RumorStore<ServiceConfigRumor> = RumorStore::default();
        let service_file_store: RumorStore<ServiceFileRumor> = RumorStore::default();
        let mut ring = CensusRing::new("member-outside".to_string());

        ring.update_from_rumors(
            &service_store,
            &election_store,
            &election_update_store,
            &member_list,
            &service_config_store,
            &service_file_store,
            &zone_list,
        );
        let census_group = ring.census_group_for(&sg).unwrap();
        assert!(census_group.me().is_none());
        assert_eq!(census_group.leader().unwrap().member_id, "member-inside");
        assert_eq!(
            census_group.update_leader().unwrap().member_id,
            "member-inside"
        );

        let test_member_inside = census_group.members()[0];

        assert_eq!(test_member_inside.member_id, "member-inside");
        // take the address from zone address, not from sys
        assert_eq!(test_member_inside.sys.ip, "1.2.3.4");
        // same for gossip ip
        assert_eq!(test_member_inside.sys.gossip_ip, "1.2.3.4");
        // same for gossip port
        assert_eq!(test_member_inside.sys.gossip_port, 44444);
        // check if configuration was changed accordingly
        assert_eq!(test_member_inside.cfg.len(), 3);
        assert_eq!(
            *test_member_inside.cfg.get("port").unwrap(),
            toml::value::Value::Integer(12345)
        );
        assert_eq!(
            *test_member_inside.cfg.get("ssl-port").unwrap(),
            toml::value::Value::String("54321".to_string())
        );
        // foo setting should be left intact
        assert_eq!(
            *test_member_inside.cfg.get("foo").unwrap(),
            toml::value::Value::Boolean(true)
        );
    }

    /// Create a bare-minimum CensusMember with the given Health
    fn test_census_member(id: MemberId, health: Health) -> CensusMember {
        CensusMember {
            member_id: id,
            pkg: None,
            application: None,
            environment: None,
            service: "test_service".to_string(),
            group: "default".to_string(),
            org: None,
            persistent: false,
            leader: false,
            follower: false,
            update_leader: false,
            update_follower: false,
            election_is_running: false,
            election_is_no_quorum: false,
            election_is_finished: false,
            update_election_is_running: false,
            update_election_is_no_quorum: false,
            update_election_is_finished: false,
            sys: SysInfo::default(),
            alive: health == Health::Alive,
            suspect: health == Health::Suspect,
            confirmed: health == Health::Confirmed,
            departed: health == Health::Departed,
            cfg: BTreeMap::new(),
        }
    }

    #[test]
    fn active_members_leaves_only_active_members() {
        let population = vec![
            test_census_member("live-one".to_string(), Health::Alive),
            test_census_member("suspect-one".to_string(), Health::Suspect),
            test_census_member("confirmed-one".to_string(), Health::Confirmed),
            test_census_member("departed-one".to_string(), Health::Departed),
        ];

        let sg: ServiceGroup = "test-service.default"
            .parse()
            .expect("This should be a valid service group");

        let mut census_group = CensusGroup::new(sg, &"live-one".to_string());
        for member in population {
            census_group
                .population
                .insert(member.member_id.clone(), member);
        }

        let active_members = census_group.active_members();
        assert_eq!(active_members.len(), 2);
        assert_eq!(active_members[0].member_id, "live-one");
        assert_eq!(active_members[1].member_id, "suspect-one");
    }

}
