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

#[macro_use]
extern crate habitat_butterfly;

#[macro_use]
extern crate lazy_static;

use habitat_butterfly::{error::Error,
                        member::{Health,
                                 Member},
                        rumor::{departure::Departure,
                                election::ElectionStatus,
                                service::{Service,
                                          SysInfo},
                                service_config::ServiceConfig,
                                service_file::ServiceFile},
                        server::{timing::Timing,
                                 Server,
                                 Suitability},
                        trace::Trace};
use habitat_core::{crypto::keys::sym_key::SymKey,
                   package::{Identifiable,
                             PackageIdent},
                   service::ServiceGroup};
use std::{net::{IpAddr,
                Ipv4Addr,
                SocketAddr},
          ops::{Deref,
                DerefMut,
                Range},
          str::FromStr,
          sync::Mutex,
          thread,
          time::Duration};
use time::SteadyTime;

lazy_static! {
    static ref SERVER_PORT: Mutex<u16> = Mutex::new(6666);
}

/// To avoid deadlocking in a test, we use `health_of_by_id_with_timeout` rather than
/// `health_of_by_id`.
const HEALTH_OF_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug)]
struct NSuitability(u64);
impl Suitability for NSuitability {
    fn get(&self, _service_group: &str) -> u64 { self.0 }
}

#[cfg(feature = "deadlock_detection")]
fn assert_no_deadlocks() {
    use parking_lot::deadlock;

    let deadlocks = deadlock::check_deadlock();
    if deadlocks.is_empty() {
        log::trace!("No deadlocks detected");
    } else {
        println!("{} deadlocks detected", deadlocks.len());
        for (i, threads) in deadlocks.iter().enumerate() {
            println!("Deadlock #{}", i);
            for t in threads {
                println!("Thread {:#?}", t.thread_id());
                println!("{:#?}", t.backtrace());
            }
        }
    }

    assert!(deadlocks.is_empty());
}

pub fn start_server(name: &str, ring_key: Option<SymKey>, suitability: u64) -> Server {
    let swim_port;
    let gossip_port;
    {
        let mut port_guard = SERVER_PORT.lock().expect("SERVER_PORT mutex poisoned");
        swim_port = *port_guard;
        *port_guard += 1;
        gossip_port = *port_guard;
        *port_guard += 1;
    }
    let listen_swim = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), swim_port);
    let listen_gossip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), gossip_port);
    let mut member = Member::default();
    member.swim_port = swim_port;
    member.gossip_port = gossip_port;
    let mut server = Server::new(listen_swim,
                                 listen_gossip,
                                 member,
                                 Trace::default(),
                                 ring_key,
                                 Some(String::from(name)),
                                 None,
                                 Box::new(NSuitability(suitability))).unwrap();
    server.start(Timing::default())
          .expect("Cannot start server");
    server
}

pub fn member_from_server(server: &Server) -> Member {
    let mut member = server.member
                           .read()
                           .expect("Member lock is poisoned")
                           .as_member();
    // AAAAAAARGH... we currently have to do this because otherwise we
    // have no notion of where we're coming from... in "real life",
    // other Supervisors would discover this from the networking stack
    // as the UDP packets come in.
    //
    // TODO (CM): Investigate this further; does this have adverse
    // effects on our tests? Are we missing something we'd otherwise catch?
    member.address = String::from("127.0.0.1");
    member
}

#[derive(Debug)]
pub struct SwimNet {
    pub members: Vec<Server>,
}

impl Deref for SwimNet {
    type Target = Vec<Server>;

    fn deref(&self) -> &Vec<Server> { &self.members }
}

impl DerefMut for SwimNet {
    fn deref_mut(&mut self) -> &mut Vec<Server> { &mut self.members }
}

impl SwimNet {
    pub fn new_with_suitability(suitabilities: Vec<u64>) -> SwimNet {
        SwimNet { members: suitabilities.into_iter()
                                        .enumerate()
                                        .map(|(x, suitability)| {
                                            start_server(&format!("{}", x), None, suitability)
                                        })
                                        .collect(), }
    }

    pub fn new(count: usize) -> SwimNet {
        let suitabilities = vec![0; count];
        SwimNet::new_with_suitability(suitabilities)
    }

    pub fn new_ring_encryption(count: usize, ring_key: &SymKey) -> SwimNet {
        let mut members = Vec::with_capacity(count);
        for x in 0..count {
            let rk = ring_key.clone();
            members.push(start_server(&format!("{}", x), Some(rk), 0));
        }
        SwimNet { members }
    }

    pub fn connect(&mut self, from_entry: usize, to_entry: usize) {
        let to = member_from_server(&self.members[to_entry]);
        trace_it!(TEST: &self.members[from_entry], format!("Connected {} {}", self.members[to_entry].name(), self.members[to_entry].member_id()));
        self.members[from_entry].insert_member(to, Health::Alive);
    }

    pub fn add_member(&mut self) {
        let number = self.members.len() + 1;
        self.members
            .push(start_server(&format!("{}", number), None, 0));
    }

    // Fully mesh the network
    pub fn mesh(&mut self) {
        trace_it!(TEST_NET: self, "Mesh");
        for pos in 0..self.members.len() {
            let mut to_mesh: Vec<Member> = Vec::new();
            for x_pos in 0..self.members.len() {
                if pos == x_pos {
                    continue;
                }
                to_mesh.push(member_from_server(&self.members[x_pos]))
            }
            for server_b in to_mesh.into_iter() {
                self.members[pos].insert_member(server_b, Health::Alive);
            }
        }
    }

    pub fn block(&self, from_entry: usize, to_entry: usize) {
        let from = self.members
                       .get(from_entry)
                       .expect("Asked for a network member who is out of bounds");
        let to = self.members
                     .get(to_entry)
                     .expect("Asked for a network member who is out of bounds");
        trace_it!(TEST: &self.members[from_entry], format!("Blocked {} {}", self.members[to_entry].name(), self.members[to_entry].member_id()));
        from.add_to_block_list(String::from(to.member_id()));
    }

    pub fn unblock(&self, from_entry: usize, to_entry: usize) {
        let from = self.members
                       .get(from_entry)
                       .expect("Asked for a network member who is out of bounds");
        let to = self.members
                     .get(to_entry)
                     .expect("Asked for a network member who is out of bounds");
        trace_it!(TEST: &self.members[from_entry], format!("Unblocked {} {}", self.members[to_entry].name(), self.members[to_entry].member_id()));
        from.remove_from_block_list(to.member_id());
    }

    pub fn health_of(&self, from_entry: usize, to_entry: usize) -> Option<Health> {
        assert!(cfg!(feature = "deadlock_detection"),
                "This test should be run with --features=deadlock_detection");

        let from = self.members
                       .get(from_entry)
                       .expect("Asked for a network member who is out of bounds");

        let to = self.members
                     .get(to_entry)
                     .expect("Asked for a network member who is out of bounds");

        match from.member_list
                  .health_of_by_id_with_timeout(to.member_id(), HEALTH_OF_TIMEOUT)
        {
            Ok(health) => Some(health),
            Err(Error::UnknownMember(_)) => None,
            Err(_) => {
                #[cfg(feature = "deadlock_detection")]
                assert_no_deadlocks();
                panic!("Timed out after waiting {:?} querying member health",
                       HEALTH_OF_TIMEOUT);
            }
        }
    }

    pub fn network_health_of(&self, to_check: usize) -> Vec<Option<Health>> {
        let mut health_summary = Vec::with_capacity(self.members.len() - 1);
        let length = self.members.len();
        for x in 0..length {
            if x == to_check {
                continue;
            }
            health_summary.push(self.health_of(x, to_check));
        }
        health_summary
    }

    pub fn max_rounds(&self) -> isize { 4 }

    pub fn max_gossip_rounds(&self) -> isize { 5 }

    pub fn rounds(&self) -> Vec<isize> { self.members.iter().map(Server::swim_rounds).collect() }

    pub fn rounds_in(&self, count: isize) -> Vec<isize> {
        self.rounds().iter().map(|r| r + count).collect()
    }

    pub fn gossip_rounds(&self) -> Vec<isize> {
        self.members.iter().map(Server::gossip_rounds).collect()
    }

    pub fn gossip_rounds_in(&self, count: isize) -> Vec<isize> {
        self.gossip_rounds().iter().map(|r| r + count).collect()
    }

    fn check_rounds_impl(&self,
                         rounds_in: &[isize],
                         get_rounds: impl Fn(&Server) -> isize)
                         -> bool {
        for (member, round) in self.members.iter().zip(rounds_in) {
            if !member.paused() && get_rounds(member) <= *round {
                return false;
            }
        }

        true
    }

    pub fn check_rounds(&self, rounds_in: &[isize]) -> bool {
        self.check_rounds_impl(rounds_in, Server::swim_rounds)
    }

    pub fn wait_for_rounds(&self, rounds: isize) {
        let rounds_in = self.rounds_in(rounds);
        loop {
            if self.check_rounds(&rounds_in) {
                return;
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

    pub fn check_gossip_rounds(&self, rounds_in: &[isize]) -> bool {
        self.check_rounds_impl(rounds_in, Server::gossip_rounds)
    }

    #[allow(dead_code)]
    pub fn wait_for_gossip_rounds(&self, rounds: isize) {
        let rounds_in = self.gossip_rounds_in(rounds);
        loop {
            if self.check_gossip_rounds(&rounds_in) {
                return;
            }
            thread::sleep(Duration::from_millis(1500));
        }
    }

    pub fn wait_for_election_status(&self,
                                    e_num: usize,
                                    key: &str,
                                    status: ElectionStatus)
                                    -> bool {
        let rounds_in = self.gossip_rounds_in(self.max_gossip_rounds());
        loop {
            let mut result = false;
            let server = self.members
                             .get(e_num)
                             .expect("Asked for a network member who is out of bounds");
            server.election_store.with_rumor(key, "election", |e| {
                                     if e.status == status {
                                         result = true;
                                     }
                                 });
            if result {
                return true;
            }
            if self.check_gossip_rounds(&rounds_in) {
                println!("Failed election check for status {:?}: {:#?}",
                         status, self.members[e_num].election_store);
                return false;
            }
        }
    }

    pub fn wait_for_equal_election(&self, left: usize, right: usize, key: &str) -> bool {
        let rounds_in = self.gossip_rounds_in(self.max_gossip_rounds());
        loop {
            let mut result = false;

            let left_server = self.members
                                  .get(left)
                                  .expect("Asked for a network member who is out of bounds");
            let right_server = self.members
                                   .get(right)
                                   .expect("Asked for a network member who is out of bounds");

            left_server.election_store.with_rumor(key, "election", |l| {
                                          right_server.election_store.with_rumor(key,
                                                                                 "election",
                                                                                 |r| {
                                                                                     result =
                                                                                         l == r;
                                                                                 });
                                      });
            if result {
                return true;
            }
            if self.check_gossip_rounds(&rounds_in) {
                println!("Failed election check for equality:\nL: {:#?}\n\nR: {:#?}",
                         self.members[left].election_store, self.members[right].election_store,);
                return false;
            }
        }
    }

    pub fn partition(&self, left_range: Range<usize>, right_range: Range<usize>) {
        let left: Vec<usize> = left_range.collect();
        let right: Vec<usize> = right_range.collect();
        for l in left.iter() {
            for r in right.iter() {
                println!("Partitioning {} from {}", *l, *r);
                if l == r {
                    continue;
                }
                self.block(*l, *r);
                self.block(*r, *l);
            }
        }
    }

    pub fn unpartition(&self, left_range: Range<usize>, right_range: Range<usize>) {
        let left: Vec<usize> = left_range.collect();
        let right: Vec<usize> = right_range.collect();
        for l in left.iter() {
            for r in right.iter() {
                println!("UnPartitioning {} from {}", *l, *r);
                self.unblock(*l, *r);
                self.unblock(*r, *l);
            }
        }
    }

    pub fn wait_for_health_of(&self, from_entry: usize, to_check: usize, health: Health) -> bool {
        let rounds_in = self.rounds_in(self.max_rounds());
        #[cfg(feature = "deadlock_detection")]
        assert_no_deadlocks();

        loop {
            if let Some(real_health) = self.health_of(from_entry, to_check) {
                if real_health == health {
                    trace_it!(TEST: &self.members[from_entry], format!("Health {} {} as {}", self.members[to_check].name(), self.members[to_check].member_id(), health));
                    return true;
                }
            }
            if self.check_rounds(&rounds_in) {
                trace_it!(TEST: &self.members[from_entry], format!("Health failed {} {} as {}", self.members[to_check].name(), self.members[to_check].member_id(), health));
                println!("MEMBERS: {:#?}", self.members);
                println!("Failed health check for\n***FROM***{:#?}\n***TO***\n{:#?}",
                         self.members[from_entry], self.members[to_check]);
                return false;
            }
        }
    }

    pub fn wait_for_network_health_of(&self, to_check: usize, health: Health) -> bool {
        let rounds_in = self.rounds_in(self.max_rounds());
        loop {
            let network_health = self.network_health_of(to_check);
            if network_health.iter().all(|&x| x == Some(health)) {
                trace_it!(TEST_NET: self,
                          format!("Health {} {} as {}",
                                  self.members[to_check].name(),
                                  self.members[to_check].member_id(),
                                  health));
                return true;
            } else if self.check_rounds(&rounds_in) {
                for (i, some_health) in network_health.iter().enumerate() {
                    match some_health {
                        Some(ref health) => {
                            println!("{}: {:?}", i, health);
                            trace_it!(TEST: &self.members[i], format!("Health failed {} {} as {}", self.members[to_check].name(), self.members[to_check].member_id(), health));
                        }
                        None => {}
                    }
                }
                // println!("Failed network health check dump: {:#?}", self);
                return false;
            }
        }
    }

    #[allow(dead_code)]
    pub fn wait_protocol_period(&self) {
        let timing = Timing::default();
        let next_period = timing.next_protocol_period();
        loop {
            if SteadyTime::now() <= next_period {
                thread::sleep(Duration::from_millis(100));
            } else {
                return;
            }
        }
    }

    pub fn add_service(&mut self, member: usize, package: &str) {
        let ident = PackageIdent::from_str(package).expect("package needs to be a fully \
                                                            qualified package identifier");
        let sg = ServiceGroup::new(None, ident.name(), "prod", None).unwrap();
        let s = Service::new(self[member].member_id().to_string(),
                             &ident,
                             sg,
                             SysInfo::default(),
                             None);
        self[member].insert_service(s);
    }

    pub fn add_service_config(&mut self, member: usize, service: &str, config: &str) {
        let config_bytes: Vec<u8> = Vec::from(config);
        let s = ServiceConfig::new(self[member].member_id(),
                                   ServiceGroup::new(None, service, "prod", None).unwrap(),
                                   config_bytes);
        self[member].insert_service_config(s);
    }

    pub fn add_service_file(&mut self, member: usize, service: &str, filename: &str, body: &str) {
        let body_bytes: Vec<u8> = Vec::from(body);
        let s = ServiceFile::new(self[member].member_id(),
                                 ServiceGroup::new(None, service, "prod", None).unwrap(),
                                 filename,
                                 body_bytes);
        self[member].insert_service_file(s);
    }

    pub fn add_departure(&mut self, member: usize) {
        let d = Departure::new(self[member].member_id());
        self[member].insert_departure(d);
    }

    pub fn add_election(&mut self, member: usize, service: &str) {
        self[member].start_election(&ServiceGroup::new(None, service, "prod", None).unwrap(), 0);
    }
}

#[macro_export]
macro_rules! assert_health_of {
    ($network:expr, $to:expr, $health:expr) => {
        assert!($network.network_health_of($to)
                        .into_iter()
                        .all(|x| x == $health),
                "Member {} does not always have health {}",
                $to,
                $health)
    };
    ($network:expr, $from:expr, $to:expr, $health:expr) => {
        assert!($network.health_of($from, $to) == $health,
                "Member {} does not see {} as {}",
                $from,
                $to,
                $health)
    };
}

#[macro_export]
macro_rules! assert_wait_for_health_of {
    ($network:expr,[$from:expr, $to:expr], $health:expr) => {
        let left: Vec<usize> = $from.collect();
        let right: Vec<usize> = $to.collect();
        for l in left.iter() {
            for r in right.iter() {
                if l == r {
                    continue;
                }
                assert!($network.wait_for_health_of(*l, *r, $health),
                        "Member {} does not see {} as {}",
                        l,
                        r,
                        $health);
                assert!($network.wait_for_health_of(*r, *l, $health),
                        "Member {} does not see {} as {}",
                        r,
                        l,
                        $health);
            }
        }
    };
    ($network:expr, $to:expr, $health:expr) => {
        assert!($network.wait_for_network_health_of($to, $health),
                "Member {} does not always have health {}",
                $to,
                $health);
    };
    ($network:expr, $from:expr, $to:expr, $health:expr) => {
        assert!($network.wait_for_health_of($from, $to, $health),
                "Member {} does not see {} as {}",
                $from,
                $to,
                $health);
    };
}

#[macro_export]
macro_rules! assert_wait_for_election_status {
    ($network:expr,[$range:expr], $key:expr, $status:expr) => {
        for x in $range {
            assert!($network.wait_for_election_status(x, $key, $status));
        }
    };
    ($network:expr, $to:expr, $key:expr, $status:expr) => {
        assert!($network.wait_for_election_status($to, $key, $status));
    };
}

#[macro_export]
macro_rules! assert_wait_for_equal_election {
    ($network:expr, $left:expr, $right:expr, $key:expr) => {
        assert!($network.wait_for_equal_election($left, $right, $key));
    };
    ($network:expr,[$from:expr, $to:expr], $key:expr) => {
        let left: Vec<usize> = $from.collect();
        let right: Vec<usize> = $to.collect();
        for l in left.iter() {
            for r in right.iter() {
                if l == r {
                    continue;
                }
                assert!($network.wait_for_equal_election(*l, *r, $key),
                        "Member {} is not equal to {}",
                        l,
                        r);
            }
        }
    };
}
