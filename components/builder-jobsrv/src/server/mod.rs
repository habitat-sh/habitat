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

pub mod log_archiver;
mod handlers;
mod worker_manager;
mod log_directory;
mod log_ingester;

use hab_net::app::prelude::*;
use hab_net::conn::RouteClient;
use protocol::jobsrv::*;

use self::log_archiver::LogArchiver;
use self::log_directory::LogDirectory;
use self::log_ingester::LogIngester;
use self::worker_manager::{WorkerMgr, WorkerMgrClient};
use config::{ArchiveCfg, Config};
use data_store::DataStore;
use error::{Error, Result};

lazy_static! {
    static ref DISPATCH_TABLE: DispatchTable<JobSrv> = {
        let mut map = DispatchTable::new();
        map.register(JobSpec::descriptor_static(None), handlers::job_create);
        map.register(JobGet::descriptor_static(None), handlers::job_get);
        map.register(ProjectJobsGet::descriptor_static(None), handlers::project_jobs_get);
        map.register(JobLogGet::descriptor_static(None), handlers::job_log_get);
        map
    };
}

#[derive(Clone)]
pub struct InitServerState {
    archive_cfg: ArchiveCfg,
    datastore: DataStore,
    log_dir: Arc<LogDirectory>,
}

impl InitServerState {
    fn new(cfg: Config, router_pipe: Arc<String>) -> Result<Self> {
        LogDirectory::validate(&cfg.log_dir)?;
        Ok(InitServerState {
            archive_cfg: cfg.archive,
            datastore: DataStore::new(&cfg.datastore, cfg.app.shards.unwrap(), router_pipe)?,
            log_dir: Arc::new(LogDirectory::new(cfg.log_dir)),
        })
    }
}

pub struct ServerState {
    archiver: Box<LogArchiver>,
    datastore: DataStore,
    worker_mgr: WorkerMgrClient,
    log_dir: Arc<LogDirectory>,
}

impl AppState for ServerState {
    type Error = Error;
    type InitState = InitServerState;

    fn build(init_state: Self::InitState) -> Result<Self> {
        let mut state = ServerState {
            archiver: log_archiver::from_config(&init_state.archive_cfg)?,
            datastore: init_state.datastore,
            log_dir: init_state.log_dir,
            worker_mgr: WorkerMgrClient::default(),
        };
        state.worker_mgr.connect()?;
        Ok(state)
    }
}

struct JobSrv;
impl Dispatcher for JobSrv {
    const APP_NAME: &'static str = "builder-jobsrv";
    const PROTOCOL: Protocol = Protocol::JobSrv;

    type Config = Config;
    type Error = Error;
    type State = ServerState;

    fn app_init(
        config: Self::Config,
        router_pipe: Arc<String>,
    ) -> Result<<Self::State as AppState>::InitState> {
        let state = InitServerState::new(config.clone(), router_pipe.clone())?;
        state.datastore.setup()?;
        state.datastore.start_async();
        LogIngester::start(&config, state.log_dir.clone(), state.datastore.clone())?;
        let conn = RouteClient::new()?;
        conn.connect(&*router_pipe)?;
        WorkerMgr::start(&config, state.datastore.clone(), conn)?;
        Ok(state)
    }

    fn dispatch_table() -> &'static DispatchTable<Self> {
        &DISPATCH_TABLE
    }
}

pub fn run(config: Config) -> AppResult<(), Error> {
    app_start::<JobSrv>(config)
}
