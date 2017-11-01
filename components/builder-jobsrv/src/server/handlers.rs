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

//! A collection of handlers for the JobSrv dispatcher

use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use hab_net::app::prelude::*;
use protobuf::RepeatedField;
use protocol::jobsrv;
use protocol::net::{self, ErrCode};

use super::ServerState;
use error::{Error, Result};
use time::PreciseTime;

pub fn job_create(req: &mut Message, conn: &mut RouteConn, state: &mut ServerState) -> Result<()> {
    let msg = req.parse::<jobsrv::JobSpec>()?;
    let mut job: jobsrv::Job = msg.into();
    let created_job = state.datastore.create_job(&mut job)?;
    debug!(
        "Job created: id={} owner_id={} state={:?}",
        created_job.get_id(),
        created_job.get_owner_id(),
        created_job.get_state()
    );
    state.worker_mgr.notify_work()?;
    conn.route_reply(req, &created_job)?;
    Ok(())
}

pub fn job_get(req: &mut Message, conn: &mut RouteConn, state: &mut ServerState) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGet>()?;
    match state.datastore.get_job(&msg) {
        Ok(Some(ref job)) => conn.route_reply(req, job)?,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-get:1");
            conn.route_reply(req, &*err)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "jb:job-get:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn project_jobs_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::ProjectJobsGet>()?;
    match state.datastore.get_jobs_for_project(&msg) {
        Ok(ref jobs) => {
            // NOTE: Currently no difference between "project has no jobs" and "no
            // such project"
            conn.route_reply(req, jobs)?;
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "jb:project-jobs-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn job_log_get(req: &mut Message, conn: &mut RouteConn, state: &mut ServerState) -> Result<()> {
    let msg = req.parse::<jobsrv::JobLogGet>()?;
    let mut get = jobsrv::JobGet::new();
    get.set_id(msg.get_id());
    let job = match state.datastore.get_job(&get) {
        Ok(Some(job)) => job,
        Ok(None) => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:1");
            conn.route_reply(req, &*err)?;
            return Ok(());
        }
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "jb:job-log-get:2");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
            return Ok(());
        }
    };

    if job.get_is_archived() {
        match state.archiver.retrieve(job.get_id()) {
            Ok(lines) => {
                let start = msg.get_start();
                let num_lines = lines.len() as u64;
                let segment;

                if start > num_lines - 1 {
                    segment = vec![];
                } else {
                    segment = lines[start as usize..].to_vec();
                }

                let mut log = jobsrv::JobLog::new();
                let log_content = RepeatedField::from_vec(segment);

                log.set_start(start);
                log.set_stop(num_lines);
                log.set_is_complete(true); // by definition
                log.set_content(log_content);
                conn.route_reply(req, &log)?;
            }
            Err(e @ Error::CaughtPanic(_, _)) => {
                // Generally, this happens when the archiver can't
                // reach it's S3 object store
                warn!("Error retrieving log: {}", e);

                // TODO: Need to return a different error here... it's
                // not quite ENTITY_NOT_FOUND
                let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:5");
                conn.route_reply(req, &*err)?;
            }
            Err(_) => {
                let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:4");
                conn.route_reply(req, &*err)?;
            }
        }
    } else {
        // retrieve fragment from on-disk file
        let start = msg.get_start();
        let file = state.log_dir.log_file_path(msg.get_id());

        match get_log_content(&file, start) {
            Some(content) => {
                let num_lines = content.len() as u64;
                let mut log = jobsrv::JobLog::new();
                log.set_start(start);
                log.set_content(RepeatedField::from_vec(content));
                log.set_stop(start + num_lines);
                log.set_is_complete(false);
                conn.route_reply(req, &log)?;
            }
            None => {
                // The job exists, but there are no logs (either yet, or ever).
                let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:3");
                conn.route_reply(req, &*err)?;
            }
        }
    }
    Ok(())
}

/// Returns the lines of the log file past `offset`.
///
/// If the file does not exist, `None` is returned; this could be
/// because there is not yet any log information for the job, or the
/// job never had any log information (e.g., it predates this
/// feature).
fn get_log_content(log_file: &PathBuf, offset: u64) -> Option<Vec<String>> {
    match OpenOptions::new().read(true).open(log_file) {
        Ok(file) => {
            let lines = BufReader::new(file)
                .lines()
                .skip(offset as usize)
                .map(|l| l.expect("Could not parse line"))
                .collect();
            Some(lines)
        }
        Err(e) => {
            warn!("Couldn't open log file {:?}: {:?}", log_file, e);
            None
        }
    }
}

// TODO (SA): This is an experimental dev-only function for now
pub fn job_group_abort(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGroupAbort>()?;
    debug!("job_group_abort message: {:?}", msg);

    match state.datastore.abort_job_group(&msg) {
        Ok(()) => {
            warn!("Job Group {} aborted", msg.get_group_id());
            conn.route_reply(req, &net::NetOk::new())?
        }
        Err(err) => {
            warn!(
                "Unable to abort job group {}, err: {:?}",
                msg.get_group_id(),
                err
            );
            let err = NetError::new(ErrCode::DATA_STORE, "jb:job-group-abort:1");
            conn.route_reply(req, &*err)?;
        }
    };

    Ok(())
}

pub fn job_group_cancel(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGroupCancel>()?;
    debug!("job_group_cancel message: {:?}", msg);

    // Get the job group
    let mut jgc = jobsrv::JobGroupGet::new();
    jgc.set_group_id(msg.get_group_id());

    let group = match state.datastore.get_job_group(&jgc) {
        Ok(group_opt) => {
            match group_opt {
                Some(group) => group,
                None => {
                    let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-group-cancel:1");
                    conn.route_reply(req, &*err)?;
                    return Ok(());
                }
            }
        }
        Err(err) => {
            warn!(
                "Failed to get group {} from datastore: {:?}",
                msg.get_group_id(),
                err
            );
            let err = NetError::new(ErrCode::DATA_STORE, "jb:job-group-cancel:2");
            conn.route_reply(req, &*err)?;
            return Ok(());
        }
    };

    // Set the Group and NotStarted projects to Cancelled
    // TODO (SA): Make the state change code below a single DB call

    state.datastore.cancel_job_group(group.get_id())?;

    // Set all the InProgress projects jobs to CancelPending
    for project in group.get_projects().iter().filter(|&ref p| {
        p.get_state() == jobsrv::JobGroupProjectState::InProgress
    })
    {
        let job_id = project.get_job_id();
        let mut req = jobsrv::JobGet::new();
        req.set_id(job_id);

        match state.datastore.get_job(&req)? {
            Some(mut job) => {
                debug!("Canceling job {:?}", job_id);
                job.set_state(jobsrv::JobState::CancelPending);
                state.datastore.update_job(&job)?;
            }
            None => {
                warn!(
                    "Unable to cancel job {:?} (not found)",
                    job_id,
                );
            }
        }
    }

    state.worker_mgr.notify_work()?;
    conn.route_reply(req, &net::NetOk::new())?;
    Ok(())
}

pub fn job_group_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGroupSpec>()?;
    debug!("job_group_create message: {:?}", msg);

    let project_name = format!("{}/{}", msg.get_origin(), msg.get_package());
    let mut projects = Vec::new();

    // Get the ident for the root package
    let mut start_time;
    let mut end_time;

    let project_ident = {
        let mut target_graph = state.graph.write().unwrap();
        let graph = match target_graph.graph_mut(msg.get_target()) {
            Some(g) => g,
            None => {
                warn!(
                    "JobGroupSpec, no graph found for target {}",
                    msg.get_target()
                );
                let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-group-create:2");
                conn.route_reply(req, &*err)?;
                return Ok(());
            }
        };

        start_time = PreciseTime::now();
        let ret = match graph.resolve(&project_name) {
            Some(s) => s,
            None => {
                warn!("JobGroupSpec, project ident not found for {}", project_name);
                // If a package has never been uploaded, we won't see it in the graph
                // Carry on with stiff upper lip
                String::from("")
            }
        };
        end_time = PreciseTime::now();
        ret
    };
    debug!("Resolved project name: {} sec\n", start_time.to(end_time));

    // Add the root package if needed
    if !msg.get_deps_only() || msg.get_package_only() {
        projects.push((project_name.clone(), project_ident.clone()));
    }

    // Search the packages graph to find the reverse dependencies
    if !msg.get_package_only() {
        let rdeps_opt = {
            let target_graph = state.graph.read().unwrap();
            let graph = target_graph.graph(msg.get_target()).unwrap(); // Unwrap OK
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

                    // If the origin_only flag is true, make sure the origin matches
                    if !msg.get_origin_only() || origin == msg.get_origin() {
                        debug!("Adding to projects: {} ({})", s.0, s.1);
                        projects.push(s.clone());
                    } else {
                        debug!("Skipping non-origin project: {} ({})", s.0, s.1);
                    }
                }
            }
            None => {
                debug!("Graph rdeps: no entries found");
            }
        }
    }

    let group = if projects.is_empty() {
        debug!("No projects need building - group is complete");

        let mut new_group = jobsrv::JobGroup::new();
        let projects = RepeatedField::new();
        new_group.set_id(0);
        new_group.set_state(jobsrv::JobGroupState::GroupComplete);
        new_group.set_projects(projects);
        new_group
    } else {
        // If already have a queued job group (queue length: 1 per project),
        // then return that group, else create a new job group
        // TODO (SA) - update the group's projects instead of just returning the group
        let new_group = match state.datastore.get_queued_job_group(&project_name)? {
            Some(group) => {
                debug!("JobGroupSpec, project {} is already queued", project_name);
                group
            }
            None => state.datastore.create_job_group(&msg, projects)?,
        };
        state.schedule_cli.notify()?;
        new_group
    };

    conn.route_reply(req, &group)?;
    Ok(())
}

pub fn job_graph_package_reverse_dependencies_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGraphPackageReverseDependenciesGet>()?;
    debug!("reverse_dependencies_get message: {:?}", msg);

    let ident = format!("{}/{}", msg.get_origin(), msg.get_name());
    let target_graph = state.graph.read().expect("Graph lock is poisoned");
    let graph = match target_graph.graph(msg.get_target()) {
        Some(g) => g,
        None => {
            warn!(
                "JobGraphPackageReverseDependenciesGet, no graph found for target {}",
                msg.get_target()
            );
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:reverse-dependencies-get:1");
            conn.route_reply(req, &*err)?;
            return Ok(());
        }
    };

    let rdeps = graph.rdeps(&ident);
    let mut rd_reply = jobsrv::JobGraphPackageReverseDependencies::new();
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

    conn.route_reply(req, &rd_reply)?;

    Ok(())
}

pub fn job_group_origin_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<proto::JobGroupOriginGet>()?;

    match state.datastore.get_job_group_origin(&msg) {
        Ok(ref jgor) => conn.route_reply(req, jgor)?,
        Err(e) => {
            let err = NetError::new(ErrCode::DATA_STORE, "jb:job-group-origin-get:1");
            error!("{}, {}", err, e);
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn job_group_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGroupGet>()?;
    debug!("group_get message: {:?}", msg);

    let group_opt = match state.datastore.get_job_group(&msg) {
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
            conn.route_reply(req, &group)?;
        }
        None => {
            let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-group-get:1");
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}

pub fn job_graph_package_create(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGraphPackageCreate>()?;
    debug!("job_graph_package_create message: {:?}", msg);
    let package = state.datastore.create_job_graph_package(&msg)?;

    // Extend the graph with new package
    {
        let mut target_graph = state.graph.write().unwrap();
        let graph = match target_graph.graph_mut(msg.get_target()) {
            Some(g) => g,
            None => {
                warn!(
                    "JobGraphPackageCreate, no graph found for target {}",
                    msg.get_target()
                );
                let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-graph-package-create:1");
                conn.route_reply(req, &*err)?;
                return Ok(());
            }
        };

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

    conn.route_reply(req, &package)?;
    Ok(())
}

pub fn job_graph_package_precreate(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGraphPackagePreCreate>()?;
    debug!("package_precreate message: {:?}", msg);
    let package: jobsrv::JobGraphPackage = msg.into();

    // Check that we can safely extend the graph with new package
    let can_extend = {
        let mut target_graph = state.graph.write().unwrap();
        let graph = match target_graph.graph_mut(package.get_target()) {
            Some(g) => g,
            None => {
                warn!(
                    "JobGraphPackagePreCreate, no graph found for target {}",
                    package.get_target()
                );
                let err = NetError::new(ErrCode::ENTITY_NOT_FOUND, "jb:job-graph-package-pc:1");
                conn.route_reply(req, &*err)?;
                return Ok(());
            }
        };

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
        conn.route_reply(req, &net::NetOk::new())?
    } else {
        let err = NetError::new(ErrCode::ENTITY_CONFLICT, "jb:job-graph-package-pc:2");
        conn.route_reply(req, &*err)?;
    }
    Ok(())
}

pub fn job_graph_package_stats_get(
    req: &mut Message,
    conn: &mut RouteConn,
    state: &mut ServerState,
) -> Result<()> {
    let msg = req.parse::<jobsrv::JobGraphPackageStatsGet>()?;
    debug!("package_stats_get message: {:?}", msg);

    match state.datastore.get_job_graph_package_stats(&msg) {
        Ok(package_stats) => conn.route_reply(req, &package_stats)?,
        Err(err) => {
            warn!(
                "Unable to retrieve package stats for {}, err: {:?}",
                msg.get_origin(),
                err
            );
            let err = NetError::new(
                ErrCode::ENTITY_NOT_FOUND,
                "jb:job-graph-package-stats-get:1",
            );
            conn.route_reply(req, &*err)?;
        }
    }
    Ok(())
}
