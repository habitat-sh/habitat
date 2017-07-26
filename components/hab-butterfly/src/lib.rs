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

extern crate hab;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_butterfly as butterfly;

#[macro_use]
extern crate clap;
extern crate log;
extern crate toml;

pub use hab::config;
pub use hab::error;
pub use hab::analytics;

pub mod cli;
pub mod command;

pub const PRODUCT: &'static str = "hab-butterfly";
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
