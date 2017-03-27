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

use time::PreciseTime;
use hab_net::server::Envelope;
use protocol::net::{self, ErrCode};
use protocol::scheduler as proto;
use zmq;

use super::ServerState;
use error::Result;

pub fn group_create(req: &mut Envelope,
                    sock: &mut zmq::Socket,
                    state: &mut ServerState)
                    -> Result<()> {
    let msg: proto::GroupCreate = try!(req.parse_msg());
    println!("group_create message: {:?}", msg);

    let project_name = format!("{}/{}", msg.get_origin(), msg.get_package());
    let mut projects: Vec<String> = Vec::new();
    projects.push(project_name.clone());

    // Search the packages graph to find the reverse dependencies
    let start_time;
    let end_time;

    let rdeps_opt = {
        let graph = state.graph().read().unwrap();
        start_time = PreciseTime::now();
        let ret = match graph.resolve(&project_name) {
            Some(s) => graph.rdeps(&s),
            None => None,
        };
        end_time = PreciseTime::now();
        ret
    };

    match rdeps_opt {
        Some(rdeps) => {
            println!("Graph rdeps: {} items ({} sec)\n",
                     rdeps.len(),
                     start_time.to(end_time));

            for s in rdeps {
                println!("Adding to projects: {}", s);
                projects.push(s);
            }
        }
        None => {
            println!("Graph rdeps: no entries found");
        }
    }

    let group = state.datastore().create_group(&msg, projects)?;

    try!(state.schedule_cli().notify_work());
    try!(req.reply_complete(sock, &group));
    Ok(())
}

pub fn group_get(req: &mut Envelope,
                 sock: &mut zmq::Socket,
                 state: &mut ServerState)
                 -> Result<()> {
    let msg: proto::GroupGet = try!(req.parse_msg());
    println!("group_get message: {:?}", msg);

    let group_opt = match state.datastore().get_group(&msg) {
        Ok(group_opt) => group_opt,
        Err(err) => {
            error!("Unable to retrieve group {}, err: {:?}",
                   msg.get_group_id(),
                   err);
            None
        }
    };

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
