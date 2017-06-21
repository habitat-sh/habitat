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

//! A collection of handlers for the JobSrv dispatcher

use error::{Error, Result};
use hab_net::server::Envelope;
use protobuf::RepeatedField;
use protocol::jobsrv as proto;
use protocol::net::{self, ErrCode};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use super::ServerState;
use zmq;

pub fn job_create(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::JobSpec = try!(req.parse_msg());
    let mut job: proto::Job = msg.into();
    state.datastore().create_job(&mut job)?;
    debug!(
        "Job created: id={} owner_id={} state={:?}",
        job.get_id(),
        job.get_owner_id(),
        job.get_state()
    );
    try!(state.worker_mgr().notify_work());
    try!(req.reply_complete(sock, &job));
    Ok(())
}

pub fn job_get(req: &mut Envelope, sock: &mut zmq::Socket, state: &mut ServerState) -> Result<()> {
    let msg: proto::JobGet = try!(req.parse_msg());
    match state.datastore().get_job(&msg) {
        Ok(Some(ref job)) => {
            //let reply: proto::Job = job.into();
            try!(req.reply_complete(sock, job));
        }
        Ok(None) => {
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "jb:job-get:1");
            try!(req.reply_complete(sock, &err));
        }
        Err(e) => {
            error!("datastore error, err={:?}", e);
            let err = net::err(ErrCode::DATA_STORE, "jb:job-get:2");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())
}

pub fn project_jobs_get(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {
    let msg: proto::ProjectJobsGet = try!(req.parse_msg());
    match state.datastore().get_jobs_for_project(&msg) {
        Ok(ref jobs) => {
            // NOTE: Currently no difference between "project has no jobs" and "no
            // such project"
            try!(req.reply_complete(sock, jobs));
        }
        Err(e) => {
            error!("datastore error, err={:?}", e);
            let err = net::err(ErrCode::DATA_STORE, "jb:project-jobs-get:2");
            try!(req.reply_complete(sock, &err));
        }
    }
    Ok(())

}

pub fn job_log_get(
    req: &mut Envelope,
    sock: &mut zmq::Socket,
    state: &mut ServerState,
) -> Result<()> {

    let msg: proto::JobLogGet = req.parse_msg()?;

    // Confirm job exists; if not, 404
    let mut get = proto::JobGet::new();
    get.set_id(msg.get_id());
    let job = match state.datastore().get_job(&get) {
        Ok(Some(job)) => job,
        Ok(None) => {
            // No such job!
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:1");
            req.reply_complete(sock, &err)?;
            return Ok(()); // early return
        }
        Err(e) => {
            error!("datastore error, err={:?}", e);
            let err = net::err(ErrCode::DATA_STORE, "jb:job-log-get:2");
            req.reply_complete(sock, &err)?;
            return Ok(()); // early return
        }
    };

    if job.get_is_archived() {
        match state.archiver().retrieve(job.get_id()) {
            Ok(lines) => {
                let start = msg.get_start();
                let num_lines = lines.len() as u64;
                let segment;

                if start > num_lines - 1 {
                    segment = vec![];
                } else {
                    segment = lines[start as usize..].to_vec();
                }

                let mut log = proto::JobLog::new();
                let log_content = RepeatedField::from_vec(segment);

                log.set_start(start);
                log.set_stop(num_lines);
                log.set_is_complete(true); // by definition
                log.set_content(log_content);
                req.reply_complete(sock, &log)?;
            }
            Err(e @ Error::CaughtPanic(_, _)) => {
                // Generally, this happens when the archiver can't
                // reach it's S3 object store
                warn!("Error retrieving log: {}", e);

                // TODO: Need to return a different error here... it's
                // not quite ENTITY_NOT_FOUND
                let err = net::err(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:5");
                req.reply_complete(sock, &err)?;
            }
            Err(_) => {
                let err = net::err(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:4");
                req.reply_complete(sock, &err)?;
            }
        }
    } else {
        // retrieve fragment from on-disk file
        let start = msg.get_start();
        let file = state.log_dir().log_file_path(msg.get_id());

        match get_log_content(&file, start) {
            Some(content) => {
                let num_lines = content.len() as u64;
                let mut log = proto::JobLog::new();
                log.set_start(start);
                log.set_content(RepeatedField::from_vec(content));
                log.set_stop(start + num_lines);
                log.set_is_complete(false);
                req.reply_complete(sock, &log)?;
            }
            None => {
                // The job exists, but there are no logs (either yet, or
                // ever).
                let err = net::err(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:3");
                req.reply_complete(sock, &err)?;
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

    let open = OpenOptions::new().read(true).open(log_file);

    match open {
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
