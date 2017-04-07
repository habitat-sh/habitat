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

extern crate habitat_eventsrv;
extern crate log;
extern crate protobuf;

mod message;

use std::env;

fn main() {
    let mut args: Vec<_> = env::args().collect();

    let port1 = args.remove(1);
    let frontend_port: i32 = port1.parse().unwrap();

    let port2 = args.remove(1);
    let backend_port: i32 = port2.parse().unwrap();

    assert!(frontend_port != backend_port);

    println!("Frontend port is {}", frontend_port);
    println!("Backend port is {}", backend_port);
    println!("Starting proxy service...");

    habitat_eventsrv::proxy(frontend_port, backend_port);
}
