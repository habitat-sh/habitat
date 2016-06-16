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

pub trait ToAddrString {
    fn to_addr_string(&self) -> String;
}

impl ToAddrString for net::SocketAddrV4 {
    fn to_addr_string(&self) -> String {
        format!("tcp://{}:{}", self.ip(), self.port())
    }
}

pub trait GitHubOAuth {
    fn github_url(&self) -> &str;
    fn github_client_id(&self) -> &str;
    fn github_client_secret(&self) -> &str;
}

pub trait RouteAddrs {
    fn route_addrs(&self) -> &Vec<net::SocketAddrV4>;

    fn heartbeat_port(&self) -> u16 {
        5563
    }
}

pub trait Shards {
    fn shards(&self) -> &Vec<u32>;
}
