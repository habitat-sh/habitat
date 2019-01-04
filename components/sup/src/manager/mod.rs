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

pub mod service;
#[macro_use]
mod debug;
pub mod commands;
mod events;
mod file_watcher;
mod peer_watcher;
mod periodic;
mod self_updater;
mod service_updater;
mod spec_dir;
mod spec_watcher;
mod sys;
mod user_config_watcher;

use std;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::mem;
use std::net::SocketAddr;
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

use num_cpus;

use butterfly;
use butterfly::member::Member;
use butterfly::server::{timing::Timing, ServerProxy, Suitability};
use butterfly::trace::Trace;
use futures::prelude::*;
use futures::sync::mpsc;
use hcore::crypto::SymKey;
use hcore::env;
use hcore::os::process::{self, Pid, Signal};
use hcore::os::signals::{self, SignalEvent};
use hcore::package::{Identifiable, PackageIdent, PackageInstall};
use hcore::service::ServiceGroup;
use launcher_client::{LauncherCli, LAUNCHER_LOCK_CLEAN_ENV, LAUNCHER_PID_ENV};
use protocol;
use rustls::{internal::pemfile, NoClientAuth, ServerConfig};
use serde_json;
use time::{self, Duration as TimeDuration, Timespec};
use tokio::{executor, runtime};

use self::peer_watcher::PeerWatcher;
use self::self_updater::{SelfUpdater, SUP_PKG_IDENT};
use self::service::{health::HealthCheck, DesiredState};
pub use self::service::{
    CompositeSpec, ConfigRendering, Service, ServiceProxy, ServiceSpec, Spec, Topology,
    UpdateStrategy,
};
use self::service_updater::ServiceUpdater;
use self::spec_dir::SpecDir;
use self::spec_watcher::SpecWatcher;

pub use self::sys::Sys;
use self::user_config_watcher::UserConfigWatcher;
use super::feat;
use census::{CensusRing, CensusRingProxy};
use common::types::EnvConfig;
use config::GossipListenAddr;
use ctl_gateway::{self, CtlRequest};
use error::{Error, Result, SupError};
use http_gateway;
use ShutdownReason;
use VERSION;

const MEMBER_ID_FILE: &'static str = "MEMBER_ID";
const PROC_LOCK_FILE: &'static str = "LOCK";

static LOGKEY: &'static str = "MR";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceOperation {
    Start(ServiceSpec),
    Stop(ServiceSpec),
    Restart {
        to_stop: ServiceSpec,
        to_start: ServiceSpec,
    },
}

/// FileSystem paths that the Manager uses to persist data to disk.
///
/// This is shared with the `http_gateway` and `service` modules for reading and writing
/// persistence data.
#[derive(Debug, Serialize)]
pub struct FsCfg {
    pub sup_root: PathBuf,

    data_path: PathBuf,
    specs_path: PathBuf,
    composites_path: PathBuf,
    member_id_file: PathBuf,
    proc_lock_file: PathBuf,
}

impl FsCfg {
    fn new<T>(sup_root: T) -> Self
    where
        T: Into<PathBuf>,
    {
        let sup_root = sup_root.into();
        FsCfg {
            specs_path: sup_root.join("specs"),
            data_path: sup_root.join("data"),
            composites_path: sup_root.join("composites"),
            member_id_file: sup_root.join(MEMBER_ID_FILE),
            proc_lock_file: sup_root.join(PROC_LOCK_FILE),
            sup_root: sup_root,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ManagerConfig {
    pub auto_update: bool,
    pub custom_state_path: Option<PathBuf>,
    pub eventsrv_group: Option<ServiceGroup>,
    pub update_url: String,
    pub update_channel: String,
    pub gossip_listen: GossipListenAddr,
    pub ctl_listen: SocketAddr,
    pub http_listen: http_gateway::ListenAddr,
    pub http_disable: bool,
    pub gossip_peers: Vec<SocketAddr>,
    pub gossip_permanent: bool,
    pub ring_key: Option<SymKey>,
    pub organization: Option<String>,
    pub watch_peer_file: Option<String>,
    pub tls_files: Option<(PathBuf, PathBuf)>,
}

impl ManagerConfig {
    pub fn sup_root(&self) -> PathBuf {
        protocol::sup_root(self.custom_state_path.as_ref())
    }
}

impl Default for ManagerConfig {
    fn default() -> Self {
        ManagerConfig {
            auto_update: false,
            custom_state_path: None,
            eventsrv_group: None,
            update_url: "".to_string(),
            update_channel: "".to_string(),
            gossip_listen: GossipListenAddr::default(),
            ctl_listen: protocol::ctl::default_addr(),
            http_listen: http_gateway::ListenAddr::default(),
            http_disable: false,
            gossip_peers: vec![],
            gossip_permanent: false,
            ring_key: None,
            organization: None,
            watch_peer_file: None,
            tls_files: None,
        }
    }
}

/// This represents an environment variable that holds an authentication token for the supervisor's
/// HTTP gateway. If the environment variable is present, then its value is the auth token and all
/// of the HTTP endpoints will require its presence. If it's not present, then everything continues
/// to work unauthenticated.
#[derive(Debug, Default)]
struct GatewayAuthToken(Option<String>);

impl FromStr for GatewayAuthToken {
    type Err = ::std::string::ParseError;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        Ok(GatewayAuthToken(Some(String::from(s))))
    }
}

impl EnvConfig for GatewayAuthToken {
    const ENVVAR: &'static str = "HAB_SUP_GATEWAY_AUTH_TOKEN";
}

/// This struct encapsulates the shared state for the supervisor. It's worth noting that if there's
/// something you want the CtlGateway to be able to operate on, it needs to be put in here. This
/// state gets shared with all the CtlGateway handlers.
pub struct ManagerState {
    /// The configuration used to instantiate this Manager instance
    pub cfg: ManagerConfig,
    pub services: Arc<RwLock<HashMap<PackageIdent, Service>>>,
    pub gateway_state: Arc<RwLock<GatewayState>>,
}

#[derive(Debug, Default)]
pub struct GatewayState {
    pub census_data: String,
    pub butterfly_data: String,
    pub services_data: String,
    pub health_check_data: HashMap<ServiceGroup, HealthCheck>,
    pub auth_token: Option<String>,
}

pub struct Manager {
    pub state: Arc<ManagerState>,
    butterfly: butterfly::Server,
    census_ring: CensusRing,
    events_group: Option<ServiceGroup>,
    fs_cfg: Arc<FsCfg>,
    launcher: LauncherCli,
    updater: ServiceUpdater,
    peer_watcher: Option<PeerWatcher>,
    spec_watcher: SpecWatcher,
    user_config_watcher: UserConfigWatcher,
    spec_dir: SpecDir,
    organization: Option<String>,
    self_updater: Option<SelfUpdater>,
    service_states: HashMap<PackageIdent, Timespec>,
    sys: Arc<Sys>,
    http_disable: bool,
}

impl Manager {
    /// Load a Manager with the given configuration.
    ///
    /// The returned Manager will be pre-populated with any cached data from disk from a previous
    /// run if available.
    pub fn load(cfg: ManagerConfig, launcher: LauncherCli) -> Result<Manager> {
        let state_path = cfg.sup_root();
        let fs_cfg = FsCfg::new(state_path);
        Self::create_state_path_dirs(&fs_cfg)?;
        Self::clean_dirty_state(&fs_cfg)?;
        if env::var(LAUNCHER_LOCK_CLEAN_ENV).is_ok() {
            release_process_lock(&fs_cfg);
        }
        obtain_process_lock(&fs_cfg)?;

        Self::new(cfg, fs_cfg, launcher)
    }

    pub fn term(cfg: &ManagerConfig) -> Result<()> {
        let fs_cfg = FsCfg::new(cfg.sup_root());
        match read_process_lock(&fs_cfg.proc_lock_file) {
            Ok(pid) => {
                process::signal(pid, Signal::TERM).map_err(|_| sup_error!(Error::SignalFailed))?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn new(cfg: ManagerConfig, fs_cfg: FsCfg, launcher: LauncherCli) -> Result<Manager> {
        debug!("new(cfg: {:?}, fs_cfg: {:?}", cfg, fs_cfg);
        let current = PackageIdent::from_str(&format!("{}/{}", SUP_PKG_IDENT, VERSION)).unwrap();
        outputln!("{} ({})", SUP_PKG_IDENT, current);
        let cfg_static = cfg.clone();
        let self_updater = if cfg.auto_update {
            if current.fully_qualified() {
                Some(SelfUpdater::new(
                    current,
                    cfg.update_url,
                    cfg.update_channel,
                ))
            } else {
                warn!("Supervisor version not fully qualified, unable to start self-updater");
                None
            }
        } else {
            None
        };
        let mut sys = Sys::new(
            cfg.gossip_permanent,
            cfg.gossip_listen,
            cfg.ctl_listen,
            cfg.http_listen,
        );
        let member = Self::load_member(&mut sys, &fs_cfg)?;
        let services = Arc::new(RwLock::new(HashMap::new()));

        let gateway_auth_token = GatewayAuthToken::configured_value();
        let mut gateway_state = GatewayState::default();
        gateway_state.auth_token = gateway_auth_token.0;

        let server = butterfly::Server::new(
            sys.gossip_listen(),
            sys.gossip_listen(),
            member,
            Trace::default(),
            cfg.ring_key,
            None,
            Some(&fs_cfg.data_path),
            Box::new(SuitabilityLookup(services.clone())),
        )?;
        outputln!("Supervisor Member-ID {}", sys.member_id);
        for peer_addr in &cfg.gossip_peers {
            let mut peer = Member::default();
            peer.address = format!("{}", peer_addr.ip());
            peer.swim_port = peer_addr.port();
            peer.gossip_port = peer_addr.port();
            server.member_list.add_initial_member(peer);
        }

        let peer_watcher = if let Some(path) = cfg.watch_peer_file {
            Some(PeerWatcher::run(path)?)
        } else {
            None
        };

        let spec_dir = SpecDir::new(&fs_cfg.specs_path)?;
        spec_dir.migrate_specs();

        let spec_watcher = SpecWatcher::run(&spec_dir)?;

        Ok(Manager {
            state: Arc::new(ManagerState {
                cfg: cfg_static,
                services: services,
                gateway_state: Arc::new(RwLock::new(gateway_state)),
            }),
            self_updater: self_updater,
            updater: ServiceUpdater::new(server.clone()),
            census_ring: CensusRing::new(sys.member_id.clone()),
            butterfly: server,
            events_group: cfg.eventsrv_group,
            launcher: launcher,
            peer_watcher: peer_watcher,
            spec_watcher: spec_watcher,
            user_config_watcher: UserConfigWatcher::new(),
            spec_dir: spec_dir,
            fs_cfg: Arc::new(fs_cfg),
            organization: cfg.organization,
            service_states: HashMap::new(),
            sys: Arc::new(sys),
            http_disable: cfg.http_disable,
        })
    }

    /// Load the initial Butterly Member which is used in initializing the Butterfly server. This
    /// will load the member-id for the initial Member from disk if a previous manager has been
    /// run.
    ///
    /// The mutable ref to `Sys` will be configured with Butterfly Member details and will also
    /// populate the initial Member.
    // TODO (CM): This functionality can / should be pulled into
    // Butterfly itself; we're already setting the incarnation number
    // in there, so splitting the initialization is needlessly
    // confusing. It's also blurs the lines between the manager and
    // Butterfly.
    fn load_member(sys: &mut Sys, fs_cfg: &FsCfg) -> Result<Member> {
        let mut member = Member::default();
        match File::open(&fs_cfg.member_id_file) {
            Ok(mut file) => {
                let mut member_id = String::new();
                file.read_to_string(&mut member_id).map_err(|e| {
                    sup_error!(Error::BadDataFile(fs_cfg.member_id_file.clone(), e))
                })?;
                member.id = member_id;
            }
            Err(_) => match File::create(&fs_cfg.member_id_file) {
                Ok(mut file) => {
                    file.write(member.id.as_bytes()).map_err(|e| {
                        sup_error!(Error::BadDataFile(fs_cfg.member_id_file.clone(), e))
                    })?;
                }
                Err(err) => {
                    return Err(sup_error!(Error::BadDataFile(
                        fs_cfg.member_id_file.clone(),
                        err
                    )))
                }
            },
        }
        sys.member_id = member.id.to_string();
        member.persistent = sys.permanent;
        Ok(member)
    }

    fn clean_dirty_state(fs_cfg: &FsCfg) -> Result<()> {
        let data_path = &fs_cfg.data_path;
        debug!("Cleaning cached health checks");
        match fs::read_dir(&data_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        match entry.path().extension().and_then(|p| p.to_str()) {
                            Some("tmp") | Some("health") => {
                                fs::remove_file(&entry.path()).map_err(|err| {
                                    sup_error!(Error::BadDataPath(data_path.clone(), err))
                                })?;
                            }
                            _ => continue,
                        }
                    }
                }
                Ok(())
            }
            Err(err) => Err(sup_error!(Error::BadDataPath(data_path.clone(), err))),
        }
    }

    fn create_state_path_dirs(fs_cfg: &FsCfg) -> Result<()> {
        let data_path = &fs_cfg.data_path;
        debug!("Creating data directory: {}", data_path.display());
        if let Some(err) = fs::create_dir_all(&data_path).err() {
            return Err(sup_error!(Error::BadDataPath(data_path.clone(), err)));
        }
        let specs_path = &fs_cfg.specs_path;
        debug!("Creating specs directory: {}", specs_path.display());
        if let Some(err) = fs::create_dir_all(&specs_path).err() {
            return Err(sup_error!(Error::BadSpecsPath(specs_path.clone(), err)));
        }

        let composites_path = &fs_cfg.composites_path;
        debug!(
            "Creating composites directory: {}",
            composites_path.display()
        );
        if let Some(err) = fs::create_dir_all(&composites_path).err() {
            return Err(sup_error!(Error::BadCompositesPath(
                composites_path.clone(),
                err
            )));
        }

        Ok(())
    }

    fn add_service(&mut self, spec: ServiceSpec) {
        // JW TODO: This clone sucks, but our data structures are a bit messy here. What we really
        // want is the service to hold the spec and, on failure, return an error with the spec
        // back to us. Since we consume and deconstruct the spec in `Service::new()` which
        // `Service::load()` eventually delegates to we just can't have that. We should clean
        // this up in the future.
        let service = match Service::load(
            self.sys.clone(),
            spec.clone(),
            self.fs_cfg.clone(),
            self.organization.as_ref().map(|org| &**org),
            self.state.gateway_state.clone(),
        ) {
            Ok(service) => {
                outputln!("Starting {} ({})", &spec.ident, service.pkg.ident);
                service
            }
            Err(err) => {
                outputln!("Unable to start {}, {}", &spec.ident, err);
                return;
            }
        };

        if let Err(e) = service.create_svc_path() {
            outputln!(
                "Can't create directory {}: {}",
                service.pkg.svc_path.display(),
                e
            );
            outputln!(
                "If this service is running as non-root, you'll need to create \
                 {} and give the current user write access to it",
                service.pkg.svc_path.display()
            );
            outputln!("{} failed to start", &spec.ident);
            return;
        }

        self.gossip_latest_service_rumor(&service);
        if service.topology == Topology::Leader {
            self.butterfly.start_election(&service.service_group, 0);
        }

        if let Err(e) = self.user_config_watcher.add(&service) {
            outputln!(
                "Unable to start UserConfigWatcher for {}: {}",
                service.spec_ident,
                e
            );
            return;
        }

        self.updater.add(&service);
        self.state
            .services
            .write()
            .expect("Services lock is poisoned!")
            .insert(service.spec_ident.clone(), service);
    }

    pub fn run(mut self, svc: Option<protocol::ctl::SvcLoad>) -> Result<()> {
        let mut runtime = runtime::Builder::new()
            .name_prefix("tokio-")
            .core_threads(TokioThreadCount::configured_value().into())
            .build()
            .expect("Couldn't build Tokio Runtime!");

        let (ctl_tx, ctl_rx) = mpsc::unbounded();
        let ctl_handler = CtlAcceptor::new(self.state.clone(), ctl_rx).for_each(move |handler| {
            executor::spawn(handler);
            Ok(())
        });

        runtime.spawn(ctl_handler);

        if let Some(svc_load) = svc {
            commands::service_load(&self.state, &mut CtlRequest::default(), svc_load)?;
        }
        // This serves to start up any services that need starting
        self.take_action_on_services()?;

        outputln!(
            "Starting gossip-listener on {}",
            self.butterfly.gossip_addr()
        );
        self.butterfly.start(Timing::default())?;
        debug!("gossip-listener started");
        self.persist_state();
        let http_listen_addr = self.sys.http_listen();
        let ctl_listen_addr = self.sys.ctl_listen();
        let ctl_secret_key = ctl_gateway::readgen_secret_key(&self.fs_cfg.sup_root)?;
        outputln!("Starting ctl-gateway on {}", &ctl_listen_addr);
        ctl_gateway::server::run(ctl_listen_addr, ctl_secret_key, ctl_tx);
        debug!("ctl-gateway started");

        if self.http_disable {
            info!("http-gateway disabled");
        } else {
            // First let's check and see if we're going to use TLS. If so, we'll generate the
            // appropriate config here, where it's easy to propagate errors, vs in a separate
            // thread, where that process is more cumbersome.

            let tls_server_config = match self.state.cfg.tls_files {
                Some((ref key_path, ref cert_path)) => match tls_config(key_path, cert_path) {
                    Ok(c) => Some(c),
                    Err(e) => return Err(e),
                },
                None => None,
            };

            // Here we use a Condvar to wait on the HTTP gateway server to start up and inspect its
            // return value. Specifically, we're looking for errors when it tries to bind to the
            // listening TCP socket, so we can alert the user.
            let pair = Arc::new((
                Mutex::new(http_gateway::ServerStartup::NotStarted),
                Condvar::new(),
            ));

            outputln!("Starting http-gateway on {}", &http_listen_addr);
            http_gateway::Server::run(
                http_listen_addr.clone(),
                tls_server_config,
                self.state.gateway_state.clone(),
                pair.clone(),
            );

            let &(ref lock, ref cvar) = &*pair;
            let mut started = lock.lock().expect("Control mutex is poisoned");

            // This will block the current thread until the HTTP gateway thread either starts
            // successfully or fails to bind. In practice, the wait here is so short as to not be
            // noticeable.
            loop {
                match *started {
                    http_gateway::ServerStartup::NotStarted => {
                        started = match cvar.wait_timeout(started, Duration::from_millis(10000)) {
                            Ok((mutex, timeout_result)) => {
                                if timeout_result.timed_out() {
                                    return Err(sup_error!(Error::BindTimeout(
                                        http_listen_addr.to_string()
                                    )));
                                } else {
                                    mutex
                                }
                            }
                            Err(e) => {
                                error!("Mutex for the HTTP gateway was poisoned. e = {:?}", e);
                                return Err(sup_error!(Error::LockPoisoned));
                            }
                        };
                    }
                    http_gateway::ServerStartup::BindFailed => {
                        return Err(sup_error!(Error::BadAddress(http_listen_addr.to_string())));
                    }
                    http_gateway::ServerStartup::Started => break,
                }
            }

            debug!("http-gateway started");
        }

        let events = match self.events_group {
            Some(ref evg) => Some(events::EventsMgr::start(evg.clone())),
            None => None,
        };
        // On Windows initializng the signal handler will create a ctrl+c handler for the
        // process which will disable default windows ctrl+c behavior and allow us to
        // handle via check_for_signal. However, if the supervsor is in a long running
        // non-run hook, the below loop will not get to check_for_signal in a reasonable
        // amount of time and the supervisor will not respond to ctrl+c. On Windows, we
        // let the launcher catch ctrl+c and gracefully shut down services. ctrl+c should
        // simply halt the supervisor
        if !feat::is_enabled(feat::IgnoreSignals) {
            signals::init();
        }

        loop {
            if feat::is_enabled(feat::TestExit) {
                if let Ok(exit_file_path) = env::var("HAB_FEAT_TEST_EXIT") {
                    if let Ok(mut exit_code_file) = File::open(&exit_file_path) {
                        let mut buffer = String::new();
                        exit_code_file
                            .read_to_string(&mut buffer)
                            .expect("couldn't read");
                        if let Ok(exit_code) = buffer.lines().next().unwrap_or("").parse::<i32>() {
                            fs::remove_file(&exit_file_path).expect("couldn't remove");
                            outputln!("Simulating abrupt, unexpected exit with code {}", exit_code);
                            std::process::exit(exit_code);
                        }
                    }
                }
            }

            let next_check = time::get_time() + TimeDuration::milliseconds(1000);
            if self.launcher.is_stopping() {
                self.shutdown(ShutdownReason::LauncherStopping);
                return Ok(());
            }
            if self.check_for_departure() {
                self.shutdown(ShutdownReason::Departed);
                return Err(sup_error!(Error::Departed));
            }
            if !feat::is_enabled(feat::IgnoreSignals) {
                if let Some(SignalEvent::Passthrough(Signal::HUP)) = signals::check_for_signal() {
                    outputln!("Supervisor shutting down for signal");
                    self.shutdown(ShutdownReason::Signal);
                    return Ok(());
                }
            }
            if let Some(package) = self.check_for_updated_supervisor() {
                outputln!(
                    "Supervisor shutting down for automatic update to {}",
                    package
                );
                self.shutdown(ShutdownReason::PkgUpdating);
                return Ok(());
            }

            if self.spec_watcher.has_events() {
                self.take_action_on_services()?;
            }

            self.update_peers_from_watch_file()?;
            self.update_running_services_from_user_config_watcher();
            self.check_for_updated_packages();
            self.restart_elections();
            self.census_ring.update_from_rumors(
                &self.butterfly.service_store,
                &self.butterfly.election_store,
                &self.butterfly.update_store,
                &self.butterfly.member_list,
                &self.butterfly.service_config_store,
                &self.butterfly.service_file_store,
            );

            if self.check_for_changed_services() {
                self.persist_state();
            }

            if self.census_ring.changed() {
                self.persist_state();
                events
                    .as_ref()
                    .map(|events| events.try_connect(&self.census_ring));

                for service in self
                    .state
                    .services
                    .read()
                    .expect("Services lock is poisoned!")
                    .values()
                {
                    if let Some(census_group) =
                        self.census_ring.census_group_for(&service.service_group)
                    {
                        if let Some(member) = census_group.me() {
                            events
                                .as_ref()
                                .map(|events| events.send_service(member, service));
                        }
                    }
                }
            }

            for service in self
                .state
                .services
                .write()
                .expect("Services lock is poisoned!")
                .values_mut()
            {
                if service.tick(&self.census_ring, &self.launcher) {
                    self.gossip_latest_service_rumor(&service);
                }
            }

            // This is really only needed until everything is running
            // in futures.
            let now = time::get_time();
            if now < next_check {
                let time_to_wait = next_check - now;
                thread::sleep(time_to_wait.to_std().unwrap());
            }
        }
    }

    fn check_for_updated_supervisor(&mut self) -> Option<PackageInstall> {
        if let Some(ref mut updater) = self.self_updater {
            return updater.updated();
        }
        None
    }

    /// Walk each service and check if it has an updated package installed via the Update Strategy.
    /// This updates the Service to point to the new service struct, and then marks it for
    /// restarting.
    ///
    /// The run loop's last updated census is a required parameter on this function to inform the
    /// main loop that we, ourselves, updated the service counter when we updated ourselves.
    fn check_for_updated_packages(&mut self) {
        for service in self
            .state
            .services
            .write()
            .expect("Services lock is poisoned!")
            .values_mut()
        {
            if self
                .updater
                .check_for_updated_package(service, &self.census_ring, &self.launcher)
            {
                self.gossip_latest_service_rumor(&service);
            }
        }
    }

    // Creates a rumor for the specified service.
    fn gossip_latest_service_rumor(&self, service: &Service) {
        let incarnation = if let Some(rumor) = self
            .butterfly
            .service_store
            .list
            .read()
            .expect("Rumor store lock poisoned")
            .get(&*service.service_group)
            .and_then(|r| r.get(&self.sys.member_id))
        {
            rumor.clone().incarnation + 1
        } else {
            1
        };

        self.butterfly.insert_service(service.to_rumor(incarnation));
    }

    fn check_for_departure(&self) -> bool {
        self.butterfly.is_departed()
    }

    fn check_for_changed_services(&mut self) -> bool {
        let mut service_states = HashMap::new();
        let mut active_services = Vec::new();
        for service in self
            .state
            .services
            .write()
            .expect("Services lock is poisoned!")
            .values_mut()
        {
            service_states.insert(service.spec_ident.clone(), service.last_state_change());
            active_services.push(service.spec_ident.clone());
        }

        for loaded in self
            .spec_dir
            .specs()
            .unwrap()
            .iter()
            .filter(|s| !active_services.contains(&s.ident))
        {
            service_states.insert(loaded.ident.clone(), Timespec::new(0, 0));
        }

        if service_states != self.service_states {
            self.service_states = service_states.clone();
            true
        } else {
            false
        }
    }

    fn persist_state(&self) {
        debug!("Updating census state");
        self.persist_census_state();
        debug!("Updating butterfly state");
        self.persist_butterfly_state();
        debug!("Updating services state");
        self.persist_services_state();
    }

    fn persist_census_state(&self) {
        let crp = CensusRingProxy::new(&self.census_ring);
        let json = serde_json::to_string(&crp).unwrap();
        self.state
            .gateway_state
            .write()
            .expect("GatewayState lock is poisoned")
            .census_data = json;
    }

    fn persist_butterfly_state(&self) {
        let bs = ServerProxy::new(&self.butterfly);
        let json = serde_json::to_string(&bs).unwrap();
        self.state
            .gateway_state
            .write()
            .expect("GatewayState lock is poisoned")
            .butterfly_data = json;
    }

    fn persist_services_state(&self) {
        let config_rendering = if feat::is_enabled(feat::RedactHTTP) {
            ConfigRendering::Redacted
        } else {
            ConfigRendering::Full
        };

        let services = self
            .state
            .services
            .read()
            .expect("Services lock is poisoned!");
        let existing_idents: Vec<PackageIdent> =
            services.values().map(|s| s.spec_ident.clone()).collect();

        // Services that are not active but are being watched for changes
        // These would include stopped persistent services or other
        // persistent services that failed to load
        let watched_services: Vec<Service> = self
            .spec_dir
            .specs()
            .unwrap()
            .iter()
            .filter(|spec| !existing_idents.contains(&spec.ident))
            .flat_map(|spec| {
                Service::load(
                    self.sys.clone(),
                    spec.clone(),
                    self.fs_cfg.clone(),
                    self.organization.as_ref().map(|org| &**org),
                    self.state.gateway_state.clone(),
                )
                .into_iter()
            })
            .collect();
        let watched_service_proxies: Vec<ServiceProxy> = watched_services
            .iter()
            .map(|s| ServiceProxy::new(s, config_rendering))
            .collect();
        let mut services_to_render: Vec<ServiceProxy> = services
            .values()
            .map(|s| ServiceProxy::new(s, config_rendering))
            .collect();

        services_to_render.extend(watched_service_proxies);

        let json = serde_json::to_string(&services_to_render).unwrap();
        self.state
            .gateway_state
            .write()
            .expect("GatewayState lock is poisoned")
            .services_data = json;
    }

    /// Remove the given service from the manager.
    ///
    /// Passing `true` for the term argument will also request the Launcher to terminate the running
    /// service. Passing a value of `false` will let the Launcher keep the service running. This
    /// useful if you want the Supervisor to shutdown temporarily and then come back and re-attach
    /// to all running processes.
    fn remove_service(&mut self, service: &mut Service, cause: ShutdownReason) {
        // JW TODO: Update service rumor to remove service from cluster
        let term = match cause {
            ShutdownReason::LauncherStopping | ShutdownReason::SvcStopCmd => true,
            _ => false,
        };

        if term {
            service.stop(&self.launcher, cause);
        }

        if let Err(_) = self.user_config_watcher.remove(service) {
            debug!(
                "Error stopping user-config watcher thread for service {}",
                service
            );
        }

        self.updater.remove(service);
    }

    /// Check if any elections need restarting.
    fn restart_elections(&mut self) {
        self.butterfly.restart_elections();
    }

    fn shutdown(&mut self, cause: ShutdownReason) {
        match cause {
            ShutdownReason::PkgUpdating | ShutdownReason::Signal => {
                // Previously, we would unconditionally set our health
                // to departed. However, given our current conflation
                // of Supervisor reachability and Service health, I
                // suspect this causes instability when simply
                // shutting down the Supervisor for a Supervisor
                // upgrade.
                //
                // In the future, we may want to notify the rest of
                // the network more formally of our intent to update,
                // but for now, NOT convincing everyone that our
                // services are gone seems like a good intermediate
                // step. (We should also, of course, stop conflating
                // Supervisor reachability and Service health!) Our
                // existing suspicion mechanism should serve us well
                // here if it takes longer than expected for the new
                // Supervisor to come back up.
                //
                // Thus, for these given ShutdownReasons, we just
                // won't send any Membership rumors out.
            }
            ShutdownReason::SvcStopCmd => {
                // Just to call it out specifically, we shouldn't ever
                // be called with this ShutdownReason.
                //
                // This is all being refactored elsewhere right now,
                // for what it's worth.
            }
            ShutdownReason::LauncherStopping | ShutdownReason::Departed => {
                // On the other hand, if we legitimately are going
                // away, tell people, even though sending something
                // out when _we've already been manually departed_ is
                // perhaps excessive.
                outputln!("Gracefully departing from butterfly network.");
                self.butterfly.set_departed();
            }
        }

        let mut svcs = HashMap::new();

        // The problem we're trying to work around here by adding this block is that `write`
        // creates an immutable borrow on `self`, and `self.remove_service` needs `&mut self`.
        // The solution is to introduce the block to drop the immutable borrow before the call to
        // `self.remove_service`, and use `mem::swap` to move the services to a variable defined
        // outside the block while we have the lock.
        {
            let mut services = self
                .state
                .services
                .write()
                .expect("Services lock is poisoned!");
            mem::swap(services.deref_mut(), &mut svcs);
        }

        for mut service in svcs.drain().map(|(_ident, service)| service) {
            self.remove_service(&mut service, cause);
        }
        release_process_lock(&self.fs_cfg);

        self.butterfly.persist_data();
    }

    /// Start, stop, or restart services to bring what's running in
    /// line with what our spec files say.
    fn take_action_on_services(&mut self) -> Result<()> {
        for op in self.reconcile_spec_files()? {
            match op {
                ServiceOperation::Stop(spec) => {
                    self.remove_service_for_spec(&spec);
                }
                ServiceOperation::Start(spec) => {
                    self.add_service(spec);
                }
                ServiceOperation::Restart {
                    to_stop: running,
                    to_start: desired,
                } => {
                    self.remove_service_for_spec(&running);
                    self.add_service(desired);
                }
            }
        }
        Ok(())
    }

    /// Determine what services we need to start, stop, or restart in
    /// order to be running what our on-disk spec files tell us we
    /// should be running.
    ///
    /// See `specs_to_operations` for the real logic.
    fn reconcile_spec_files(&mut self) -> Result<Vec<ServiceOperation>> {
        let services = self
            .state
            .services
            .read()
            .expect("Services lock is poisoned");
        let currently_running_specs = services.values().map(|s| s.to_spec());
        let on_disk_specs = self.spec_dir.specs()?;
        Ok(Self::specs_to_operations(
            currently_running_specs,
            on_disk_specs,
        ))
    }

    /// Pure utility function to generate a list of operations to
    /// perform to bring what's currently running with what _should_ be
    /// running, based on the current on-disk spec files.
    fn specs_to_operations<C, D>(
        currently_running_specs: C,
        on_disk_specs: D,
    ) -> Vec<ServiceOperation>
    where
        C: IntoIterator<Item = ServiceSpec>,
        D: IntoIterator<Item = ServiceSpec>,
    {
        let mut svc_states = HashMap::new();

        #[derive(Default)]
        struct ServiceState {
            running: Option<ServiceSpec>,
            disk: Option<(DesiredState, ServiceSpec)>,
        }

        for rs in currently_running_specs {
            svc_states.insert(
                rs.ident.clone(),
                ServiceState {
                    running: Some(rs),
                    disk: None,
                },
            );
        }

        for ds in on_disk_specs {
            let ident = ds.ident.clone();
            svc_states
                .entry(ident)
                .or_insert(ServiceState::default())
                .disk = Some((ds.desired_state, ds));
        }

        svc_states
            .into_iter()
            .filter_map(|(ident, ss)| match ss {
                ServiceState {
                    disk: Some((DesiredState::Up, disk_spec)),
                    running: None,
                } => {
                    debug!("Reconciliation: '{}' queued for start", ident);
                    Some(ServiceOperation::Start(disk_spec))
                }

                ServiceState {
                    disk: Some((DesiredState::Up, disk_spec)),
                    running: Some(running_spec),
                } => {
                    if running_spec == disk_spec {
                        debug!("Reconciliation: '{}' unchanged", ident);
                        None
                    } else {
                        // TODO (CM): In the future, this would be the
                        // place where we can evaluate what has changed
                        // between the spec-on-disk and our in-memory
                        // representation and potentially just bring our
                        // in-memory representation in line without having
                        // to restart the entire service.
                        debug!("Reconciliation: '{}' queued for restart", ident);
                        Some(ServiceOperation::Restart {
                            to_stop: running_spec,
                            to_start: disk_spec,
                        })
                    }
                }

                ServiceState {
                    disk: Some((DesiredState::Down, _)),
                    running: Some(running_spec),
                } => {
                    debug!("Reconciliation: '{}' queued for stop", ident);
                    Some(ServiceOperation::Stop(running_spec))
                }

                ServiceState {
                    disk: Some((DesiredState::Down, _)),
                    running: None,
                } => {
                    debug!("Reconciliation: '{}' should be down, and is", ident);
                    None
                }

                ServiceState {
                    disk: None,
                    running: Some(running_spec),
                } => {
                    debug!("Reconciliation: '{}' queued for shutdown", ident);
                    Some(ServiceOperation::Stop(running_spec))
                }

                ServiceState {
                    disk: None,
                    running: None,
                } => unreachable!(),
            })
            .collect()
    }

    fn update_peers_from_watch_file(&mut self) -> Result<()> {
        if !self.butterfly.need_peer_seeding() {
            return Ok(());
        }
        match self.peer_watcher {
            None => Ok(()),
            Some(ref watcher) => {
                if watcher.has_fs_events() {
                    let members = watcher.get_members()?;
                    self.butterfly.member_list.set_initial_members(members);
                }
                Ok(())
            }
        }
    }

    fn update_running_services_from_user_config_watcher(&mut self) {
        let mut services = self
            .state
            .services
            .write()
            .expect("Services lock is poisoned");
        for service in services.values_mut() {
            if self.user_config_watcher.have_events_for(service) {
                outputln!("user.toml changes detected for {}", &service.spec_ident);
                service.user_config_updated = true;
            }
        }
    }

    fn remove_service_for_spec(&mut self, spec: &ServiceSpec) {
        let svc = self
            .state
            .services
            .write()
            .expect("Services lock is poisoned")
            .remove(&spec.ident);
        match svc {
            Some(mut service) => {
                self.remove_service(&mut service, ShutdownReason::SvcStopCmd);
            }
            None => {
                outputln!(
                    "Tried to remove service for {} but could not find it running, skipping",
                    &spec.ident
                );
            }
        }
    }
}

fn tls_config<A, B>(key_path: A, cert_path: B) -> Result<ServerConfig>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    let mut config = ServerConfig::new(NoClientAuth::new());
    let key_file = &mut BufReader::new(File::open(&key_path)?);
    let cert_file = &mut BufReader::new(File::open(&cert_path)?);

    // Note that we must explicitly map these errors because rustls returns () as the error from both
    // pemfile::certs() as well as pemfile::rsa_private_keys() and we want to return different errors
    // for each.
    let cert_chain = pemfile::certs(cert_file)
        .and_then(|c| if c.is_empty() { Err(()) } else { Ok(c) })
        .map_err(|_| sup_error!(Error::InvalidCertFile(cert_path.as_ref().to_path_buf())))?;

    let key = pemfile::rsa_private_keys(key_file)
        .and_then(|mut k| k.pop().ok_or(()))
        .map_err(|_| sup_error!(Error::InvalidKeyFile(key_path.as_ref().to_path_buf())))?;

    config.set_single_cert(cert_chain, key)?;
    config.ignore_client_order = true;
    Ok(config)
}

/// Represents how many threads to start for our main Tokio runtime
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
struct TokioThreadCount(usize);

impl Default for TokioThreadCount {
    fn default() -> Self {
        // This is the same internal logic used in Tokio itself.
        // https://docs.rs/tokio/0.1.12/src/tokio/runtime/builder.rs.html#68
        TokioThreadCount(num_cpus::get().max(1))
    }
}

impl FromStr for TokioThreadCount {
    type Err = Error;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let raw = s
            .parse::<usize>()
            .map_err(|_| Error::InvalidTokioThreadCount)?;
        if raw > 0 {
            Ok(TokioThreadCount(raw))
        } else {
            Err(Error::InvalidTokioThreadCount)
        }
    }
}

impl EnvConfig for TokioThreadCount {
    const ENVVAR: &'static str = "HAB_TOKIO_THREAD_COUNT";
}

impl Into<usize> for TokioThreadCount {
    fn into(self) -> usize {
        self.0
    }
}

#[derive(Debug)]
struct SuitabilityLookup(Arc<RwLock<HashMap<PackageIdent, Service>>>);

impl Suitability for SuitabilityLookup {
    fn get(&self, service_group: &str) -> u64 {
        self.0
            .read()
            .expect("Services lock is poisoned!")
            .values()
            .find(|s| *s.service_group == service_group)
            .and_then(|s| s.suitability())
            .unwrap_or(u64::min_value())
    }
}

fn obtain_process_lock(fs_cfg: &FsCfg) -> Result<()> {
    match write_process_lock(&fs_cfg.proc_lock_file) {
        Ok(()) => Ok(()),
        Err(_) => match read_process_lock(&fs_cfg.proc_lock_file) {
            Ok(pid) => {
                if process::is_alive(pid) {
                    return Err(sup_error!(Error::ProcessLocked(pid)));
                }
                release_process_lock(&fs_cfg);
                write_process_lock(&fs_cfg.proc_lock_file)
            }
            Err(SupError {
                err: Error::ProcessLockCorrupt,
                ..
            }) => {
                release_process_lock(&fs_cfg);
                write_process_lock(&fs_cfg.proc_lock_file)
            }
            Err(err) => Err(err),
        },
    }
}

fn read_process_lock<T>(lock_path: T) -> Result<Pid>
where
    T: AsRef<Path>,
{
    match File::open(lock_path.as_ref()) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => match line.parse::<Pid>() {
                    Ok(pid) => Ok(pid),
                    Err(_) => Err(sup_error!(Error::ProcessLockCorrupt)),
                },
                _ => Err(sup_error!(Error::ProcessLockCorrupt)),
            }
        }
        Err(err) => Err(sup_error!(Error::ProcessLockIO(
            lock_path.as_ref().to_path_buf(),
            err
        ))),
    }
}

fn release_process_lock(fs_cfg: &FsCfg) {
    if let Err(err) = fs::remove_file(&fs_cfg.proc_lock_file) {
        debug!("Couldn't cleanup Supervisor process lock, {}", err);
    }
}

fn write_process_lock<T>(lock_path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(lock_path.as_ref())
    {
        Ok(mut file) => {
            let pid = match env::var(LAUNCHER_PID_ENV) {
                Ok(pid) => pid.parse::<Pid>().expect("Unable to parse launcher pid"),
                Err(_) => process::current_pid(),
            };
            match write!(&mut file, "{}", pid) {
                Ok(()) => Ok(()),
                Err(err) => Err(sup_error!(Error::ProcessLockIO(
                    lock_path.as_ref().to_path_buf(),
                    err
                ))),
            }
        }
        Err(err) => Err(sup_error!(Error::ProcessLockIO(
            lock_path.as_ref().to_path_buf(),
            err
        ))),
    }
}

struct CtlAcceptor {
    rx: ctl_gateway::server::MgrReceiver,
    state: Arc<ManagerState>,
}

impl CtlAcceptor {
    fn new(state: Arc<ManagerState>, rx: ctl_gateway::server::MgrReceiver) -> Self {
        CtlAcceptor {
            state: state,
            rx: rx,
        }
    }
}

impl Stream for CtlAcceptor {
    type Item = CtlHandler;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.rx.poll() {
            Ok(Async::Ready(Some(cmd))) => {
                let task = CtlHandler::new(cmd, self.state.clone());
                Ok(Async::Ready(Some(task)))
            }
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => {
                debug!("CtlAcceptor error, {:?}", e);
                Err(())
            }
        }
    }
}

struct CtlHandler {
    cmd: ctl_gateway::server::CtlCommand,
    state: Arc<ManagerState>,
}

impl CtlHandler {
    fn new(cmd: ctl_gateway::server::CtlCommand, state: Arc<ManagerState>) -> Self {
        CtlHandler {
            cmd: cmd,
            state: state,
        }
    }
}

impl Future for CtlHandler {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.cmd.run(&self.state) {
            Ok(()) => (),
            Err(err) => {
                debug!("CtlHandler failed, {:?}", err);
                if self.cmd.req.transactional() {
                    self.cmd.req.reply_complete(err);
                }
            }
        }
        Ok(Async::Ready(()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use protocol::STATE_PATH_PREFIX;
    use std::path::PathBuf;

    #[test]
    fn manager_state_path_default() {
        let cfg = ManagerConfig::default();
        let path = cfg.sup_root();

        assert_eq!(
            PathBuf::from(format!("{}/default", STATE_PATH_PREFIX.to_string_lossy())),
            path
        );
    }

    #[test]
    fn manager_state_path_custom() {
        let mut cfg = ManagerConfig::default();
        cfg.custom_state_path = Some(PathBuf::from("/tmp/peanuts-and-cake"));
        let path = cfg.sup_root();

        assert_eq!(PathBuf::from("/tmp/peanuts-and-cake"), path);
    }

    #[test]
    fn manager_state_path_custom_beats_name() {
        let mut cfg = ManagerConfig::default();
        cfg.custom_state_path = Some(PathBuf::from("/tmp/partay"));
        let path = cfg.sup_root();

        assert_eq!(PathBuf::from("/tmp/partay"), path);
    }

    mod tokio_thread_count {
        use super::*;

        locked_env_var!(HAB_TOKIO_THREAD_COUNT, lock_thread_count);

        #[test]
        fn default_is_number_of_cpus() {
            let tc = lock_thread_count();
            tc.unset();

            assert_eq!(TokioThreadCount::configured_value().0, num_cpus::get());
        }

        #[test]
        fn can_be_overridden_by_env_var() {
            let tc = lock_thread_count();
            tc.set("128");
            assert_eq!(TokioThreadCount::configured_value().0, 128);
        }

        #[test]
        fn cannot_be_overridden_to_zero() {
            let tc = lock_thread_count();
            tc.set("0");

            assert_ne!(TokioThreadCount::configured_value().0, 0);
            assert_eq!(TokioThreadCount::configured_value().0, num_cpus::get());
        }

    }

    mod specs_to_operations {
        //! Testing out the reconciliation of on-disk spec files with
        //! what is currently running.

        use super::super::*;

        /// Helper function for generating a basic spec from an
        /// identifier string
        fn new_spec(ident: &str) -> ServiceSpec {
            ServiceSpec::default_for(
                PackageIdent::from_str(ident).expect("couldn't parse ident str"),
            )
        }

        #[test]
        fn no_specs_yield_no_changes() {
            assert!(Manager::specs_to_operations(vec![], vec![]).is_empty());
        }

        /// If all the currently running services match all the
        /// current specs, we shouldn't have anything to change.
        #[test]
        fn identical_specs_yield_no_changes() {
            let specs = vec![new_spec("core/foo"), new_spec("core/bar")];
            assert!(Manager::specs_to_operations(specs.clone(), specs.clone()).is_empty());
        }

        #[test]
        fn missing_spec_on_disk_means_stop() {
            let running = vec![new_spec("core/foo")];
            let on_disk = vec![];

            let operations = Manager::specs_to_operations(running, on_disk);
            assert_eq!(operations.len(), 1);
            assert_eq!(operations[0], ServiceOperation::Stop(new_spec("core/foo")));
        }

        #[test]
        fn missing_active_spec_means_start() {
            let running = vec![];
            let on_disk = vec![new_spec("core/foo")];

            let operations = Manager::specs_to_operations(running, on_disk);
            assert_eq!(operations.len(), 1);
            assert_eq!(operations[0], ServiceOperation::Start(new_spec("core/foo")));
        }

        #[test]
        fn down_spec_on_disk_means_stop_running_service() {
            let spec = new_spec("core/foo");

            let running = vec![spec.clone()];

            let down_spec = {
                let mut s = spec.clone();
                s.desired_state = DesiredState::Down;
                s
            };

            let on_disk = vec![down_spec];

            let operations = Manager::specs_to_operations(running, on_disk);
            assert_eq!(operations.len(), 1);
            assert_eq!(operations[0], ServiceOperation::Stop(spec));
        }

        #[test]
        fn down_spec_on_disk_with_no_running_service_yields_no_changes() {
            let running = vec![];
            let down_spec = {
                let mut s = new_spec("core/foo");
                s.desired_state = DesiredState::Down;
                s
            };
            let on_disk = vec![down_spec];

            let operations = Manager::specs_to_operations(running, on_disk);
            assert!(operations.is_empty());
        }

        #[test]
        fn modified_spec_on_disk_means_restart() {
            let running_spec = new_spec("core/foo");

            let on_disk_spec = {
                let mut s = running_spec.clone();
                s.update_strategy = UpdateStrategy::AtOnce;
                s
            };
            assert_ne!(running_spec.update_strategy, on_disk_spec.update_strategy);

            let running = vec![running_spec];
            let on_disk = vec![on_disk_spec];

            let operations = Manager::specs_to_operations(running, on_disk);
            assert_eq!(operations.len(), 1);

            match operations[0] {
                ServiceOperation::Restart {
                    to_stop: ref old,
                    to_start: ref new,
                } => {
                    assert_eq!(old.ident, new.ident);
                    assert_eq!(old.update_strategy, UpdateStrategy::None);
                    assert_eq!(new.update_strategy, UpdateStrategy::AtOnce);
                }
                ref other => {
                    panic!("Should have been a restart operation: got {:?}", other);
                }
            }
        }

        #[test]
        fn multiple_operations_can_be_determined_at_once() {
            // Nothing should happen with this; it's already how it
            // needs to be.
            let svc_1_running = new_spec("core/foo");
            let svc_1_on_disk = svc_1_running.clone();

            // Should get shut down.
            let svc_2_running = new_spec("core/bar");
            let svc_2_on_disk = {
                let mut s = svc_2_running.clone();
                s.desired_state = DesiredState::Down;
                s
            };

            // Should get restarted.
            let svc_3_running = new_spec("core/baz");
            let svc_3_on_disk = {
                let mut s = svc_3_running.clone();
                s.update_strategy = UpdateStrategy::AtOnce;
                s
            };

            // Nothing should happen with this; it's already down.
            let svc_4_on_disk = {
                let mut s = new_spec("core/quux");
                s.desired_state = DesiredState::Down;
                s
            };

            // This should get started
            let svc_5_on_disk = new_spec("core/wat");

            // This should get shut down
            let svc_6_running = new_spec("core/lolwut");

            let running = vec![
                svc_1_running.clone(),
                svc_2_running.clone(),
                svc_3_running.clone(),
                svc_6_running.clone(),
            ];

            let on_disk = vec![
                svc_1_on_disk.clone(),
                svc_2_on_disk.clone(),
                svc_3_on_disk.clone(),
                svc_4_on_disk.clone(),
                svc_5_on_disk.clone(),
            ];

            let operations = Manager::specs_to_operations(running, on_disk);

            let expected_operations = vec![
                ServiceOperation::Stop(svc_2_running.clone()),
                ServiceOperation::Restart {
                    to_stop: svc_3_running.clone(),
                    to_start: svc_3_on_disk.clone(),
                },
                ServiceOperation::Start(svc_5_on_disk.clone()),
                ServiceOperation::Stop(svc_6_running.clone()),
            ];

            // Ideally, we'd just sort `operations` and
            // `expected_operations`, but we can't, since that would
            // mean we'd need a total ordering on `PackageIdent`,
            // which we can't do, since identifiers of different
            // packages (say, `core/foo` and `core/bar`) are not
            // comparable.
            //
            // Instead, we'll just do the verification one at a time.
            assert_eq!(
                operations.len(),
                expected_operations.len(),
                "Didn't generate the expected number of operations"
            );
            for op in expected_operations {
                assert!(
                    operations.contains(&op),
                    "Should have expected operation: {:?}",
                    op
                );
            }
        }
    }
}
