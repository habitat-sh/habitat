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

use std::fmt;
use std::net::{IpAddr, Ipv4Addr};

pub use core::config::ConfigFile;
pub use protocol::sharding::{ShardId, SHARD_COUNT};
use num_cpus;
use protocol::routesrv::DEFAULT_ROUTER_PORT;

use socket::ToAddrString;

/// Configuration structure for connecting to a Router
#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct RouterAddr {
    /// Listening address of command and heartbeat socket
    pub host: IpAddr,
    /// Listening port of command socket
    pub port: u16,
}

impl Default for RouterAddr {
    fn default() -> Self {
        RouterAddr {
            host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: DEFAULT_ROUTER_PORT,
        }
    }
}

impl ToAddrString for RouterAddr {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.host, self.port)
    }
}

impl fmt::Display for RouterAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Applied to back-end services connecting to RouteSrv.
pub trait AppCfg: Send + Sync {
    /// Default size of Dispatch worker pool.
    fn default_worker_count() -> usize {
        // JW TODO: increase default count after r2d2 connection pools are moved to be owned
        // by main thread of servers instead of dispatcher threads.
        // num_cpus::get() * 8
        num_cpus::get()
    }

    /// Return a list of router addresses.
    fn route_addrs(&self) -> &[RouterAddr];

    /// Return a list of shards which this service is hosting.
    ///
    /// A value of `None` indicates that this is not a sharded service.
    fn shards(&self) -> Option<&[ShardId]>;

    /// Count of Dispatch workers to start and supervise.
    fn worker_count(&self) -> usize {
        Self::default_worker_count()
    }
}
