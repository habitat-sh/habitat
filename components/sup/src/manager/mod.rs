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
mod events;
mod file_watcher;
mod peer_watcher;
mod periodic;
mod self_updater;
mod service_updater;
mod spec_watcher;
mod sys;
mod user_config_watcher;

use std;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::mem;
use std::net::SocketAddr;
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::result;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use butterfly;
use butterfly::member::Member;
use butterfly::server::{timing::Timing, ServerProxy, Suitability};
use butterfly::trace::Trace;
use common::command::package::install::InstallSource;
use common::ui::UIWriter;
use futures::prelude::*;
use futures::sync::mpsc;
use hcore::crypto::SymKey;
use hcore::env;
use hcore::fs::FS_ROOT_PATH;
use hcore::os::process::{self, Pid, Signal};
use hcore::os::signals::{self, SignalEvent};
use hcore::package::metadata::PackageType;
use hcore::package::{Identifiable, PackageIdent, PackageInstall, PackageTarget};
use hcore::service::ServiceGroup;
use launcher_client::{LauncherCli, LAUNCHER_LOCK_CLEAN_ENV, LAUNCHER_PID_ENV};
use protocol;
use protocol::net::{self, ErrCode, NetResult};
use serde;
use serde_json;
use time::{self, Duration as TimeDuration, Timespec};
use tokio_core::reactor;
use toml;

use self::peer_watcher::PeerWatcher;
use self::self_updater::{SelfUpdater, SUP_PKG_IDENT};
pub use self::service::{
    CompositeSpec, ConfigRendering, Service, ServiceBind, ServiceProxy, ServiceSpec, Spec,
    Topology, UpdateStrategy,
};
use self::service::{DesiredState, IntoServiceSpec, Pkg, ProcessState};
use self::service_updater::ServiceUpdater;
use self::spec_watcher::{SpecWatcher, SpecWatcherEvent};
pub use self::sys::Sys;
use self::user_config_watcher::UserConfigWatcher;
use super::feat;
use census::{CensusRing, CensusRingProxy};
use config::GossipListenAddr;
use ctl_gateway::{self, CtlRequest};
use error::{Error, Result, SupError};
use http_gateway;
use manager::service::spec::DesiredState as SpecDesiredState;
use util;
use ShutdownReason;
use VERSION;

const MEMBER_ID_FILE: &'static str = "MEMBER_ID";
const PROC_LOCK_FILE: &'static str = "LOCK";

static LOGKEY: &'static str = "MR";

/// FileSystem paths that the Manager uses to persist data to disk.
///
/// This is shared with the `http_gateway` and `service` modules for reading and writing
/// persistence data.
#[derive(Debug, Serialize)]
pub struct FsCfg {
    pub butterfly_data_path: PathBuf,
    pub census_data_path: PathBuf,
    pub services_data_path: PathBuf,
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
        let data_path = sup_root.join("data");
        FsCfg {
            butterfly_data_path: data_path.join("butterfly.dat"),
            census_data_path: data_path.join("census.dat"),
            services_data_path: data_path.join("services.dat"),
            specs_path: sup_root.join("specs"),
            composites_path: sup_root.join("composites"),
            data_path: data_path,
            member_id_file: sup_root.join(MEMBER_ID_FILE),
            proc_lock_file: sup_root.join(PROC_LOCK_FILE),
            sup_root: sup_root,
        }
    }

    pub fn health_check_cache(&self, service_group: &ServiceGroup) -> PathBuf {
        self.data_path
            .join(format!("{}.health", service_group.service()))
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
    pub name: Option<String>,
    pub organization: Option<String>,
    pub watch_peer_file: Option<String>,
}

impl ManagerConfig {
    pub fn sup_root(&self) -> PathBuf {
        protocol::sup_root(self.name.as_ref(), self.custom_state_path.as_ref())
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
            name: None,
            organization: None,
            watch_peer_file: None,
        }
    }
}

pub struct ManagerState {
    /// The configuration used to instantiate this Manager instance
    pub cfg: ManagerConfig,
    pub services: Arc<RwLock<Vec<Service>>>,
}

pub struct Manager {
    pub state: Rc<ManagerState>,
    butterfly: butterfly::Server,
    census_ring: CensusRing,
    events_group: Option<ServiceGroup>,
    fs_cfg: Arc<FsCfg>,
    launcher: LauncherCli,
    updater: ServiceUpdater,
    peer_watcher: Option<PeerWatcher>,
    spec_watcher: SpecWatcher,
    user_config_watcher: UserConfigWatcher,
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
        Self::create_state_path_dirs(&state_path)?;
        Self::clean_dirty_state(&state_path)?;
        let fs_cfg = FsCfg::new(state_path);
        if env::var(LAUNCHER_LOCK_CLEAN_ENV).is_ok() {
            release_process_lock(&fs_cfg);
        }
        obtain_process_lock(&fs_cfg)?;

        Self::new(cfg, fs_cfg, launcher)
    }

    /// Given an installed package, generate a spec (or specs, in the case
    /// of composite packages!) from it and the arguments passed in on the
    /// command line.
    pub fn generate_new_specs_from_package(
        package: &PackageInstall,
        opts: &protocol::ctl::SvcLoad,
    ) -> Result<Vec<ServiceSpec>> {
        let specs = match package.pkg_type()? {
            PackageType::Standalone => {
                let mut spec = ServiceSpec::default();
                opts.into_spec(&mut spec);
                vec![spec]
            }
            PackageType::Composite => opts.into_composite_spec(
                package.ident().name.clone(),
                package.pkg_services()?,
                package.bind_map()?,
            ),
        };
        Ok(specs)
    }

    pub fn service_status(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcStatus,
    ) -> NetResult<()> {
        let statuses = Self::status(&mgr.cfg)?;
        if let Some(ident) = opts.ident {
            for status in statuses {
                if status.pkg.ident.satisfies(&ident) {
                    let mut msg: protocol::types::ServiceStatus = status.into();
                    req.reply_complete(msg);
                    return Ok(());
                }
            }
            return Err(net::err(
                ErrCode::NotFound,
                format!("Service not loaded, {}", ident),
            ));
        } else if statuses.is_empty() {
            req.reply_complete(net::ok());
        } else {
            let mut list = statuses.into_iter().peekable();
            while let Some(status) = list.next() {
                let mut msg: protocol::types::ServiceStatus = status.into();
                if list.peek().is_some() {
                    req.reply_partial(msg);
                } else {
                    req.reply_complete(msg);
                }
            }
        }
        Ok(())
    }

    pub fn status(cfg: &ManagerConfig) -> Result<Vec<ServiceStatus>> {
        let fs_cfg = FsCfg::new(cfg.sup_root());

        let dat = File::open(&fs_cfg.services_data_path)?;
        serde_json::from_reader(&dat).map_err(|e| sup_error!(Error::ServiceDeserializationError(e)))
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

    /// Read all spec files and rewrite them to disk migrating their format from a previous
    /// Supervisor's to the one currently running.
    fn migrate_specs(fs_cfg: &FsCfg) {
        // JW: In the future we should write spec files to the Supervisor's DAT file in a more
        // appropriate machine readable format. We'll need to wait until we modify how we load and
        // unload services, though. Right now we watch files on disk and communicate with the
        // Supervisor asynchronously. We need to move to communicating directly with the
        // Supervisor's main loop through IPC.
        match SpecWatcher::spec_files(&fs_cfg.specs_path) {
            Ok(specs) => for spec_file in specs {
                match ServiceSpec::from_file(&spec_file) {
                    Ok(spec) => {
                        if let Err(err) = spec.to_file(&spec_file) {
                            outputln!(
                                "Unable to migrate service spec, {}, {}",
                                spec_file.display(),
                                err
                            );
                        }
                    }
                    Err(err) => {
                        outputln!(
                            "Unable to migrate service spec, {}, {}",
                            spec_file.display(),
                            err
                        );
                    }
                }
            },
            Err(err) => outputln!("Unable to migrate service specs, {}", err),
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
        let services = Arc::new(RwLock::new(Vec::new()));
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
            peer.swim_port = peer_addr.port() as i32;
            peer.gossip_port = peer_addr.port() as i32;
            server.member_list.add_initial_member(peer);
        }
        Self::migrate_specs(&fs_cfg);
        let peer_watcher = if let Some(path) = cfg.watch_peer_file {
            Some(PeerWatcher::run(path)?)
        } else {
            None
        };
        Ok(Manager {
            state: Rc::new(ManagerState {
                cfg: cfg_static,
                services: services,
            }),
            self_updater: self_updater,
            updater: ServiceUpdater::new(server.clone()),
            census_ring: CensusRing::new(sys.member_id.clone()),
            butterfly: server,
            events_group: cfg.eventsrv_group,
            launcher: launcher,
            peer_watcher: peer_watcher,
            spec_watcher: SpecWatcher::run(&fs_cfg.specs_path)?,
            user_config_watcher: UserConfigWatcher::new(),
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

    pub fn spec_path_for(cfg: &ManagerConfig, spec: &ServiceSpec) -> PathBuf {
        Self::specs_path(cfg.sup_root()).join(spec.file_name())
    }

    pub fn composite_path_for(cfg: &ManagerConfig, spec: &CompositeSpec) -> PathBuf {
        Self::composites_path(cfg.sup_root()).join(spec.file_name())
    }

    // TODO (CM): BAAAAARF
    pub fn composite_path_by_ident(cfg: &ManagerConfig, ident: &PackageIdent) -> PathBuf {
        let mut p = Self::composites_path(cfg.sup_root()).join(&ident.name);
        p.set_extension("spec");
        p
    }

    /// Given a `PackageIdent`, return current specs if they exist. If
    /// the package is a standalone service, only that spec will be
    /// returned, but if it is a composite, the composite spec as well as
    /// the specs for all the services in the composite will be returned.
    pub fn existing_specs_for_ident(
        cfg: &ManagerConfig,
        ident: &PackageIdent,
    ) -> Result<Option<Spec>> {
        let default_spec = ServiceSpec::default_for(ident.clone());
        let spec_file = Self::spec_path_for(cfg, &default_spec);

        // Try it as a service first
        if let Ok(spec) = ServiceSpec::from_file(&spec_file) {
            Ok(Some(Spec::Service(spec)))
        } else {
            // Try it as a composite next
            let composite_spec_file = Self::composite_path_by_ident(&cfg, ident);
            match CompositeSpec::from_file(composite_spec_file) {
                Ok(composite_spec) => {
                    let fs_root_path = Path::new(&*FS_ROOT_PATH);
                    let package =
                        PackageInstall::load(composite_spec.package_ident(), Some(fs_root_path))?;
                    let mut specs = vec![];

                    let services = package.pkg_services()?;
                    for service in services {
                        let spec = ServiceSpec::from_file(Manager::spec_path_for(
                            cfg,
                            &ServiceSpec::default_for(service),
                        ))?;
                        specs.push(spec);
                    }

                    Ok(Some(Spec::Composite(composite_spec, specs)))
                }
                // Looks like we have no specs for this thing at all
                Err(_) => Ok(None),
            }
        }
    }

    pub fn save_spec_for(cfg: &ManagerConfig, spec: &ServiceSpec) -> Result<()> {
        spec.to_file(Self::spec_path_for(cfg, spec))
    }

    pub fn save_composite_spec_for(cfg: &ManagerConfig, spec: &CompositeSpec) -> Result<()> {
        spec.to_file(Self::composite_path_for(cfg, spec))
    }

    fn clean_dirty_state<T>(state_path: T) -> Result<()>
    where
        T: AsRef<Path>,
    {
        let data_path = Self::data_path(&state_path);
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
            Err(err) => Err(sup_error!(Error::BadDataPath(data_path, err))),
        }
    }

    fn create_state_path_dirs<T>(state_path: T) -> Result<()>
    where
        T: AsRef<Path>,
    {
        let data_path = Self::data_path(&state_path);
        debug!("Creating data directory: {}", data_path.display());
        if let Some(err) = fs::create_dir_all(&data_path).err() {
            return Err(sup_error!(Error::BadDataPath(data_path, err)));
        }
        let specs_path = Self::specs_path(&state_path);
        debug!("Creating specs directory: {}", specs_path.display());
        if let Some(err) = fs::create_dir_all(&specs_path).err() {
            return Err(sup_error!(Error::BadSpecsPath(specs_path, err)));
        }

        let composites_path = Self::composites_path(&state_path);
        debug!(
            "Creating composites directory: {}",
            composites_path.display()
        );
        if let Some(err) = fs::create_dir_all(&composites_path).err() {
            return Err(sup_error!(Error::BadCompositesPath(composites_path, err)));
        }

        Ok(())
    }

    #[inline]
    fn data_path<T>(state_path: T) -> PathBuf
    where
        T: AsRef<Path>,
    {
        state_path.as_ref().join("data")
    }

    #[inline]
    fn specs_path<T>(state_path: T) -> PathBuf
    where
        T: AsRef<Path>,
    {
        state_path.as_ref().join("specs")
    }

    #[inline]
    fn composites_path<T>(state_path: T) -> PathBuf
    where
        T: AsRef<Path>,
    {
        state_path.as_ref().join("composites")
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
            self.butterfly
                .start_election(service.service_group.clone(), 0);
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
            .push(service);
    }

    pub fn run(mut self, svc: Option<protocol::ctl::SvcLoad>) -> Result<()> {
        let mut core = reactor::Core::new().expect("Couldn't start main reactor");
        let handle = core.handle();
        let (ctl_tx, ctl_rx) = mpsc::unbounded();
        let ctl_handler = CtlAcceptor::new(self.state.clone(), ctl_rx).for_each(move |handler| {
            handle.spawn(handler);
            Ok(())
        });
        core.handle().spawn(ctl_handler);
        if let Some(svc_load) = svc {
            Self::service_load(&self.state, &mut CtlRequest::default(), svc_load)?;
        }
        self.start_initial_services_from_spec_watcher()?;

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
            outputln!("Starting http-gateway on {}", &http_listen_addr);
            http_gateway::Server::new(self.fs_cfg.clone(), http_listen_addr).start()?;
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
            self.update_running_services_from_spec_watcher()?;
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
                    .iter()
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
                .iter_mut()
            {
                if service.tick(&self.census_ring, &self.launcher) {
                    self.gossip_latest_service_rumor(&service);
                }
            }
            let time_to_wait = ((next_check - time::get_time()).num_milliseconds()).max(100);
            core.turn(Some(Duration::from_millis(time_to_wait as u64)));
        }
    }

    pub fn service_cfg(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcGetDefaultCfg,
    ) -> NetResult<()> {
        let ident: PackageIdent = opts.ident.ok_or(err_update_client())?.into();
        let mut msg = protocol::types::ServiceCfg {
            format: Some(protocol::types::service_cfg::Format::Toml as i32),
            default: None,
        };
        for service in mgr.services.read().unwrap().iter() {
            if service.pkg.ident.satisfies(&ident) {
                if let Some(ref cfg) = service.cfg.default {
                    msg.default = Some(
                        toml::to_string_pretty(&toml::value::Value::Table(cfg.clone())).unwrap(),
                    );
                    req.reply_complete(msg);
                }
                return Ok(());
            }
        }
        Err(net::err(
            ErrCode::NotFound,
            format!("Service not loaded, {}", ident),
        ))
    }

    pub fn service_cfg_validate(
        _mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcValidateCfg,
    ) -> NetResult<()> {
        let cfg = opts.cfg.ok_or(err_update_client())?;
        let format = opts
            .format
            .and_then(protocol::types::service_cfg::Format::from_i32)
            .unwrap_or_default();
        if cfg.len() > protocol::butterfly::MAX_SVC_CFG_SIZE {
            return Err(net::err(
                ErrCode::EntityTooLarge,
                "Configuration too large.",
            ));
        }
        if format != protocol::types::service_cfg::Format::Toml {
            return Err(net::err(
                ErrCode::NotSupported,
                format!("Configuration format {} not available.", format),
            ));
        }
        let _new_cfg: toml::value::Table = toml::from_slice(&cfg).map_err(|e| {
            net::err(
                ErrCode::BadPayload,
                format!("Unable to decode configuration as {}, {}", format, e),
            )
        })?;
        req.reply_complete(net::ok());
        Ok(())
        // JW TODO: Hold off on validation until we can validate services which aren't currently
        // loaded in the Supervisor but are known through rumor propagation.
        // let service_group: ServiceGroup = opts.service_group.into();
        // for service in mgr.services.read().unwrap().iter() {
        //     if service.service_group != service_group {
        //         continue;
        //     }
        //     if let Some(interface) = service.cfg.interface() {
        //         match Cfg::validate(interface, &new_cfg) {
        //             None => req.reply_complete(net::ok()),
        //             Some(errors) => {
        //                 for error in errors {
        //                     req.reply_partial(net::err(ErrCode::InvalidPayload, error));
        //                 }
        //                 req.reply_complete(net::ok());
        //             }
        //         }
        //         return Ok(());
        //     } else {
        //         // No interface, this service can't be configured.
        //         return Err(net::err(
        //             ErrCode::NotFound,
        //             "Service has no configurable attributes.",
        //         ));
        //     }
        // }
        // Err(net::err(
        //     ErrCode::NotFound,
        //     format!("Service not loaded, {}", service_group),
        // ))
    }

    pub fn service_cfg_set(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcSetCfg,
    ) -> NetResult<()> {
        let cfg = opts.cfg.ok_or(err_update_client())?;
        let is_encrypted = opts.is_encrypted.unwrap_or(false);
        let version = opts.version.ok_or(err_update_client())?;
        let service_group: ServiceGroup = opts.service_group.ok_or(err_update_client())?.into();
        if cfg.len() > protocol::butterfly::MAX_SVC_CFG_SIZE {
            return Err(net::err(
                ErrCode::EntityTooLarge,
                "Configuration too large.",
            ));
        }
        outputln!(
            "Setting new configuration version {} for {}",
            version,
            service_group,
        );
        let mut client = match butterfly::client::Client::new(
            mgr.cfg.gossip_listen.local_addr(),
            mgr.cfg.ring_key.clone(),
        ) {
            Ok(client) => client,
            Err(err) => {
                outputln!("Failed to connect to own gossip server, {}", err);
                return Err(net::err(ErrCode::Internal, err.to_string()));
            }
        };
        match client.send_service_config(service_group, version, cfg, is_encrypted) {
            Ok(()) => {
                req.reply_complete(net::ok());
                return Ok(());
            }
            Err(e) => return Err(net::err(ErrCode::Internal, e.to_string())),
        }
    }

    pub fn service_file_put(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcFilePut,
    ) -> NetResult<()> {
        let content = opts.content.ok_or(err_update_client())?;
        let filename = opts.filename.ok_or(err_update_client())?;
        let is_encrypted = opts.is_encrypted.unwrap_or(false);
        let version = opts.version.ok_or(err_update_client())?;
        let service_group: ServiceGroup = opts.service_group.ok_or(err_update_client())?.into();
        if content.len() > protocol::butterfly::MAX_FILE_PUT_SIZE_BYTES {
            return Err(net::err(ErrCode::EntityTooLarge, "File content too large."));
        }
        outputln!(
            "Receiving new version {} of file {} for {}",
            version,
            filename,
            service_group,
        );
        let mut client = match butterfly::client::Client::new(
            mgr.cfg.gossip_listen.local_addr(),
            mgr.cfg.ring_key.clone(),
        ) {
            Ok(client) => client,
            Err(err) => {
                outputln!("Failed to connect to own gossip server, {}", err);
                return Err(net::err(ErrCode::Internal, err.to_string()));
            }
        };
        match client.send_service_file(service_group, filename, version, content, is_encrypted) {
            Ok(()) => {
                req.reply_complete(net::ok());
                return Ok(());
            }
            Err(e) => return Err(net::err(ErrCode::Internal, e.to_string())),
        }
    }

    pub fn service_load(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcLoad,
    ) -> NetResult<()> {
        let ident: PackageIdent = opts.ident.clone().ok_or(err_update_client())?.into();
        let bldr_url = opts
            .bldr_url
            .clone()
            .unwrap_or(protocol::DEFAULT_BLDR_URL.to_string());
        let bldr_channel = opts
            .bldr_channel
            .clone()
            .unwrap_or(protocol::DEFAULT_BLDR_CHANNEL.to_string());
        let force = opts.force.clone().unwrap_or(false);
        let source = InstallSource::Ident(ident.clone(), *PackageTarget::active_target());
        match Self::existing_specs_for_ident(&mgr.cfg, source.as_ref())? {
            None => {
                // We don't have any record of this thing; let's set it up!
                //
                // If a package exists on disk that satisfies the
                // desired package identifier, it will be used;
                // otherwise, we'll install the latest suitable
                // version from the specified Builder channel.
                let installed =
                    util::pkg::satisfy_or_install(req, &source, &bldr_url, &bldr_channel)?;

                let mut specs = Self::generate_new_specs_from_package(&installed, &opts)?;

                for spec in specs.iter_mut() {
                    Self::save_spec_for(&mgr.cfg, spec)?;
                    req.info(format!(
                        "The {} service was successfully loaded",
                        spec.ident
                    ))?;
                }

                // Only saves a composite spec if it's, well, a composite
                if let Ok(composite_spec) =
                    CompositeSpec::from_package_install(source.as_ref(), &installed)
                {
                    Self::save_composite_spec_for(&mgr.cfg, &composite_spec)?;
                    req.info(format!(
                        "The {} composite was successfully loaded",
                        composite_spec.ident()
                    ))?;
                }
            }
            Some(spec) => {
                // We've seen this service / composite before. Thus `load`
                // basically acts as a way to edit spec files on the
                // command line. As a result, we a) check that you
                // *really* meant to change an existing spec, and b) DO
                // NOT download a potentially new version of the package
                // in question

                if !force {
                    // TODO (CM): make this error reflect composites
                    return Err(net::err(
                        ErrCode::Conflict,
                        format!("Service already loaded, unload '{}' and try again", ident),
                    ));
                }

                match spec {
                    Spec::Service(mut service_spec) => {
                        opts.into_spec(&mut service_spec);

                        // Only install if we don't have something
                        // locally; otherwise you could potentially
                        // upgrade each time you load.
                        //
                        // Also make sure you're pulling from where you're
                        // supposed to be pulling from!
                        util::pkg::satisfy_or_install(
                            req,
                            &source,
                            &service_spec.bldr_url,
                            &service_spec.channel,
                        )?;

                        Self::save_spec_for(&mgr.cfg, &service_spec)?;
                        req.info(format!(
                            "The {} service was successfully loaded",
                            service_spec.ident
                        ))?;
                    }
                    Spec::Composite(composite_spec, mut existing_service_specs) => {
                        if source.as_ref() == composite_spec.ident() {
                            let mut bind_map =
                                match util::pkg::installed(composite_spec.package_ident()) {
                                    Some(package) => package.bind_map()?,
                                    // TODO (CM): this should be a proper error
                                    None => unreachable!(),
                                };

                            for mut service_spec in existing_service_specs.iter_mut() {
                                opts.update_composite(&mut bind_map, &mut service_spec);
                                Self::save_spec_for(&mgr.cfg, service_spec)?;
                                req.info(format!(
                                    "The {} service was successfully loaded",
                                    service_spec.ident
                                ))?;
                            }
                            req.info(format!(
                                "The {} composite was successfully loaded",
                                composite_spec.ident()
                            ))?;
                        } else {
                            // It changed!
                            // OK, here's the deal.
                            //
                            // We're going to install a new composite if
                            // we need to in order to satisfy the spec
                            // we've now got. That also means that the
                            // services that are currently running may get
                            // unloaded (because they are no longer in the
                            // composite), and new services may start
                            // (because they were added to the composite).

                            let installed_package = util::pkg::satisfy_or_install(
                                req,
                                &source,
                                // This (updating from the command-line
                                // args) is a difference from
                                // force-loading a spec, because
                                // composites don't auto-update themselves
                                // like services can.
                                &bldr_url,
                                &bldr_channel,
                            )?;

                            // Generate new specs from the new composite package and
                            // CLI inputs
                            let new_service_specs =
                                Self::generate_new_specs_from_package(&installed_package, &opts)?;

                            // Delete any specs that are not in the new
                            // composite
                            let mut old_spec_names = HashSet::new();
                            for s in existing_service_specs.iter() {
                                old_spec_names.insert(s.ident.name.clone());
                            }
                            let mut new_spec_names = HashSet::new();
                            for s in new_service_specs.iter() {
                                new_spec_names.insert(s.ident.name.clone());
                            }

                            let specs_to_delete: HashSet<_> =
                                old_spec_names.difference(&new_spec_names).collect();
                            for spec in existing_service_specs.iter() {
                                if specs_to_delete.contains(&spec.ident.name) {
                                    let file = Manager::spec_path_for(&mgr.cfg, spec);
                                    req.info(format!("Unloading {:?}", file))?;
                                    std::fs::remove_file(&file).map_err(|err| {
                                        sup_error!(Error::ServiceSpecFileIO(file, err))
                                    })?;
                                }
                            }
                            // <-- end of deletion

                            // Save all the new specs. If there are
                            // services that exist in both composites,
                            // their service spec files will have the same
                            // name, so they'll be taken care of here (we
                            // don't need to treat them differently)
                            for spec in new_service_specs.iter() {
                                Self::save_spec_for(&mgr.cfg, spec)?;
                                req.info(format!(
                                    "The {} service was successfully loaded",
                                    spec.ident
                                ))?;
                            }

                            // Generate and save the new spec
                            let new_composite_spec = CompositeSpec::from_package_install(
                                source.as_ref(),
                                &installed_package,
                            )?;
                            Self::save_composite_spec_for(&mgr.cfg, &new_composite_spec)?;
                            req.info(format!(
                                "The {} composite was successfully loaded",
                                new_composite_spec.ident()
                            ))?;
                        }
                    }
                }
            }
        }
        req.reply_complete(net::ok());
        Ok(())
    }

    pub fn service_unload(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcUnload,
    ) -> NetResult<()> {
        let ident: PackageIdent = opts.ident.ok_or(err_update_client())?.into();
        // Gather up the paths to all the spec files we care about,
        // along with their corresponding idents (we do this to ensure
        // we emit a proper "unloading X" message for each member of a
        // composite).
        //
        // This includes all service specs as well as any composite
        // spec.
        let path_ident_pairs = match Self::existing_specs_for_ident(&mgr.cfg, &ident)? {
            Some(Spec::Service(spec)) => vec![(Self::spec_path_for(&mgr.cfg, &spec), ident)],
            Some(Spec::Composite(composite_spec, specs)) => {
                let mut paths = Vec::with_capacity(specs.len() + 1);
                for spec in specs.iter() {
                    paths.push((Self::spec_path_for(&mgr.cfg, spec), spec.ident.clone()));
                }
                paths.push((Self::composite_path_for(&mgr.cfg, &composite_spec), ident));
                paths
            }
            None => vec![],
        };

        for (file, ident) in path_ident_pairs {
            if let Err(err) = std::fs::remove_file(&file) {
                return Err(net::err(
                    ErrCode::Internal,
                    format!("{}", sup_error!(Error::ServiceSpecFileIO(file, err))),
                ));
            };
            // JW TODO: Change this to unloaded from unloading when the Supervisor waits for
            // the work to complete.
            req.info(format!("Unloading {}", ident))?;
        }
        req.reply_complete(net::ok());
        Ok(())
    }

    pub fn service_start(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcStart,
    ) -> NetResult<()> {
        let ident = opts.ident.ok_or(err_update_client())?.into();
        let updated_specs = match Self::existing_specs_for_ident(&mgr.cfg, &ident)? {
            Some(Spec::Service(mut spec)) => {
                let mut updated_specs = vec![];
                if spec.desired_state == DesiredState::Down {
                    spec.desired_state = DesiredState::Up;
                    updated_specs.push(spec);
                }
                updated_specs
            }
            Some(Spec::Composite(_, service_specs)) => {
                let mut updated_specs = vec![];
                for mut spec in service_specs {
                    if spec.desired_state == DesiredState::Down {
                        spec.desired_state = DesiredState::Up;
                        updated_specs.push(spec);
                    }
                }
                updated_specs
            }
            None => {
                return Err(net::err(
                    ErrCode::NotFound,
                    format!("Service not loaded, {}", &ident),
                ));
            }
        };
        let specs_changed = updated_specs.len() > 0;
        for spec in updated_specs.iter() {
            Self::save_spec_for(&mgr.cfg, spec)?;
        }
        if specs_changed {
            // JW TODO: Change the language of the message below to "started" when we actually
            // synchronously control services from the ctl gateway.
            req.info(format!(
                "Supervisor starting {}. See the Supervisor output for more details.",
                &ident
            ))?;
        }
        req.reply_complete(net::ok());
        Ok(())
    }

    pub fn service_stop(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SvcStop,
    ) -> NetResult<()> {
        let ident: PackageIdent = opts.ident.ok_or(err_update_client())?.into();
        let updated_specs = match Self::existing_specs_for_ident(&mgr.cfg, &ident)? {
            Some(Spec::Service(mut spec)) => {
                let mut updated_specs = vec![];
                if spec.desired_state == DesiredState::Up {
                    spec.desired_state = DesiredState::Down;
                    updated_specs.push(spec);
                }
                updated_specs
            }
            Some(Spec::Composite(_, service_specs)) => {
                let mut updated_specs = vec![];
                for mut spec in service_specs {
                    if spec.desired_state == DesiredState::Up {
                        spec.desired_state = DesiredState::Down;
                        updated_specs.push(spec);
                    }
                }
                updated_specs
            }
            None => {
                return Err(net::err(
                    ErrCode::NotFound,
                    format!("Service not loaded, {}", &ident),
                ));
            }
        };
        let specs_changed = updated_specs.len() > 0;
        for spec in updated_specs.iter() {
            Self::save_spec_for(&mgr.cfg, spec)?;
        }
        if specs_changed {
            // JW TODO: Change the langauge of the message below to "stopped" when we actually
            // synchronously control services from the ctl gateway.
            req.info(format!(
                "Supervisor stopping {}. See the Supervisor output for more details.",
                &ident
            ))?;
        }
        req.reply_complete(net::ok());
        Ok(())
    }

    pub fn supervisor_depart(
        mgr: &ManagerState,
        req: &mut CtlRequest,
        opts: protocol::ctl::SupDepart,
    ) -> NetResult<()> {
        let member_id = opts.member_id.ok_or(err_update_client())?;
        let mut client = match butterfly::client::Client::new(
            mgr.cfg.gossip_listen.local_addr(),
            mgr.cfg.ring_key.clone(),
        ) {
            Ok(client) => client,
            Err(err) => {
                outputln!("Failed to connect to own gossip server, {}", err);
                return Err(net::err(ErrCode::Internal, err.to_string()));
            }
        };
        outputln!("Attempting to depart member: {}", member_id);
        match client.send_departure(member_id) {
            Ok(()) => {
                req.reply_complete(net::ok());
                Ok(())
            }
            Err(e) => Err(net::err(ErrCode::Internal, e.to_string())),
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
            .iter_mut()
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
        let mut incarnation = 1;
        {
            let list = self
                .butterfly
                .service_store
                .list
                .read()
                .expect("Rumor store lock poisoned");
            if let Some(rumor) = list
                .get(&*service.service_group)
                .and_then(|r| r.get(&self.sys.member_id))
            {
                incarnation = rumor.clone().incarnation + 1;
            }
        }
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
            .iter_mut()
        {
            service_states.insert(service.spec_ident.clone(), service.last_state_change());
            active_services.push(service.spec_ident.clone());
        }

        for loaded in self
            .spec_watcher
            .specs_from_watch_path()
            .unwrap()
            .values()
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
        debug!("Writing census state to disk");
        self.persist_census_state();
        debug!("Writing butterfly state to disk");
        self.persist_butterfly_state();
        debug!("Writing services state to disk");
        self.persist_services_state();
    }

    fn persist_census_state(&self) {
        let crp = CensusRingProxy::new(&self.census_ring);
        let tmp_file = self.fs_cfg.census_data_path.with_extension("dat.tmp");
        let file = match File::create(&tmp_file) {
            Ok(file) => file,
            Err(err) => {
                warn!("Couldn't open temporary census state file, {}", err);
                return;
            }
        };
        let mut writer = BufWriter::new(file);
        if let Some(err) = writer
            .write(serde_json::to_string(&crp).unwrap().as_bytes())
            .err()
        {
            warn!("Couldn't write to census state file, {}", err);
        }
        if let Some(err) = writer.flush().err() {
            warn!("Couldn't flush census state buffer to disk, {}", err);
        }
        if let Some(err) = fs::rename(&tmp_file, &self.fs_cfg.census_data_path).err() {
            warn!("Couldn't finalize census state on disk, {}", err);
        }
    }

    fn persist_butterfly_state(&self) {
        let bs = ServerProxy::new(&self.butterfly);
        let tmp_file = self.fs_cfg.butterfly_data_path.with_extension("dat.tmp");
        let file = match File::create(&tmp_file) {
            Ok(file) => file,
            Err(err) => {
                warn!("Couldn't open temporary butterfly state file, {}", err);
                return;
            }
        };
        let mut writer = BufWriter::new(file);
        if let Some(err) = writer
            .write(serde_json::to_string(&bs).unwrap().as_bytes())
            .err()
        {
            warn!("Couldn't write to butterfly state file, {}", err);
        }
        if let Some(err) = writer.flush().err() {
            warn!("Couldn't flush butterfly state buffer to disk, {}", err);
        }
        if let Some(err) = fs::rename(&tmp_file, &self.fs_cfg.butterfly_data_path).err() {
            warn!("Couldn't finalize butterfly state on disk, {}", err);
        }
    }

    fn persist_services_state(&self) {
        let tmp_file = self.fs_cfg.services_data_path.with_extension("dat.tmp");
        let file = match File::create(&tmp_file) {
            Ok(file) => file,
            Err(err) => {
                warn!("Couldn't open temporary services state file, {}", err);
                return;
            }
        };

        let config_rendering = if feat::is_enabled(feat::RedactHTTP) {
            ConfigRendering::Redacted
        } else {
            ConfigRendering::Full
        };

        let mut writer = BufWriter::new(file);
        let services = self
            .state
            .services
            .read()
            .expect("Services lock is poisoned!");
        let existing_idents: Vec<PackageIdent> =
            services.iter().map(|s| s.spec_ident.clone()).collect();

        // Services that are not active but are being watched for changes
        // These would include stopped persistent services or other
        // persistent services that failed to load
        let watched_services: Vec<Service> = self
            .spec_watcher
            .specs_from_watch_path()
            .unwrap()
            .values()
            .filter(|spec| !existing_idents.contains(&spec.ident))
            .flat_map(|spec| {
                Service::load(
                    self.sys.clone(),
                    spec.clone(),
                    self.fs_cfg.clone(),
                    self.organization.as_ref().map(|org| &**org),
                ).into_iter()
            }).collect();
        let watched_service_proxies: Vec<ServiceProxy> = watched_services
            .iter()
            .map(|s| ServiceProxy::new(s, config_rendering.clone()))
            .collect();
        let mut services_to_render: Vec<ServiceProxy> = services
            .iter()
            .map(|s| ServiceProxy::new(s, config_rendering.clone()))
            .collect();

        services_to_render.extend(watched_service_proxies);

        if let Some(err) = writer
            .write(
                serde_json::to_string(&services_to_render)
                    .unwrap()
                    .as_bytes(),
            ).err()
        {
            warn!("Couldn't write to butterfly state file, {}", err);
        }
        if let Some(err) = writer.flush().err() {
            warn!("Couldn't flush services state buffer to disk, {}", err);
        }
        if let Some(err) = fs::rename(&tmp_file, &self.fs_cfg.services_data_path).err() {
            warn!("Couldn't finalize services state on disk, {}", err);
        }
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
        if let Err(err) = fs::remove_file(self.fs_cfg.health_check_cache(&service.service_group)) {
            outputln!(
                "Unable to cleanup service health cache, {}, {}",
                service,
                err
            );
        }
        if let Err(_) = self.user_config_watcher.remove(service) {
            debug!(
                "Error stopping user-config watcher thread for service {}",
                service
            );
        }
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

        let mut svcs = Vec::new();

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

        for mut service in svcs.drain(..) {
            self.remove_service(&mut service, cause);
        }
        release_process_lock(&self.fs_cfg);
    }

    fn start_initial_services_from_spec_watcher(&mut self) -> Result<()> {
        for service_event in self.spec_watcher.initial_events()? {
            match service_event {
                SpecWatcherEvent::AddService(spec) => {
                    if spec.desired_state == DesiredState::Up {
                        // JW TODO: Should we retry starting services which we failed to add?
                        self.add_service(spec);
                    }
                }
                _ => warn!("Skipping unexpected watcher event: {:?}", service_event),
            }
        }
        Ok(())
    }

    fn update_running_services_from_spec_watcher(&mut self) -> Result<()> {
        let mut active_specs = HashMap::new();
        for service in self
            .state
            .services
            .read()
            .expect("Services lock is poisoned!")
            .iter()
        {
            let spec = service.to_spec();
            active_specs.insert(spec.ident.name.clone(), spec);
        }

        for service_event in self.spec_watcher.new_events(active_specs)? {
            match service_event {
                SpecWatcherEvent::AddService(spec) => {
                    if spec.desired_state == DesiredState::Up {
                        self.add_service(spec);
                    }
                }
                SpecWatcherEvent::RemoveService(spec) => self.remove_service_for_spec(&spec)?,
            }
        }

        Ok(())
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

        for service in services.iter_mut() {
            if self.user_config_watcher.have_events_for(service) {
                outputln!("Reloading service {}", &service.spec_ident);
                service.user_config_updated = true;
            }
        }
    }

    fn remove_service_for_spec(&mut self, spec: &ServiceSpec) -> Result<()> {
        let mut service: Service;

        {
            let mut services = self
                .state
                .services
                .write()
                .expect("Services lock is poisoned");
            // TODO fn: storing services as a `Vec` is a bit crazy when you have to do these
            // shenanigans--maybe we want to consider changing the data structure in the future?
            let services_idx = match services.iter().position(|ref s| s.spec_ident == spec.ident) {
                Some(i) => i,
                None => {
                    outputln!(
                        "Tried to remove service for {} but could not find it running, skipping",
                        &spec.ident
                    );
                    return Ok(());
                }
            };

            service = services.remove(services_idx);
        }

        self.remove_service(&mut service, ShutdownReason::SvcStopCmd);
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct ProcessStatus {
    #[serde(
        deserialize_with = "deserialize_time",
        rename = "state_entered"
    )]
    pub elapsed: TimeDuration,
    pub pid: Option<u32>,
    pub state: ProcessState,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.pid {
            Some(pid) => write!(
                f,
                "state:{}, time:{}, pid:{}",
                self.state, self.elapsed, pid
            ),
            None => write!(f, "state:{}, time:{}", self.state, self.elapsed),
        }
    }
}

#[derive(Deserialize)]
pub struct ServiceStatus {
    pub pkg: Pkg,
    pub process: ProcessStatus,
    pub service_group: ServiceGroup,
    pub composite: Option<String>,
    pub desired_state: DesiredState,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}), {}, group:{}",
            self.pkg.ident,
            self.composite.as_ref().unwrap_or(&"standalone".to_string()),
            self.process,
            self.service_group,
        )
    }
}

#[derive(Debug)]
struct SuitabilityLookup(Arc<RwLock<Vec<Service>>>);

impl Suitability for SuitabilityLookup {
    fn get(&self, service_group: &ServiceGroup) -> u64 {
        self.0
            .read()
            .expect("Services lock is poisoned!")
            .iter()
            .find(|s| s.service_group == *service_group)
            .and_then(|s| s.suitability())
            .unwrap_or(u64::min_value())
    }
}

fn err_update_client() -> net::NetErr {
    net::err(ErrCode::UpdateClient, "client out of date")
}

fn deserialize_time<'de, D>(d: D) -> result::Result<TimeDuration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct FromTimespec;

    impl<'de> serde::de::Visitor<'de> for FromTimespec {
        type Value = TimeDuration;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a i64 integer")
        }

        fn visit_u64<R>(self, value: u64) -> result::Result<TimeDuration, R>
        where
            R: serde::de::Error,
        {
            let tspec = Timespec {
                sec: (value as i64),
                nsec: 0,
            };
            Ok(time::get_time() - tspec)
        }
    }

    d.deserialize_u64(FromTimespec)
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
    state: Rc<ManagerState>,
}

impl CtlAcceptor {
    fn new(state: Rc<ManagerState>, rx: ctl_gateway::server::MgrReceiver) -> Self {
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
    state: Rc<ManagerState>,
}

impl CtlHandler {
    fn new(cmd: ctl_gateway::server::CtlCommand, state: Rc<ManagerState>) -> Self {
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

impl From<ProcessStatus> for protocol::types::ProcessStatus {
    fn from(other: ProcessStatus) -> Self {
        let mut proto = protocol::types::ProcessStatus::default();
        proto.elapsed = Some(other.elapsed.num_seconds());
        proto.state = other.state.into();
        if let Some(pid) = other.pid {
            proto.pid = Some(pid);
        }
        proto
    }
}

impl From<service::ServiceBind> for protocol::types::ServiceBind {
    fn from(bind: service::ServiceBind) -> Self {
        let mut proto = protocol::types::ServiceBind::default();
        proto.name = bind.name;
        proto.service_group = bind.service_group.into();
        proto
    }
}

impl From<ServiceStatus> for protocol::types::ServiceStatus {
    fn from(other: ServiceStatus) -> Self {
        let mut proto = protocol::types::ServiceStatus::default();
        proto.ident = other.pkg.ident.into();
        proto.process = Some(other.process.into());
        proto.service_group = other.service_group.into();
        if let Some(composite) = other.composite {
            proto.composite = Some(composite);
        }
        proto.desired_state = Some(other.desired_state.into());
        proto
    }
}

impl Into<service::ServiceBind> for protocol::types::ServiceBind {
    fn into(self) -> service::ServiceBind {
        service::ServiceBind {
            name: self.name,
            service_group: self.service_group.into(),
            service_name: self.service_name,
        }
    }
}

impl From<SpecDesiredState> for i32 {
    fn from(other: SpecDesiredState) -> Self {
        match other {
            DesiredState::Down => 0,
            DesiredState::Up => 1,
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use protocol::STATE_PATH_PREFIX;

    use super::ManagerConfig;

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
    fn manager_state_path_with_name() {
        let mut cfg = ManagerConfig::default();
        cfg.name = Some(String::from("peanuts"));
        let path = cfg.sup_root();

        assert_eq!(
            PathBuf::from(format!("{}/peanuts", STATE_PATH_PREFIX.to_string_lossy())),
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
        cfg.name = Some(String::from("nope"));
        let path = cfg.sup_root();

        assert_eq!(PathBuf::from("/tmp/partay"), path);
    }
}
