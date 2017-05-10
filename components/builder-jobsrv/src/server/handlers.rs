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

use hab_net::server::Envelope;
use protocol::net::{self, ErrCode};
use protocol::jobsrv as proto;
use zmq;
use protobuf::RepeatedField;
use super::ServerState;
use error::Result;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

pub fn job_create(req: &mut Envelope,
                  sock: &mut zmq::Socket,
                  state: &mut ServerState)
                  -> Result<()> {
    let msg: proto::JobSpec = try!(req.parse_msg());
    let mut job: proto::Job = msg.into();
    state.datastore().create_job(&mut job)?;
    debug!("Job created: id={} owner_id={} state={:?}",
           job.get_id(),
           job.get_owner_id(),
           job.get_state());
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

pub fn project_jobs_get(req: &mut Envelope,
                        sock: &mut zmq::Socket,
                        state: &mut ServerState)
                        -> Result<()> {
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

pub fn job_log_get(req: &mut Envelope,
                   sock: &mut zmq::Socket,
                   state: &mut ServerState)
                   -> Result<()> {

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

    // If the job has a log URL set, then we need to retrieve the
    // contents from that location. (once we start shipping them
    // somewhere else; for now we're just keeping the files on the
    // filesystem and setting a file:// URL).
    //
    // If it does NOT have a log URL set, we operate under the
    // assumption that the build could be running right now and thus
    // log output is accumulating in the filesystem. We try to read
    // the file; if it's not there then we just don't have any data.
    //
    // Once we start deleting the local files, there's a possibility
    // for a race condition here (we backup and delete the local file
    // before we can access it). We could potentially deal with that
    // by delaying the deletion of the local files by some small
    // amount of time.

    let start = msg.get_start();
    let file = state.log_dir().log_file_path(msg.get_id());

    match get_log_content(&file, start) {
        Some(content) => {
            let num_lines = content.len() as u64;
            let mut log = proto::JobLog::new();
            log.set_start(start);
            log.set_content(RepeatedField::from_vec(content));
            log.set_stop(start + num_lines);
            log.set_is_complete(job.has_log_url());
            req.reply_complete(sock, &log)?;
        }
        None => {
            // The job exists, but there are no logs (either yet, or
            // ever).
            let err = net::err(ErrCode::ENTITY_NOT_FOUND, "jb:job-log-get:3");
            req.reply_complete(sock, &err)?;
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
