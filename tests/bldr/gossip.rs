// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::io::Read;
use std::thread;

use time::{SteadyTime, Duration};
use std::time::Duration as StdDuration;
use rustc_serialize::json::Json;
use hyper::client::Client;

use setup;
use util::docker::{self, Docker};
use regex::Regex;

use bldr_lib::gossip::server::GOSSIP_DEFAULT_PORT;

#[allow(dead_code)]
struct Supervisor {
    docker: Docker,
    ip: String,
    peer_addr: String,
    id: String,
    split: Vec<String>,
}

impl Supervisor {
    fn new() -> Supervisor {
        Supervisor::from_docker(docker::run("test/simple_service_gossip"))
    }

    fn new_with_permanent() -> Supervisor {
        Supervisor::from_docker(docker::run_with_permanent("test/simple_service_gossip"))
    }

    fn with_peer(peer: &Supervisor) -> Supervisor {
        Supervisor::from_docker(docker::run_with_peer("test/simple_service_gossip",
                                                      &peer.peer_addr))
    }

    fn with_peer_permanent(peer: &Supervisor) -> Supervisor {
        Supervisor::from_docker(docker::run_with_peer_permanent("test/simple_service_gossip",
                                                                &peer.peer_addr))
    }

    fn from_docker(sup: Docker) -> Supervisor {
        let sup_ip = sup.ipaddress();
        let sup_peer_address = format!("{}:{}", sup_ip, GOSSIP_DEFAULT_PORT);
        let sup_id = if sup.wait_until(r"Supervisor (.+)") {
            let re = Regex::new(r"Supervisor (.+)").unwrap();
            let logs = sup.logs();
            String::from(re.captures(&logs).unwrap().at(1).unwrap())
        } else {
            panic!("Cannot get supervisor ID for Supervisor");
        };
        if sup.wait_until(r"Shipping out to Boston") == false {
            panic!("Container did not start");
        }
        Supervisor {
            docker: sup,
            ip: sup_ip,
            peer_addr: sup_peer_address,
            id: sup_id,
            split: Vec::new(),
        }
    }

    fn status(&self) -> Json {
        let client = Client::new();
        let retry_max = 10;
        let mut retry_count = 0;
        while retry_count < retry_max {
            let mut res = match client.get(&format!("http://{}:9631/gossip", self.ip)).send() {
                Ok(res) => res,
                Err(e) => {
                    println!("Cannot get status: {:?}", e);
                    thread::sleep(StdDuration::from_secs(1));
                    retry_count = retry_count + 1;
                    continue;
                }
            };
            let mut gossip_string = String::new();
            res.read_to_string(&mut gossip_string).unwrap();
            return Json::from_str(&gossip_string).unwrap();
        }
        panic!("Cannot get status; 10 tries!");
    }

    fn incarnation(&self) -> u64 {
        let gossip = self.status();
        let member_list = gossip.find_path(&["member_list", "members"])
                                .unwrap()
                                .as_object()
                                .unwrap();
        let member = member_list.get(&self.id).unwrap().as_object().unwrap();
        member.get("incarnation").unwrap().find("counter").unwrap().as_u64().unwrap()
    }

    fn has_member(&self, sup: &Supervisor) -> bool {
        let gossip = self.status();
        let member_list = gossip.find_path(&["member_list", "members"])
                                .unwrap()
                                .as_object()
                                .unwrap();
        member_list.contains_key(&sup.id)
    }

    fn health_of_member(&self, sup: &Supervisor) -> String {
        let gossip = self.status();
        let member_list = gossip.find_path(&["member_list", "members"])
                                .unwrap()
                                .as_object()
                                .unwrap();
        let member = member_list.get(&sup.id).unwrap().as_object().unwrap();
        String::from(member.get("health").unwrap().as_string().unwrap())
    }

    fn netsplit(&mut self, sup: &Supervisor) {
        self.docker.exec(&["iptables", "-A", "INPUT", "-s", &sup.ip, "-j", "DROP"]);
        sup.docker.exec(&["iptables", "-A", "INPUT", "-s", &self.ip, "-j", "DROP"]);
    }

    fn netjoin(&self, sup: &Supervisor) {
        self.docker.exec(&["iptables", "-D", "INPUT", "-s", &sup.ip, "-j", "DROP"]);
        sup.docker.exec(&["iptables", "-D", "INPUT", "-s", &self.ip, "-j", "DROP"]);
    }

    fn keeps_member_alive(&self, sup: &Supervisor) -> bool {
        let mut now = SteadyTime::now();
        let end_time = SteadyTime::now() + Duration::milliseconds(40000);
        while now < end_time {
            let member_health = self.health_of_member(sup);
            if member_health != "Alive" {
                println!("Failed to keep member alive: {}", self.status());
                return false;
            }
            now = SteadyTime::now();
        }
        true
    }

    fn wait_for_alive(&self, sup: &Supervisor) -> bool {
        self.wait_for_health(sup, "Alive", Duration::milliseconds(10000))
    }

    fn wait_for_suspect(&self, sup: &Supervisor) -> bool {
        self.wait_for_health(sup, "Suspect", Duration::milliseconds(40000))
    }

    fn wait_for_confirmed(&self, sup: &Supervisor) -> bool {
        self.wait_for_health(sup, "Confirmed", Duration::milliseconds(60000))
    }

    fn wait_for_health(&self, sup: &Supervisor, health: &str, timeout: Duration) -> bool {
        let mut now = SteadyTime::now();
        let end_time = SteadyTime::now() + timeout;
        while now < end_time {
            let member_health = self.health_of_member(sup);
            if health == member_health {
                return true;
            }
            now = SteadyTime::now();
        }
        println!("Failed health check {}: {}", health, self.status());
        false
    }
}

// Start two supervisors, and make sure they see each other
#[test]
fn two_supervisors_link() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_a.has_member(&sup_b));
}

// Start two supervisors, stop one, make sure they see the failure
#[test]
fn two_supervisors_detect_failure() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_a.has_member(&sup_b));

    sup_b.docker.stop();

    // You need time for the connection to time out, then the pingreq fails
    assert!(sup_a.wait_for_suspect(&sup_b));

    // Then the grace window closes, and we call you dead
    assert!(sup_a.wait_for_confirmed(&sup_b));
}

// Start three supervisors, a, b, and c. A is linked to b, and b is linked to c. a, b, and c all
// see the other two members, even though they were not provided initially.
#[test]
fn members_are_gossiped() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);
    let sup_c = Supervisor::with_peer(&sup_b);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_b.has_member(&sup_c));
    assert!(sup_a.has_member(&sup_b));
    assert!(sup_a.has_member(&sup_c));
    assert!(sup_c.has_member(&sup_a));
    assert!(sup_c.has_member(&sup_b));
}

// Start three supervisors, a, b, c. Split a and c; confirm that neither a nor c is marked
// as anything but alive, confirming they are routing their gossip through b.
#[test]
fn routes_around_failure() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let mut sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);
    let sup_c = Supervisor::with_peer(&sup_b);

    assert!(sup_b.has_member(&sup_a));
    assert!(sup_b.has_member(&sup_c));
    assert!(sup_a.has_member(&sup_b));
    assert!(sup_a.has_member(&sup_c));
    assert!(sup_c.has_member(&sup_a));
    assert!(sup_c.has_member(&sup_b));

    sup_a.netsplit(&sup_c);
    assert!(sup_a.keeps_member_alive(&sup_c));
    assert!(sup_c.keeps_member_alive(&sup_a));
}

// Incarnation update on suspicion. Start A, B, and C. A and C split from from B; B is marked
// suspect. A and C then rejoin B, and upon receipt of the suspect rumor about itself, B increments
// its incarnation and shares its Alive rumor. A and C then see B as alive again.
#[test]
fn incarnation_updates_on_suspicion() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let mut sup_a = Supervisor::new();
    let sup_b = Supervisor::with_peer(&sup_a);
    let mut sup_c = Supervisor::with_peer(&sup_b);

    // Make sure we are all alive
    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));
    assert!(sup_c.wait_for_alive(&sup_b));

    // Split a and c from b
    sup_a.netsplit(&sup_b);
    sup_c.netsplit(&sup_b);

    // B should be suspected
    assert!(sup_a.wait_for_suspect(&sup_b));
    assert!(sup_c.wait_for_suspect(&sup_b));

    // Rejoin and confirm b is alive
    sup_a.netjoin(&sup_b);
    sup_c.netjoin(&sup_b);

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_c.wait_for_alive(&sup_b));

    // Validate that the incarnation has increased
    assert_eq!(sup_b.incarnation(), 1);
}

// Ressurection and partition tolerance. Start a, and b; a and b are both permanent members. a
// splits from b; a is marked confirmed dead. The split is then joined, and both A and B update
// their incarnation and are then marked alive.
#[test]
fn ressurection_of_permanent_members() {
    setup::gpg_import();
    setup::simple_service_gossip();

    let mut sup_a = Supervisor::new_with_permanent();
    let sup_b = Supervisor::with_peer_permanent(&sup_a);

    // Make sure we are both alive
    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));

    // Split them apart
    sup_a.netsplit(&sup_b);

    // Confirm they are dead
    assert!(sup_a.wait_for_confirmed(&sup_b));
    assert!(sup_b.wait_for_confirmed(&sup_a));

    // Rejoin and confirm they are alive
    sup_a.netjoin(&sup_b);

    assert!(sup_a.wait_for_alive(&sup_b));
    assert!(sup_b.wait_for_alive(&sup_a));

    // Validate that the incarnation has increased
    assert_eq!(sup_a.incarnation(), 1);
    assert_eq!(sup_b.incarnation(), 1);
}
