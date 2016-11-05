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
use time::{SteadyTime, Duration};
use std::time::Duration as StdDuration;
use rustc_serialize::json::Json;
use util::docker::{self, Docker};
use hyper::client::Client;
use std::io::Read;
use regex::Regex;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Supervisor {
    pub docker: Docker,
    pub ip: String,
    pub peer_addr: String,
    pub id: String,
    pub census_id: String,
    pub split: Vec<String>,
    pub running: bool,
}

impl Supervisor {
    pub fn new() -> Supervisor {
        Supervisor::from_docker(docker::run("test/simple_service_gossip"))
    }

    pub fn new_with_topology(topology: &str) -> Supervisor {
        Supervisor::from_docker(docker::run_with_topology("test/simple_service_gossip", topology))
    }

    pub fn new_with_permanent() -> Supervisor {
        Supervisor::from_docker(docker::run_with_permanent("test/simple_service_gossip"))
    }

    pub fn new_with_permanent_topology(topology: &str) -> Supervisor {
        Supervisor::from_docker(docker::run_with_permanent_topology("test/simple_service_gossip",
                                                                    topology))
    }

    pub fn with_peer(peer: &Supervisor) -> Supervisor {
        Supervisor::from_docker(docker::run_with_peer("test/simple_service_gossip",
                                                      &peer.peer_addr))
    }

    pub fn with_peer_topology(peer: &Supervisor, topology: &str) -> Supervisor {
        Supervisor::from_docker(docker::run_with_peer_topology("test/simple_service_gossip",
                                                               &peer.peer_addr,
                                                               topology))
    }

    pub fn with_peer_permanent(peer: &Supervisor) -> Supervisor {
        Supervisor::from_docker(docker::run_with_peer_permanent("test/simple_service_gossip",
                                                                &peer.peer_addr))
    }

    pub fn with_peer_permanent_topology(peer: &Supervisor, topology: &str) -> Supervisor {
        Supervisor::from_docker(docker::run_with_peer_permanent_topology("test/simple_service_gossip",
                                                                         &peer.peer_addr,
                                                                         topology))
    }

    pub fn from_docker(sup: Docker) -> Supervisor {
        let sup_ip = sup.ipaddress();
        let sup_peer_address = format!("{}:{}", sup_ip, GOSSIP_DEFAULT_PORT);
        let sup_id = if sup.wait_until(r"Supervisor .+: (.+)") {
            let re = Regex::new(r"Supervisor .+: (.+)").unwrap();
            let logs = sup.logs();
            String::from(re.captures(&logs).unwrap().at(1).unwrap())
        } else {
            panic!("Cannot get supervisor ID for Supervisor");
        };
        let census_id = if sup.wait_until(r"Census .+: (.+)") {
            let re = Regex::new(r"Census .+: (.+)").unwrap();
            let logs = sup.logs();
            String::from(re.captures(&logs).unwrap().at(1).unwrap())
        } else {
            panic!("Cannot get supervisor ID for Supervisor");
        };

        if !sup.wait_until("Starting census health adjustor") {
            panic!("Census failed to initialize");
        }
        Supervisor {
            docker: sup,
            ip: sup_ip,
            peer_addr: sup_peer_address,
            id: sup_id,
            census_id: census_id,
            split: Vec::new(),
            running: true,
        }
    }

    pub fn get_sidecar(&self, path: &str) -> Json {
        let client = Client::new();
        let retry_max = 10;
        let mut retry_count = 0;
        while retry_count < retry_max {
            let mut res = match client.get(&format!("http://{}:9631/{}", self.ip, path)).send() {
                Ok(res) => res,
                Err(e) => {
                    println!("Cannot get {}: {:?}", path, e);
                    thread::sleep(StdDuration::from_secs(1));
                    retry_count = retry_count + 1;
                    continue;
                }
            };
            let mut gossip_string = String::new();
            res.read_to_string(&mut gossip_string).unwrap();
            return Json::from_str(&gossip_string).unwrap();
        }
        panic!("Cannot get {}; 10 tries!", path);
    }

    pub fn gossip(&self) -> Json {
        self.get_sidecar("gossip")
    }

    pub fn census(&self) -> Json {
        self.get_sidecar("census")
    }

    pub fn election(&self) -> Json {
        self.get_sidecar("election")
    }

    pub fn status(&self) -> String {
        let client = Client::new();
        let retry_max = 10;
        let mut retry_count = 0;
        while retry_count < retry_max {
            let mut res = match client.get(&format!("http://{}:9631/status", self.ip)).send() {
                Ok(res) => res,
                Err(e) => {
                    println!("Cannot get /status: {:?}", e);
                    thread::sleep(StdDuration::from_secs(1));
                    retry_count = retry_count + 1;
                    continue;
                }
            };
            let mut gossip_string = String::new();
            res.read_to_string(&mut gossip_string).unwrap();
            return gossip_string;
        }
        panic!("Cannot get /status; 10 tries!");
    }

    pub fn status_down(&self) -> bool {
        let status = self.status();
        let re = Regex::new(r"^down:").unwrap();
        re.is_match(&status)
    }

    pub fn wait_for_status_down(&self) -> bool {
        self.wait_for_it(Duration::seconds(10), || self.status_down())
    }

    pub fn incarnation(&self) -> u64 {
        let gossip = self.gossip();
        let member_list = gossip.find_path(&["member_list", "members"])
            .unwrap()
            .as_object()
            .unwrap();
        let member = member_list.get(&self.id).unwrap().as_object().unwrap();
        member.get("incarnation").unwrap().find("counter").unwrap().as_u64().unwrap()
    }

    pub fn has_member(&self, sup: &Supervisor) -> bool {
        let gossip = self.gossip();
        let member_list = gossip.find_path(&["member_list", "members"])
            .unwrap()
            .as_object()
            .unwrap();
        member_list.contains_key(&sup.id)
    }

    pub fn health_of_member(&self, sup: &Supervisor) -> String {
        let gossip = self.gossip();
        let member_list = gossip.find_path(&["member_list", "members"])
            .unwrap()
            .as_object()
            .unwrap();
        let member = match member_list.get(&sup.id) {
            Some(m) => m.as_object().unwrap(),
            None => return "Nonexistent".to_string(),
        };
        String::from(member.get("health").unwrap().as_string().unwrap())
    }

    pub fn netsplit(&self, sup: &Supervisor) {
        self.docker
            .exec(&["/bin/sh", "-l", "-c", &format!("iptables -A INPUT -s {} -j DROP", &sup.ip)]);
        sup.docker
            .exec(&["/bin/sh", "-l", "-c", &format!("iptables -A INPUT -s {} -j DROP", &self.ip)]);
    }

    pub fn netjoin(&self, sup: &Supervisor) {
        self.docker
            .exec(&["/bin/sh", "-l", "-c", &format!("iptables -D INPUT -s {} -j DROP", &sup.ip)]);
        sup.docker
            .exec(&["/bin/sh", "-l", "-c", &format!("iptables -D INPUT -s {} -j DROP", &self.ip)]);
    }

    pub fn keeps_member_alive(&self, sup: &Supervisor) -> bool {
        let mut now = SteadyTime::now();
        let end_time = SteadyTime::now() + Duration::milliseconds(40000);
        while now < end_time {
            let member_health = self.health_of_member(sup);
            if member_health != "Alive" {
                println!("Failed to keep member alive: {}", self.gossip());
                return false;
            }
            now = SteadyTime::now();
        }
        true
    }

    pub fn wait_for_alive_longer(&self, sup: &Supervisor) -> bool {
        self.wait_for_health(sup, "Alive", Duration::milliseconds(30000))
    }

    pub fn wait_for_alive(&self, sup: &Supervisor) -> bool {
        self.wait_for_health(sup, "Alive", Duration::milliseconds(10000))
    }

    pub fn wait_for_suspect(&self, sup: &Supervisor) -> bool {
        self.wait_for_health(sup, "Suspect", Duration::milliseconds(40000))
    }

    pub fn wait_for_confirmed(&self, sup: &Supervisor) -> bool {
        self.wait_for_health(sup, "Confirmed", Duration::milliseconds(60000))
    }

    pub fn wait_for_health(&self, sup: &Supervisor, health: &str, timeout: Duration) -> bool {
        self.wait_for_it(timeout, || {
            let member_health = self.health_of_member(sup);
            health == member_health
        })
    }

    pub fn wait_for_it<F>(&self, timeout: Duration, check_it: F) -> bool
        where F: Fn() -> bool
    {
        let mut now = SteadyTime::now();
        let end_time = SteadyTime::now() + timeout;
        while now < end_time {
            if check_it() {
                return true;
            }
            now = SteadyTime::now();
        }
        false
    }

    pub fn minimum_quorum(&self) -> bool {
        self.wait_for_it(Duration::seconds(2), || {
            let census = self.census();
            println!("{}", census.pretty());
            match census.find("minimum_quorum") {
                Some(q) => q.as_boolean().unwrap(),
                None => false,
            }
        })
    }

    pub fn term(&self) -> u64 {
        let election = self.election();
        match election.find_path(&["mine", "term"]) {
            Some(id) => id.as_u64().unwrap(),
            None => panic!("Cannot get term: {}", election.pretty()),
        }
    }

    pub fn wait_for_term_newer_than(&self, old_term: u64) -> bool {
        // Confirmed + kicker - :(
        self.wait_for_it(Duration::seconds(65), || {
            let election = self.election();
            match election.find_path(&["mine", "term"]) {
                Some(oterm) => {
                    let term = oterm.as_u64().unwrap();
                    term > old_term
                }
                None => false,
            }
        })
    }

    pub fn leader(&self) -> String {
        let election = self.election();
        match election.find_path(&["mine", "leader_id"]) {
            Some(id) => String::from(id.as_string().unwrap()),
            None => panic!("Cannot get leaders id: {}", election.pretty()),
        }
    }

    pub fn wait_for_leader(&self) -> bool {
        self.wait_for_it(Duration::seconds(5), || {
            let election = self.election();
            match election.find_path(&["mine", "status"]) {
                Some(status) => {
                    match status.as_string() {
                        Some("Finished") => true,
                        Some(_) => false,
                        None => false,
                    }
                }
                None => false,
            }
        })
    }

    pub fn wait_for_election_running(&self) -> bool {
        self.wait_for_it(Duration::seconds(5), || {
            let election = self.election();
            match election.find_path(&["mine", "status"]) {
                Some(status) => {
                    match status.as_string() {
                        Some("Running") => true,
                        Some(_) => false,
                        None => false,
                    }
                }
                None => false,
            }
        })
    }


    pub fn stop(&mut self) {
        self.docker.stop();
        self.running = false;
    }

    pub fn stop_if_leader<'a>(&'a mut self) -> Option<String> {
        let leader = self.leader();
        if leader == self.id {
            self.stop();
            Some(self.id.clone())
        } else {
            None
        }
    }
}

// {
//   "quorum": true,
//   "minimum_quorum": false,
//   "local_census": {
//     "group": "default",
//     "service": "redis",
//     "in_event": false,
//     "population": {
//       "c6678f58-8dbb-409d-8c2a-7634a5a572c0": {
//         "incarnation": {
//           "counter": 0
//         },
//         "detached": false,
//         "confirmed": false,
//         "suspect": false,
//         "alive": true,
//         "group": "default",
//         "leader": false,
//         "exposes": null,
//         "port": null,
//         "suitability": 0,
//         "ip": "172.17.0.6",
//         "hostname": "90f5bb3c9fc8",
//         "member_id": "1323856f-83ef-4bf7-b8a3-078668077008",
//         "id": "c6678f58-8dbb-409d-8c2a-7634a5a572c0",
//         "follower": false,
//         "data_init": false,
//         "vote": null,
//         "election": null,
//         "needs_write": null,
//         "initialized": false,
//         "keep_me": true,
//         "service": "redis"
//       }
//     },
//     "me": "c6678f58-8dbb-409d-8c2a-7634a5a572c0"
//   },
//   "me": {
//     "incarnation": {
//       "counter": 0
//     },
//     "detached": false,
//     "confirmed": false,
//     "suspect": false,
//     "alive": true,
//     "group": "default",
//     "leader": false,
//     "exposes": null,
//     "port": null,
//     "suitability": 0,
//     "ip": "172.17.0.6",
//     "hostname": "90f5bb3c9fc8",
//     "member_id": "1323856f-83ef-4bf7-b8a3-078668077008",
//     "id": "c6678f58-8dbb-409d-8c2a-7634a5a572c0",
//     "follower": false,
//     "data_init": false,
//     "vote": null,
//     "election": null,
//     "needs_write": null,
//     "initialized": false,
//     "keep_me": true,
//     "service": "redis"
//   },
//   "census_list": {
//     "censuses": {
//       "redis.default": {
//         "group": "default",
//         "service": "redis",
//         "in_event": false,
//         "population": {
//           "c6678f58-8dbb-409d-8c2a-7634a5a572c0": {
//             "incarnation": {
//               "counter": 0
//             },
//             "detached": false,
//             "confirmed": false,
//             "suspect": false,
//             "alive": true,
//             "group": "default",
//             "leader": false,
//             "exposes": null,
//             "port": null,
//             "suitability": 0,
//             "ip": "172.17.0.6",
//             "hostname": "90f5bb3c9fc8",
//             "member_id": "1323856f-83ef-4bf7-b8a3-078668077008",
//             "id": "c6678f58-8dbb-409d-8c2a-7634a5a572c0",
//             "follower": false,
//             "data_init": false,
//             "vote": null,
//             "election": null,
//             "needs_write": null,
//             "initialized": false,
//             "keep_me": true,
//             "service": "redis"
//           }
//         },
//         "me": "c6678f58-8dbb-409d-8c2a-7634a5a572c0"
//       }
//     },
//     "local_census": "redis.default"
//   },
//   "id": "c6678f58-8dbb-409d-8c2a-7634a5a572c0"
// }
