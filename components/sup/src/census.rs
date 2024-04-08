use crate::error::Error;
use habitat_butterfly::{member::{Health,
                                 Member,
                                 MemberList,
                                 Membership},
                        rumor::{election::{Election as ElectionRumor,
                                           ElectionStatus as ElectionStatusRumor,
                                           ElectionUpdate as ElectionUpdateRumor},
                                service::{Service as ServiceRumor,
                                          SysInfo},
                                service_config::ServiceConfig as ServiceConfigRumor,
                                service_file::ServiceFile as ServiceFileRumor,
                                ConstIdRumor as _,
                                RumorStore}};
use habitat_common::outputln;
use habitat_core::{self,
                   crypto::keys::KeyCache,
                   package::PackageIdent,
                   service::ServiceGroup};
use log::warn;
use serde::{ser::SerializeStruct,
            Serialize,
            Serializer};
use std::{borrow::Cow,
          collections::{BTreeMap,
                        HashMap,
                        HashSet},
          fmt,
          iter::IntoIterator,
          result,
          str::{self,
                FromStr}};

static LOGKEY: &str = "CE";

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
}

impl CensusRing {
    /// Indicates whether the census has changed since the last time
    /// we looked at rumors.
    pub fn changed(&self) -> bool { self.changed }

    pub fn new<I>(local_member_id: I) -> Self
        where I: Into<MemberId>
    {
        CensusRing { changed: false,
                     census_groups: HashMap::new(),
                     local_member_id: local_member_id.into(),
                     last_service_counter: 0,
                     last_election_counter: 0,
                     last_election_update_counter: 0,
                     last_membership_counter: 0,
                     last_service_config_counter: 0,
                     last_service_file_counter: 0, }
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    #[allow(clippy::too_many_arguments)]
    pub fn update_from_rumors_rsr_mlr(&mut self,
                                      key_cache: &KeyCache,
                                      service_rumors: &RumorStore<ServiceRumor>,
                                      election_rumors: &RumorStore<ElectionRumor>,
                                      election_update_rumors: &RumorStore<ElectionUpdateRumor>,
                                      member_list: &MemberList,
                                      service_config_rumors: &RumorStore<ServiceConfigRumor>,
                                      service_file_rumors: &RumorStore<ServiceFileRumor>) {
        // If ANY new rumor, of any type, has been received,
        // reconstruct the entire census state to ensure consistency
        if (service_rumors.get_update_counter() > self.last_service_counter)
           || (member_list.get_update_counter() > self.last_membership_counter)
           || (election_rumors.get_update_counter() > self.last_election_counter)
           || (election_update_rumors.get_update_counter() > self.last_election_update_counter)
           || (service_config_rumors.get_update_counter() > self.last_service_config_counter)
           || (service_file_rumors.get_update_counter() > self.last_service_file_counter)
        {
            self.changed = true;

            self.populate_census_rsr_mlr(service_rumors, member_list);
            self.update_from_election_store_rsr(election_rumors);
            self.update_from_election_update_store_rsr(election_update_rumors);
            self.update_from_service_config_rsr(key_cache, service_config_rumors);
            self.update_from_service_files_rsr(key_cache, service_file_rumors);

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

    pub fn groups(&self) -> Vec<&CensusGroup> { self.census_groups.values().collect() }

    /// Populates the census from `ServiceRumor`s and Butterfly-level
    /// membership lists.
    ///
    /// (Butterfly provides the health, the ServiceRumors provide the
    /// rest).
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    fn populate_census_rsr_mlr(&mut self,
                               service_rumors: &RumorStore<ServiceRumor>,
                               member_list: &MemberList) {
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
        for (service_group, rumors) in service_rumors.lock_rsr().iter() {
            if let Ok(sg) = service_group_from_str(service_group) {
                let local_member_id = Cow::from(&self.local_member_id);
                let census_group = self.census_groups
                                       .entry(sg.clone())
                                       .or_insert_with(|| CensusGroup::new(sg, &local_member_id));
                census_group.update_from_service_rumors(rumors);
            }
        }

        member_list.with_memberships_mlr(|Membership { member, health }| {
                       for group in self.census_groups.values_mut() {
                           if let Some(census_member) = group.find_member_mut(&member.id) {
                               census_member.update_from_member(&member);
                               census_member.update_from_health(health);
                           }
                       }
                       Ok(())
                   })
                   .ok();
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    fn update_from_election_store_rsr(&mut self, election_rumors: &RumorStore<ElectionRumor>) {
        for (service_group, rumors) in election_rumors.lock_rsr().iter() {
            let election = rumors.get(ElectionRumor::const_id()).unwrap();
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(census_group) = self.census_groups.get_mut(&sg) {
                    census_group.update_from_election_rumor(election);
                }
            }
        }
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    fn update_from_election_update_store_rsr(&mut self,
                                             election_update_rumors: &RumorStore<ElectionUpdateRumor>)
    {
        for (service_group, rumors) in election_update_rumors.lock_rsr().iter() {
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(census_group) = self.census_groups.get_mut(&sg) {
                    let election = rumors.get(ElectionUpdateRumor::const_id()).unwrap();
                    census_group.update_from_election_update_rumor(election);
                }
            }
        }
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    fn update_from_service_config_rsr(&mut self,
                                      key_cache: &KeyCache,
                                      service_config_rumors: &RumorStore<ServiceConfigRumor>) {
        for (service_group, rumors) in service_config_rumors.lock_rsr().iter() {
            if let Ok(sg) = service_group_from_str(service_group) {
                if let Some(service_config) = rumors.get(ServiceConfigRumor::const_id()) {
                    if let Some(census_group) = self.census_groups.get_mut(&sg) {
                        census_group.update_from_service_config_rumor(key_cache, service_config);
                    }
                }
            }
        }
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    fn update_from_service_files_rsr(&mut self,
                                     key_cache: &KeyCache,
                                     service_file_rumors: &RumorStore<ServiceFileRumor>) {
        for (service_group, rumors) in service_file_rumors.lock_rsr().iter() {
            if let Ok(sg) = service_group_from_str(service_group) {
                let local_member_id = Cow::from(&self.local_member_id);
                let census_group = self.census_groups
                                       .entry(sg.clone())
                                       .or_insert_with(|| CensusGroup::new(sg, &local_member_id));
                census_group.update_from_service_file_rumors(key_cache, rumors);
            }
        }
    }
}

/// This is a proxy struct to represent what information we're writing to the dat file, and
/// therefore what information gets sent out via the HTTP API. Right now, we're just wrapping the
/// actual CensusRing struct, but this will give us something we can refactor against without
/// worrying about breaking the data returned to users.
pub struct CensusRingProxy<'a>(&'a CensusRing);

impl<'a> CensusRingProxy<'a> {
    pub fn new(c: &'a CensusRing) -> Self { CensusRingProxy(c) }
}

impl<'a> Serialize for CensusRingProxy<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("census_ring", 9)?;
        strukt.serialize_field("changed", &self.0.changed)?;
        strukt.serialize_field("census_groups", &self.0.census_groups)?;
        strukt.serialize_field("local_member_id", &self.0.local_member_id)?;
        strukt.serialize_field("last_service_counter", &self.0.last_service_counter)?;
        strukt.serialize_field("last_election_counter", &self.0.last_election_counter)?;
        strukt.serialize_field("last_election_update_counter",
                               &self.0.last_election_update_counter)?;
        strukt.serialize_field("last_membership_counter", &self.0.last_membership_counter)?;
        strukt.serialize_field("last_service_config_counter",
                               &self.0.last_service_config_counter)?;
        strukt.serialize_field("last_service_file_counter",
                               &self.0.last_service_file_counter)?;
        strukt.end()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ElectionStatus {
    #[default]
    None,
    ElectionInProgress,
    ElectionNoQuorum,
    ElectionFinished,
}

impl fmt::Display for ElectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "in-progress" => Ok(ElectionStatus::ElectionInProgress),
            "no-quorum" => Ok(ElectionStatus::ElectionNoQuorum),
            "finished" => Ok(ElectionStatus::ElectionFinished),
            "none" => Ok(ElectionStatus::None),
            _ => Err(Error::BadElectionStatus(value.to_string())),
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
    pub filename:    String,
    pub incarnation: u64,
    pub body:        Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct ServiceConfig {
    pub incarnation: u64,
    pub value:       toml::value::Table,
}

#[derive(Debug)]
pub struct CensusGroup {
    pub service_group:          ServiceGroup,
    pub election_status:        ElectionStatus,
    pub update_election_status: ElectionStatus,
    pub pkg_incarnation:        u64,
    pub leader_id:              Option<MemberId>,
    pub service_config:         Option<ServiceConfig>,

    local_member_id:       MemberId,
    population:            BTreeMap<MemberId, CensusMember>,
    update_leader_id:      Option<MemberId>,
    changed_service_files: HashSet<String>,
    service_files:         HashMap<String, ServiceFile>,
}

impl CensusGroup {
    fn new(sg: ServiceGroup, local_member_id: &str) -> Self {
        CensusGroup { service_group:          sg,
                      election_status:        ElectionStatus::None,
                      update_election_status: ElectionStatus::None,
                      pkg_incarnation:        0,
                      local_member_id:        local_member_id.to_string(),
                      population:             BTreeMap::new(),
                      leader_id:              None,
                      update_leader_id:       None,
                      service_config:         None,
                      service_files:          HashMap::new(),
                      changed_service_files:  HashSet::new(), }
    }

    /// Returns the census member in the census ring for the running Supervisor.
    pub fn me(&self) -> Option<&CensusMember> { self.population.get(&self.local_member_id) }

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
    pub fn members(&self) -> impl Iterator<Item = &CensusMember> { self.population.values() }

    /// Same as `members`, but only returns members that are either
    /// alive or suspect, i.e., nothing that is confirmed dead or
    /// departed. These are the members that we'll reasonably be
    /// interacting with at runtime.
    pub fn active_members(&self) -> impl Iterator<Item = &CensusMember> {
        self.population
            .values()
            .filter(|cm| cm.alive() || cm.suspect())
    }

    /// Return references to all a `CensusGroup`'s `ServiceFiles`.
    pub fn service_files(&self) -> impl IntoIterator<Item = &ServiceFile> {
        self.service_files.values()
    }

    /// Return references to all a `CensusGroup`'s `ServiceFiles` that
    /// have changed since the last time the rumor network was
    /// consulted.
    pub fn changed_service_files(&self) -> Vec<&ServiceFile> {
        self.service_files()
            .into_iter()
            .filter(|f| self.changed_service_files.contains(&f.filename))
            .collect()
    }

    /// Return previous alive peer, the peer to your left in the ordered members list, or None if
    /// you have no alive peers.
    // XXX: Is me ever None or not Alive?
    // XXX: Should we include Suspect members too, or only strictly Alive ones?
    pub fn previous_peer(&self) -> Option<&CensusMember> {
        self.me()
            .and_then(|me| Self::previous_peer_impl(self.population.values(), me))
    }

    fn previous_peer_impl<'a>(members: impl Iterator<Item = &'a CensusMember>,
                              me: &CensusMember)
                              -> Option<&'a CensusMember> {
        let mut alive_members = members.filter(|cm| cm.alive());
        let mut previous = None;

        for member in alive_members.by_ref() {
            if member.member_id == me.member_id {
                return previous.or_else(|| alive_members.last());
            } else {
                previous = Some(member);
            }
        }

        None
    }

    fn update_from_service_rumors(&mut self, rumors: &HashMap<String, ServiceRumor>) {
        for (member_id, service_rumor) in rumors.iter() {
            // The group pkg_incarnation holds the highest incarnation of all of its
            // members. You might ask, "shouldn't it just take the incarnation of the
            // update leader?" Fair question! If a leader dies, the new leader may likely
            // be behind and we would not want to trust its incarnation. In the end, only
            // the update leader can increment the incarnation of its service rumor.
            if service_rumor.pkg_incarnation > self.pkg_incarnation {
                self.pkg_incarnation = service_rumor.pkg_incarnation;
            }
            // Yeah - we are ourself - we're alive.
            let is_self = member_id == &self.local_member_id;
            let member = self.population
                             .entry(member_id.to_string())
                             .or_insert_with(|| {
                                 // Note: this is where CensusMembers are created
                                 CensusMember { alive: is_self,
                                                ..Default::default() }
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

    fn update_from_service_config_rumor(&mut self,
                                        key_cache: &KeyCache,
                                        service_config: &ServiceConfigRumor) {
        match service_config.config(key_cache) {
            Ok(config) => {
                if self.service_config.is_none()
                   || service_config.incarnation > self.service_config.as_ref().unwrap().incarnation
                {
                    self.service_config = Some(ServiceConfig { incarnation:
                                                                   service_config.incarnation,
                                                               value:       config, });
                }
            }
            Err(err) => warn!("{}", err),
        }
    }

    fn update_from_service_file_rumors(&mut self,
                                       key_cache: &KeyCache,
                                       service_file_rumors: &HashMap<String, ServiceFileRumor>)
    {
        self.changed_service_files.clear();
        for (_m_id, service_file_rumor) in service_file_rumors.iter() {
            let filename = service_file_rumor.filename.to_string();
            let file = self.service_files.entry(filename.clone()).or_default();

            if service_file_rumor.incarnation > file.incarnation {
                match service_file_rumor.body(key_cache) {
                    Ok(body) => {
                        self.changed_service_files.insert(filename.clone());
                        file.filename = filename.clone();
                        file.incarnation = service_file_rumor.incarnation;
                        file.body = body;
                    }
                    Err(e) => {
                        warn!("Cannot decrypt service file for {} {} {}: {}",
                              self.service_group,
                              service_file_rumor.filename,
                              service_file_rumor.incarnation,
                              e)
                    }
                }
            }
        }
    }

    fn find_member_mut(&mut self, member_id: &str) -> Option<&mut CensusMember> {
        self.population.get_mut(member_id)
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
    pub fn group_exports(&self) -> Result<HashSet<&String>, Error> {
        self.leader()
            .or_else(|| self.active_members().next())
            .ok_or_else(|| Error::NoActiveMembers(self.service_group.clone()))
            .map(|m| m.cfg.keys().collect())
    }
}

impl Serialize for CensusGroup {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("census_group", 10)?;
        strukt.serialize_field("service_group", &self.service_group)?;
        strukt.serialize_field("election_status", &self.election_status)?;
        strukt.serialize_field("update_election_status", &self.update_election_status)?;
        strukt.serialize_field("pkg_incarnation", &self.pkg_incarnation)?;
        strukt.serialize_field("leader_id", &self.leader_id)?;
        strukt.serialize_field("service_config", &self.service_config)?;
        strukt.serialize_field("local_member_id", &self.local_member_id)?;

        let new_pop: BTreeMap<MemberId, CensusMemberProxy<'_>> =
            self.population
                .iter()
                .map(|(k, v)| (k.clone(), CensusMemberProxy::new(v)))
                .collect();

        strukt.serialize_field("population", &new_pop)?;
        strukt.serialize_field("update_leader_id", &self.update_leader_id)?;
        strukt.serialize_field("changed_service_files", &self.changed_service_files)?;
        strukt.serialize_field("service_files", &self.service_files)?;
        strukt.end()
    }
}

// User-facing documentation is available at
// https://www.habitat.sh/docs/reference/#template-data; update that
// as required.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CensusMember {
    pub member_id: MemberId,
    pub pkg: PackageIdent,
    pub pkg_incarnation: u64,
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
    pub alive: bool,
    pub suspect: bool,
    pub confirmed: bool,
    pub departed: bool,
    pub cfg: toml::value::Table,
}

impl CensusMember {
    fn update_from_service_rumor(&mut self, sg: &ServiceGroup, rumor: &ServiceRumor) {
        self.member_id = rumor.member_id.to_string();
        self.service = sg.service().to_string();
        self.group = sg.group().to_string();
        if let Some(org) = sg.org() {
            self.org = Some(org.to_string());
        }
        match PackageIdent::from_str(&rumor.pkg) {
            Ok(ident) => self.pkg = ident,
            Err(err) => warn!("Received a bad package ident from gossip data, err={}", err),
        };
        self.pkg_incarnation = rumor.pkg_incarnation;
        self.sys = rumor.sys.clone();
        self.cfg =
            toml::from_str(str::from_utf8(&rumor.cfg).unwrap_or_default()).unwrap_or_default();
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

    fn update_from_member(&mut self, member: &Member) {
        self.sys.gossip_ip = member.address.to_string();
        self.sys.gossip_port = u32::from(member.gossip_port);
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
    pub fn alive(&self) -> bool { self.alive }

    pub fn suspect(&self) -> bool { self.suspect }

    pub fn confirmed(&self) -> bool { self.confirmed }

    pub fn departed(&self) -> bool { self.departed }
}

/// This data structure just wraps the CensusMember and allows us to tweak the serialization logic.
#[derive(Debug, Clone)]
pub struct CensusMemberProxy<'a>(Cow<'a, CensusMember>);

impl<'a> CensusMemberProxy<'a> {
    pub fn new(c: &'a CensusMember) -> Self { CensusMemberProxy(Cow::Borrowed(c)) }

    #[cfg(test)]
    pub fn new_owned(c: CensusMember) -> Self { CensusMemberProxy(Cow::Owned(c)) }

    #[cfg(test)]
    pub fn to_mut(&mut self) -> &mut CensusMember { self.0.to_mut() }
}

impl std::ops::Deref for CensusMemberProxy<'_> {
    type Target = CensusMember;

    fn deref(&self) -> &Self::Target { &(self.0) }
}

impl<'a> Serialize for CensusMemberProxy<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("census_member", 24)?;
        strukt.serialize_field("member_id", &self.member_id)?;
        strukt.serialize_field("pkg", &self.pkg)?;
        strukt.serialize_field("pkg_incarnation", &self.pkg_incarnation)?;

        strukt.serialize_field("package", &self.pkg.to_string())?;
        strukt.serialize_field("service", &self.service)?;
        strukt.serialize_field("group", &self.group)?;
        strukt.serialize_field("org", &self.org)?;
        strukt.serialize_field("persistent", &self.persistent)?;
        strukt.serialize_field("leader", &self.leader)?;
        strukt.serialize_field("follower", &self.follower)?;
        strukt.serialize_field("update_leader", &self.update_leader)?;
        strukt.serialize_field("update_follower", &self.update_follower)?;
        strukt.serialize_field("election_is_running", &self.election_is_running)?;
        strukt.serialize_field("election_is_no_quorum", &self.election_is_no_quorum)?;
        strukt.serialize_field("election_is_finished", &self.election_is_finished)?;
        strukt.serialize_field("update_election_is_running",
                               &self.update_election_is_running)?;
        strukt.serialize_field("update_election_is_no_quorum",
                               &self.update_election_is_no_quorum)?;
        strukt.serialize_field("update_election_is_finished",
                               &self.update_election_is_finished)?;
        strukt.serialize_field("sys", &self.sys)?;
        strukt.serialize_field("alive", &self.alive)?;
        strukt.serialize_field("suspect", &self.suspect)?;
        strukt.serialize_field("confirmed", &self.confirmed)?;
        strukt.serialize_field("departed", &self.departed)?;
        strukt.serialize_field("cfg", &self.cfg)?;
        strukt.end()
    }
}

fn service_group_from_str(sg: &str) -> Result<ServiceGroup, habitat_core::Error> {
    ServiceGroup::from_str(sg).map_err(|e| {
                                  outputln!("Malformed service group; cannot populate \
                                             configuration data. Aborting.: {}",
                                            e);
                                  e
                              })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use habitat_butterfly::{member::{Health,
                                     MemberList},
                            rumor::{election::{self,
                                               Election as ElectionRumor,
                                               ElectionUpdate as ElectionUpdateRumor},
                                    service::{Service as ServiceRumor,
                                              SysInfo},
                                    service_config::ServiceConfig as ServiceConfigRumor,
                                    service_file::ServiceFile as ServiceFileRumor,
                                    RumorStore}};
    use habitat_core::{fs::CACHE_KEY_PATH,
                       package::ident::PackageIdent,
                       service::ServiceGroup};

    #[test]
    fn update_from_rumors() {
        let (ring, sg_one, sg_two) = test_census_ring();
        let census_group_one = ring.census_group_for(&sg_one).unwrap();
        assert!(census_group_one.me().is_none());
        assert_eq!(census_group_one.leader().unwrap().member_id, "member-a");
        assert!(census_group_one.update_leader().is_none());

        let census_group_two = ring.census_group_for(&sg_two).unwrap();
        assert_eq!(census_group_two.me().unwrap().member_id,
                   "member-b".to_string());
        assert_eq!(census_group_two.update_leader().unwrap().member_id,
                   "member-b".to_string());
        assert_eq!(census_group_two.pkg_incarnation, 2);

        let mut members = census_group_two.members();
        assert_eq!(members.next().unwrap().member_id, "member-a");
        assert_eq!(members.next().unwrap().member_id, "member-b");
    }

    #[test]
    fn census_ring_proxy_conforms_to_the_schema() {
        let (ring, ..) = test_census_ring();
        let crp = CensusRingProxy::new(&ring);
        let json = serde_json::to_string(&crp).unwrap();
        assert_valid(&json, "http_gateway_census_schema.json");
    }

    fn test_census_ring() -> (CensusRing, ServiceGroup, ServiceGroup) {
        let sys_info = SysInfo { ip: "1.2.3.4".to_string(),
                                 hostname: "hostname".to_string(),
                                 gossip_ip: "0.0.0.0".to_string(),
                                 gossip_port: 7777,
                                 http_gateway_ip: "0.0.0.0".to_string(),
                                 http_gateway_port: 9631,
                                 ..Default::default() };
        let pg_id = PackageIdent::new("starkandwayne",
                                      "shield",
                                      Some("0.10.4"),
                                      Some("20170419115548"));
        let sg_one = ServiceGroup::new("shield", "one", None).unwrap();

        let service_store: RumorStore<ServiceRumor> = RumorStore::default();
        let service_one = ServiceRumor::new("member-a".to_string(),
                                            &pg_id,
                                            sg_one.clone(),
                                            sys_info.clone(),
                                            None);
        let sg_two = ServiceGroup::new("shield", "two", None).unwrap();
        let mut service_two = ServiceRumor::new("member-b".to_string(),
                                                &pg_id,
                                                sg_two.clone(),
                                                sys_info.clone(),
                                                None);
        service_two.pkg_incarnation = 1;
        let mut service_three = ServiceRumor::new("member-a".to_string(),
                                                  &pg_id,
                                                  sg_two.clone(),
                                                  sys_info,
                                                  None);
        service_three.pkg_incarnation = 2;

        service_store.insert_rsw(service_one);
        service_store.insert_rsw(service_two);
        service_store.insert_rsw(service_three);

        let election_store: RumorStore<ElectionRumor> = RumorStore::default();
        let mut election = ElectionRumor::new("member-a",
                                              &sg_one,
                                              election::Term::default(),
                                              10,
                                              true /* has_quorum */);
        election.finish();
        election_store.insert_rsw(election);

        let election_update_store: RumorStore<ElectionUpdateRumor> = RumorStore::default();
        let mut election_update = ElectionUpdateRumor::new("member-b",
                                                           &sg_two,
                                                           election::Term::default(),
                                                           10,
                                                           true /* has_quorum */);
        election_update.finish();
        election_update_store.insert_rsw(election_update);

        let member_list = MemberList::new();

        let service_config_store: RumorStore<ServiceConfigRumor> = RumorStore::default();
        let service_file_store: RumorStore<ServiceFileRumor> = RumorStore::default();
        let mut ring = CensusRing::new("member-b".to_string());
        ring.update_from_rumors_rsr_mlr(&KeyCache::new(&*CACHE_KEY_PATH),
                                        &service_store,
                                        &election_store,
                                        &election_update_store,
                                        &member_list,
                                        &service_config_store,
                                        &service_file_store);

        (ring, sg_one, sg_two)
    }

    /// Create a bare-minimum CensusMember with the given Health
    fn test_census_member(id: &str, health: Health) -> CensusMember {
        let pkg = "habitat-testing/test_service".parse()
                                                .expect("valid package ident");
        CensusMember { member_id: id.into(),
                       pkg,
                       pkg_incarnation: 0,
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
                       cfg: toml::value::Table::new() }
    }

    #[test]
    fn active_members_leaves_only_active_members() {
        let population = vec![test_census_member("live-one", Health::Alive),
                              test_census_member("suspect-one", Health::Suspect),
                              test_census_member("confirmed-one", Health::Confirmed),
                              test_census_member("departed-one", Health::Departed),];

        let sg: ServiceGroup =
            "test-service.default".parse()
                                  .expect("This should be a valid service group");

        let mut census_group = CensusGroup::new(sg, "live-one");
        for member in population {
            census_group.population
                        .insert(member.member_id.clone(), member);
        }

        let mut active_members = census_group.active_members();
        assert_eq!(active_members.next().unwrap().member_id, "live-one");
        assert_eq!(active_members.next().unwrap().member_id, "suspect-one");
        assert!(active_members.next().is_none());
    }

    fn assert_eq_member_ids(cm: Option<&CensusMember>, id: Option<&str>) {
        assert_eq!(cm.map(|cm| cm.member_id.as_str()), id);
    }

    #[test]
    fn previous_peer_with_no_members() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me), None);
    }

    #[test]
    fn previous_peer_with_no_alive_members() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![test_census_member("left_of_me", Health::Confirmed),
                           me.clone(),];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me), None);
    }

    #[test]
    fn previous_peer_with_only_me() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![me.clone()];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me), None);
    }

    #[test]
    fn previous_peer_simple() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![test_census_member("left_of_me", Health::Alive), me.clone()];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me),
                             Some("left_of_me"));
    }

    #[test]
    fn previous_peer_wraparound() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![me.clone(),
                           test_census_member("left_of_me_with_wrapping", Health::Alive),];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me),
                             Some("left_of_me_with_wrapping"));
    }

    #[test]
    fn previous_peer_normal() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![test_census_member("2_left_of_me", Health::Alive),
                           test_census_member("left_of_me", Health::Alive),
                           me.clone(),
                           test_census_member("right_of_me", Health::Alive),];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me),
                             Some("left_of_me"));
    }

    #[test]
    fn previous_peer_with_confirmed() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![test_census_member("2_left_of_me", Health::Alive),
                           test_census_member("left_of_me", Health::Confirmed),
                           me.clone(),
                           test_census_member("right_of_me", Health::Alive),];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me),
                             Some("2_left_of_me"));
    }

    #[test]
    fn previous_peer_with_confirmed_and_wraparound() {
        let me = test_census_member("me", Health::Alive);
        let members = vec![test_census_member("left_of_me", Health::Confirmed),
                           me.clone(),
                           test_census_member("left_of_me_with_wrapping", Health::Alive),
                           test_census_member("2_right_of_me", Health::Confirmed),];
        assert_eq_member_ids(CensusGroup::previous_peer_impl(members.iter(), &me),
                             Some("left_of_me_with_wrapping"));
    }
}
