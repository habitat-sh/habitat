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

use std::thread;
use std::ops::{Deref, DerefMut, Range};
use std::time::Duration;

use time::SteadyTime;

use common;
use habitat_swim::server::Server;
use habitat_swim::member::{Member, Health};
use habitat_swim::server::outbound::Timing;

#[derive(Debug)]
pub struct SwimNet {
    members: Vec<Server>,
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
    pub fn new(count: usize) -> SwimNet {
        let mut members = Vec::with_capacity(count);
        for x in 0..count {
            members.push(common::start_server(&format!("{}", x)));
        }
        SwimNet { members: members }
    }

    pub fn connect(&mut self, from_entry: usize, to_entry: usize) {
        let to = common::member_from_server(&self.members[to_entry]);
        self.members[from_entry].insert_member(to, Health::Alive);
    }

    // Fully mesh the network
    pub fn mesh(&mut self) {
        for pos in 0..self.members.len() {
            let mut to_mesh: Vec<Member> = Vec::new();
            for x_pos in 0..self.members.len() {
                if pos == x_pos {
                    continue;
                }
                let server_b = self.members.get(x_pos).unwrap();
                to_mesh.push(common::member_from_server(server_b))
            }
            let server_a = self.members.get(pos).unwrap();
            for server_b in to_mesh.into_iter() {
                server_a.insert_member(server_b, Health::Alive);
            }
        }
    }

    pub fn blacklist(&self, from_entry: usize, to_entry: usize) {
        let from =
            self.members.get(from_entry).expect("Asked for a network member who is out of bounds");
        let to =
            self.members.get(to_entry).expect("Asked for a network member who is out of bounds");
        let to_addr = to.socket.local_addr().expect("Socket does not have an address");
        from.add_to_blacklist(to_addr);
    }

    pub fn unblacklist(&self, from_entry: usize, to_entry: usize) {
        let from =
            self.members.get(from_entry).expect("Asked for a network member who is out of bounds");
        let to =
            self.members.get(to_entry).expect("Asked for a network member who is out of bounds");
        let to_addr = to.socket.local_addr().expect("Socket does not have an address");
        from.remove_from_blacklist(&to_addr);
    }

    pub fn health_of(&self, from_entry: usize, to_entry: usize) -> Option<Health> {
        let from =
            self.members.get(from_entry).expect("Asked for a network member who is out of bounds");

        let to =
            self.members.get(to_entry).expect("Asked for a network member who is out of bounds");
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
        3
    }

    pub fn rounds(&self) -> Vec<isize> {
        self.members.iter().map(|m| m.rounds()).collect()
    }

    pub fn rounds_in(&self, count: isize) -> Vec<isize> {
        self.rounds().iter().map(|r| r + count).collect()
    }

    pub fn check_rounds(&self, rounds_in: &Vec<isize>) -> bool {
        let mut finished = Vec::with_capacity(rounds_in.len());
        for (i, round) in rounds_in.into_iter().enumerate() {
            if self.members[i].paused() {
                finished.push(true);
            } else {
                if self.members[i].rounds() > *round {
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

    pub fn partition(&self, left_range: Range<usize>, right_range: Range<usize>) {
        let left: Vec<usize> = left_range.collect();
        let right: Vec<usize> = right_range.collect();
        for l in left.iter() {
            for r in right.iter() {
                println!("Partitioning {} from {}", *l, *r);
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
                    return true;
                }
            }
            if self.check_rounds(&rounds_in) {
                println!("Failed health check for\n***FROM***{:#?}\n***TO***\n{:#?}",
                         self.members[from_entry],
                         self.members[to_check]);
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
            }) {
                return true;
            } else if self.check_rounds(&rounds_in) {
                for (i, some_health) in network_health.iter().enumerate() {
                    match some_health {
                        &Some(ref health) => println!("{}: {:?}", i, health),
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
}

macro_rules! assert_health_of {
    ($network:expr, $to:expr, $health:expr) => {
        assert!($network.network_health_of($to).into_iter().all(|x| x == $health), "Member {} does not always have health {}", $to, $health)
    };
    ($network:expr, $from: expr, $to:expr, $health:expr) => {
        assert!($network.health_of($from, $to) == $health, "Member {} does not see {} as {}", $from, $to, $health)
    }
}

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
        assert!($network.wait_for_network_health_of($to, $health), "Member {} does not always have health {}", $to, $health)
    };
    ($network:expr, $from: expr, $to:expr, $health:expr) => {
        assert!($network.wait_for_health_of($from, $to, $health), "Member {} does not see {} as {}", $from, $to, $health)
    };
}
