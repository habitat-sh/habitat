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

//! The Butterfly server.
//!
//! Creates `Server` structs, that hold everything we need to run the SWIM and Gossip protocol.
//! Winds up with 5 separate threads - inbound (incoming connections), outbound (the Probe
//! protocol), expire (turning Suspect members into Confirmed members), push (the fan-out rumors),
//! and pull (the inbound receipt of rumors.).

mod expire;
mod inbound;
mod outbound;
mod pull;
mod push;
pub mod timing;

use std::collections::HashSet;
use std::ffi;
use std::fmt::{self, Debug};
use std::fs;
use std::path::PathBuf;
use std::result;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};
use std::sync::mpsc::{self, channel};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::time::{Duration, Instant};

use habitat_core::crypto::SymKey;
use habitat_core::service::ServiceGroup;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use error::{Error, Result};
use member::{Health, Member, MemberList};
use message;
use network::{AddressAndPort, AddressForNetwork, Network};
use rumor::dat_file::DatFile;
use rumor::departure::Departure;
use rumor::election::{Election, ElectionUpdate};
use rumor::heat::RumorHeat;
use rumor::service::Service;
use rumor::service_config::ServiceConfig;
use rumor::service_file::ServiceFile;
use rumor::{Rumor, RumorKey, RumorStore, RumorType};
use swim::Ack;
use trace::{Trace, TraceKind};

/// The maximum number of other members we should notify when we shut
/// down and leave the ring.
const SELF_DEPARTURE_RUMOR_FANOUT: usize = 10;

type AckReceiver<A> = mpsc::Receiver<(A, Ack)>;
type AckSender<A> = mpsc::Sender<(A, Ack)>;

pub trait Suitability: Debug + Send + Sync {
    fn get(&self, service_group: &ServiceGroup) -> u64;
}

/// The server struct. Is thread-safe.
#[derive(Debug)]
pub struct Server<N: Network> {
    name: Arc<String>,
    member_id: Arc<String>,
    pub member: Arc<RwLock<Member>>,
    pub member_list: MemberList,
    pub host_address: AddressForNetwork<N>,
    ring_key: Arc<Option<SymKey>>,
    rumor_heat: RumorHeat,
    pub service_store: RumorStore<Service>,
    pub service_config_store: RumorStore<ServiceConfig>,
    pub service_file_store: RumorStore<ServiceFile>,
    pub election_store: RumorStore<Election>,
    pub update_store: RumorStore<ElectionUpdate>,
    pub departure_store: RumorStore<Departure>,
    suitability_lookup: Arc<Box<Suitability>>,
    data_path: Arc<Option<PathBuf>>,
    dat_file: Arc<RwLock<Option<DatFile>>>,
    swim_sender: Option<N::SwimSender>,
    departed: Arc<AtomicBool>,
    pub network: Arc<RwLock<N>>,
    // These are all here for testing support
    pause: Arc<AtomicBool>,
    pub trace: Arc<RwLock<Trace>>,
    swim_rounds: Arc<AtomicIsize>,
    gossip_rounds: Arc<AtomicIsize>,
    block_list: Arc<RwLock<HashSet<String>>>,
}

impl<N: Network> Clone for Server<N> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            member_id: self.member_id.clone(),
            member: self.member.clone(),
            member_list: self.member_list.clone(),
            host_address: self.host_address.clone(),
            ring_key: self.ring_key.clone(),
            rumor_heat: self.rumor_heat.clone(),
            service_store: self.service_store.clone(),
            service_config_store: self.service_config_store.clone(),
            service_file_store: self.service_file_store.clone(),
            election_store: self.election_store.clone(),
            update_store: self.update_store.clone(),
            departure_store: self.departure_store.clone(),
            suitability_lookup: self.suitability_lookup.clone(),
            data_path: self.data_path.clone(),
            dat_file: self.dat_file.clone(),
            departed: self.departed.clone(),
            network: self.network.clone(),
            pause: self.pause.clone(),
            trace: self.trace.clone(),
            swim_rounds: self.swim_rounds.clone(),
            gossip_rounds: self.gossip_rounds.clone(),
            block_list: self.block_list.clone(),
            swim_sender: None,
        }
    }
}

impl<N: Network> Server<N> {
    /// Create a new server, bound to the `addr`, hosting a particular `member`, and with a
    /// `Trace` struct, a ring_key if you want encryption on the wire, and an optional server name.
    pub fn new<P>(
        network: N,
        host_address: AddressForNetwork<N>,
        mut member: Member,
        trace: Trace,
        ring_key: Option<SymKey>,
        name: Option<String>,
        data_path: Option<P>,
        suitability_lookup: Box<Suitability>,
    ) -> Self
    where
        P: Into<PathBuf> + AsRef<ffi::OsStr>,
    {
        member.address = host_address.to_string();
        member.swim_port = network.get_swim_addr().get_port();
        member.gossip_port = network.get_gossip_addr().get_port();
        Self {
            name: Arc::new(name.unwrap_or(String::from(member.id.clone()))),
            member_id: Arc::new(member.id.clone()),
            member: Arc::new(RwLock::new(member)),
            member_list: MemberList::new(),
            host_address: host_address,
            ring_key: Arc::new(ring_key),
            rumor_heat: RumorHeat::default(),
            service_store: RumorStore::default(),
            service_config_store: RumorStore::default(),
            service_file_store: RumorStore::default(),
            election_store: RumorStore::default(),
            update_store: RumorStore::default(),
            departure_store: RumorStore::default(),
            suitability_lookup: Arc::new(suitability_lookup),
            data_path: Arc::new(data_path.as_ref().map(|p| p.into())),
            dat_file: Arc::new(RwLock::new(None)),
            departed: Arc::new(AtomicBool::new(false)),
            network: Arc::new(RwLock::new(network)),
            pause: Arc::new(AtomicBool::new(false)),
            trace: Arc::new(RwLock::new(trace)),
            swim_rounds: Arc::new(AtomicIsize::new(0)),
            gossip_rounds: Arc::new(AtomicIsize::new(0)),
            block_list: Arc::new(RwLock::new(HashSet::new())),
            swim_sender: None,
        }
    }

    /// Every iteration of the outbound protocol (which means every member has been pinged if they
    /// are available) increments the round. If we exceed an isize in rounds, we reset to 0.
    ///
    /// This is useful in integration testing, to allow tests to time out after a worst-case
    /// boundary in rounds.
    pub fn swim_rounds(&self) -> isize {
        self.swim_rounds.load(Ordering::SeqCst)
    }

    /// Adds 1 to the current round, atomically.
    fn update_swim_round(&self) {
        let current_round = self.swim_rounds.load(Ordering::SeqCst);
        match current_round.checked_add(1) {
            Some(_number) => {
                self.swim_rounds.fetch_add(1, Ordering::SeqCst);
            }
            None => {
                debug!(
                    "Exceeded an isize integer in swim-rounds. Congratulations, this is a \
                     very long running Supervisor!"
                );
                self.swim_rounds.store(0, Ordering::SeqCst);
            }
        }
    }

    /// Every iteration of the gossip protocol (which means every member has been sent if they
    /// are available) increments the round. If we exceed an isize in rounds, we reset to 0.
    ///
    /// This is useful in integration testing, to allow tests to time out after a worst-case
    /// boundary in rounds.
    pub fn gossip_rounds(&self) -> isize {
        self.gossip_rounds.load(Ordering::SeqCst)
    }

    /// Adds 1 to the current round, atomically.
    fn update_gossip_round(&self) {
        let current_round = self.gossip_rounds.load(Ordering::SeqCst);
        match current_round.checked_add(1) {
            Some(_number) => {
                self.gossip_rounds.fetch_add(1, Ordering::SeqCst);
            }
            None => {
                debug!(
                    "Exceeded an isize integer in gossip-rounds. Congratulations, this is a \
                     very long running Supervisor!"
                );
                self.gossip_rounds.store(0, Ordering::SeqCst);
            }
        }
    }

    /// Start the server, along with a `Timing` for outbound connections. Spawns the `inbound`,
    /// `outbound`, and `expire` threads.
    ///
    /// # Errors
    ///
    /// * Returns `Error::CannotBind` if the socket cannot be bound
    /// * Returns `Error::SocketSetReadTimeout` if the socket read timeout cannot be set
    /// * Returns `Error::SocketSetWriteTimeout` if the socket write timeout cannot be set
    pub fn start(&mut self, timing: timing::Timing) -> Result<()> {
        debug!("entering habitat_butterfly::server::Server::start");
        let (tx_outbound, rx_inbound) = channel();
        if let Some(ref path) = *self.data_path {
            if let Some(err) = fs::create_dir_all(path).err() {
                return Err(Error::BadDataPath(path.to_path_buf(), err));
            }
            let mut file = DatFile::new(&self.member_id, path);
            if file.path().exists() {
                match file.read_into(self) {
                    Ok(_) => debug!(
                        "Successfully ingested rumors from {}",
                        file.path().display()
                    ),
                    Err(Error::DatFileIO(path, err)) => error!("{}", Error::DatFileIO(path, err)),
                    Err(err) => return Err(err),
                };
            }
            let mut dat_file = self.dat_file.write().expect("DatFile lock is poisoned");
            *dat_file = Some(file);
        }

        let server_a = self.clone();
        let swim_sender = self.read_network().create_swim_sender()?;
        let swim_sender_a = self.read_network().create_swim_sender()?;
        let swim_receiver = self.read_network().create_swim_receiver()?;
        self.swim_sender = Some(swim_sender);

        let _ = thread::Builder::new()
            .name(format!("inbound-{}", self.name()))
            .spawn(move || {
                inbound::Inbound::new(server_a, swim_receiver, swim_sender_a, tx_outbound).run();
                panic!("You should never, ever get here, judy");
            });

        let server_b = self.clone();
        let swim_sender_b = self.read_network().create_swim_sender()?;
        let timing_b = timing.clone();
        let _ = thread::Builder::new()
            .name(format!("outbound-{}", self.name()))
            .spawn(move || {
                outbound::Outbound::new(server_b, swim_sender_b, rx_inbound, timing_b).run();
                panic!("You should never, ever get here, bob");
            });

        let server_c = self.clone();
        let timing_c = timing.clone();
        let _ = thread::Builder::new()
            .name(format!("expire-{}", self.name()))
            .spawn(move || {
                expire::Expire::new(server_c, timing_c).run();
                panic!("You should never, ever get here, frank");
            });

        let server_d = self.clone();
        let _ = thread::Builder::new()
            .name(format!("pull-{}", self.name()))
            .spawn(move || {
                pull::Pull::new(server_d).run();
                panic!("You should never, ever get here, davey");
            });

        let server_e = self.clone();
        let _ = thread::Builder::new()
            .name(format!("push-{}", self.name()))
            .spawn(move || {
                push::Push::new(server_e, timing).run();
                panic!("You should never, ever get here, liu");
            });

        if self
            .dat_file
            .read()
            .expect("DatFile lock poisoned")
            .is_some()
        {
            let server_f = self.clone();
            let _ = thread::Builder::new()
                .name(format!("persist-{}", self.name()))
                .spawn(move || {
                    persist_loop(server_f);
                    panic!("Data persistence loop unexpectedly quit!");
                });
        }

        Ok(())
    }

    pub fn need_peer_seeding(&self) -> bool {
        let m = self
            .member_list
            .members
            .read()
            .expect("Members lock is poisoned");
        m.is_empty()
    }

    /// Persistently block a given address, causing no traffic to be seen.
    pub fn add_to_block_list(&self, member_id: String) {
        let mut block_list = self
            .block_list
            .write()
            .expect("Write lock for block_list is poisoned");
        block_list.insert(member_id);
    }

    /// Remove a given address from the block_list.
    pub fn remove_from_block_list(&self, member_id: &str) {
        let mut block_list = self
            .block_list
            .write()
            .expect("Write lock for block_list is poisoned");
        block_list.remove(member_id);
    }

    /// Check if a given member ID is on the block_list.
    fn is_member_blocked(&self, member_id: &str) -> bool {
        let block_list = self
            .block_list
            .write()
            .expect("Write lock for block_list is poisoned");
        block_list.contains(member_id)
    }

    /// Stop the outbound and inbound threads from processing work.
    pub fn pause(&mut self) {
        self.pause.compare_and_swap(false, true, Ordering::Relaxed);
    }

    /// Whether this server is currently paused.
    pub fn paused(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }

    /// Return the swim address we are bound to
    fn swim_addr(&self) -> N::AddressAndPort {
        self.read_network().get_swim_addr()
    }

    /// Return the port number of the swim socket we are bound to.
    pub fn swim_port(&self) -> u16 {
        self.read_network().get_swim_addr().get_port()
    }

    /// Return the gossip address we are bound to
    pub fn gossip_addr(&self) -> N::AddressAndPort {
        self.read_network().get_gossip_addr()
    }

    /// Return the port number of the gossip socket we are bound to.
    pub fn gossip_port(&self) -> u16 {
        self.read_network().get_gossip_addr().get_port()
    }

    pub fn read_network(&self) -> RwLockReadGuard<N> {
        self.network.read().expect("Network lock is poisoned")
    }

    /// Return the member ID of this server.
    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    /// Return the name of this server.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Insert a member to the `MemberList`, and update its `RumorKey` appropriately.
    pub fn insert_member(&self, member: Member, health: Health) {
        let rk: RumorKey = RumorKey::from(&member);
        // NOTE: This sucks so much right here. Check out how we allocate no matter what, because
        // of just how the logic goes. The value of the trace is really high, though, so we suck it
        // for now.
        let trace_member_id = member.id.clone();
        let trace_incarnation = member.incarnation;
        let trace_health = health.clone();
        if self.member_list.insert(member, health) {
            trace_it!(
                MEMBERSHIP: self,
                TraceKind::MemberUpdate,
                trace_member_id,
                trace_incarnation,
                trace_health
            );
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    /// Set our member to departed, then send up to 10 out of order ack messages to other
    /// members to seed our status.
    pub fn set_departed(&self) {
        if self.swim_sender.is_some() {
            let member = {
                let mut me = self.write_member();
                let mut incarnation = me.incarnation;
                incarnation += 1;
                me.incarnation = incarnation;
                me.departed = true;
                me.clone()
            };
            let trace_member_id = member.id.clone();
            let trace_incarnation = member.incarnation;
            self.member_list
                .insert_health_by_id(&member.id, Health::Departed);
            trace_it!(
                MEMBERSHIP: self,
                TraceKind::MemberUpdate,
                trace_member_id,
                trace_incarnation,
                Health::Departed
            );

            // We need to mark this as "hot" in order to propagate it.
            //
            // TODO (CM): This exact code is present numerous places;
            // factor it out to facilitate further code consolidation.
            self.rumor_heat.start_hot_rumor(RumorKey::new(
                RumorType::Member,
                member.id.clone(),
                "",
            ));

            let check_list = self.member_list.check_list(&member.id);

            // TODO (CM): Even though we marked the rumor as hot
            // above, when we gossip, we send out the 5 "coolest but
            // still warm" rumors. Sending to 10 members increases the
            // chances that we'll get to this hot one now, but I don't
            // think that we can strictly guarantee that this
            // departure health update actually gets out in all cases.
            for member in check_list.iter().take(SELF_DEPARTURE_RUMOR_FANOUT) {
                let addr = member.swim_socket_address();
                outbound::ack(
                    &self,
                    // Safe because we checked above
                    self.swim_sender.as_ref().unwrap(),
                    member,
                    addr,
                    None,
                );
            }
        } else {
            debug!("No socket present; server was never started, so nothing to depart");
        }
    }

    /// Given a membership record and some health, insert it into the Member List.
    fn insert_member_from_rumor(&self, member: Member, mut health: Health) {
        let mut incremented_incarnation = false;
        let rk: RumorKey = RumorKey::from(&member);
        if member.id == self.member_id() {
            if health != Health::Alive {
                let mut me = self.write_member();
                let mut incarnation = me.incarnation;
                incarnation += 1;
                me.incarnation = incarnation;
                health = Health::Alive;
                incremented_incarnation = true;
            }
        }
        // NOTE: This sucks so much right here. Check out how we allocate no matter what, because
        // of just how the logic goes. The value of the trace is really high, though, so we suck it
        // for now.
        let trace_member_id = member.id.clone();
        let trace_incarnation = member.incarnation;
        let trace_health = health.clone();

        if self.member_list.insert(member, health) || incremented_incarnation {
            trace_it!(
                MEMBERSHIP: self,
                TraceKind::MemberUpdate,
                trace_member_id,
                trace_incarnation,
                trace_health
            );
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    /// Insert a service rumor into the service store.
    pub fn insert_service(&self, service: Service) {
        let rk = RumorKey::from(&service);

        // * If we don't have a rumor
        // * And we do have Confirmed members for this service
        // * Select the first sorted Confirmed member, and change it to departed
        if !self.service_store.contains_rumor(&rk.key, &rk.id) {
            let mut service_entries: Vec<Service> = Vec::new();
            self.service_store.with_rumors(&rk.key, |service_rumor| {
                if self
                    .member_list
                    .check_health_of_by_id(&service_rumor.member_id, Health::Confirmed)
                {
                    service_entries.push(service_rumor.clone());
                }
            });
            service_entries.sort_by_key(|k| k.member_id.clone());
            for service_rumor in service_entries.iter().take(1) {
                if self
                    .member_list
                    .insert_health_by_id(&service_rumor.member_id, Health::Departed)
                {
                    // TODO (CM): Why are we inferring departure from
                    // a service rumor?
                    self.rumor_heat.start_hot_rumor(RumorKey::new(
                        RumorType::Member,
                        service_rumor.member_id.clone(),
                        "",
                    ));
                }
            }
        }
        if self.service_store.insert(service) {
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    /// Insert a service config rumor into the service store.
    pub fn insert_service_config(&self, service_config: ServiceConfig) {
        let rk = RumorKey::from(&service_config);
        if self.service_config_store.insert(service_config) {
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    /// Insert a service file rumor into the service file store.
    pub fn insert_service_file(&self, service_file: ServiceFile) {
        let rk = RumorKey::from(&service_file);
        if self.service_file_store.insert(service_file) {
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    /// Insert a departure rumor into the departure store.
    pub fn insert_departure(&self, departure: Departure) {
        let rk = RumorKey::from(&departure);
        if &*self.member_id == &departure.member_id {
            self.departed
                .compare_and_swap(false, true, Ordering::Relaxed);
        }
        if self
            .member_list
            .insert_health_by_id(&departure.member_id, Health::Departed)
        {
            self.rumor_heat.start_hot_rumor(RumorKey::new(
                RumorType::Member,
                departure.member_id.clone(),
                "",
            ));
        }
        if self.departure_store.insert(departure) {
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    /// Get all the Member ID's who are present in a given service group, and eligible to vote
    /// (alive)
    fn get_electorate(&self, key: &str) -> Vec<String> {
        let mut electorate = vec![];
        self.service_store.with_rumors(key, |s| {
            if self
                .member_list
                .check_health_of_by_id(&s.member_id, Health::Alive)
            {
                electorate.push(s.member_id.clone());
            }
        });
        electorate
    }

    /// Get all the Member ID's who are present in a given service group, and count towards quorum.
    pub fn get_total_population(&self, key: &str) -> usize {
        let mut total_pop = 0;
        self.service_store.with_rumors(key, |s| {
            if self
                .member_list
                .check_in_voting_population_by_id(&s.member_id)
            {
                total_pop += 1;
            }
        });
        total_pop
    }

    /// Check if a given service group has quorum to run an election.
    ///
    /// A given group has quorum if, from this servers perspective, it has an alive population that
    /// is over 50%, and at least 3 members.
    fn check_quorum(&self, key: &str) -> bool {
        let electorate = self.get_electorate(key);

        let total_population = self.get_total_population(key);
        let alive_population = electorate.len();

        if total_population < 3 {
            trace!(
                "Quorum size: {}/3 - election cannot complete",
                total_population
            );
            return false;
        }

        alive_population >= ((total_population / 2) + 1)
    }

    /// Start an election for the given service group, declaring this members suitability and the
    /// term for the election.
    pub fn start_election(&self, sg: ServiceGroup, term: u64) {
        let suitability = self.suitability_lookup.get(&sg);
        let mut e = Election::new(self.member_id(), sg, suitability);
        e.term = term;
        let ek = RumorKey::from(&e);
        if !self.check_quorum(e.key()) {
            e.no_quorum();
        }
        self.election_store.insert(e);
        self.rumor_heat.start_hot_rumor(ek);
    }

    pub fn start_update_election(&self, sg: ServiceGroup, suitability: u64, term: u64) {
        let mut e = ElectionUpdate::new(self.member_id(), sg, suitability);
        e.term = term;
        let ek = RumorKey::from(&e);
        if !self.check_quorum(e.key()) {
            e.no_quorum();
        }
        self.update_store.insert(e);
        self.rumor_heat.start_hot_rumor(ek);
    }

    /// Check to see if this server needs to restart a given election. This happens when:
    ///
    /// a) We are the leader, and we have lost quorum with the rest of the group.
    /// b) We are not the leader, and we have detected that the leader is confirmed dead.
    pub fn restart_elections(&self) {
        let mut elections_to_restart: Vec<(String, u64)> = vec![];
        let mut update_elections_to_restart: Vec<(String, u64)> = vec![];

        self.election_store.with_keys(|(service_group, rumors)| {
            if self
                .service_store
                .contains_rumor(&service_group, self.member_id())
            {
                // This is safe; there is only one id for an election, and it is "election"
                let election = rumors
                    .get("election")
                    .expect("Lost an election struct between looking it up and reading it.");
                // If we are finished, and the leader is dead, we should restart the election
                if election.is_finished() && election.member_id == self.member_id() {
                    // If we are the leader, and we have lost quorum, we should restart the election
                    if self.check_quorum(election.key()) == false {
                        warn!(
                            "Restarting election with a new term as the leader has lost \
                             quorum: {:?}",
                            election
                        );
                        elections_to_restart
                            .push((String::from(&service_group[..]), election.term));
                    }
                } else if election.is_finished() {
                    if self
                        .member_list
                        .check_health_of_by_id(&election.member_id, Health::Confirmed)
                    {
                        warn!(
                            "Restarting election with a new term as the leader is dead {}: {:?}",
                            self.member_id(),
                            election
                        );
                        elections_to_restart
                            .push((String::from(&service_group[..]), election.term));
                    }
                }
            }
        });

        self.update_store.with_keys(|(service_group, rumors)| {
            if self
                .service_store
                .contains_rumor(&service_group, self.member_id())
            {
                // This is safe; there is only one id for an election, and it is "election"
                let election = rumors
                    .get("election")
                    .expect("Lost an update election struct between looking it up and reading it.");
                // If we are finished, and the leader is dead, we should restart the election
                if election.is_finished() && election.member_id == self.member_id() {
                    // If we are the leader, and we have lost quorum, we should restart the election
                    if self.check_quorum(election.key()) == false {
                        warn!(
                            "Restarting election with a new term as the leader has lost \
                             quorum: {:?}",
                            election
                        );
                        update_elections_to_restart
                            .push((String::from(&service_group[..]), election.term));
                    }
                } else if election.is_finished() {
                    if self
                        .member_list
                        .check_health_of_by_id(&election.member_id, Health::Confirmed)
                    {
                        warn!(
                            "Restarting election with a new term as the leader is dead {}: {:?}",
                            self.member_id(),
                            election
                        );
                        update_elections_to_restart
                            .push((String::from(&service_group[..]), election.term));
                    }
                }
            }
        });

        for (service_group, old_term) in elections_to_restart {
            let sg = match ServiceGroup::from_str(&service_group) {
                Ok(sg) => sg,
                Err(e) => {
                    error!(
                        "Failed to process service group from string '{}': {}",
                        service_group, e
                    );
                    return;
                }
            };
            let term = old_term + 1;
            warn!("Starting a new election for {} {}", sg, term);
            self.election_store.remove(&service_group, "election");
            self.start_election(sg, term);
        }

        for (service_group, old_term) in update_elections_to_restart {
            let sg = match ServiceGroup::from_str(&service_group) {
                Ok(sg) => sg,
                Err(e) => {
                    error!(
                        "Failed to process service group from string '{}': {}",
                        service_group, e
                    );
                    return;
                }
            };
            let term = old_term + 1;
            warn!("Starting a new election for {} {}", sg, term);
            self.update_store.remove(&service_group, "election");
            self.start_update_election(sg, 0, term);
        }
    }

    /// Insert an election into the election store. Handles creating a new election rumor for this
    /// member on receipt of an election rumor for a service this server cares about. Also handles
    /// stopping the election if we are the winner and we have enough votes.
    pub fn insert_election(&self, mut election: Election) {
        let rk = RumorKey::from(&election);

        // If this is an election for a service group we care about
        if self
            .service_store
            .contains_rumor(&election.service_group, self.member_id())
        {
            // And the election store already has an election rumor for this election
            if self
                .election_store
                .contains_rumor(election.key(), election.id())
            {
                let mut new_term = false;
                self.election_store
                    .with_rumor(election.key(), election.id(), |ce| {
                        new_term = election.term > ce.unwrap().term
                    });
                if new_term {
                    self.election_store.remove(election.key(), election.id());
                    let sg = match ServiceGroup::from_str(&election.service_group) {
                        Ok(sg) => sg,
                        Err(e) => {
                            error!("Election malformed; cannot parse service group: {}", e);
                            return;
                        }
                    };
                    self.start_election(sg, election.term);
                }
                // If we are the member that this election is voting for, then check to see if the
                // election is over! If it is, mark this election as final before you process it.
                if self.member_id() == election.member_id {
                    if self.check_quorum(election.key()) {
                        let electorate = self.get_electorate(election.key());
                        let mut num_votes = 0;
                        for vote in election.votes.iter() {
                            if electorate.contains(vote) {
                                num_votes += 1;
                            }
                        }
                        if num_votes == electorate.len() {
                            debug!("Election is finished: {:#?}", election);
                            election.finish();
                        } else {
                            debug!(
                                "I have quorum, but election is not finished {}/{}",
                                num_votes,
                                electorate.len()
                            );
                        }
                    } else {
                        election.no_quorum();
                        warn!("Election lacks quorum: {:#?}", election);
                    }
                }
            } else {
                // Otherwise, we need to create a new election object for ourselves prior to
                // merging.
                let sg = match ServiceGroup::from_str(&election.service_group) {
                    Ok(sg) => sg,
                    Err(e) => {
                        error!("Election malformed; cannot parse service group: {}", e);
                        return;
                    }
                };
                self.start_election(sg, election.term);
            }
            if !election.is_finished() {
                let has_quorum = self.check_quorum(election.key());
                if has_quorum {
                    election.running();
                } else {
                    election.no_quorum();
                }
            }
        }
        if self.election_store.insert(election) {
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    pub fn insert_update_election(&self, mut election: ElectionUpdate) {
        let rk = RumorKey::from(&election);

        // If this is an election for a service group we care about
        if self
            .service_store
            .contains_rumor(&election.service_group, self.member_id())
        {
            // And the election store already has an election rumor for this election
            if self
                .update_store
                .contains_rumor(election.key(), election.id())
            {
                let mut new_term = false;
                self.update_store
                    .with_rumor(election.key(), election.id(), |ce| {
                        new_term = election.term > ce.unwrap().term
                    });
                if new_term {
                    self.update_store.remove(election.key(), election.id());
                    self.start_update_election(election.service_group.clone(), 0, election.term);
                }
                // If we are the member that this election is voting for, then check to see if the
                // election is over! If it is, mark this election as final before you process it.
                if self.member_id() == election.member_id {
                    if self.check_quorum(election.key()) {
                        let electorate = self.get_electorate(election.key());
                        let mut num_votes = 0;
                        for vote in election.votes.iter() {
                            if electorate.contains(vote) {
                                num_votes += 1;
                            }
                        }
                        if num_votes == electorate.len() {
                            debug!("Election is finished: {:#?}", election);
                            election.finish();
                        } else {
                            debug!(
                                "I have quorum, but election is not finished {}/{}",
                                num_votes,
                                electorate.len()
                            );
                        }
                    } else {
                        election.no_quorum();
                        warn!("Election lacks quorum: {:#?}", election);
                    }
                }
            } else {
                // Otherwise, we need to create a new election object for ourselves prior to
                // merging.
                let sg = match ServiceGroup::from_str(&election.service_group) {
                    Ok(sg) => sg,
                    Err(e) => {
                        error!("Election malformed; cannot parse service group: {}", e);
                        return;
                    }
                };
                self.start_update_election(sg, 0, election.term);
            }
            if !election.is_finished() {
                let has_quorum = self.check_quorum(election.key());
                if has_quorum {
                    election.running();
                } else {
                    election.no_quorum();
                }
            }
        }
        if self.update_store.insert(election) {
            self.rumor_heat.start_hot_rumor(rk);
        }
    }

    fn generate_wire(&self, payload: Vec<u8>) -> Result<Vec<u8>> {
        message::generate_wire(payload, (*self.ring_key).as_ref())
    }

    fn unwrap_wire(&self, payload: &[u8]) -> Result<Vec<u8>> {
        message::unwrap_wire(payload, (*self.ring_key).as_ref())
    }

    fn persist_data(&self) {
        if let Some(ref dat_file) = *self.dat_file.write().expect("DatFile lock poisoned") {
            if let Some(err) = dat_file.write(self).err() {
                error!("Error persisting rumors to disk, {}", err);
            } else {
                debug!(
                    "Successfully persisted rumors to disk: {}",
                    dat_file.path().display()
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_departed(&self) -> bool {
        self.departed.load(Ordering::Relaxed)
    }

    pub fn read_member(&self) -> RwLockReadGuard<Member> {
        self.member.read().expect("Member lock is poisoned")
    }

    pub fn write_member(&self) -> RwLockWriteGuard<Member> {
        self.member.write().expect("Member lock is poisoned")
    }
}

impl<N: Network> Serialize for Server<N> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("butterfly", 7)?;
        strukt.serialize_field("member", &self.member_list)?;
        strukt.serialize_field("service", &self.service_store)?;
        strukt.serialize_field("service_config", &self.service_config_store)?;
        strukt.serialize_field("service_file", &self.service_file_store)?;
        strukt.serialize_field("election", &self.election_store)?;
        strukt.serialize_field("election_update", &self.update_store)?;
        strukt.serialize_field("departure", &self.departure_store)?;
        strukt.end()
    }
}

impl<N: Network> fmt::Display for Server<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}@{}/{}",
            self.name(),
            self.swim_port(),
            self.gossip_port()
        )
    }
}

impl<N: Network> Drop for Server<N> {
    fn drop(&mut self) {
        self.persist_data();
    }
}

fn persist_loop<N: Network>(server: Server<N>) {
    loop {
        let next_check = Instant::now() + Duration::from_millis(30_000);
        server.persist_data();
        let time_to_wait = next_check - Instant::now();
        thread::sleep(time_to_wait);
    }
}

#[cfg(test)]
mod tests {
    mod server {
        use habitat_core::service::ServiceGroup;
        use member::Member;
        use network::{Network, RealNetwork};
        use server::timing::Timing;
        use server::{Server, Suitability};
        use std::fs::File;
        use std::io::prelude::*;
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use std::path::PathBuf;
        use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
        use tempdir::TempDir;
        use trace::Trace;

        static SWIM_PORT: AtomicUsize = ATOMIC_USIZE_INIT;
        static GOSSIP_PORT: AtomicUsize = ATOMIC_USIZE_INIT;

        #[derive(Debug)]
        struct ZeroSuitability;
        impl Suitability for ZeroSuitability {
            fn get(&self, _service_group: &ServiceGroup) -> u64 {
                0
            }
        }

        fn start_server() -> Server<RealNetwork> {
            SWIM_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
            GOSSIP_PORT.compare_and_swap(0, 7777, Ordering::Relaxed);
            let swim_port = SWIM_PORT.fetch_add(1, Ordering::Relaxed);
            let swim_listen = localhost_addr(swim_port as u16);
            let gossip_port = GOSSIP_PORT.fetch_add(1, Ordering::Relaxed);
            let gossip_listen = localhost_addr(gossip_port as u16);
            let mut member = Member::default();
            member.swim_port = swim_port as u16;
            member.gossip_port = gossip_port as u16;
            let network = RealNetwork::new_for_server(swim_listen, gossip_listen);
            let host_address = network
                .get_host_address()
                .expect("Cannot get real host address");
            Server::new(
                network,
                host_address,
                member,
                Trace::default(),
                None,
                None,
                None::<PathBuf>,
                Box::new(ZeroSuitability),
            )
        }

        fn start_with_corrupt_rumor_file(tmpdir: &TempDir) -> Server<RealNetwork> {
            SWIM_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
            GOSSIP_PORT.compare_and_swap(0, 7777, Ordering::Relaxed);
            let swim_port = SWIM_PORT.fetch_add(1, Ordering::Relaxed);
            let swim_listen = localhost_addr(swim_port as u16);
            let gossip_port = GOSSIP_PORT.fetch_add(1, Ordering::Relaxed);
            let gossip_listen = localhost_addr(gossip_port as u16);
            let mut member = Member::default();
            member.swim_port = swim_port as u16;
            member.gossip_port = gossip_port as u16;
            let rumor_name = format!("{}{}", member.id.to_string(), ".rst");
            let file_path = tmpdir.path().to_owned().join(rumor_name);
            let mut rumor_file = File::create(file_path).unwrap();
            writeln!(rumor_file, "This is not a valid rumor file!").unwrap();
            let network = RealNetwork::new_for_server(swim_listen, gossip_listen);
            let host_address = network
                .get_host_address()
                .expect("Cannot get real host address");
            Server::new(
                network,
                host_address,
                member,
                Trace::default(),
                None,
                None,
                Some(tmpdir.path()),
                Box::new(ZeroSuitability),
            )
        }

        fn localhost_addr(port: u16) -> SocketAddr {
            let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
            SocketAddr::new(ip, port)
        }

        #[test]
        fn new() {
            start_server();
        }

        #[test]
        fn new_with_corrupt_rumor_file() {
            let tmpdir = TempDir::new("data").unwrap();
            let mut server = start_with_corrupt_rumor_file(&tmpdir);
            server
                .start(Timing::default())
                .expect("Server failed to start");
        }

        #[test]
        fn start_listener() {
            let mut server = start_server();
            server
                .start(Timing::default())
                .expect("Server failed to start");
        }
    }
}
