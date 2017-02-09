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

//! A collection of handlers for the Scheduler dispatcher

use hab_net::server::Envelope;
use protocol::net::{self, ErrCode};
use protocol::scheduler as proto;
use zmq;

use super::ServerState;
use error::Result;

// TBD: This is currently a stub, to be fleshed out later.
pub fn schedule(req: &mut Envelope, sock: &mut zmq::Socket, state: &mut ServerState) -> Result<()> {
    let msg: proto::Schedule = try!(req.parse_msg());
    println!("Scheduling: {:?}", msg);

    let mut group = proto::Group::new();
    group.set_group_id(0);
    group.set_state(proto::GroupState::Pending);

    try!(req.reply_complete(sock, &group));
    Ok(())
}
