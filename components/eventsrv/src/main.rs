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

extern crate habitat_core as core;
extern crate habitat_eventsrv as eventsrv;
extern crate log;
extern crate protobuf;

mod message;

use std::env;

use core::config::ConfigFile;
use eventsrv::config::Config;

fn main() {
    let config = if let Some(path) = env::args().nth(1) {
        Config::from_file(&path).unwrap_or(Config::default())
    } else {
        Config::default()
    };

    assert!(config.producer_port != config.consumer_port);

    println!("Producer port is {}", config.producer_port);
    println!("Consumer port is {}", config.consumer_port);
    println!("Starting proxy service...");

    eventsrv::proxy(config.producer_port, config.consumer_port);
}
