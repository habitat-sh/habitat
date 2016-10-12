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

use habitat_swim::server::Server;
use habitat_swim::member::Member;
use habitat_swim::trace::Trace;
use habitat_swim::server::outbound::Timing;

static SERVER_PORT: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn start_server(name: &str) -> Server {
    SERVER_PORT.compare_and_swap(0, 6666, Ordering::Relaxed);
    let my_port = SERVER_PORT.fetch_add(1, Ordering::Relaxed);
    let listen = format!("127.0.0.1:{}", my_port);
    let server = Server::new(&listen[..], Member::new(), Trace::default()).unwrap();
    {
        let mut server_name = server.name.write().expect("Server name lock is poisoned");
        server_name.clear();
        server_name.push_str(name);
    }
    server.start(Timing::default());
    server
}

pub fn member_from_server(server: &Server) -> Member {
    let mut new_member = Member::new();
    let server_member = server.member.read().expect("Member lock is poisoned");
    new_member.set_id(String::from(server_member.get_id()));
    new_member.set_incarnation(server_member.get_incarnation());
    new_member.set_address(format!("127.0.0.1:{}", server.port()));
    new_member
}
