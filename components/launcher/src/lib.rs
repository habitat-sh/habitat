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

extern crate ansi_term;
#[macro_use]
extern crate habitat_core as core;
extern crate habitat_launcher_protocol as protocol;
extern crate ipc_channel;
extern crate libc;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate semver;
extern crate time;
#[cfg(windows)]
extern crate winapi;

pub mod error;
pub mod server;
pub mod service;
mod sys;

pub const SUP_CMD: &'static str = "hab-sup";
pub const SUP_PACKAGE_IDENT: &'static str = "core/hab-sup";
