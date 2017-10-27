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

pub mod studio;
mod docker;
mod log_pipe;
mod postprocessor;
mod publisher;
mod toml_builder;
mod util;
mod workspace;

use std::path::PathBuf;
use std::fs;
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};

pub use protocol::jobsrv::JobState;
use bldr_core::job::Job;
use bldr_core::logger::Logger;
use chrono::UTC;
use depot_client;
use hab_core::crypto;
use hab_core::package::archive::PackageArchive;
use hab_net::socket::DEFAULT_CONTEXT;
use protocol::{message, jobsrv as proto};
use protocol::originsrv::OriginPackageIdent;
use protocol::net::{self, ErrCode};
use zmq;

use {PRODUCT, VERSION};
use self::log_pipe::LogPipe;
use self::postprocessor::post_process;
use self::studio::Studio;
use self::docker::DockerExporter;
use self::workspace::Workspace;
use config::Config;
use error::{Error, Result};
use retry::retry;
use vcs::VCS;

// TODO fn: copied from `components/common/src/ui.rs`. As this component doesn't currently depend
// on habitat_common it didnt' seem worth it to add a dependency for only this constant. Probably
// means that the constant should be relocated to habitat_core.
/// Environment variable to disable progress bars in Habitat programs
const NONINTERACTIVE_ENVVAR: &'static str = "HAB_NONINTERACTIVE";

/// Environment variable to enable or disable debug output in runner's studio
const RUNNER_DEBUG_ENVVAR: &'static str = "BUILDER_RUNNER_DEBUG";
/// In-memory zmq address of Job RunnerMgr
const INPROC_ADDR: &'static str = "inproc://runner";
/// Protocol message to indicate the Job Runner has received a work request
const WORK_ACK: &'static str = "A";
/// Protocol message to indicate the Job Runner has completed a work request
const WORK_COMPLETE: &'static str = "C";

pub const RETRIES: u64 = 10;
pub const RETRY_WAIT: u64 = 60000;

pub struct Runner {
    config: Arc<Config>,
    depot_cli: depot_client::Client,
    workspace: Workspace,
    logger: Logger,
}

impl Runner {
    pub fn new(job: Job, config: Arc<Config>, net_ident: &str) -> Self {
        let depot_cli = depot_client::Client::new(&config.bldr_url, PRODUCT, VERSION, None)
            .unwrap();

        let log_path = config.log_path.clone();
        let mut logger = Logger::init(PathBuf::from(log_path), "builder-worker.log");
        logger.log_ident(net_ident);

        Runner {
            workspace: Workspace::new(&config.data_path, job),
            config: config,
            depot_cli: depot_cli,
            logger: logger,
        }
    }

    pub fn job(&self) -> &Job {
        &self.workspace.job
    }

    pub fn job_mut(&mut self) -> &mut Job {
        &mut self.workspace.job
    }

    pub fn run(mut self) -> Job {
        if let Some(err) = util::validate_integrations(&self.workspace).err() {
            let msg = format!(
                "Failed to validate integrations for {}, err={:?}",
                self.workspace.job.get_project().get_name(),
                err
            );
            debug!("{}", msg);
            self.logger.log(&msg);
            return self.fail(net::err(ErrCode::INVALID_INTEGRATIONS, "wk:run:7"));
        };
        if let Some(err) = self.setup().err() {
            let msg = format!(
                "Failed to setup workspace for {}, err={:?}",
                self.workspace.job.get_project().get_name(),
                err
            );
            warn!("{}", msg);
            self.logger.log(&msg);
            return self.fail(net::err(ErrCode::WORKSPACE_SETUP, "wk:run:1"));
        }
        if let Some(err) = self.install_origin_secret_key().err() {
            let msg = format!(
                "Failed to install origin secret key {}, err={:?}",
                self.workspace.job.get_project().get_origin_name(),
                err
            );
            debug!("{}", msg);
            self.logger.log(&msg);
            return self.fail(net::err(ErrCode::SECRET_KEY_FETCH, "wk:run:3"));
        }
        let vcs = VCS::from_job(&self.job(), self.config.github.clone());
        if let Some(err) = vcs.clone(&self.workspace.src()).err() {
            let msg = format!(
                "Failed to clone remote source repository for {}, err={:?}",
                self.workspace.job.get_project().get_name(),
                err
            );
            debug!("{}", msg);
            self.logger.log(&msg);
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
                let msg = format!(
                    "Failed studio build for {}, err={:?}",
                    self.workspace.job.get_project().get_name(),
                    err
                );
                debug!("{}", msg);
                self.logger.log(&msg);
                return self.fail(net::err(ErrCode::BUILD, "wk:run:5"));
            }
        };

        // Converting from a core::PackageIdent to an OriginPackageIdent
        let ident = OriginPackageIdent::from(archive.ident().unwrap());
        self.workspace.job.set_package_ident(ident);

        if !post_process(
            &mut archive,
            &self.workspace,
            &self.config,
            &mut self.logger,
        )
        {
            return self.fail(net::err(ErrCode::POST_PROCESSOR, "wk:run:6"));
        }

        if let Some(err) = fs::remove_dir_all(self.workspace.out()).err() {
            warn!(
                "Failed to delete directory during cleanup, dir={}, err={:?}",
                self.workspace.out().display(),
                err
            )
        }
        self.teardown().err().map(|e| error!("{}", e));
        self.complete()
    }

    fn install_origin_secret_key(&mut self) -> Result<()> {
        match retry(
            RETRIES,
            RETRY_WAIT,
            || {
                self.depot_cli.fetch_origin_secret_key(
                    self.job().origin(),
                    &self.config.auth_token,
                    &crypto::default_cache_key_path(None),
                )
            },
            |res| {
                if res.is_err() {
                    debug!("Failed to fetch origin secret key, err={:?}", res);
                };
                res.is_ok()
            },
        ) {
            Ok(res) => {
                debug!("Imported origin secret key, dst={:?}.", res.unwrap());
                Ok(())
            }
            Err(err) => {
                let msg = format!(
                    "Failed to import secret key {} after {} retries",
                    self.job().origin(),
                    RETRIES
                );
                debug!("{}", msg);
                self.logger.log(&msg);
                Err(Error::Retry(err))
            }
        }
    }

    fn build(&mut self) -> Result<PackageArchive> {
        let mut log_pipe = LogPipe::new(&self.workspace);
        log_pipe.pipe_stdout(b"\n--- BEGIN: Studio build ---\n")?;
        let mut status = Studio::new(
            &self.workspace,
            &self.config.bldr_url,
            &self.config.auth_token,
        ).build(&mut log_pipe)?;
        log_pipe.pipe_stdout(b"\n--- END: Studio build ---\n")?;

        if fs::rename(self.workspace.src().join("results"), self.workspace.out()).is_err() {
            return Err(Error::BuildFailure(status.code().unwrap_or(-2)));
        }

        if self.has_docker_integration() && status.success() {
            // TODO fn: This check should be updated in PackageArchive is check for run hooks.
            if self.workspace.last_built()?.is_a_service() {
                debug!("Found runnable package, running docker export");
                log_pipe.pipe_stdout(b"\n--- BEGIN: Docker export ---\n")?;
                status = DockerExporter::new(
                    util::docker_exporter_spec(&self.workspace),
                    &self.workspace,
                    &self.config.bldr_url,
                ).export(&mut log_pipe)?;
                log_pipe.pipe_stdout(b"\n--- END: Docker export ---\n")?;
            } else {
                debug!("Package not runnable, skipping docker export");
            }
        }

        if status.success() {
            self.workspace.last_built()
        } else {
            let ident = self.workspace.attempted_build()?;
            let op_ident = OriginPackageIdent::from(ident);
            self.workspace.job.set_package_ident(op_ident);
            Err(Error::BuildFailure(status.code().unwrap_or(-1)))
        }
    }

    fn complete(mut self) -> Job {
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

        if self.workspace.src().exists() {
            if let Some(err) = fs::remove_dir_all(self.workspace.src()).err() {
                warn!(
                    "Failed to delete directory during setup, dir={}, err={:?}",
                    self.workspace.src().display(),
                    err
                )
            }
        }
        if let Some(err) = fs::create_dir_all(self.workspace.src()).err() {
            return Err(Error::WorkspaceSetup(
                format!("{}", self.workspace.src().display()),
                err,
            ));
        }

        Ok(())
    }

    fn teardown(&mut self) -> Result<()> {
        let exit_status = Studio::new(
            &self.workspace,
            &self.config.bldr_url,
            &self.config.auth_token,
        ).rm()?;

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

        // TODO fn: purge the secret origin key from worker
    }

    /// Determines whether or not there is a Docker integration for the job.
    ///
    /// TODO fn: remember that for the time being we are only expecting a Docker export integration
    /// and we are assuming that any calls to this method will happen after the integration data
    /// has been validated.
    fn has_docker_integration(&self) -> bool {
        !self.workspace.job.get_project_integrations().is_empty()
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
        let sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER).unwrap();
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
    config: Arc<Config>,
    net_ident: Arc<String>,
    msg: zmq::Message,
    sock: zmq::Socket,
}

impl RunnerMgr {
    /// Start the Job Runner
    pub fn start(config: Arc<Config>, net_ident: Arc<String>) -> Result<JoinHandle<()>> {
        let (tx, rx) = mpsc::sync_channel(0);
        let mut runner = Self::new(config, net_ident).unwrap();
        let handle = thread::Builder::new()
            .name("runner".to_string())
            .spawn(move || { runner.run(tx).unwrap(); })
            .unwrap();
        match rx.recv() {
            Ok(()) => Ok(handle),
            Err(e) => panic!("runner thread startup error, err={}", e),
        }
    }

    fn new(config: Arc<Config>, net_ident: Arc<String>) -> Result<Self> {
        let sock = (**DEFAULT_CONTEXT).as_mut().socket(zmq::DEALER)?;
        Ok(RunnerMgr {
            config: config,
            msg: zmq::Message::new().unwrap(),
            net_ident: net_ident,
            sock: sock,
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
        let runner = Runner::new(job, self.config.clone(), &self.net_ident);
        debug!("Executing work, job={:?}", runner.job());
        let job = runner.run();
        self.send_complete(&job)
    }

    fn recv_job(&mut self) -> Result<Job> {
        self.sock.recv(&mut self.msg, 0)?;
        let job = message::decode::<proto::Job>(&self.msg)?;
        Ok(Job::new(job))
    }

    fn send_ack(&mut self, job: &Job) -> Result<()> {
        debug!("Received work, job={:?}", job);
        self.sock.send_str(WORK_ACK, zmq::SNDMORE)?;
        self.sock.send(&message::encode(&**job)?, 0)?;
        Ok(())
    }

    fn send_complete(&mut self, job: &Job) -> Result<()> {
        debug!("Completed work, job={:?}", job);
        self.sock.send_str(WORK_COMPLETE, zmq::SNDMORE)?;
        self.sock.send(&message::encode(&**job)?, 0)?;
        Ok(())
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
