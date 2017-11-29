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
mod scheduler;

use std::process;
use std::sync::RwLock;
use time::PreciseTime;

use hab_net::app::prelude::*;
use hab_net::conn::RouteClient;
use protocol::jobsrv::*;
use bldr_core::target_graph::TargetGraph;

use self::log_archiver::LogArchiver;
use self::log_directory::LogDirectory;
use self::log_ingester::LogIngester;
use self::worker_manager::{WorkerMgr, WorkerMgrClient};
use self::scheduler::{ScheduleMgr, ScheduleClient};
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
        map.register(JobGroupSpec::descriptor_static(None), handlers::job_group_create);
        map.register(JobGroupAbort::descriptor_static(None), handlers::job_group_abort);
        map.register(JobGroupCancel::descriptor_static(None), handlers::job_group_cancel);
        map.register(JobGroupGet::descriptor_static(None), handlers::job_group_get);
        map.register(JobGroupOriginGet::descriptor_static(None), handlers::job_group_origin_get);
        map.register(JobGraphPackageCreate::descriptor_static(None), handlers::job_graph_package_create);
        map.register(JobGraphPackagePreCreate::descriptor_static(None), handlers::job_graph_package_precreate);
        map.register(JobGraphPackageStatsGet::descriptor_static(None), handlers::job_graph_package_stats_get);
        map.register(JobGraphPackageReverseDependenciesGet::descriptor_static(None),
            handlers::job_graph_package_reverse_dependencies_get);
        map
    };
}

#[derive(Clone)]
pub struct InitServerState {
    archive_cfg: ArchiveCfg,
    datastore: DataStore,
    graph: Arc<RwLock<TargetGraph>>,
    log_dir: Arc<LogDirectory>,
}

impl InitServerState {
    fn new(cfg: Config, datastore: DataStore, graph: TargetGraph) -> Result<Self> {
        LogDirectory::validate(&cfg.log_dir)?;
        Ok(InitServerState {
            archive_cfg: cfg.archive,
            datastore: datastore,
            graph: Arc::new(RwLock::new(graph)),
            log_dir: Arc::new(LogDirectory::new(cfg.log_dir)),
        })
    }
}

pub struct ServerState {
    archiver: Box<LogArchiver>,
    datastore: DataStore,
    worker_mgr: WorkerMgrClient,
    graph: Arc<RwLock<TargetGraph>>,
    schedule_cli: ScheduleClient,
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
            graph: init_state.graph,
            schedule_cli: ScheduleClient::default(),
        };
        state.worker_mgr.connect()?;
        state.schedule_cli.connect()?;
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
        let action = config.action.clone();
        let datastore = DataStore::new(&config.datastore)?;

        if action == "migrate" {
            datastore.setup()?;
            println!("Migrations finished. Exiting.");
            process::exit(0);
        }

        let mut graph = TargetGraph::new();
        let packages = datastore.get_job_graph_packages()?;
        let start_time = PreciseTime::now();
        let res = graph.build(packages.into_iter());
        let end_time = PreciseTime::now();
        info!("Graph build stats ({} sec):", start_time.to(end_time));

        for stat in res {
            info!(
                "Target {}: {} nodes, {} edges",
                stat.target,
                stat.node_count,
                stat.edge_count,
            );
        }

        let state = InitServerState::new(config.clone(), datastore, graph)?;

        LogIngester::start(&config, state.log_dir.clone(), state.datastore.clone())?;
        let conn = RouteClient::new()?;
        conn.connect(&*router_pipe)?;
        WorkerMgr::start(&config, state.datastore.clone(), conn)?;
        ScheduleMgr::start(state.datastore.clone(), config.log_path, router_pipe)?;
        Ok(state)
    }

    fn dispatch_table() -> &'static DispatchTable<Self> {
        &DISPATCH_TABLE
    }
}

pub fn run(config: Config) -> AppResult<(), Error> {
    app_start::<JobSrv>(config)
}
