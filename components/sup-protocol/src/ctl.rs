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

//! Specific request, responses, and types used to specifically communicate with the Supervisor's
//! Control Gateway.
//!
//! Note: See `protocols/ctl.proto` for type level documentation for generated types.

include!("generated/sup.ctl.rs");
include!("generated/sup.ctl.impl.rs");

use socket_addr_env_or_default;
use std::fmt;
use std::net::{Ipv4Addr, SocketAddr};

/// Default listening port for the CtlGateway listener.
pub const DEFAULT_PORT: u16 = 9632;

/// Default environment variable override for CtlGateway listener address.
pub const DEFAULT_ADDRESS_ENVVAR: &'static str = "HAB_LISTEN_CTL";

/// Return a SocketAddr with the default listening address and port.
pub fn default_addr() -> SocketAddr {
    socket_addr_env_or_default(
        DEFAULT_ADDRESS_ENVVAR,
        SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), DEFAULT_PORT)),
    )
}

impl fmt::Display for ConsoleLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.line)
    }
}
