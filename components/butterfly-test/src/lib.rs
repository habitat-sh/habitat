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
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate env_logger;
extern crate time;
#[macro_use]
extern crate habitat_butterfly;
extern crate habitat_core;

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::thread;
use std::ops::{Deref, DerefMut, Range};
use std::path::PathBuf;
use std::time::Duration;
use std::str::FromStr;

use time::SteadyTime;

use habitat_butterfly::server::{Server, Suitability};
use habitat_butterfly::member::{Member, Health};
use habitat_butterfly::server::timing::Timing;
use habitat_butterfly::rumor::departure::Departure;
use habitat_butterfly::rumor::service::{Service, SysInfo};
use habitat_butterfly::rumor::service_config::ServiceConfig;
use habitat_butterfly::rumor::service_file::ServiceFile;
use habitat_butterfly::message::swim::Election_Status;
use habitat_core::service::ServiceGroup;
use habitat_core::package::{Identifiable, PackageIdent};
use habitat_core::crypto::keys::sym_key::SymKey;
use habitat_butterfly::trace::Trace;

static SERVER_PORT: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Debug)]
struct NSuitability(u64);
impl Suitability for NSuitability {
    fn get(&self, _service_group: &ServiceGroup) -> u64 {
        self.0
    }
}

pub fn start_server(name: &str, ring_key: Option<SymKey>, suitability: u64) -> Server {
    SERVER_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
    let swim_port = SERVER_PORT.fetch_add(1, Ordering::Relaxed);
    let gossip_port = SERVER_PORT.fetch_add(1, Ordering::Relaxed);
    let listen_swim = format!("127.0.0.1:{}", swim_port);
    let listen_gossip = format!("127.0.0.1:{}", gossip_port);
    let mut member = Member::default();
    member.set_swim_port(swim_port as i32);
    member.set_gossip_port(gossip_port as i32);
    let mut server = Server::new(
        &listen_swim[..],
        &listen_gossip[..],
        member,
        Trace::default(),
        ring_key,
        Some(String::from(name)),
        None::<PathBuf>,
        Box::new(NSuitability(suitability)),
    ).unwrap();
    server.start(Timing::default()).expect(
        "Cannot start server",
    );
    server
}

pub fn member_from_server(server: &Server) -> Member {
    let mut new_member = Member::default();
    let server_member = server.member.read().expect("Member lock is poisoned");
    new_member.set_id(String::from(server_member.get_id()));
    new_member.set_incarnation(server_member.get_incarnation());
    new_member.set_address(String::from("127.0.0.1"));
    new_member.set_swim_port(server.swim_port() as i32);
    new_member.set_gossip_port(server.gossip_port() as i32);
    new_member
}


#[derive(Debug)]
pub struct SwimNet {
    pub members: Vec<Server>,
}

impl Deref for SwimNet {
    type Target = Vec<Server>;

    fn deref(&self) -> &Vec<Server> {
        &self.members
    }
}

impl DerefMut for SwimNet {
    fn deref_mut(&mut self) -> &mut Vec<Server> {
        &mut self.members
    }
}

impl SwimNet {
    pub fn new_with_suitability(suitabilities: Vec<u64>) -> SwimNet {
        let count = suitabilities.len();
        let mut members = Vec::with_capacity(count);
        for x in 0..count {
            members.push(start_server(&format!("{}", x), None, suitabilities[x]));
        }
        SwimNet { members: members }
    }

    pub fn new(count: usize) -> SwimNet {
        let suitabilities = vec![0; count];
        SwimNet::new_with_suitability(suitabilities)
    }

    pub fn new_ring_encryption(count: usize, ring_key: Option<SymKey>) -> SwimNet {
        let mut members = Vec::with_capacity(count);
        for x in 0..count {
            let rk = ring_key.clone();
            members.push(start_server(&format!("{}", x), rk, 0));
        }
        SwimNet { members: members }
    }

    pub fn connect(&mut self, from_entry: usize, to_entry: usize) {
        let to = member_from_server(&self.members[to_entry]);
        trace_it!(TEST: &self.members[from_entry], format!("Connected {} {}", self.members[to_entry].name(), self.members[to_entry].member_id()));
        self.members[from_entry].insert_member(to, Health::Alive);
    }

    pub fn add_member(&mut self) {
        let number = self.members.len() + 1;
        self.members.push(
            start_server(&format!("{}", number), None, 0),
        );
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
                let server_b = self.members.get(x_pos).unwrap();
                to_mesh.push(member_from_server(server_b))
            }
            let server_a = self.members.get(pos).unwrap();
            for server_b in to_mesh.into_iter() {
                server_a.insert_member(server_b, Health::Alive);
            }
        }
    }

    pub fn blacklist(&self, from_entry: usize, to_entry: usize) {
        let from = self.members.get(from_entry).expect(
            "Asked for a network member who is out of bounds",
        );
        let to = self.members.get(to_entry).expect(
            "Asked for a network member who is out of bounds",
        );
        trace_it!(TEST: &self.members[from_entry], format!("Blacklisted {} {}", self.members[to_entry].name(), self.members[to_entry].member_id()));
        from.add_to_blacklist(String::from(
            to.member.read().expect("Member lock is poisoned").get_id(),
        ));
    }

    pub fn unblacklist(&self, from_entry: usize, to_entry: usize) {
        let from = self.members.get(from_entry).expect(
            "Asked for a network member who is out of bounds",
        );
        let to = self.members.get(to_entry).expect(
            "Asked for a network member who is out of bounds",
        );
        trace_it!(TEST: &self.members[from_entry], format!("Un-Blacklisted {} {}", self.members[to_entry].name(), self.members[to_entry].member_id()));
        from.remove_from_blacklist(to.member_id());
    }

    pub fn health_of(&self, from_entry: usize, to_entry: usize) -> Option<Health> {
        let from = self.members.get(from_entry).expect(
            "Asked for a network member who is out of bounds",
        );

        let to = self.members.get(to_entry).expect(
            "Asked for a network member who is out of bounds",
        );
        let to_member = to.member.read().expect("Member lock is poisoned");
        from.member_list.health_of(&to_member)
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

    pub fn max_rounds(&self) -> isize {
        4
    }

    pub fn max_gossip_rounds(&self) -> isize {
        5
    }

    pub fn rounds(&self) -> Vec<isize> {
        self.members.iter().map(|m| m.swim_rounds()).collect()
    }

    pub fn rounds_in(&self, count: isize) -> Vec<isize> {
        self.rounds().iter().map(|r| r + count).collect()
    }

    pub fn gossip_rounds(&self) -> Vec<isize> {
        self.members.iter().map(|m| m.gossip_rounds()).collect()
    }

    pub fn gossip_rounds_in(&self, count: isize) -> Vec<isize> {
        self.gossip_rounds().iter().map(|r| r + count).collect()
    }

    pub fn check_rounds(&self, rounds_in: &Vec<isize>) -> bool {
        let mut finished = Vec::with_capacity(rounds_in.len());
        for (i, round) in rounds_in.into_iter().enumerate() {
            if self.members[i].paused() {
                finished.push(true);
            } else {
                if self.members[i].swim_rounds() > *round {
                    finished.push(true);
                } else {
                    finished.push(false);
                }
            }
        }
        if finished.iter().all(|m| m == &true) {
            return true;
        } else {
            return false;
        }
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

    pub fn check_gossip_rounds(&self, rounds_in: &Vec<isize>) -> bool {
        let mut finished = Vec::with_capacity(rounds_in.len());
        for (i, round) in rounds_in.into_iter().enumerate() {
            if self.members[i].paused() {
                finished.push(true);
            } else {
                if self.members[i].gossip_rounds() > *round {
                    finished.push(true);
                } else {
                    finished.push(false);
                }
            }
        }
        if finished.iter().all(|m| m == &true) {
            return true;
        } else {
            return false;
        }
    }

    #[allow(dead_code)]
    pub fn wait_for_gossip_rounds(&self, rounds: isize) {
        let rounds_in = self.gossip_rounds_in(rounds);
        loop {
            if self.check_gossip_rounds(&rounds_in) {
                return;
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

    pub fn wait_for_election_status(
        &self,
        e_num: usize,
        key: &str,
        status: Election_Status,
    ) -> bool {
        let rounds_in = self.gossip_rounds_in(self.max_gossip_rounds());
        loop {
            let mut result = false;
            let server = self.members.get(e_num).expect(
                "Asked for a network member who is out of bounds",
            );
            server.election_store.with_rumor(
                key,
                "election",
                |e| if e.is_some() &&
                    e.unwrap().get_status() == status
                {
                    result = true;
                },
            );
            if result {
                return true;
            }
            if self.check_gossip_rounds(&rounds_in) {
                println!(
                    "Failed election check for status {:?}: {:#?}",
                    status,
                    self.members[e_num].election_store
                );
                return false;
            }
        }
    }

    pub fn wait_for_equal_election(&self, left: usize, right: usize, key: &str) -> bool {
        let rounds_in = self.gossip_rounds_in(self.max_gossip_rounds());
        loop {
            let mut result = false;

            let left_server = self.members.get(left).expect(
                "Asked for a network member who is out of bounds",
            );
            let right_server = self.members.get(right).expect(
                "Asked for a network member who is out of bounds",
            );

            left_server.election_store.with_rumor(key, "election", |l| {
                right_server.election_store.with_rumor(
                    key,
                    "election",
                    |r| {
                        result = l.is_some() && r.is_some() && l.unwrap() == r.unwrap();
                    },
                );
            });
            if result {
                return true;
            }
            if self.check_gossip_rounds(&rounds_in) {
                println!("Failed election check for equality:\nL: {:#?}\n\nR: {:#?}",
                         self.members[left].election_store,
                         self.members[right].election_store,
                         );
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
                self.blacklist(*l, *r);
                self.blacklist(*r, *l);
            }
        }
    }

    pub fn unpartition(&self, left_range: Range<usize>, right_range: Range<usize>) {
        let left: Vec<usize> = left_range.collect();
        let right: Vec<usize> = right_range.collect();
        for l in left.iter() {
            for r in right.iter() {
                println!("UnPartitioning {} from {}", *l, *r);
                self.unblacklist(*l, *r);
                self.unblacklist(*r, *l);
            }
        }
    }

    pub fn wait_for_health_of(&self, from_entry: usize, to_check: usize, health: Health) -> bool {
        let rounds_in = self.rounds_in(self.max_rounds());
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
                println!(
                    "Failed health check for\n***FROM***{:#?}\n***TO***\n{:#?}",
                    self.members[from_entry],
                    self.members[to_check]
                );
                return false;
            }
        }
    }

    pub fn wait_for_network_health_of(&self, to_check: usize, health: Health) -> bool {
        let rounds_in = self.rounds_in(self.max_rounds());
        loop {
            let network_health = self.network_health_of(to_check);
            if network_health.iter().all(|x| if let &Some(ref h) = x {
                *h == health
            } else {
                false
            })
            {
                trace_it!(
                    TEST_NET: self,
                    format!(
                        "Health {} {} as {}",
                        self.members[to_check].name(),
                        self.members[to_check].member_id(),
                        health
                    )
                );
                return true;
            } else if self.check_rounds(&rounds_in) {
                for (i, some_health) in network_health.iter().enumerate() {
                    match some_health {
                        &Some(ref health) => {
                            println!("{}: {:?}", i, health);
                            trace_it!(TEST: &self.members[i], format!("Health failed {} {} as {}", self.members[to_check].name(), self.members[to_check].member_id(), health));
                        }
                        &None => {}
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
        let ident = PackageIdent::from_str(package).expect(
            "package needs to be a fully qualified package identifier",
        );
        let sg = ServiceGroup::new(None, ident.name(), "prod", None).unwrap();
        let s = Service::new(
            self[member].member_id().to_string(),
            &ident,
            &sg,
            &SysInfo::default(),
            None,
        );
        self[member].insert_service(s);
    }

    pub fn add_service_config(&mut self, member: usize, service: &str, config: &str) {
        let config_bytes: Vec<u8> = Vec::from(config);
        let s = ServiceConfig::new(
            self[member].member_id(),
            ServiceGroup::new(None, service, "prod", None).unwrap(),
            config_bytes,
        );
        self[member].insert_service_config(s);
    }

    pub fn add_service_file(&mut self, member: usize, service: &str, filename: &str, body: &str) {
        let body_bytes: Vec<u8> = Vec::from(body);
        let s = ServiceFile::new(
            self[member].member_id(),
            ServiceGroup::new(None, service, "prod", None).unwrap(),
            filename,
            body_bytes,
        );
        self[member].insert_service_file(s);
    }

    pub fn add_departure(&mut self, member: usize) {
        let d = Departure::new(self[member].member_id());
        self[member].insert_departure(d);
    }

    pub fn add_election(&mut self, member: usize, service: &str) {
        self[member].start_election(ServiceGroup::new(None, service, "prod", None).unwrap(), 0);
    }
}

#[macro_export]
macro_rules! assert_health_of {
    ($network:expr, $to:expr, $health:expr) => {
        assert!($network.network_health_of($to).into_iter().all(|x| x == $health), "Member {} does not always have health {}", $to, $health)
    };
    ($network:expr, $from: expr, $to:expr, $health:expr) => {
        assert!($network.health_of($from, $to) == $health, "Member {} does not see {} as {}", $from, $to, $health)
    }
}

#[macro_export]
macro_rules! assert_wait_for_health_of {
    ($network:expr, [$from: expr, $to:expr], $health:expr) => {
        let left: Vec<usize> = $from.collect();
        let right: Vec<usize> = $to.collect();
        for l in left.iter() {
            for r in right.iter() {
                if l == r {
                    continue;
                }
                assert!($network.wait_for_health_of(*l, *r, $health), "Member {} does not see {} as {}", l, r, $health);
                assert!($network.wait_for_health_of(*r, *l, $health), "Member {} does not see {} as {}", r, l, $health);
            }
        }
    };
    ($network:expr, $to:expr, $health:expr) => {
        assert!($network.wait_for_network_health_of($to, $health), "Member {} does not always have health {}", $to, $health);
    };
    ($network:expr, $from: expr, $to:expr, $health:expr) => {
        assert!($network.wait_for_health_of($from, $to, $health), "Member {} does not see {} as {}", $from, $to, $health);
    };
}

#[macro_export]
macro_rules! assert_wait_for_election_status {
    ($network:expr, [$range:expr], $key:expr, $status: expr) => {
        for x in $range {
            assert!($network.wait_for_election_status(x, $key, $status));
        }
    };
    ($network:expr, $to:expr, $key:expr, $status: expr) => {
        assert!($network.wait_for_election_status($to, $key, $status));
    };
}

#[macro_export]
macro_rules! assert_wait_for_equal_election {
    ($network:expr, $left:expr, $right:expr, $key:expr) => {
        assert!($network.wait_for_equal_election($left, $right, $key));
    };
    ($network:expr, [$from: expr, $to:expr], $key:expr) => {
        let left: Vec<usize> = $from.collect();
        let right: Vec<usize> = $to.collect();
        for l in left.iter() {
            for r in right.iter() {
                if l == r {
                    continue;
                }
                assert!($network.wait_for_equal_election(*l, *r, $key), "Member {} is not equal to {}", l, r);
            }
        }
    };
}
