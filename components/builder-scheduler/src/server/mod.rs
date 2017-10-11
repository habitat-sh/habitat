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

mod handlers;
mod scheduler;

use std::sync::RwLock;
use time::PreciseTime;

use hab_net::app::prelude::*;
use protocol::scheduler::*;

use self::scheduler::{ScheduleMgr, ScheduleClient};
use bldr_core::target_graph::TargetGraph;
use config::Config;
use data_store::DataStore;
use error::{SrvError, SrvResult};

lazy_static! {
    static ref DISPATCH_TABLE: DispatchTable<SchedulerSrv> = {
        let mut map = DispatchTable::new();
        map.register(GroupCreate::descriptor_static(None), handlers::group_create);
        map.register(GroupGet::descriptor_static(None), handlers::group_get);
        map.register(PackageCreate::descriptor_static(None), handlers::package_create);
        map.register(PackagePreCreate::descriptor_static(None), handlers::package_precreate);
        map.register(JobStatus::descriptor_static(None), handlers::job_status);
        map.register(PackageStatsGet::descriptor_static(None), handlers::package_stats_get);
        map.register(ReverseDependenciesGet::descriptor_static(None),
            handlers::reverse_dependencies_get);
        map
    };
}

#[derive(Clone)]
pub struct InitServerState {
    datastore: DataStore,
    graph: Arc<RwLock<TargetGraph>>,
}

impl InitServerState {
    pub fn new(datastore: DataStore, graph: TargetGraph) -> Self {
        InitServerState {
            datastore: datastore,
            graph: Arc::new(RwLock::new(graph)),
        }
    }
}

pub struct ServerState {
    datastore: DataStore,
    graph: Arc<RwLock<TargetGraph>>,
    schedule_cli: ScheduleClient,
}

impl AppState for ServerState {
    type Error = SrvError;
    type InitState = InitServerState;

    fn build(init_state: Self::InitState) -> SrvResult<Self> {
        let mut state = ServerState {
            datastore: init_state.datastore,
            graph: init_state.graph,
            schedule_cli: ScheduleClient::default(),
        };
        state.schedule_cli.connect()?;
        Ok(state)
    }
}

struct SchedulerSrv;
impl Dispatcher for SchedulerSrv {
    const APP_NAME: &'static str = "builder-scheduler";
    const PROTOCOL: Protocol = Protocol::Scheduler;

    type Config = Config;
    type Error = SrvError;
    type State = ServerState;

    fn app_init(
        config: Self::Config,
        router_pipe: Arc<String>,
    ) -> SrvResult<<Self::State as AppState>::InitState> {
        let datastore = DataStore::new(&config.datastore)?;
        datastore.setup()?;
        let mut graph = TargetGraph::new();
        let packages = datastore.get_packages()?;
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
        let state = InitServerState::new(datastore, graph);
        ScheduleMgr::start(state.datastore.clone(), config.log_path, router_pipe)?;
        Ok(state)
    }

    fn dispatch_table() -> &'static DispatchTable<Self> {
        &DISPATCH_TABLE
    }
}

pub fn run(config: Config) -> AppResult<(), SrvError> {
    app_start::<SchedulerSrv>(config)
}
