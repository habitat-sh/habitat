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

pub fn schedule(req: &mut Envelope, sock: &mut zmq::Socket, state: &mut ServerState) -> Result<()> {
    let msg: proto::Schedule = try!(req.parse_msg());
    println!("Schedule: {:?}", msg);

    let project_name = format!("{}/{}", msg.get_origin(), msg.get_package());
    let mut projects = Vec::new();
    projects.push(project_name);

    // TBD - project dependencies will be added to the projects list later

    let group;
    {
        let mut ds = state.datastore().write().unwrap();
        group = ds.create_group(projects)?;
    }

    try!(state.schedule_cli().notify_work());
    try!(req.reply_complete(sock, &group));
    Ok(())
}

pub fn schedule_get(req: &mut Envelope,
                    sock: &mut zmq::Socket,
                    state: &mut ServerState)
                    -> Result<()> {
    let msg: proto::ScheduleGet = try!(req.parse_msg());
    println!("ScheduleGet: {:?}", msg);

    let group_opt;
    {
        let ds = state.datastore().read().unwrap();
        group_opt = ds.get_group(msg.get_group_id());
    }

    match group_opt {
        Some(group) => {
            try!(req.reply_complete(sock, &group));
        }
        None => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "sc:schedule-get:1");
            try!(req.reply_complete(sock, &err));
        }
    }

    Ok(())
}
