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

extern crate hab;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_depot_client as depot_client;
extern crate habitat_http_client as http_client;
extern crate habitat_butterfly as butterfly;

extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate log;
extern crate pbr;
extern crate retry;
extern crate toml;
extern crate url;
// Temporary dependency for gossip/rumor injection code duplication.
extern crate utp;
extern crate uuid;
extern crate walkdir;

// pub use hab::cli;
pub use hab::config;
pub use hab::error;
pub use hab::analytics;

// pub mod analytics;
pub mod cli;
pub mod command;
// pub mod config;
// pub mod error;
// mod exec;
// pub mod gossip;

pub const PRODUCT: &'static str = "hab-butterfly";
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
