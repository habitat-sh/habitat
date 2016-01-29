// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

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
use std::mem;
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};

use toml;
use uuid::Uuid;
use wonder::actor::{self, GenServer, HandleResult, InitResult, StopReason, ActorSender};

use config::Config;
use error::{BldrResult, BldrError, ErrorKind};
use discovery::etcd::{self, EtcdWrite};
use util;
use package::Package;

static LOGKEY: &'static str = "CN";

/// A CensusEntry. Manages all the data about a given member of the census.
#[derive(Debug, Clone, RustcDecodable, RustcEncodable, Eq)]
pub struct CensusEntry {
    pub id: Uuid,
    pub hostname: String,
    pub ip: String,
    suitability: u64,
    port: Option<String>,
    exposes: Option<Vec<String>>,
    pub leader: Option<bool>,
    pub follower: Option<bool>,
    pub data_init: Option<bool>,
    pub vote: Option<String>,
    pub election: Option<bool>,
    pub needs_write: Option<bool>,
    pub initialized: bool,
    keep_me: bool,
}

impl PartialEq for CensusEntry {
    // We are equal, but we don't care about some fields.
    fn eq(&self, other: &CensusEntry) -> bool {
        if self.id != other.id {
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
        } else {
            true
        }
    }
}

impl CensusEntry {
    /// Create a new CensusEntry for this supervisor.
    pub fn new() -> CensusEntry {
        CensusEntry {
            id: Uuid::new_v4(),
            hostname: util::sys::hostname().unwrap_or(String::from("unknown")),
            ip: util::sys::ip().unwrap_or(String::from("127.0.0.1")),
            suitability: 0,
            port: None,
            exposes: None,
            leader: None,
            follower: None,
            data_init: None,
            vote: None,
            election: None,
            needs_write: None,
            initialized: false,
            keep_me: true,
        }
    }

    /// Set our suitability number. This is an arbitrary determination of our 'suitability' to a
    /// task; most likely, being the leader in an election.
    pub fn suitability(&mut self, suitability: u64) {
        self.suitability = suitability;
        self.needs_write = Some(true);
    }

    /// Set a port number; often used as the default for watches
    pub fn port(&mut self, port: Option<String>) {
        self.port = port;
        self.needs_write = Some(true);
    }

    /// Set an array of port numbers we expose.
    pub fn exposes(&mut self, exposes: Option<Vec<String>>) {
        self.exposes = exposes;
        self.needs_write = Some(true);
    }

    /// Set our status at the leader.
    pub fn leader(&mut self, leader: Option<bool>) {
        self.leader = leader;
        self.needs_write = Some(true);
    }

    /// Set our status as a follower.
    pub fn follower(&mut self, follower: Option<bool>) {
        self.follower = follower;
        self.needs_write = Some(true);
    }

    /// Set our application initialization status to true.
    pub fn initialized(&mut self) {
        self.initialized = true;
        self.needs_write = Some(true);
    }

    /// Set our status on having initialzied data.
    pub fn data_init(&mut self, data_init: Option<bool>) {
        self.data_init = data_init;
        self.needs_write = Some(true);
    }

    /// Set our vote.
    pub fn vote(&mut self, vote: Option<String>) {
        self.vote = vote;
        self.needs_write = Some(true);
    }

    /// Are we in an election?
    pub fn election(&mut self, election: Option<bool>) {
        self.election = election;
        self.needs_write = Some(true);
    }

    /// Return the string we use for this CensusEntry when it is a candidate in an election.
    pub fn candidate_string(&self) -> String {
        format!("{}", self.id)
    }

    /// Turn this CensusEntry into an etcd write.
    pub fn as_etcd_write(&mut self, pkg: &Package, config: &Config) -> EtcdWrite {
        let mut toml = format!("[census.{}]\n", self.id.to_simple_string());
        let toml_ce = toml::encode_str(self);
        toml.push_str(&toml_ce);
        self.needs_write = None;
        EtcdWrite {
            key: format!("{}/{}/census/{}", pkg.name, config.group(), self.id),
            value: Some(toml),
            ttl: Some(30),
            dir: None,
            prevExist: None,
            prevIndex: None,
            prevValue: None,
        }
    }
}

/// Messages for our CensusEntryActor
#[derive(Debug)]
pub enum Message {
    /// A write
    Write(EtcdWrite),
    /// Ok
    Ok,
    /// Stop
    Stop,
}

// The time between persisting our CensusEntry to the backend.
const ENTRY_TIMEOUT_MS: u64 = 20000;

/// The actor for our CensusEntry
#[derive(Debug)]
pub struct CensusEntryActor;

impl CensusEntryActor {
    /// Write this entry to the discovery backend
    pub fn write_to_discovery(&self, write: &EtcdWrite) -> BldrResult<()> {
        try!(etcd::write(write));
        Ok(())
    }

    /// Request the actor write this census entry
    pub fn write(actor: &actor::Actor<Message>, write: EtcdWrite) -> BldrResult<()> {
        match try!(actor.call(Message::Write(write))) {
            Message::Ok => Ok(()),
            _ => unreachable!(),
        }
    }
}

/// A simple map of Census Entries; used for decoding toml data
#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct CensusMap {
    pub census: BTreeMap<String, CensusEntry>,
}

impl GenServer for CensusEntryActor {
    type T = Message;
    type S = EtcdWrite;
    type E = BldrError;

    fn init(&self, _tx: &ActorSender<Self::T>, _toml_string: &mut Self::S) -> InitResult<Self::E> {
        Ok(Some(0))
    }

    /// Makes sure we regularly persist to etcd, beating our entries 30 second ttl.
    fn handle_timeout(&self,
                      _tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        match self.write_to_discovery(state) {
            Ok(_) => HandleResult::NoReply(Some(ENTRY_TIMEOUT_MS)),
            Err(e) => {
                return HandleResult::Stop(StopReason::Fatal(format!("Census Entry Actor caught \
                                                                     unexpected error: {:?}",
                                                                    e)),
                                          None);
            }
        }
    }

    /// Take a given etcd write, and persist it. Or stop. It's cool. Whatevs.
    fn handle_call(&self,
                   message: Self::T,
                   _caller: &ActorSender<Self::T>,
                   _me: &ActorSender<Self::T>,
                   state: &mut Self::S)
                   -> HandleResult<Self::T> {
        match message {
            Message::Stop => HandleResult::Stop(StopReason::Normal, Some(Message::Ok)),
            Message::Write(etcdwrite) => {
                mem::replace(state, etcdwrite);
                match self.write_to_discovery(state) {
                    Ok(_) => {}
                    Err(e) => debug!("Failed to write to discovery: {:?}", e),
                }
                HandleResult::Reply(Message::Ok, Some(ENTRY_TIMEOUT_MS))
            }
            Message::Ok => {
                HandleResult::Stop(StopReason::Fatal(format!("You don't send me Ok! I send YOU \
                                                              Ok!")),
                                   Some(Message::Ok))
            }
        }
    }
}

/// The census!
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
    /// Whether we need to be written out
    pub needs_write: bool,
}

impl Census {
    /// Creates a new Census. Takes a CensusEntry for the current supervisor.
    pub fn new(ce: CensusEntry) -> Census {
        let my_id = ce.id.clone();
        let mut hm = HashMap::new();
        hm.insert(my_id, ce);
        Census {
            me: my_id,
            population: hm,
            in_event: false,
            needs_write: true,
        }
    }

    /// A reference to the current supervisors entry in the census.
    ///
    /// # Failures
    ///
    /// * If the entry doesn't exist
    pub fn me(&self) -> BldrResult<&CensusEntry> {
        self.population
            .get(&self.me)
            .ok_or(bldr_error!(ErrorKind::CensusNotFound(self.me.to_simple_string())))
    }

    /// A mutable reference to the current supervisors entry in the census.
    ///
    /// # Failures
    ///
    /// * If the entry doesn't exist
    pub fn me_mut(&mut self) -> BldrResult<&mut CensusEntry> {
        self.population
            .get_mut(&self.me)
            .ok_or(bldr_error!(ErrorKind::CensusNotFound(self.me.to_simple_string())))
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
    pub fn update(&mut self, census_string: &str) -> BldrResult<()> {
        let mut toml_parser = toml::Parser::new(census_string);
        let toml = try!(toml_parser.parse()
                                   .ok_or(bldr_error!(ErrorKind::TomlParser(toml_parser.errors))));
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
    pub fn to_toml(&mut self) -> BldrResult<String> {
        let mut final_toml = String::new();
        let mut sorted_keys: Vec<_> = self.population
                                          .keys()
                                          .map(|&x| x.to_simple_string())
                                          .collect();
        sorted_keys.sort();
        for key in sorted_keys {
            let uuid_key = try!(Uuid::parse_str(&key));
            let value = self.population.get(&uuid_key).unwrap();
            final_toml.push_str(&format!("\n[[census.members]]\n{}", toml::encode_str(value)));
        }
        let me = try!(self.me());
        final_toml.push_str(&format!("\n[census.me]\n{}", toml::encode_str(&me)));
        match self.get_leader() {
            Some(leader) => {
                final_toml.push_str(&format!("\n[census.leader]\n{}", toml::encode_str(&leader)));
            }
            None => {}
        }
        Ok(final_toml)
    }

    /// Have all members of the census initialized their data?
    pub fn dataset_initialized(&self) -> bool {
        let count = self.population
                        .values()
                        .filter(|&ce| {
                            if let Some(true) = ce.data_init {
                                true
                            } else {
                                false
                            }
                        })
                        .count();
        if count > 0 {
            true
        } else {
            false
        }
    }

    /// Is there a leader in the census? Returns that entry.
    pub fn get_leader(&self) -> Option<&CensusEntry> {
        let mut leader: Vec<&CensusEntry> = self.population
                                                .values()
                                                .filter(|&ce| {
                                                    if let Some(true) = ce.leader {
                                                        true
                                                    } else {
                                                        false
                                                    }
                                                })
                                                .collect();
        if leader.len() == 1 {
            leader.pop()
        } else {
            None
        }
    }

    /// Is there a leader in the census?
    pub fn has_leader(&self) -> bool {
        let count = self.population
                        .values()
                        .filter(|&ce| {
                            if let Some(true) = ce.leader {
                                true
                            } else {
                                false
                            }
                        })
                        .count();
        if count > 0 {
            true
        } else {
            false
        }
    }

    /// Is there one leader, and everyone else is a follower?
    pub fn has_all_followers(&self) -> bool {
        let size = self.population.len() - 1;

        let count = self.population
                        .values()
                        .filter(|&ce| {
                            if let Some(true) = ce.follower {
                                true
                            } else {
                                false
                            }
                        })
                        .count();
        if count == size {
            true
        } else {
            false
        }
    }

    /// Decide who we should vote for, and return their CensusEntry.
    ///
    /// * Choose the node with the highest `suitability` number
    /// * If all those are equal, choose the node whose `id` field sorts first lexicographically
    pub fn determine_vote(&self) -> &CensusEntry {
        let acc: Option<&CensusEntry> = None;
        let vote: &CensusEntry = self.population
                                     .values()
                                     .fold(acc, |acc, ref rce| {
                                         match acc {
                                             Some(lce) => {
                                                 if rce.suitability > lce.suitability {
                                                     Some(rce)
                                                 } else if lce.suitability == rce.suitability {
                                                     if rce.id.to_simple_string() >
                                                        lce.id.to_simple_string() {
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

        let ce = self.me().unwrap();
        let my_vote = ce.clone().vote.unwrap();

        for (_lid, lce) in self.population.iter() {
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
}

/// The messages for the CensusActor
#[derive(Debug)]
pub enum CensusMessage {
    /// Return the last toml data
    CensusToml(Option<String>),
    /// Request the latest toml data
    Census,
    /// Ok
    Ok,
    /// Knock it off!
    Stop,
}

/// How often to check for changes
const CENSUS_TIMEOUT_MS: u64 = 200;

/// Our CensusActor
#[derive(Debug)]
pub struct CensusActor;

impl CensusActor {
    /// Get the latest Census toml.
    ///
    /// # Failures
    ///
    /// * If the call to the actor fails.
    pub fn census_string(actor: &actor::Actor<CensusMessage>) -> BldrResult<Option<String>> {
        match try!(actor.call(CensusMessage::Census)) {
            CensusMessage::CensusToml(census_string) => Ok(census_string),
            _ => unreachable!(),
        }
    }
}

/// The state for our Census Actor. Holds a watch key, the last census_string, and the channels for
/// communicating with etcd.
pub struct CensusActorState {
    ctx: Option<Sender<bool>>,
    crx: Option<Receiver<Option<String>>>,
    census_string: Option<String>,
    watch_key: String,
}

impl CensusActorState {
    /// Return a new CensusActorState, configured for the given watch.
    pub fn new(watch_key: String) -> CensusActorState {
        CensusActorState {
            ctx: None,
            crx: None,
            census_string: None,
            watch_key: watch_key,
        }
    }
}

impl GenServer for CensusActor {
    type T = CensusMessage;
    type S = CensusActorState;
    type E = BldrError;

    /// Set up the watch.
    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        let (ctx, wrx) = channel();
        let (wtx, crx) = channel();
        etcd::watch(&state.watch_key, 1, true, true, wtx, wrx);
        state.ctx = Some(ctx);
        state.crx = Some(crx);
        Ok(Some(0))
    }

    /// Check for data from the watch, and update the census_string.
    fn handle_timeout(&self,
                      _tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        if let Some(ref crx) = state.crx {
            match crx.try_recv() {
                Ok(Some(toml_string)) => {
                    state.census_string = Some(toml_string);
                }
                Ok(None) => {
                    state.census_string = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(e) => {
                    return HandleResult::Stop(StopReason::Fatal(format!("Census Actor caught \
                                                                         unexpected error: {:?}",
                                                                        e)),
                                              None);
                }
            }
        }
        HandleResult::NoReply(Some(CENSUS_TIMEOUT_MS))
    }

    /// Return the census_string.
    fn handle_call(&self,
                   message: Self::T,
                   _caller: &ActorSender<Self::T>,
                   _me: &ActorSender<Self::T>,
                   state: &mut Self::S)
                   -> HandleResult<Self::T> {
        if let Some(ref crx) = state.crx {
            match crx.try_recv() {
                Ok(Some(toml_string)) => {
                    state.census_string = Some(toml_string);
                }
                Ok(None) => {
                    state.census_string = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(e) => {
                    return HandleResult::Stop(StopReason::Fatal(format!("Census Actor caught \
                                                                         unexpected error: {:?}",
                                                                        e)),
                                              Some(CensusMessage::Ok));
                }
            }
        }

        match message {
            CensusMessage::Stop => HandleResult::Stop(StopReason::Normal, Some(CensusMessage::Ok)),
            CensusMessage::Census => {
                match state.census_string {
                    Some(ref toml_string) => {
                        HandleResult::Reply(CensusMessage::CensusToml(Some(toml_string.clone())),
                                            Some(CENSUS_TIMEOUT_MS))
                    }
                    None => {
                        HandleResult::Reply(CensusMessage::CensusToml(None),
                                            Some(CENSUS_TIMEOUT_MS))
                    }
                }
            }
            CensusMessage::Ok => {
                HandleResult::Stop(StopReason::Fatal(format!("You don't send me Ok! I send YOU \
                                                              Ok!")),
                                   Some(CensusMessage::Ok))
            }
            CensusMessage::CensusToml(_) => {
                HandleResult::Stop(StopReason::Fatal(format!("You don't send me CensusToml(_)! \
                                                              I send YOU CensusToml(_)!")),
                                   Some(CensusMessage::Ok))
            }
        }
    }
}
