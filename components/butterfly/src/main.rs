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
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate env_logger;
extern crate habitat_butterfly;
extern crate habitat_core;
extern crate log;

use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use habitat_butterfly::network::RealNetwork;
use habitat_butterfly::server::Suitability;
use habitat_butterfly::{member, server, trace};
use habitat_core::service::ServiceGroup;

#[derive(Debug)]
struct ZeroSuitability;
impl Suitability for ZeroSuitability {
    fn get(&self, _service_group: &ServiceGroup) -> u64 {
        0
    }
}

fn main() {
    env_logger::init();
    let mut args = env::args();
    let _ = args.next();

    let bind_to = args.next().unwrap();
    println!("Binding to {}", bind_to);
    println!("Starting test butterfly");

    let bind_to_addr = bind_to.parse::<SocketAddr>().unwrap();
    let bind_port = bind_to_addr.port();
    let mut gossip_bind_addr = bind_to_addr.clone();
    let gport = bind_port + 1;
    gossip_bind_addr.set_port(gport);

    let mut member = member::Member::default();
    member.swim_port = bind_port;
    member.gossip_port = gport;

    let network = RealNetwork::new_for_server(bind_to_addr, gossip_bind_addr);
    let mut server = server::Server::<RealNetwork>::new(
        network,
        member,
        trace::Trace::default(),
        None,
        None,
        None::<PathBuf>,
        Box::new(ZeroSuitability),
    );
    println!("Server ID: {}", server.member_id());

    let targets: Vec<String> = args.collect();
    for target in &targets {
        let addr: SocketAddr = target.parse().unwrap();
        let mut member = member::Member::default();
        member.address = format!("{}", addr.ip());
        member.swim_port = addr.port();
        member.gossip_port = addr.port();
        server.member_list.add_initial_member(member);
    }

    server
        .start(server::timing::Timing::default())
        .expect("Cannot start server");
    loop {
        println!("{:#?}", server.member_list);
        thread::sleep(Duration::from_millis(1000));
    }
}
