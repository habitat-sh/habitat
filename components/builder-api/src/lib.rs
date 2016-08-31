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

extern crate bodyparser;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as hab_core;
extern crate habitat_depot as depot;
extern crate habitat_net as hab_net;
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mount;
extern crate persistent;
extern crate protobuf;
extern crate redis;
#[macro_use]
extern crate router;
extern crate rustc_serialize;
extern crate serde_json;
extern crate staticfile;
extern crate toml;
extern crate unicase;
extern crate zmq;

pub mod config;
pub mod error;
pub mod http;
pub mod server;

pub use self::config::Config;
pub use self::error::{Error, Result};
