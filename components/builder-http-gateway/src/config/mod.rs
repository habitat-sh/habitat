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

pub mod prelude;

use std::net::IpAddr;

use hab_net::app::config::RouterAddr;
use num_cpus;

pub trait GatewayCfg {
    /// Default number of worker threads to simultaneously handle HTTP requests.
    fn default_handler_count() -> usize {
        num_cpus::get() * 8
    }

    /// Number of worker threads to simultaneously handle HTTP requests.
    fn handler_count(&self) -> usize {
        Self::default_handler_count()
    }

    fn listen_addr(&self) -> &IpAddr;

    fn listen_port(&self) -> u16;

    /// Return a list of router addresses
    fn route_addrs(&self) -> &[RouterAddr];
}
