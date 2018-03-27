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

#![recursion_limit="128"]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_depot_client as depot_client;
extern crate habitat_api_client as api_client;
extern crate habitat_http_client as http_client;
extern crate handlebars;

extern crate ansi_term;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate features;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate pbr;
extern crate regex;
extern crate retry;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate url;
extern crate uuid;
extern crate walkdir;
extern crate base64;
#[cfg(test)]
extern crate tempdir;

pub mod analytics;
pub mod cli;
pub mod command;
pub mod config;
pub mod error;
pub mod scaffolding;
mod exec;

pub const PRODUCT: &'static str = "hab";
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const ORIGIN_ENVVAR: &'static str = "HAB_ORIGIN";

pub use hcore::AUTH_TOKEN_ENVVAR;

features! {
    pub mod feat {
        const List = 0b00000001,
        const OfflineInstall = 0b00000010
    }
}
