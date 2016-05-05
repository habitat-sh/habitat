// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::net;

pub trait RouteAddrs {
    fn route_addrs(&self) -> &Vec<net::SocketAddrV4>;

    fn heartbeat_port(&self) -> u16 {
        5563
    }
}

pub trait Shards {
    fn shards(&self) -> &Vec<u32>;
}
