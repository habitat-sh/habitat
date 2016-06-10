// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::net;

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
