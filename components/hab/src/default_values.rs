// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! This file contains the string defaults values as well as environment variable strings
//! for use in the clap layer of the application. This is not the final form for defaults.
//! Eventually this will likely reside in it's own crate and be composed of fully typed
//! default values. But as a first step we need a spot to consolidate those values and help simplify
//! some of the logic around them.

pub const GOSSIP_DEFAULT_IP: &'static str = "0.0.0.0";
pub const GOSSIP_DEFAULT_PORT: u16 = 9638;
lazy_static! {
    pub static ref GOSSIP_DEFAULT_ADDR: String =
        { format!("{}:{}", GOSSIP_DEFAULT_IP, GOSSIP_DEFAULT_PORT) };
}
pub const GOSSIP_LISTEN_ADDRESS_ENVVAR: &'static str = "HAB_LISTEN_GOSSIP";
pub const RING_ENVVAR: &'static str = "HAB_RING";
pub const RING_KEY_ENVVAR: &'static str = "HAB_RING_KEY";
