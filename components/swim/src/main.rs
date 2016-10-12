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

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate habitat_swim;

use std::env;
use std::thread;
use std::time::Duration;
use std::net::SocketAddr;

use habitat_swim::{server, member, trace};

fn main() {
    env_logger::init().unwrap();
    let mut args = env::args();
    let _ = args.next();

    let bind_to = args.next().unwrap();
    println!("Binding to {}", bind_to);

    let server = server::Server::new(&bind_to[..], member::Member::new(), trace::Trace::default())
        .unwrap();
    println!("Server ID: {}", server.member.read().unwrap().get_id());

    server.start(server::outbound::Timing::default()).expect("Cannot start server");

    let targets: Vec<String> = args.collect();
    for target in &targets {
        let addr: SocketAddr = target.parse().unwrap();
        let member = member::Member::from(addr);
        server::outbound::ping(&server, &member, addr, None);
    }
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}
