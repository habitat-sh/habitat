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

pub mod log_pipe;
pub mod workspace;
pub mod postprocessor;

use std::path::PathBuf;
use std::ffi::OsString;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, JoinHandle};

use bldr_core::logger::Logger;
pub use protocol::jobsrv::JobState;
use chrono::UTC;
use depot_client;
use hab_core::{crypto, env};
use hab_core::package::archive::PackageArchive;
use hab_core::package::install::PackageInstall;
use hab_core::package::PackageIdent;
use hab_net::server::ZMQ_CONTEXT;
use protobuf::{parse_from_bytes, Message};
use protocol::jobsrv as proto;
use protocol::originsrv::OriginPackageIdent;
use protocol::net::{self, ErrCode};
use zmq;

use {PRODUCT, VERSION};
use self::log_pipe::LogPipe;
use self::postprocessor::PostProcessor;
use self::workspace::Workspace;
use config::Config;
use error::{Error, Result};
use server::Server;
use hab_net::server::NetIdent;
use vcs;

/// Environment variable to enable or disable debug output in runner's studio
const RUNNER_DEBUG_ENV: &'static str = "BUILDER_RUNNER_DEBUG";
/// In-memory zmq address of Job RunnerMgr
const INPROC_ADDR: &'static str = "inproc://runner";
/// Protocol message to indicate the Job Runner has received a work request
const WORK_ACK: &'static str = "A";
/// Protocol message to indicate the Job Runner has completed a work request
const WORK_COMPLETE: &'static str = "C";

lazy_static! {
    // JW TODO: expose public API functions in the core crate to check if the Rust process which
    // is currently executing is, itself, packaged by Habitat. If so, then we should expose another
    // public API function to load the PackageInstall of a dep for the given `origin`/`name`
    // combination. If we can't, then we should fall back to the latest of core/hab-studio because
    // that means we're just in a dev shell.
    static ref STUDIO_PKG: PackageIdent = PackageIdent::from_str("core/hab-studio").unwrap();
}

#[derive(Debug)]
pub struct Job(proto::Job);

impl Job {
    pub fn new(job: proto::Job) -> Self {
        Job(job)
    }

    pub fn vcs(&self) -> vcs::VCS {
        match self.0.get_project().get_vcs_type() {
            "git" => {
                vcs::VCS::new(
                    String::from(self.0.get_project().get_vcs_type()),
                    String::from(self.0.get_project().get_vcs_data()),
                )
            }
            _ => panic!("unknown vcs associated with jobs project"),
        }
    }

    pub fn origin(&self) -> &str {
        let items = self.0
            .get_project()
            .get_name()
            .split("/")
            .collect::<Vec<&str>>();
        assert!(
            items.len() == 2,
            format!(
                "Invalid project identifier - {}",
                self.0.get_project().get_id()
            )
        );
        items[0]
    }
}

impl Deref for Job {
    type Target = proto::Job;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Job {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Runner {
    config: Config,
    depot_cli: depot_client::Client,
    log_pipe: Option<LogPipe>,
    workspace: Workspace,
    logger: Logger,
}

impl Runner {
    pub fn new(job: Job, config: Config) -> Self {
        let depot_cli = depot_client::Client::new(&config.depot_url, PRODUCT, VERSION, None)
            .unwrap();

        let log_path = config.log_path.clone();
        let mut logger = Logger::init(PathBuf::from(log_path), "builder-worker.log");
        logger.log_ident(&Server::net_ident());

        Runner {
            workspace: Workspace::new(config.data_path.clone(), job),
            config: config,
            depot_cli: depot_cli,
            log_pipe: None,
            logger: logger,
        }
    }

    pub fn job(&self) -> &Job {
        &self.workspace.job
    }

    pub fn job_mut(&mut self) -> &mut Job {
        &mut self.workspace.job
    }

    pub fn log_pipe(&mut self) -> &mut LogPipe {
        self.log_pipe.as_mut().expect("LogPipe not initialized")
    }

    pub fn run(mut self) -> Job {
        if let Some(err) = self.setup().err() {
            error!("WORKSPACE SETUP ERR={:?}", err);
            return self.fail(net::err(ErrCode::WORKSPACE_SETUP, "wk:run:1"));
        }

        if self.config.auth_token.is_empty() {
            warn!("WARNING: No auth token specified, will likely fail fetching secret key");
        };

        match self.depot_cli.fetch_origin_secret_key(
            self.job().origin(),
            &self.config.auth_token,
        ) {
            Ok(key) => {
                let cache = crypto::default_cache_key_path(None);
                let s: String = String::from_utf8(key.body).expect("Found invalid UTF-8");

                match crypto::SigKeyPair::write_file_from_str(&s, &cache) {
                    Ok((pair, pair_type)) => {
                        debug!(
                            "Imported {} origin key {}.",
                            &pair_type,
                            &pair.name_with_rev()
                        );
                    }
                    Err(err) => {
                        error!("Unable to import secret key, err={}", err);
                        return self.fail(net::err(ErrCode::SECRET_KEY_IMPORT, "wk:run:2"));
                    }
                }
            }
            Err(err) => {
                error!("Unable to retrieve secret key, err={}", err);
                return self.fail(net::err(ErrCode::SECRET_KEY_FETCH, "wk:run:3"));
            }
        }
        if let Some(err) = self.job().vcs().clone(&self.workspace.src()).err() {
            error!("Unable to clone remote source repository, err={}", err);
            return self.fail(net::err(ErrCode::VCS_CLONE, "wk:run:4"));
        }

        self.workspace.job.set_build_started_at(
            UTC::now().to_rfc3339(),
        );

        // TODO: We don't actually update the state of the job to
        // "Processing" (that should happen here), so an outside
        // observer will see a job up going from "Dispatched" directly
        // to "Complete" (or "Failed", etc.). As a result, we won't
        // get the `build_started_at` time set until the job is actually
        // finished.
        let mut archive = match self.build() {
            Ok(archive) => {
                self.workspace.job.set_build_finished_at(
                    UTC::now().to_rfc3339(),
                );
                archive
            }
            Err(err) => {
                self.workspace.job.set_build_finished_at(
                    UTC::now().to_rfc3339(),
                );
                error!("Unable to build in studio, err={}", err);
                return self.fail(net::err(ErrCode::BUILD, "wk:run:5"));
            }
        };

        // TODO: It doesn't appear that we can currently get a package
        // identifier out of the build process unless it successfully
        // completes. This means that only jobs that are successful
        // will have complete identifiers, but that makes it difficult
        // (impossible?) to associate failed jobs with specific
        // package versions. We may be able to do some kind of log
        // output snooping (or something more robust than that :) and
        // cross-thread communication to retrieve this information for
        // *all* job runs. But not today.

        // Converting from a core::PackageIdent to an OriginPackageIdent
        let ident = OriginPackageIdent::from(archive.ident().unwrap());
        self.workspace.job.set_package_ident(ident);

        let mut post_processor = PostProcessor::new(&self.workspace);
        if !post_processor.run(&mut archive, &self.config) {
            // JW TODO: We should shelve the built artifacts and allow a retry on post-processing.
            // If the job is killed then we can kill the shelved artifacts.
            return self.fail(net::err(ErrCode::POST_PROCESSOR, "wk:run:6"));
        }

        if let Some(err) = fs::remove_dir_all(self.workspace.out()).err() {
            error!(
                "unable to remove out directory ({}), ERR={:?}",
                self.workspace.out().display(),
                err
            )
        }
        self.complete()
    }

    fn build(&mut self) -> Result<PackageArchive> {
        let args = vec![
            OsString::from("-s"), // source path
            OsString::from(self.workspace.src()),
            OsString::from("-r"), // hab studio root
            OsString::from(self.workspace.studio()),
            OsString::from("-k"), // origin keys to use
            OsString::from(self.job().origin()),
            OsString::from("build"),
            OsString::from(
                Path::new(self.job().get_project().get_plan_path())
                    .parent()
                    .unwrap()
            ),
        ];
        let command = studio_cmd();
        debug!("building, cmd={:?}, args={:?}", command, args);
        let mut child = match env::var(RUNNER_DEBUG_ENV) {
            Ok(val) => {
                Command::new(command)
                    .args(&args)
                    .env_clear()
                    .env("HAB_NONINTERACTIVE", "true")
                    .env("HAB_DEPOT_URL", &self.config.depot_url)
                    .env("DEBUG", val)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("failed to spawn child")
            }
            Err(_) => {
                Command::new(command)
                    .args(&args)
                    .env_clear()
                    .env("HAB_NONINTERACTIVE", "true")
                    .env("HAB_DEPOT_URL", &self.config.depot_url)
                    .env("TERM", "xterm-256color") // Gives us ANSI color codes
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .expect("failed to spawn child")
            }
        };
        self.log_pipe().pipe(&mut child)?;
        let exit_status = child.wait().expect("failed to wait on child");
        debug!("build complete, status={:?}", exit_status);
        if exit_status.success() {
            fs::rename(self.workspace.src().join("results"), self.workspace.out())?;
            self.workspace.last_built()
        } else {
            Err(Error::BuildFailure(exit_status.code().unwrap_or(-1)))
        }
    }

    fn complete(mut self) -> Job {
        self.teardown().err().map(|e| error!("{}", e));
        self.workspace.job.set_state(JobState::Complete);
        self.logger.log_worker_job(&self.workspace.job);
        self.workspace.job
    }

    fn fail(mut self, err: net::NetError) -> Job {
        self.teardown().err().map(|e| error!("{}", e));
        self.workspace.job.set_state(JobState::Failed);
        self.workspace.job.set_error(err);
        self.logger.log_worker_job(&self.workspace.job);
        self.workspace.job
    }

    fn setup(&mut self) -> Result<()> {
        self.logger.log_worker_job(&self.workspace.job);

        if let Some(err) = fs::remove_dir_all(self.workspace.src()).err() {
            error!(
                "unable to remove out directory ({}), ERR={:?}",
                self.workspace.out().display(),
                err
            )
        }
        if let Some(err) = fs::create_dir_all(self.workspace.src()).err() {
            return Err(Error::WorkspaceSetup(
                format!("{}", self.workspace.src().display()),
                err,
            ));
        }
        self.log_pipe = Some(LogPipe::new(&self.workspace));
        Ok(())
    }

    fn teardown(&mut self) -> Result<()> {
        let args = vec![
            OsString::from("-s"), // source path
            OsString::from(self.workspace.src()),
            OsString::from("-r"), // hab studio root
            OsString::from(self.workspace.studio()),
            OsString::from("rm"),
        ];

        let command = studio_cmd();
        debug!("removing studio, cmd={:?}, args={:?}", command, args);
        let mut child = match env::var(RUNNER_DEBUG_ENV) {
            Ok(val) => {
                Command::new(command)
                    .args(&args)
                    .env_clear()
                    .env("HAB_NONINTERACTIVE", "true")
                    .env("DEBUG", val)
                    .spawn()
                    .expect("failed to spawn child")
            }
            Err(_) => {
                Command::new(command)
                    .args(&args)
                    .env_clear()
                    .env("HAB_NONINTERACTIVE", "true")
                    .spawn()
                    .expect("failed to spawn child")
            }
        };
        let exit_status = child.wait().expect("failed to wait on child");
        debug!("studio removal complete, status={:?}", exit_status);
        if exit_status.success() {
            if let Some(err) = fs::remove_dir_all(self.workspace.src()).err() {
                return Err(Error::WorkspaceTeardown(
                    format!("{}", self.workspace.src().display()),
                    err,
                ));
            }
            Ok(())
        } else {
            Err(Error::BuildFailure(exit_status.code().unwrap_or(-1)))
        }
    }
}

/// Client for sending and receiving messages to and from the Job Runner
pub struct RunnerCli {
    sock: zmq::Socket,
    msg: zmq::Message,
}

impl RunnerCli {
    /// Create a new Job Runner client
    pub fn new() -> Self {
        let sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
        RunnerCli {
            sock: sock,
            msg: zmq::Message::new().unwrap(),
        }
    }

    /// Return a poll item used in `zmq::poll` for awaiting messages on multiple sockets
    pub fn as_poll_item<'a>(&'a self, events: i16) -> zmq::PollItem<'a> {
        self.sock.as_poll_item(events)
    }

    /// Connect to the Job Runner
    pub fn connect(&mut self) -> Result<()> {
        self.sock.connect(INPROC_ADDR)?;
        Ok(())
    }

    /// Wait until client receives a work received acknowledgement by the Runner and return
    /// the assigned JobID.
    pub fn recv_ack(&mut self) -> Result<&zmq::Message> {
        self.sock.recv(&mut self.msg, 0)?;
        if Some(WORK_ACK) != self.msg.as_str() {
            unreachable!("wk:run:1, received unexpected response from runner");
        }
        self.sock.recv(&mut self.msg, 0)?;
        Ok(&self.msg)
    }

    /// Wait until client receives a work complete message by the Runner and return an encoded
    /// representation of the job.
    pub fn recv_complete(&mut self) -> Result<&zmq::Message> {
        self.sock.recv(&mut self.msg, 0)?;
        if Some(WORK_COMPLETE) != self.msg.as_str() {
            unreachable!("wk:run:2, received unexpected response from runner");
        }
        self.sock.recv(&mut self.msg, 0)?;
        Ok(&self.msg)
    }

    /// Send a message to the Job Runner
    pub fn send(&mut self, msg: &zmq::Message) -> Result<()> {
        self.sock.send(&*msg, 0)?;
        Ok(())
    }
}

/// Receives work notifications from a `RunnerCli` and performs long-running tasks in a
/// separate thread.
pub struct RunnerMgr {
    sock: zmq::Socket,
    msg: zmq::Message,
    config: Arc<RwLock<Config>>,
}

impl RunnerMgr {
    /// Start the Job Runner
    pub fn start(config: Arc<RwLock<Config>>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let handle = thread::Builder::new()
            .name("runner".to_string())
            .spawn(move || {
                let mut runner = Self::new(config).unwrap();
                runner.run(tx).unwrap();
            })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("runner thread startup error, err={}", e),
        }
    }

    fn new(config: Arc<RwLock<Config>>) -> Result<Self> {
        let sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::DEALER)?;
        Ok(RunnerMgr {
            sock: sock,
            msg: zmq::Message::new().unwrap(),
            config: config,
        })
    }

    // Main loop for server
    fn run(&mut self, rz: mpsc::SyncSender<()>) -> Result<()> {
        self.sock.bind(INPROC_ADDR)?;
        rz.send(()).unwrap();
        loop {
            let job = self.recv_job()?;
            self.send_ack(&job)?;
            self.execute_job(job)?;
        }
    }

    fn execute_job(&mut self, job: Job) -> Result<()> {
        let runner = {
            Runner::new(job, (*self.config.read().unwrap()).clone())
        };
        debug!("executing work, job={:?}", runner.job());
        let job = runner.run();
        self.send_complete(&job)
    }

    fn recv_job(&mut self) -> Result<Job> {
        self.sock.recv(&mut self.msg, 0)?;
        let job: proto::Job = parse_from_bytes(&self.msg).unwrap();
        Ok(Job::new(job))
    }

    fn send_ack(&mut self, job: &Job) -> Result<()> {
        debug!("received work, job={:?}", job);
        self.sock.send_str(WORK_ACK, zmq::SNDMORE)?;
        self.sock.send(&*job.write_to_bytes().unwrap(), 0)?;
        Ok(())
    }

    fn send_complete(&mut self, job: &Job) -> Result<()> {
        debug!("work complete, job={:?}", job);
        self.sock.send_str(WORK_COMPLETE, zmq::SNDMORE)?;
        self.sock.send(&*job.write_to_bytes().unwrap(), 0)?;
        Ok(())
    }
}

fn studio_cmd() -> String {
    match PackageInstall::load(&STUDIO_PKG, None) {
        Ok(package) => format!("{}/hab-studio", package.paths().unwrap()[0].display()),
        Err(_) => {
            panic!(
                "core/hab-studio not found! This should be available as it is a runtime \
                    dependency in the worker's plan.sh and also present in our dev Dockerfile"
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{jobsrv, originsrv};

    #[test]
    fn extract_origin_from_job() {
        let mut inner = jobsrv::Job::new();
        let mut project = originsrv::OriginProject::new();
        project.set_name("core/nginx".to_string());
        inner.set_project(project);
        let job = Job::new(inner);
        assert_eq!(job.origin(), "core");
    }
}
