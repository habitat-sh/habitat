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
use protobuf::RepeatedField;
use zmq;

use super::ServerState;
use error::Result;

pub fn group_create(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::GroupCreate = req.parse_msg()?;
    debug!("group_create message: {:?}", msg);

    let project_name = format!("{}/{}", msg.get_origin(), msg.get_package());
    let mut projects = Vec::new();

    // Get the ident for the root package
    let mut start_time;
    let mut end_time;

    let project_ident = {
        let graph = state.graph().read().unwrap();
        start_time = PreciseTime::now();
        let ret = match graph.resolve(&project_name) {
            Some(s) => s,
            None => {
                warn!("GroupCreate, project ident not found");
                let err = net::err(ErrCode::ENTITY_NOT_FOUND, "sc:group-create:1");
                req.reply_complete(sock, &err)?;
                return Ok(());
            }
        };
        end_time = PreciseTime::now();
        ret
    };
    debug!("Resolved project name: {} sec\n", start_time.to(end_time));

    // Add the root package if needed
    if !msg.get_deps_only() {
        projects.push((project_name.clone(), project_ident.clone()));
    }

    // Search the packages graph to find the reverse dependencies
    let rdeps_opt = {
        let graph = state.graph().read().unwrap();
        start_time = PreciseTime::now();
        let ret = graph.rdeps(&project_name);
        end_time = PreciseTime::now();
        ret
    };

    match rdeps_opt {
        Some(rdeps) => {
            debug!(
                "Graph rdeps: {} items ({} sec)\n",
                rdeps.len(),
                start_time.to(end_time)
            );

            for s in rdeps {
                let origin = s.0.split("/").nth(0).unwrap();

                // We only build core packages for now
                if origin == "core" {
                    debug!("Adding to projects: {} ({})", s.0, s.1);
                    projects.push(s.clone());
                } else {
                    debug!("Skipping non-core project: {} ({})", s.0, s.1);
                }
            }
        }
        None => {
            debug!("Graph rdeps: no entries found");
        }
    }

    let group = if projects.is_empty() {
        debug!("No projects need building - group is complete");

        let mut new_group = proto::Group::new();
        let projects = RepeatedField::new();
        new_group.set_id(0);
        new_group.set_state(proto::GroupState::Complete);
        new_group.set_projects(projects);
        new_group
    } else {
        let new_group = state.datastore().create_group(&msg, projects)?;
        state.schedule_cli().notify_work()?;
        new_group
    };

    req.reply_complete(sock, &group)?;
    Ok(())
}

pub fn reverse_dependencies_get(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::ReverseDependenciesGet = req.parse_msg()?;
    debug!("reverse_dependencies_get message: {:?}", msg);

    let ident = format!("{}/{}", msg.get_origin(), msg.get_name());
    let graph = state.graph().read().expect("Graph lock is poisoned");
    let rdeps = graph.rdeps(&ident);
    let mut rd_reply = proto::ReverseDependencies::new();
    rd_reply.set_origin(msg.get_origin().to_string());
    rd_reply.set_name(msg.get_name().to_string());

    match rdeps {
        Some(rd) => {
            let mut short_deps = RepeatedField::new();

            // the tuples inside rd are of the form: (core/redis, core/redis/3.2.4/20170717232232)
            // we're only interested in the short form, not the fully qualified form
            for (id, _fully_qualified_id) in rd {
                short_deps.push(id);
            }

            short_deps.sort();
            rd_reply.set_rdeps(short_deps);
        }
        None => debug!("No rdeps found for {}", ident),
    }

    req.reply_complete(sock, &rd_reply)?;

    Ok(())
}

pub fn group_get(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::GroupGet = req.parse_msg()?;
    debug!("group_get message: {:?}", msg);

    let group_opt = match state.datastore().get_group(&msg) {
        Ok(group_opt) => group_opt,
        Err(err) => {
            warn!(
                "Unable to retrieve group {}, err: {:?}",
                msg.get_group_id(),
                err
            );
            None
        }
    };

    match group_opt {
        Some(group) => {
            req.reply_complete(sock, &group)?;
        }
        None => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "sc:schedule-get:1");
            req.reply_complete(sock, &err)?;
        }
    }

    Ok(())
}

pub fn package_create(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::PackageCreate = req.parse_msg()?;
    debug!("package_create message: {:?}", msg);

    let package = state.datastore().create_package(&msg)?;

    // Extend the graph with new package
    {
        let mut graph = state.graph().write().unwrap();
        let start_time = PreciseTime::now();
        let (ncount, ecount) = graph.extend(&package);
        let end_time = PreciseTime::now();

        debug!(
            "Extended graph, nodes: {}, edges: {} ({} sec)\n",
            ncount,
            ecount,
            start_time.to(end_time)
        );
    };

    req.reply_complete(sock, &package)?;
    Ok(())
}

pub fn package_precreate(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::PackagePreCreate = req.parse_msg()?;
    debug!("package_precreate message: {:?}", msg);

    let package: proto::Package = msg.into();

    // Check that we can safely extend the graph with new package
    let can_extend = {
        let mut graph = state.graph().write().unwrap();
        let start_time = PreciseTime::now();
        let ret = graph.check_extend(&package);
        let end_time = PreciseTime::now();

        debug!(
            "Graph pre-check: {} ({} sec)\n",
            ret,
            start_time.to(end_time)
        );

        ret
    };

    if can_extend {
        req.reply_complete(sock, &net::NetOk::new())?
    } else {
        let err = net::err(ErrCode::ENTITY_CONFLICT, "sc:schedule-pc:1");
        req.reply_complete(sock, &err)?;
    }

    Ok(())
}

pub fn job_status(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::JobStatus = req.parse_msg()?;
    debug!("job_status message: {:?}", msg);

    // TODO BUG: SA There is a potential race condition here where the job status can get lost
    // if the process goes away (for whatever reason) before the status gets processed by
    // the scheduler thread. We can fix it by persisting the status and then handing it
    // asynchronously, or by making the status update handling synchronous.
    state.schedule_cli().notify_status(&msg.get_job())?;

    req.reply_complete(sock, &msg)?;
    Ok(())
}

pub fn package_stats_get(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::PackageStatsGet = req.parse_msg()?;
    debug!("package_stats_get message: {:?}", msg);

    match state.datastore().get_package_stats(&msg) {
        Ok(package_stats) => req.reply_complete(sock, &package_stats)?,
        Err(err) => {
            warn!(
                "Unable to retrieve package stats for {}, err: {:?}",
                msg.get_origin(),
                err
            );
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "sc:package-stats-get:1");
            req.reply_complete(sock, &err)?;
        }
    };

    Ok(())
}
