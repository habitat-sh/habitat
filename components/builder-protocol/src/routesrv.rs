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

pub use message::routesrv::*;
use message::Routable;

pub const DEFAULT_ROUTER_PORT: u16 = 5562;
pub const PING_INTERVAL_MS: i64 = 30_000;

impl Routable for Disconnect {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        None
    }
}

impl Routable for Heartbeat {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        None
    }
}

impl Routable for Registration {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        None
    }
}
