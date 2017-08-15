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
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_net as hab_net;
extern crate habitat_builder_db as db;
extern crate builder_core as bldr_core;
extern crate habitat_depot_client as depot_client;
extern crate habitat_api_client as api_client;
extern crate linked_hash_map;
#[macro_use]
extern crate log;
extern crate rand;
extern crate time;
extern crate postgres;
extern crate protobuf;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate zmq;
extern crate chrono;
extern crate hyper;

pub mod config;
pub mod data_store;
pub mod error;
pub mod server;
pub mod migration;

pub use self::config::Config;
pub use self::error::{Error, Result};

pub const PRODUCT: &'static str = "builder-scheduler";
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
