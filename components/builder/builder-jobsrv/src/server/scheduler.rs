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

use std::collections::HashMap;
use std::path::Path;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

use hab_net::ErrCode;
use hab_net::conn::RouteClient;
use hab_net::socket::DEFAULT_CONTEXT;
use zmq;

use protocol::jobsrv;
use protocol::originsrv;
use data_store::DataStore;
use error::{Result, Error};

use bldr_core::logger::Logger;
use hab_core::channel::bldr_channel_name;

use super::worker_manager::WorkerMgrClient;

const SCHEDULER_ADDR: &'static str = "inproc://scheduler";
const SOCKET_TIMEOUT_MS: i64 = 60_000;

pub struct ScheduleClient {
    socket: zmq::Socket,
}

impl ScheduleClient {
    pub fn connect(&mut self) -> Result<()> {
        self.socket.connect(SCHEDULER_ADDR)?;
        Ok(())
    }

    pub fn notify(&mut self) -> Result<()> {
        self.socket.send(&[1], 0)?;
        Ok(())
    }
}

impl Default for ScheduleClient {
    fn default() -> ScheduleClient {
        let socket = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
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
    worker_mgr: WorkerMgrClient,
}

impl ScheduleMgr {
    pub fn new<T>(datastore: DataStore, log_path: T, router_pipe: Arc<String>) -> Result<Self>
    where
        T: AsRef<Path>,
    {
        let socket = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;

        let mut schedule_cli = ScheduleClient::default();
        schedule_cli.connect()?;

        let route_conn = RouteClient::new()?;
        route_conn.connect(&*router_pipe)?;

        let mut worker_mgr = WorkerMgrClient::default();
        worker_mgr.connect()?;

        Ok(ScheduleMgr {
            datastore: datastore,
            logger: Logger::init(log_path, "builder-scheduler.log"),
            msg: zmq::Message::new()?,
            route_conn: route_conn,
            schedule_cli: schedule_cli,
            socket: socket,
            worker_mgr: worker_mgr,
        })
    }

    pub fn start<T>(
        datastore: DataStore,
        log_path: T,
        route_pipe: Arc<String>,
    ) -> Result<JoinHandle<()>>
    where
        T: AsRef<Path>,
    {
        let (tx, rx) = mpsc::sync_channel(1);
        let mut schedule_mgr = Self::new(datastore, log_path, route_pipe)?;
        let handle = thread::Builder::new()
            .name("scheduler".to_string())
            .spawn(move || { schedule_mgr.run(tx).unwrap(); })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("scheduler thread startup error, err={}", e),
        }
    }

    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        self.socket.bind(SCHEDULER_ADDR)?;

        let mut socket = false;
        rz.send(()).unwrap();
        loop {
            {
                let mut items = [self.socket.as_poll_item(1)];
                if let Err(err) = zmq::poll(&mut items, SOCKET_TIMEOUT_MS) {
                    warn!("Scheduler unable to complete ZMQ poll: err {:?}", err);
                };

                if (items[0].get_revents() & zmq::POLLIN) > 0 {
                    socket = true;
                }
            }

            if let Err(err) = self.process_status() {
                warn!("Scheduler unable to process status: err {:?}", err);
            }

            if let Err(err) = self.process_queue() {
                warn!("Scheduler unable to process queue: err {:?}", err);
            }

            if let Err(err) = self.process_work() {
                warn!("Scheduler unable to process work: err {:?}", err);
            }

            if socket {
                if let Err(err) = self.socket.recv(&mut self.msg, 0) {
                    warn!("Scheduler unable to complete socket receive: err {:?}", err);
                }
                socket = false;
            }
        }
    }

    fn log_error(&mut self, msg: String) {
        warn!("{}", msg);
        self.logger.log(&msg);
    }

    fn process_queue(&mut self) -> Result<()> {
        let groups = self.datastore.get_queued_job_groups()?;

        for group in groups.iter() {
            assert!(group.get_state() == jobsrv::JobGroupState::GroupQueued);

            if !self.datastore.is_job_group_active(group.get_project_name())? {
                debug!(
                    "Setting group {} from queued to pending",
                    group.get_project_name()
                );
                self.datastore.set_job_group_state(
                    group.get_id(),
                    jobsrv::JobGroupState::GroupPending,
                )?;
            }
        }

        Ok(())
    }

    fn process_work(&mut self) -> Result<()> {
        loop {
            // Take one group from the pending list
            let mut groups = self.datastore.pending_job_groups(1)?;

            // 0 means there are no pending groups, so we should consume our notice that we have
            // work
            if groups.len() == 0 {
                break;
            }

            // This unwrap is fine, because we just checked our length
            let group = groups.pop().unwrap();
            assert!(group.get_state() == jobsrv::JobGroupState::GroupDispatching);

            self.dispatch_group(&group)?;
            self.update_group_state(group.get_id())?;
        }
        Ok(())
    }

    fn dispatch_group(&mut self, group: &jobsrv::JobGroup) -> Result<()> {
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

            assert!(project.get_state() == jobsrv::JobGroupProjectState::NotStarted);

            match self.schedule_job(group.get_id(), project.get_name()) {
                Ok(job_opt) => {
                    match job_opt {
                        Some(job) => self.datastore.set_job_group_job_state(&job).unwrap(),
                        None => {
                            debug!("Skipping project: {:?}", project.get_name());
                            self.datastore.set_job_group_project_state(
                                group.get_id(),
                                project.get_name(),
                                jobsrv::JobGroupProjectState::Skipped,
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

                    // TODO: Is this the right thing to do?
                    self.datastore.set_job_group_state(
                        group.get_id(),
                        jobsrv::JobGroupState::GroupFailed,
                    )?;
                    self.datastore.set_job_group_project_state(
                        group.get_id(),
                        project.get_name(),
                        jobsrv::JobGroupProjectState::Failure,
                    )?;

                    // TODO: Make this cleaner later
                    let mut updated_group = group.clone();
                    updated_group.set_state(jobsrv::JobGroupState::GroupFailed);
                    self.logger.log_group(&updated_group);

                    break;
                }
            }
        }
        Ok(())
    }

    fn dispatchable_projects(
        &mut self,
        group: &jobsrv::JobGroup,
    ) -> Result<Vec<jobsrv::JobGroupProject>> {
        let mut projects = Vec::new();
        for project in group.get_projects().into_iter().filter(|x| {
            x.get_state() == jobsrv::JobGroupProjectState::NotStarted
        })
        {
            // Check the deps for the project. If we don't find any dep that
            // is in our project list and needs to be built, we can dispatch the project.
            let dispatchable = if project.get_ident().is_empty() {
                true
            } else {
                let mut check_status = true;
                let package = self.datastore.get_job_graph_package(&project.get_ident())?;
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
        debug!(
            "Found {} dispatchable projects for group {}",
            projects.len(),
            group.get_id()
        );
        Ok(projects)
    }

    fn check_dispatchable(&mut self, group: &jobsrv::JobGroup, name: &str) -> bool {
        for project in group.get_projects() {
            if (project.get_name() == name) &&
                (project.get_state() != jobsrv::JobGroupProjectState::Success)
            {
                return false;
            }
        }
        true
    }

    fn skip_projects(
        &mut self,
        group: &jobsrv::JobGroup,
        project_name: &str,
    ) -> Result<Vec<String>> {
        let mut skipped = HashMap::new();
        skipped.insert(project_name.to_string(), true);

        for project in group.get_projects().into_iter().filter(|x| {
            x.get_state() == jobsrv::JobGroupProjectState::NotStarted
        })
        {
            // Check the deps for the project. If we find any dep that is in the
            // skipped list, we set the project status to Skipped and add it to the list
            let package = match self.datastore.get_job_graph_package(&project.get_ident()) {
                Ok(package) => package,
                Err(err) => {
                    warn!(
                        "Unable to retrieve job graph package {}, err: {:?}",
                        project.get_ident(),
                        err
                    );
                    continue;
                }
            };
            let deps = package.get_deps();

            for dep in deps {
                let parts: Vec<&str> = dep.split("/").collect();
                assert!(parts.len() >= 2);
                let name = format!("{}/{}", parts[0], parts[1]);

                if skipped.contains_key(&name) {
                    debug!("Skipping project {:?}", project.get_name());
                    self.datastore.set_job_group_project_state(
                        group.get_id(),
                        project.get_name(),
                        jobsrv::JobGroupProjectState::Skipped,
                    )?;
                    skipped.insert(project.get_name().to_string(), true);
                    break;
                }
            }
        }

        Ok(skipped.keys().map(|s| s.to_string()).collect())
    }

    fn schedule_job(&mut self, group_id: u64, project_name: &str) -> Result<Option<jobsrv::Job>> {
        let mut project_get = originsrv::OriginProjectGet::new();
        project_get.set_name(String::from(project_name));

        let project = match self.route_conn.route::<originsrv::OriginProjectGet, originsrv::OriginProject>(
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

        let mut job_spec = jobsrv::JobSpec::new();
        job_spec.set_owner_id(group_id);
        job_spec.set_project(project);
        job_spec.set_channel(bldr_channel_name(group_id));

        let mut job: jobsrv::Job = job_spec.into();
        match self.datastore.create_job(&mut job) {
            Ok(job) => {
                debug!("Job created: {:?}", job);
                self.worker_mgr.notify_work()?;
                Ok(Some(job))
            }
            Err(err) => {
                warn!("Unable to create job, err: {:?}", err);
                Err(Error::from(err))
            }
        }
    }

    fn get_group(&mut self, group_id: u64) -> Result<jobsrv::JobGroup> {
        let mut msg: jobsrv::JobGroupGet = jobsrv::JobGroupGet::new();
        msg.set_group_id(group_id);

        match self.datastore.get_job_group(&msg) {
            Ok(group_opt) => {
                match group_opt {
                    Some(group) => Ok(group),
                    None => Err(Error::UnknownJobGroup),
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

    fn process_status(&mut self) -> Result<()> {
        // Get a list of jobs with un-sync'd status
        let jobs = self.datastore.sync_jobs()?;
        debug!("Process status: found {} updated jobs", jobs.len());

        for job in jobs {
            debug!("Syncing job status: job={:?}", job);

            let group: jobsrv::JobGroup = match self.get_group(job.get_owner_id()) {
                Ok(group) => group,
                Err(Error::UnknownJobGroup) => {
                    // UnknownGroup is ok, just unset the sync and move on
                    debug!("Skipping unknown group {:?}", job.get_owner_id());
                    self.datastore.set_job_sync(job.get_owner_id())?;
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            };

            self.logger.log_group_job(&group, &job);

            match self.datastore.set_job_group_job_state(&job) {
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
                        jobsrv::JobState::Failed |
                        jobsrv::JobState::CancelComplete => {
                            self.update_group_state(job.get_owner_id())?
                        }

                        jobsrv::JobState::Pending |
                        jobsrv::JobState::Processing |
                        jobsrv::JobState::Dispatched |
                        jobsrv::JobState::CancelPending |
                        jobsrv::JobState::CancelProcessing |
                        jobsrv::JobState::Rejected => (),
                    }

                    // Unset the sync state
                    self.datastore.set_job_sync(job.get_id())?;
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

    fn update_group_state(&mut self, group_id: u64) -> Result<()> {
        let group = self.get_group(group_id)?;

        // Group state transition rules:
        // |   Start Group State     |  Projects State  |   New Group State   |
        // |-------------------------|------------------|---------------------|
        // |     Queued              |     N/A          |        N/A          |
        // |     Pending             |     N/A          |        N/A          |
        // |     Dispatching         |   no remaining   |      Complete       |
        // |     Dispatching         |   dispatchable?  |      Pending        |
        // |     Dispatching         |   otherwise      |      Dispatching    |
        // |     Complete            |     N/A          |        N/A          |
        // |     Failed              |     N/A          |        N/A          |

        if group.get_state() == jobsrv::JobGroupState::GroupDispatching {
            let mut failed = 0;
            let mut succeeded = 0;
            let mut skipped = 0;
            let mut canceled = 0;

            for project in group.get_projects() {
                match project.get_state() {
                    jobsrv::JobGroupProjectState::Failure => failed = failed + 1,
                    jobsrv::JobGroupProjectState::Success => succeeded = succeeded + 1,
                    jobsrv::JobGroupProjectState::Skipped => skipped = skipped + 1,
                    jobsrv::JobGroupProjectState::Canceled => canceled = canceled + 1,

                    jobsrv::JobGroupProjectState::NotStarted |
                    jobsrv::JobGroupProjectState::InProgress => (),
                }
            }

            let dispatchable = self.dispatchable_projects(&group)?;

            let new_state = if (succeeded + skipped + failed) == group.get_projects().len() {
                jobsrv::JobGroupState::GroupComplete
            } else if canceled > 0 {
                jobsrv::JobGroupState::GroupCanceled
            } else if dispatchable.len() > 0 {
                jobsrv::JobGroupState::GroupPending
            } else {
                jobsrv::JobGroupState::GroupDispatching
            };

            self.datastore.set_job_group_state(group_id, new_state)?;

            if new_state == jobsrv::JobGroupState::GroupPending {
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
