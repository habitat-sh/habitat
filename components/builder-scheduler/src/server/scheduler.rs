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

use std::sync::mpsc;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};
use std::path::PathBuf;
use std::collections::HashMap;
use std::str::FromStr;

use hab_net::server::ZMQ_CONTEXT;
use hab_net::routing::Broker;
use hyper::status::StatusCode;
use zmq;

use protobuf::parse_from_bytes;
use protocol::jobsrv::{self, Job, JobSpec};
use protocol::originsrv::*;
use protocol::scheduler as proto;
use data_store::DataStore;
use error::{Result, Error};

use config::Config;
use bldr_core::logger::Logger;
use hab_core::channel::bldr_channel_name;
use depot_client;
use {PRODUCT, VERSION};

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
        let socket = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        socket.set_sndhwm(1).unwrap();
        socket.set_linger(0).unwrap();
        socket.set_immediate(true).unwrap();

        ScheduleClient { socket: socket }
    }
}

pub struct ScheduleMgr {
    datastore: DataStore,
    socket: zmq::Socket,
    schedule_cli: ScheduleClient,
    depot_url: String,
    auth_token: String,
    promote_channel: String,
    msg: zmq::Message,
    logger: Logger,
}

impl ScheduleMgr {
    pub fn new(datastore: DataStore, config: Arc<RwLock<Config>>) -> Result<Self> {
        let socket = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER)?;
        socket.set_rcvhwm(1)?;
        socket.set_linger(0)?;
        socket.set_immediate(true)?;

        let msg = zmq::Message::new()?;
        let mut schedule_cli = ScheduleClient::default();
        schedule_cli.connect()?;

        let (log_path, depot_url, auth_token, promote_channel) = {
            let cfg = config.read().unwrap();
            (
                PathBuf::from(cfg.log_path.clone()),
                cfg.depot_url.clone(),
                cfg.auth_token.clone(),
                cfg.promote_channel.clone(),
            )
        };

        let logger = Logger::init(log_path, "builder-scheduler.log");

        Ok(ScheduleMgr {
            datastore: datastore,
            socket: socket,
            schedule_cli: schedule_cli,
            depot_url: depot_url,
            auth_token: auth_token,
            promote_channel: promote_channel,
            msg: msg,
            logger: logger,
        })
    }

    pub fn start(ds: DataStore, config: Arc<RwLock<Config>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(1);
        let handle = thread::Builder::new()
            .name("scheduler".to_string())
            .spawn(move || {
                let mut schedule_mgr = Self::new(ds, config).unwrap();
                schedule_mgr.run(tx).unwrap();
            })
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

    fn process_work(&mut self) -> Result<()> {
        loop {
            // Take one group from the pending list
            let mut groups = self.datastore.pending_groups(1)?;

            // 0 means there are no pending groups, so we should consume our notice that we have work
            if groups.len() == 0 {
                break;
            }

            // This unwrap is fine, because we just checked our length
            let group = groups.pop().unwrap();
            info!("Got pending group {}", group.get_id());
            assert!(group.get_state() == proto::GroupState::Dispatching);

            self.dispatch_group(&group)?;
            self.update_group_state(group.get_id())?;
        }
        Ok(())
    }

    fn dispatch_group(&mut self, group: &proto::Group) -> Result<()> {
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
                                    warn!(
                                        "Error skipping projects for {:?}: {:?}",
                                        project.get_name(),
                                        e
                                    );
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
                    warn!(
                        "Failed to schedule job for {}, err: {:?}",
                        project.get_name(),
                        err
                    );

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

    fn dispatchable_projects(&mut self, group: &proto::Group) -> Result<Vec<proto::Project>> {
        let mut projects = Vec::new();
        for project in group.get_projects().into_iter().filter(|x| {
            x.get_state() == proto::ProjectState::NotStarted
        })
        {
            // Check the deps for the project. If we don't find any dep that
            // is in our project list and needs to be built, we can dispatch the project.
            let package = self.datastore.get_package(&project.get_ident())?;
            let deps = package.get_deps();

            let mut dispatchable = true;
            for dep in deps {
                let parts: Vec<&str> = dep.split("/").collect();
                assert!(parts.len() >= 2);
                let name = format!("{}/{}", parts[0], parts[1]);

                if !self.check_dispatchable(group, &name) {
                    dispatchable = false;
                    break;
                };
            }

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

    fn skip_projects(&mut self, group: &proto::Group, project_name: &str) -> Result<Vec<String>> {
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

    fn schedule_job(&mut self, group_id: u64, project_name: &str) -> Result<Option<Job>> {
        let mut project_get = OriginProjectGet::new();

        project_get.set_name(String::from(project_name));

        let mut conn = Broker::connect().unwrap();
        let project = match conn.route::<OriginProjectGet, OriginProject>(&project_get) {
            Ok(project) => project,
            Err(err) => {
                warn!(
                    "Unable to retrieve project: {:?}, error: {:?} - Skipping",
                    project_name,
                    err
                );

                return Ok(None);
            }
        };

        let mut job_spec: JobSpec = JobSpec::new();
        job_spec.set_owner_id(group_id);
        job_spec.set_project(project);

        match conn.route::<JobSpec, Job>(&job_spec) {
            Ok(job) => {
                debug!("Job created: {:?}", job);
                Ok(Some(job))
            }
            Err(err) => {
                warn!("Job creation error: {:?}", err);
                Err(Error::ProtoNetError(err))
            }
        }
    }

    fn get_group(&mut self, group_id: u64) -> Result<proto::Group> {
        let mut msg: proto::GroupGet = proto::GroupGet::new();
        msg.set_group_id(group_id);

        match self.datastore.get_group(&msg) {
            Ok(group_opt) => {
                match group_opt {
                    Some(group) => Ok(group),
                    None => Err(Error::UnknownGroup),
                }
            }
            Err(err) => {
                warn!("Group retrieve error: {:?}", err);
                Err(Error::UnknownGroup)
            }
        }
    }

    fn process_status(&mut self) -> Result<()> {
        loop {
            // Take the top job status message from the message queue
            let mut msgs = self.datastore.peek_message(1)?;

            if msgs.len() == 0 {
                break;
            }

            // This unwrap is fine, because we just checked our length
            let (msg_id, msg) = msgs.pop().unwrap();

            assert_eq!(msg.get_message_id(), "JobStatus");
            let job_status: proto::JobStatus = parse_from_bytes(&msg.get_body())?;
            let job = job_status.get_job();

            debug!("Got job status: id={} job={:?}", msg_id, job);

            let group: proto::Group = self.get_group(job.get_owner_id())?;

            self.logger.log_group_job(&group, &job);

            match self.datastore.set_group_job_state(&job) {
                Ok(_) => {
                    if job.get_state() == jobsrv::JobState::Failed {
                        match self.skip_projects(&group, job.get_project().get_name()) {
                            Ok(_) => (),
                            Err(e) => {
                                warn!(
                                    "Error skipping projects for {:?}: {:?}",
                                    job.get_project().get_name(),
                                    e
                                );
                            }
                        };
                    }

                    match job.get_state() {
                        jobsrv::JobState::Complete => {
                            self.promote_to_sandbox(&job)?;
                            self.update_group_state(job.get_owner_id())?
                        }

                        jobsrv::JobState::Failed => self.update_group_state(job.get_owner_id())?,

                        _ => (),
                    }

                    // Delete the processed message from the queue
                    self.datastore.delete_message(msg_id)?;
                }
                Err(err) => debug!("Did not set job state: {:?}", err),
            }
        }

        Ok(())
    }

    fn promote_to_sandbox(&self, job: &jobsrv::Job) -> Result<()> {
        self.promote_package(
            &job.get_package_ident(),
            &bldr_channel_name(job.get_owner_id()),
        )
    }

    fn promote_package(&self, ident: &OriginPackageIdent, channel: &str) -> Result<()> {
        debug!("Promoting '{:?}' to '{}'", ident, channel);

        // We re-create the depot client instead of caching and re-using it due to the
        // connection getting dropped when it is attempted to be re-used. This _may_ be
        // a known issue with hyper connection pool getting reset in some cases.
        // TODO: Revisit this code when we upgrade hyper to 0.11.x (which will require
        // significant changes)
        let depot_cli = depot_client::Client::new(&self.depot_url, PRODUCT, VERSION, None).unwrap();

        if channel != "stable" {
            match depot_cli.create_channel(ident.get_origin(), channel, &self.auth_token) {
                Ok(_) => (),
                Err(depot_client::Error::APIError(StatusCode::Conflict, _)) => (),
                Err(err) => {
                    warn!("Failed to create '{}' channel: {:?}", channel, err);
                    return Err(Error::ChannelCreate(err));
                }
            };
        };

        if let Some(err) = depot_cli
            .promote_package(ident, channel, &self.auth_token)
            .err()
        {
            warn!(
                "Unable to promote package '{:?}' to channel '{}': {:?}",
                ident,
                channel,
                err
            );
            return Err(Error::PackagePromote(err));
        };

        Ok(())
    }

    fn promote_group(&self, group: &proto::Group, channel: &str) -> Result<()> {
        debug!("Promoting group {} to {} channel", group.get_id(), channel);

        for project in group.get_projects().into_iter().filter(|x| {
            x.get_state() == proto::ProjectState::Success
        })
        {
            self.promote_package(
                &OriginPackageIdent::from_str(project.get_ident())
                    .unwrap(),
                channel,
            )?;
        }

        Ok(())
    }

    fn update_group_state(&mut self, group_id: u64) -> Result<()> {
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
                // Promote everything to stable if there are no failures
                // TODO: Add a better heuristic for promotion, eg, check leaf failure nodes
                // and don't block promotion if the failures don't have any packages that
                // depend on them.
                if failed == 0 && self.promote_channel != "unstable" {
                    self.promote_group(&group, &self.promote_channel)?;
                }

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
            warn!(
                "Unexpected group state {:?} for group id: {}",
                group.get_state(),
                group_id
            );
        }

        Ok(())
    }
}
