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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate bitflags;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_core as core;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate protobuf;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate time as simple_time;
extern crate toml;
extern crate uuid;
extern crate zmq;

pub mod app;
pub mod conn;
pub mod error;
pub mod privilege;
pub mod socket;
pub mod time;

pub use self::error::{ErrCode, NetError, NetOk, NetResult};
