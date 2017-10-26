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

extern crate base64;
extern crate bodyparser;
extern crate builder_core as bldr_core;
extern crate github_api_client;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as core;
extern crate habitat_net as hab_net;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate iron;
#[macro_use]
extern crate log;
extern crate mount;
extern crate num_cpus;
extern crate params;
extern crate persistent;
extern crate protobuf;
extern crate router;
extern crate segment_api_client;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate staticfile;
extern crate toml;
extern crate unicase;
extern crate urlencoded;
extern crate zmq;

pub mod app;
pub mod config;
pub mod conn;
pub mod http;

pub use app::start;
