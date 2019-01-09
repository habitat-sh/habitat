// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use clap;
use habitat_core as core;
use habitat_eventsrv as eventsrv;

use crate::core::config::ConfigFile;
use crate::eventsrv::config::Config;

fn main() {
    let matches = app().get_matches();
    let config = match matches.value_of("config") {
        Some(cfg) => Config::from_file(&cfg).unwrap_or(Config::default()),
        None => Config::default(),
    };

    assert!(config.producer_port != config.consumer_port);

    println!("Producer port is {}", config.producer_port);
    println!("Consumer port is {}", config.consumer_port);
    println!("Starting proxy service...");

    eventsrv::proxy(config.producer_port, config.consumer_port);
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap::App::new("Habitat EventSrv")
        .author("The Habitat Maintainers <humans@habitat.sh>")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Filepath to configuration file")
                .takes_value(true),
        )
}
