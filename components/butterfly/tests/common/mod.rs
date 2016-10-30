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

#[macro_use]
pub mod net;

use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use habitat_butterfly::server::Server;
use habitat_butterfly::member::Member;
use habitat_butterfly::trace::Trace;
use habitat_butterfly::server::timing::Timing;
use habitat_core::crypto::keys::sym_key::SymKey;

static SERVER_PORT: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn start_server(name: &str, ring_key: Option<SymKey>) -> Server {
    SERVER_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
    let swim_port = SERVER_PORT.fetch_add(1, Ordering::Relaxed);
    let gossip_port = SERVER_PORT.fetch_add(1, Ordering::Relaxed);
    let listen_swim = format!("127.0.0.1:{}", swim_port);
    let listen_gossip = format!("127.0.0.1:{}", gossip_port);
    let mut member = Member::new();
    member.set_swim_port(swim_port as i32);
    member.set_gossip_port(gossip_port as i32);
    let server = Server::new(&listen_swim[..],
                             &listen_gossip[..],
                             member,
                             Trace::default(),
                             ring_key,
                             Some(String::from(name)))
        .unwrap();
    server.start(Timing::default()).expect("Cannot start server");
    server
}

pub fn member_from_server(server: &Server) -> Member {
    let mut new_member = Member::new();
    let server_member = server.member.read().expect("Member lock is poisoned");
    new_member.set_id(String::from(server_member.get_id()));
    new_member.set_incarnation(server_member.get_incarnation());
    new_member.set_address(String::from("127.0.0.1"));
    new_member.set_swim_port(server.swim_port() as i32);
    new_member.set_gossip_port(server.gossip_port() as i32);
    new_member
}
