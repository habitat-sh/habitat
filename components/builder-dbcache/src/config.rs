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

use std::net;

use num_cpus;

pub trait DataStoreCfg {
    fn default_connection_retry_ms() -> u64 {
        5_000
    }

    fn default_pool_size() -> u32 {
        (num_cpus::get() * 8) as u32
    }

    fn connection_retry_ms(&self) -> u64;
    fn datastore_addr(&self) -> &net::SocketAddrV4;
    fn pool_size(&self) -> u32;
}
