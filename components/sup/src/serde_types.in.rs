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

pub mod census {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `manager/census.rs`

    use std::collections::HashMap;

    use hcore::package::ident::PackageIdent;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Census {
        // JW TODO: This needs to become an Ordered HashMap keyed on member_id. This will reduce our
        // allocations when ordering the population to determine who should update next in a rolling
        // update strategy. For now, we allocate a new vector every server tick by the members() and
        // members_ordered() functions.
        pub population: HashMap<String, CensusEntry>,
        pub member_id: String,
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Default)]
    pub struct CensusEntry {
        pub member_id: Option<String>,
        pub service: Option<String>,
        pub group: Option<String>,
        pub org: Option<String>,
        pub hostname: Option<String>,
        pub address: Option<String>,
        pub ip: Option<String>,
        pub port: Option<String>,
        pub exposes: Vec<String>,
        pub package_ident: Option<PackageIdent>,
        pub leader: Option<bool>,
        pub follower: Option<bool>,
        pub update_leader: Option<bool>,
        pub update_follower: Option<bool>,
        pub election_is_running: Option<bool>,
        pub election_is_no_quorum: Option<bool>,
        pub election_is_finished: Option<bool>,
        pub update_election_is_running: Option<bool>,
        pub update_election_is_no_quorum: Option<bool>,
        pub update_election_is_finished: Option<bool>,
        pub initialized: Option<bool>,
        pub alive: Option<bool>,
        pub suspect: Option<bool>,
        pub confirmed: Option<bool>,
        pub persistent: Option<bool>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CensusList {
        pub censuses: HashMap<String, Census>,
    }
}

pub mod package {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `package/mod.rs`

    use hcore::package::{PackageIdent, PackageInstall};

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Package {
        pub origin: String,
        pub name: String,
        pub version: String,
        pub release: String,
        pub deps: Vec<PackageIdent>,
        pub tdeps: Vec<PackageIdent>,
        pub pkg_install: PackageInstall,
    }
}

pub mod service {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `manager/service/mod.rs`

    use std::collections::HashMap;

    use hcore::service::ServiceGroup;

    use package::Package;
    use supervisor::Supervisor;

    #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
    pub enum LastRestartDisplay {
        None,
        ElectionInProgress,
        ElectionNoQuorum,
        ElectionFinished,
    }

    #[derive(Debug, Serialize)]
    pub struct Service {
        pub needs_restart: bool,
        pub package: Package,
        pub service_config_incarnation: Option<u64>,
        pub service_group: ServiceGroup,
        pub topology: Topology,
        pub update_strategy: UpdateStrategy,
        pub current_service_files: HashMap<String, u64>,
        pub initialized: bool,
        pub last_restart_display: LastRestartDisplay,
        pub supervisor: Supervisor,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Copy, Deserialize, Serialize)]
    pub enum Topology {
        Standalone,
        Leader,
        Initializer,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
    pub enum UpdateStrategy {
        None,
        AtOnce,
        Rolling,
    }
}

pub mod service_config {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `manager/service/config.rs`

    use toml;

    /// The top level struct for all our configuration - this corresponds to the top level
    /// namespaces available in `config.toml`.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ServiceConfig {
        pub hab: Hab,
        pub pkg: Pkg,
        pub sys: Sys,
        pub cfg: Cfg,
        pub svc: Svc,
        pub bind: Bind,
        // Set to 'true' if we have data that needs to be sent to a configuration file
        #[serde(skip_deserializing)]
        pub needs_write: bool,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Bind(pub toml::Table);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Svc(pub toml::Table);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Cfg {
        pub default: Option<toml::Value>,
        pub user: Option<toml::Value>,
        pub gossip: Option<toml::Value>,
        pub environment: Option<toml::Value>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Pkg {
        pub origin: String,
        pub name: String,
        pub version: String,
        pub release: String,
        pub ident: String,
        pub deps: Vec<Pkg>,
        pub exposes: Vec<String>,
        pub path: String,
        pub svc_path: String,
        pub svc_config_path: String,
        pub svc_data_path: String,
        pub svc_files_path: String,
        pub svc_static_path: String,
        pub svc_var_path: String,
        pub svc_user: Option<String>,
        pub svc_group: Option<String>,
        pub svc_user_default: String,
        pub svc_group_default: String,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Sys {
        pub ip: String,
        pub hostname: String,
        pub sidecar_ip: String,
        pub sidecar_port: u16,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Hab {
        pub version: String,
    }
}

pub mod supervisor {
    // JW TODO: After updating to Rust 1.15, move the types contained in this module back into
    // `supervisor.rs`

    /// Additional params used to start the Supervisor.
    /// These params are outside the scope of what is in
    /// Supervisor.package_ident, and aren't runtime params that are stored
    /// in the top-level Supervisor struct (such as PID etc)
    #[derive(Debug, Deserialize, Serialize)]
    pub struct RuntimeConfig {
        pub svc_user: String,
        pub svc_group: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub enum ProcessState {
        Down,
        Up,
        Start,
        Restart,
    }
}
