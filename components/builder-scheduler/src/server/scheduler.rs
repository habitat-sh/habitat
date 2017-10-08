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

use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::collections::HashMap;

use hab_net::ErrCode;
use hab_net::conn::RouteClient;
use hab_net::socket::DEFAULT_CONTEXT;
use zmq;

use protocol::jobsrv::{self, Job, JobSpec};
use protocol::originsrv::*;
use protocol::scheduler as proto;
use data_store::DataStore;
use error::{SrvResult, SrvError};

use config::Config;
use bldr_core::logger::Logger;
use hab_core::channel::bldr_channel_name;

const SCHEDULER_ADDR: &'static str = "inproc://scheduler";
const SOCKET_TIMEOUT_MS: i64 = 60_000;

pub struct ScheduleClient {
    socket: zmq::Socket,
}

impl ScheduleClient {
    pub fn connect(&mut self) -> SrvResult<()> {
        self.socket.connect(SCHEDULER_ADDR)?;
        Ok(())
    }

    pub fn notify(&mut self) -> SrvResult<()> {
        self.socket.send(&[1], 0)?;
        Ok(())
    }
}

impl Default for ScheduleClient {
    fn default() -> ScheduleClient {
        let socket = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        socket.set_sndhwm(1).unwrap();
        socket.set_linger(0).unwrap();
        socket.set_immediate(true).unwrap();

        ScheduleClient { socket: socket }
    }
}

pub struct ScheduleMgr {
    datastore: DataStore,
    logger: Logger,
    msg: zmq::Message,
    route_conn: RouteClient,
    schedule_cli: ScheduleClient,
    socket: zmq::Socket,
}

impl ScheduleMgr {
    pub fn new(datastore: DataStore, config: &Config, router_pipe: Arc<String>) -> SrvResult<Self> {
        let socket = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;
        socket.set_rcvhwm(1)?;
        socket.set_linger(0)?;
        socket.set_immediate(true)?;
        let mut schedule_cli = ScheduleClient::default();
        schedule_cli.connect()?;
        let route_conn = RouteClient::new()?;
        route_conn.connect(&*router_pipe)?;
        Ok(ScheduleMgr {
            datastore: datastore,
            logger: Logger::init(&config.log_path, "builder-scheduler.log"),
            msg: zmq::Message::new()?,
            route_conn: route_conn,
            schedule_cli: schedule_cli,
            socket: socket,
        })
    }

    pub fn start(
        datastore: DataStore,
        config: &Config,
        route_pipe: Arc<String>,
    ) -> SrvResult<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let mut schedule_mgr = Self::new(datastore, config, route_pipe)?;
        let handle = thread::Builder::new()
            .name("scheduler".to_string())
            .spawn(move || { schedule_mgr.run(tx).unwrap(); })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("scheduler thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> SrvResult<()> {
        self.socket.bind(SCHEDULER_ADDR)?;

        let mut socket = false;
        rz.send(()).unwrap();
        loop {
            {
                let mut items = [self.socket.as_poll_item(1)];
                zmq::poll(&mut items, SOCKET_TIMEOUT_MS)?;

                if (items[0].get_revents() & zmq::POLLIN) > 0 {
                    socket = true;
                }
            }

            if let Err(err) = self.process_status() {
                warn!("Unable to process status: err {:?}", err);
            }

            if let Err(err) = self.process_work() {
                warn!("Unable to process work: err {:?}", err);
            }

            if socket {
                self.socket.recv(&mut self.msg, 0)?;
                socket = false;
            }
        }
    }

    fn log_error(&mut self, msg: String) {
        warn!("{}", msg);
        self.logger.log(&msg);
    }

    fn process_work(&mut self) -> SrvResult<()> {
        loop {
            // Take one group from the pending list
            let mut groups = self.datastore.pending_groups(1)?;

            // 0 means there are no pending groups, so we should consume our notice that we have
            // work
            if groups.len() == 0 {
                break;
            }

            // This unwrap is fine, because we just checked our length
            let group = groups.pop().unwrap();
            assert!(group.get_state() == proto::GroupState::Dispatching);

            self.dispatch_group(&group)?;
            self.update_group_state(group.get_id())?;
        }
        Ok(())
    }

    fn dispatch_group(&mut self, group: &proto::Group) -> SrvResult<()> {
        debug!("Dispatching group {}", group.get_id());
        self.logger.log_group(&group);

        let mut skipped = HashMap::new();
        let dispatchable = self.dispatchable_projects(&group)?;

        for project in dispatchable {
            if skipped.contains_key(project.get_name()) {
                continue;
            }

            debug!("Dispatching project: {:?}", project.get_name());
            self.logger.log_group_project(&group, &project);

            assert!(project.get_state() == proto::ProjectState::NotStarted);

            match self.schedule_job(group.get_id(), project.get_name()) {
                Ok(job_opt) => {
                    match job_opt {
                        Some(job) => self.datastore.set_group_job_state(&job).unwrap(),
                        None => {
                            debug!("Skipping project: {:?}", project.get_name());
                            self.datastore.set_group_project_state(
                                group.get_id(),
                                project.get_name(),
                                proto::ProjectState::Skipped,
                            )?;

                            let skip_list = match self.skip_projects(&group, project.get_name()) {
                                Ok(v) => v,
                                Err(e) => {
                                    self.log_error(format!(
                                        "Error skipping projects for {:?} (group: {}): {:?}",
                                        project.get_name(),
                                        group.get_id(),
                                        e
                                    ));
                                    return Err(e);
                                }
                            };
                            for name in skip_list {
                                skipped.insert(name, true);
                            }
                        }
                    }
                }
                Err(err) => {
                    self.log_error(format!(
                        "Failed to schedule job for {} (group: {}), err: {:?}",
                        project.get_name(),
                        group.get_id(),
                        err
                    ));

                    self.datastore.set_group_state(
                        group.get_id(),
                        proto::GroupState::Failed,
                    )?;
                    self.datastore.set_group_project_state(
                        group.get_id(),
                        project.get_name(),
                        proto::ProjectState::Failure,
                    )?;

                    // TODO: Make this cleaner later
                    let mut updated_group = group.clone();
                    updated_group.set_state(proto::GroupState::Failed);
                    self.logger.log_group(&updated_group);

                    break;
                }
            }
        }
        Ok(())
    }

    fn dispatchable_projects(&mut self, group: &proto::Group) -> SrvResult<Vec<proto::Project>> {
        let mut projects = Vec::new();
        for project in group.get_projects().into_iter().filter(|x| {
            x.get_state() == proto::ProjectState::NotStarted
        })
        {
            // Check the deps for the project. If we don't find any dep that
            // is in our project list and needs to be built, we can dispatch the project.
            let dispatchable = if project.get_ident().is_empty() {
                true
            } else {
                let mut check_status = true;
                let package = self.datastore.get_package(&project.get_ident())?;
                let deps = package.get_deps();

                for dep in deps {
                    let parts: Vec<&str> = dep.split("/").collect();
                    assert!(parts.len() >= 2);
                    let name = format!("{}/{}", parts[0], parts[1]);

                    if !self.check_dispatchable(group, &name) {
                        check_status = false;
                        break;
                    };
                }
                check_status
            };

            if dispatchable {
                projects.push(project.clone());
            }
        }
        Ok(projects)
    }

    fn check_dispatchable(&mut self, group: &proto::Group, name: &str) -> bool {
        for project in group.get_projects() {
            if (project.get_name() == name) &&
                (project.get_state() != proto::ProjectState::Success)
            {
                return false;
            }
        }
        true
    }

    fn skip_projects(
        &mut self,
        group: &proto::Group,
        project_name: &str,
    ) -> SrvResult<Vec<String>> {
        let mut skipped = HashMap::new();
        skipped.insert(project_name.to_string(), true);

        for project in group.get_projects().into_iter().filter(|x| {
            x.get_state() == proto::ProjectState::NotStarted
        })
        {
            // Check the deps for the project. If we find any dep that is in the
            // skipped list, we set the project status to Skipped and add it to the list
            let package = self.datastore.get_package(&project.get_ident())?;
            let deps = package.get_deps();

            for dep in deps {
                let parts: Vec<&str> = dep.split("/").collect();
                assert!(parts.len() >= 2);
                let name = format!("{}/{}", parts[0], parts[1]);

                if skipped.contains_key(&name) {
                    debug!("Skipping project {:?}", project.get_name());
                    self.datastore.set_group_project_state(
                        group.get_id(),
                        project.get_name(),
                        proto::ProjectState::Skipped,
                    )?;
                    skipped.insert(project.get_name().to_string(), true);
                    break;
                }
            }
        }

        Ok(skipped.keys().map(|s| s.to_string()).collect())
    }

    fn schedule_job(&mut self, group_id: u64, project_name: &str) -> SrvResult<Option<Job>> {
        let mut project_get = OriginProjectGet::new();
        project_get.set_name(String::from(project_name));

        let project = match self.route_conn.route::<OriginProjectGet, OriginProject>(
            &project_get,
        ) {
            Ok(project) => project,
            Err(err) => {
                if err.get_code() == ErrCode::ENTITY_NOT_FOUND {
                    // It's valid to not have a project connected
                    debug!("Unable to retrieve project: {:?} (not found)", project_name);
                    return Ok(None);
                } else {
                    // If we're not able to retrieve the project for other reasons,
                    // it's likely a legit error - just log it for now, and return
                    // Ok to keep the scheduler going.  TODO: Tighten this up later
                    self.log_error(format!(
                        "Unable to retrieve project: {:?} (group: {}), error: {:?}",
                        project_name,
                        group_id,
                        err
                    ));
                    return Ok(None);
                }
            }
        };

        let mut job_spec: JobSpec = JobSpec::new();
        job_spec.set_owner_id(group_id);
        job_spec.set_project(project);
        job_spec.set_channel(bldr_channel_name(group_id));

        match self.route_conn.route::<JobSpec, Job>(&job_spec) {
            Ok(job) => {
                debug!("Job created: {:?}", job);
                Ok(Some(job))
            }
            Err(err) => Err(SrvError::from(err)),
        }
    }

    fn get_group(&mut self, group_id: u64) -> SrvResult<proto::Group> {
        let mut msg: proto::GroupGet = proto::GroupGet::new();
        msg.set_group_id(group_id);

        match self.datastore.get_group(&msg) {
            Ok(group_opt) => {
                match group_opt {
                    Some(group) => Ok(group),
                    None => Err(SrvError::UnknownGroup),
                }
            }
            Err(err) => {
                self.log_error(format!(
                    "Failed to get group {} from datastore: {:?}",
                    group_id,
                    err
                ));
                Err(err)
            }
        }
    }

    fn process_status(&mut self) -> SrvResult<()> {
        loop {
            // Take the top job status message from the message queue
            let mut msgs = self.datastore.peek_message(1)?;

            if msgs.len() == 0 {
                break;
            }

            // This unwrap is fine, because we just checked our length
            let (msg_id, job_status) = msgs.pop().unwrap();
            let job = job_status.get_job();

            debug!("Got job status: id={} job={:?}", msg_id, job);

            let group: proto::Group = match self.get_group(job.get_owner_id()) {
                Ok(group) => group,
                Err(SrvError::UnknownGroup) => {
                    // UnknownGroup is ok, just delete the message and move on
                    debug!("Skipping unknown group {:?}", job.get_owner_id());
                    self.datastore.delete_message(msg_id)?;
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            };

            self.logger.log_group_job(&group, &job);

            match self.datastore.set_group_job_state(&job) {
                Ok(_) => {
                    if job.get_state() == jobsrv::JobState::Failed {
                        match self.skip_projects(&group, job.get_project().get_name()) {
                            Ok(_) => (),
                            Err(e) => {
                                self.log_error(format!(
                                    "Error skipping projects for {:?} (group: {}): {:?}",
                                    job.get_project().get_name(),
                                    job.get_owner_id(),
                                    e
                                ));
                            }
                        };
                    }

                    match job.get_state() {
                        jobsrv::JobState::Complete |
                        jobsrv::JobState::Failed => self.update_group_state(job.get_owner_id())?,

                        _ => (),
                    }

                    // Delete the processed message from the queue
                    self.datastore.delete_message(msg_id)?;
                }
                Err(err) => {
                    self.log_error(format!(
                        "Failed to update job state for {} (group: {}): {:?}",
                        job.get_project().get_name(),
                        job.get_owner_id(),
                        err
                    ))
                }
            }
        }

        Ok(())
    }

    fn update_group_state(&mut self, group_id: u64) -> SrvResult<()> {
        let group = self.get_group(group_id)?;

        // Group state transition rules:
        // |   Start Group State     |  Projects State  |   New Group State   |
        // |-------------------------|------------------|---------------------|
        // |     Pending             |     N/A          |        N/A          |
        // |     Dispatching         |   any Failure    |      Failed         |
        // |     Dispatching         |   all Success    |      Complete       |
        // |     Dispatching         |   dispatchable?  |      Pending        |
        // |     Dispatching         |   otherwise      |      Dispatching    |
        // |     Complete            |     N/A          |        N/A          |
        // |     Failed              |     N/A          |        N/A          |

        if group.get_state() == proto::GroupState::Dispatching {
            let mut failed = 0;
            let mut succeeded = 0;
            let mut skipped = 0;

            for project in group.get_projects() {
                match project.get_state() {
                    proto::ProjectState::Failure => failed = failed + 1,
                    proto::ProjectState::Success => succeeded = succeeded + 1,
                    proto::ProjectState::Skipped => skipped = skipped + 1,
                    _ => (),
                }
            }

            let dispatchable = self.dispatchable_projects(&group)?;

            let new_state = if (succeeded + skipped + failed) == group.get_projects().len() {
                proto::GroupState::Complete
            } else if dispatchable.len() > 0 {
                proto::GroupState::Pending
            } else {
                proto::GroupState::Dispatching
            };

            self.datastore.set_group_state(group_id, new_state)?;

            if new_state == proto::GroupState::Pending {
                self.schedule_cli.notify()?;
            } else {
                // TODO: Make this cleaner later
                let mut updated_group = group.clone();
                updated_group.set_state(new_state);
                self.logger.log_group(&updated_group);
            }
        } else {
            debug!(
                "Skipping group update because state is {:?} for group id: {}",
                group.get_state(),
                group_id
            );
        }

        Ok(())
    }
}
