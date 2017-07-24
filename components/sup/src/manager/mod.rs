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
mod events;
mod self_updater;
mod service_updater;
mod spec_watcher;
mod sys;

use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::result;
use std::thread;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use butterfly;
use butterfly::member::Member;
use butterfly::trace::Trace;
use butterfly::server::timing::Timing;
use butterfly::server::Suitability;
use hcore::crypto::{default_cache_key_path, SymKey};
use hcore::fs::FS_ROOT_PATH;
use hcore::service::ServiceGroup;
use hcore::os::process::{self, Signal};
use hcore::package::{Identifiable, PackageIdent, PackageInstall};
use launcher_client::LauncherCli;
use serde;
use serde_json;
use time::{self, Timespec, Duration as TimeDuration};

pub use self::service::{Service, ServiceSpec, UpdateStrategy, Topology};
pub use self::sys::Sys;
use self::self_updater::{SUP_PKG_IDENT, SelfUpdater};
use self::service::{DesiredState, Pkg, ProcessState, StartStyle};
use self::service_updater::ServiceUpdater;
use self::spec_watcher::{SpecWatcher, SpecWatcherEvent};
use VERSION;
use error::{Error, Result, SupError};
use config::GossipListenAddr;
use census::CensusRing;
use http_gateway;

const MEMBER_ID_FILE: &'static str = "MEMBER_ID";
const PROC_LOCK_FILE: &'static str = "LOCK";

static LOGKEY: &'static str = "MR";

lazy_static! {
    /// The root path containing all runtime service directories and files
    pub static ref STATE_PATH_PREFIX: PathBuf = {
        Path::new(&*FS_ROOT_PATH).join("hab/sup")
    };
}

/// FileSystem paths that the Manager uses to persist data to disk.
///
/// This is shared with the `http_gateway` and `service` modules for reading and writing
/// persistence data.
#[derive(Debug, Serialize)]
pub struct FsCfg {
    pub butterfly_data_path: PathBuf,
    pub census_data_path: PathBuf,
    pub services_data_path: PathBuf,

    data_path: PathBuf,
    specs_path: PathBuf,
    member_id_file: PathBuf,
    proc_lock_file: PathBuf,
}

impl FsCfg {
    fn new<T>(sup_svc_root: T) -> Self
    where
        T: Into<PathBuf>,
    {
        let sup_svc_root = sup_svc_root.into();
        let data_path = sup_svc_root.join("data");
        FsCfg {
            butterfly_data_path: data_path.join("butterfly.dat"),
            census_data_path: data_path.join("census.dat"),
            services_data_path: data_path.join("services.dat"),
            specs_path: sup_svc_root.join("specs"),
            data_path: data_path,
            member_id_file: sup_svc_root.join(MEMBER_ID_FILE),
            proc_lock_file: sup_svc_root.join(PROC_LOCK_FILE),
        }
    }

    pub fn health_check_cache(&self, service_group: &ServiceGroup) -> PathBuf {
        self.data_path.join(
            format!("{}.health", service_group.service()),
        )
    }
}

#[derive(Clone, Default)]
pub struct ManagerConfig {
    pub auto_update: bool,
    pub eventsrv_group: Option<ServiceGroup>,
    pub update_url: String,
    pub update_channel: String,
    pub gossip_listen: GossipListenAddr,
    pub http_listen: http_gateway::ListenAddr,
    pub gossip_peers: Vec<SocketAddr>,
    pub gossip_permanent: bool,
    pub ring: Option<String>,
    pub name: Option<String>,
    pub organization: Option<String>,

    custom_state_path: Option<PathBuf>,
}

pub struct Manager {
    butterfly: butterfly::Server,
    census_ring: CensusRing,
    events_group: Option<ServiceGroup>,
    fs_cfg: Arc<FsCfg>,
    launcher: LauncherCli,
    services: Arc<RwLock<Vec<Service>>>,
    updater: ServiceUpdater,
    watcher: SpecWatcher,
    organization: Option<String>,
    self_updater: Option<SelfUpdater>,
    service_states: HashMap<PackageIdent, Timespec>,
    sys: Arc<Sys>,
}

impl Manager {
    /// Determines if there is already a Habitat Supervisor running on the host system.
    pub fn is_running(cfg: &ManagerConfig) -> Result<bool> {
        let state_path = Self::state_path_from(&cfg);
        let fs_cfg = FsCfg::new(state_path);

        match read_process_lock(&fs_cfg.proc_lock_file) {
            Ok(pid) => Ok(process::is_alive(pid)),
            Err(SupError { err: Error::ProcessLockCorrupt, .. }) => Ok(false),
            Err(SupError { err: Error::ProcessLockIO(_, _), .. }) => {
                // JW TODO: We need to check the raw OS error and translate it to a "file not found"
                // case. This is an acceptable reason to assume that another manager is not running
                // but other IO errors are an actual problem. For now, let's just assume an IO
                // error here is a file not found.
                Ok(false)
            }
            Err(err) => Err(err),
        }
    }

    /// Load a Manager with the given configuration.
    ///
    /// The returned Manager will be pre-populated with any cached data from disk from a previous
    /// run if available.
    pub fn load(cfg: ManagerConfig, launcher: LauncherCli) -> Result<Manager> {
        let state_path = Self::state_path_from(&cfg);
        Self::create_state_path_dirs(&state_path)?;
        Self::clean_dirty_state(&state_path)?;
        let fs_cfg = FsCfg::new(state_path);
        obtain_process_lock(&fs_cfg)?;

        Self::new(cfg, fs_cfg, launcher)
    }

    pub fn service_status(cfg: ManagerConfig, ident: PackageIdent) -> Result<ServiceStatus> {
        for status in Self::status(cfg)? {
            if status.pkg.ident.satisfies(&ident) {
                return Ok(status);
            }
        }
        Err(sup_error!(Error::ServiceNotLoaded(ident)))
    }

    pub fn status(cfg: ManagerConfig) -> Result<Vec<ServiceStatus>> {
        let state_path = Self::state_path_from(&cfg);
        let fs_cfg = FsCfg::new(state_path);

        let dat = File::open(&fs_cfg.services_data_path)?;
        serde_json::from_reader(&dat).map_err(|e| sup_error!(Error::ServiceDeserializationError(e)))
    }

    pub fn term(cfg: &ManagerConfig) -> Result<()> {
        let state_path = Self::state_path_from(&cfg);
        let fs_cfg = FsCfg::new(state_path);
        match read_process_lock(&fs_cfg.proc_lock_file) {
            Ok(pid) => {
                process::signal(pid, Signal::TERM).map_err(|_| {
                    sup_error!(Error::SignalFailed)
                })?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn new(cfg: ManagerConfig, fs_cfg: FsCfg, launcher: LauncherCli) -> Result<Manager> {
        let current = PackageIdent::from_str(&format!("{}/{}", SUP_PKG_IDENT, VERSION)).unwrap();
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
        let mut sys = Sys::new(cfg.gossip_permanent, cfg.gossip_listen, cfg.http_listen);
        let member = Self::load_member(&mut sys, &fs_cfg)?;
        let ring_key = match cfg.ring {
            Some(ref ring_with_revision) => {
                outputln!("Joining ring {}", ring_with_revision);
                Some(SymKey::get_pair_for(
                    &ring_with_revision,
                    &default_cache_key_path(None),
                )?)
            }
            None => None,
        };
        let services = Arc::new(RwLock::new(Vec::new()));
        let server = butterfly::Server::new(
            sys.gossip_listen(),
            sys.gossip_listen(),
            member,
            Trace::default(),
            ring_key,
            None,
            Some(&fs_cfg.data_path),
            Box::new(SuitabilityLookup(services.clone())),
        )?;
        outputln!("Supervisor Member-ID {}", sys.member_id);
        for peer_addr in &cfg.gossip_peers {
            let mut peer = Member::default();
            peer.set_address(format!("{}", peer_addr.ip()));
            peer.set_swim_port(peer_addr.port() as i32);
            peer.set_gossip_port(peer_addr.port() as i32);
            server.member_list.add_initial_member(peer);
        }
        Ok(Manager {
            self_updater: self_updater,
            updater: ServiceUpdater::new(server.clone()),
            census_ring: CensusRing::new(sys.member_id.clone()),
            butterfly: server,
            events_group: cfg.eventsrv_group,
            launcher: launcher,
            services: services,
            watcher: SpecWatcher::run(&fs_cfg.specs_path)?,
            fs_cfg: Arc::new(fs_cfg),
            organization: cfg.organization,
            service_states: HashMap::new(),
            sys: Arc::new(sys),
        })
    }

    /// Load the initial Butterly Member which is used in initializing the Butterfly server. This
    /// will load the member-id for the initial Member from disk if a previous manager has been
    /// run.
    ///
    /// The mutable ref to `Sys` will be configured with Butterfly Member details and will also
    /// populate the initial Member.
    fn load_member(sys: &mut Sys, fs_cfg: &FsCfg) -> Result<Member> {
        let mut member = Member::default();
        match File::open(&fs_cfg.member_id_file) {
            Ok(mut file) => {
                let mut member_id = String::new();
                file.read_to_string(&mut member_id).map_err(|e| {
                    sup_error!(Error::BadDataFile(fs_cfg.member_id_file.clone(), e))
                })?;
                member.set_id(member_id);
            }
            Err(_) => {
                match File::create(&fs_cfg.member_id_file) {
                    Ok(mut file) => {
                        file.write(member.get_id().as_bytes()).map_err(|e| {
                            sup_error!(Error::BadDataFile(fs_cfg.member_id_file.clone(), e))
                        })?;
                    }
                    Err(err) => {
                        return Err(sup_error!(
                            Error::BadDataFile(fs_cfg.member_id_file.clone(), err)
                        ))
                    }
                }
            }
        }
        sys.member_id = member.get_id().to_string();
        member.set_persistent(sys.permanent);
        Ok(member)
    }

    pub fn spec_path_for(cfg: &ManagerConfig, spec: &ServiceSpec) -> PathBuf {
        Self::specs_path(&Self::state_path_from(cfg)).join(spec.file_name())
    }

    pub fn save_spec_for(cfg: &ManagerConfig, spec: ServiceSpec) -> Result<()> {
        spec.to_file(Self::spec_path_for(cfg, &spec))
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

    fn state_path_from(cfg: &ManagerConfig) -> PathBuf {
        match cfg.custom_state_path {
            Some(ref custom) => custom.clone(),
            None => {
                match cfg.name {
                    Some(ref name) => STATE_PATH_PREFIX.join(name),
                    None => STATE_PATH_PREFIX.join("default"),
                }
            }
        }
    }

    fn add_service(&mut self, spec: ServiceSpec) {
        outputln!("Starting {}", &spec.ident);
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
            Ok(service) => service,
            Err(err) => {
                outputln!("Unable to start {}, {}", &spec.ident, err);
                if spec.start_style == StartStyle::Transient {
                    self.remove_spec(&spec);
                }
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
            self.butterfly.start_election(
                service.service_group.clone(),
                0,
            );
        }
        self.updater.add(&service);
        self.services
            .write()
            .expect("Services lock is poisoned!")
            .push(service);
    }

    fn remove_service(&self, service: &mut Service, term: bool) {
        // JW TODO: Update service rumor to remove service from cluster
        if term {
            service.stop(Some(&self.launcher));
        } else {
            service.stop(None);
        }
        if service.start_style == StartStyle::Transient {
            // JW TODO: If we cleanup our Service structure to hold the ServiceSpec instead of
            // deconstruct it (see my comments in `add_service()` in this module) then we could
            // leverage `remove_spec()` instead of duplicaing this logic here.
            if let Err(err) = fs::remove_file(&service.spec_file) {
                outputln!(
                    "Unable to cleanup service spec for transient service, {}, {}",
                    service,
                    err
                );
            }
        }
        if let Err(err) = fs::remove_file(self.fs_cfg.health_check_cache(&service.service_group)) {
            outputln!(
                "Unable to cleanup service health cache, {}, {}",
                service,
                err
            );
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.start_initial_services_from_watcher()?;

        outputln!(
            "Starting gossip-listener on {}",
            self.butterfly.gossip_addr()
        );
        self.butterfly.start(Timing::default())?;
        debug!("gossip-listener started");
        self.persist_state();
        let http_listen_addr = self.sys.http_listen();
        outputln!("Starting http-gateway on {}", &http_listen_addr);
        http_gateway::Server::new(self.fs_cfg.clone(), http_listen_addr)
            .start()?;
        debug!("http-gateway started");
        let events = match self.events_group {
            Some(ref evg) => Some(events::EventsMgr::start(evg.clone())),
            None => None,
        };
        loop {
            let next_check = time::get_time() + TimeDuration::milliseconds(1000);
            if self.launcher.is_stopping() {
                self.shutdown();
                return Ok(());
            }
            if self.check_for_departure() {
                self.shutdown();
                return Err(sup_error!(Error::Departed));
            }
            if let Some(package) = self.check_for_updated_supervisor() {
                outputln!(
                    "Supervisor shutting down for automatic update to {}",
                    package
                );
                self.shutdown();
                return Ok(());
            }
            self.update_running_services_from_watcher()?;
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

            if self.census_ring.changed {
                self.persist_state();
                events.as_ref().map(|events| {
                    events.try_connect(&self.census_ring)
                });

                for service in self.services
                    .read()
                    .expect("Services lock is poisoned!")
                    .iter()
                {
                    if let Some(census_group) =
                        self.census_ring.census_group_for(&service.service_group)
                    {
                        if let Some(member) = census_group.me() {
                            events.as_ref().map(|events| events.send_census(member));
                        }
                    }
                }
            }

            for service in self.services
                .write()
                .expect("Services lock is poisoned!")
                .iter_mut()
            {
                if service.tick(&self.census_ring, &self.launcher) {
                    self.gossip_latest_service_rumor(&service);
                }
            }
            let time_to_wait = (next_check - time::get_time()).num_milliseconds();
            if time_to_wait > 0 {
                thread::sleep(Duration::from_millis(time_to_wait as u64));
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
        for service in self.services
            .write()
            .expect("Services lock is poisoned!")
            .iter_mut()
        {
            if self.updater.check_for_updated_package(
                service,
                &self.census_ring,
            )
            {
                self.gossip_latest_service_rumor(&service);
            }
        }
    }

    fn gossip_latest_service_rumor(&self, service: &Service) {
        let mut incarnation = 1;
        {
            let list = self.butterfly.service_store.list.read().expect(
                "Rumor store lock poisoned",
            );
            if let Some(rumor) = list.get(&*service.service_group).and_then(|r| {
                r.get(&self.sys.member_id)
            })
            {
                incarnation = rumor.clone().get_incarnation() + 1;
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
        for service in self.services
            .write()
            .expect("Services lock is poisoned!")
            .iter_mut()
        {
            service_states.insert(service.spec_ident.clone(), service.last_state_change());
            active_services.push(service.spec_ident.clone());
        }

        for loaded in self.watcher
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
            .write(serde_json::to_string(&self.census_ring).unwrap().as_bytes())
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
            .write(serde_json::to_string(&self.butterfly).unwrap().as_bytes())
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
        let mut writer = BufWriter::new(file);
        if let Some(err) = writer.get_mut().write("[".as_bytes()).err() {
            warn!("Couldn't write to service state file, {}", err);
        }

        let mut is_first = true;
        let mut persisted_idents = Vec::new();

        for service in self.services
            .read()
            .expect("Services lock is poisoned!")
            .iter()
        {
            persisted_idents.push(service.spec_ident.clone());
            if let Some(err) = self.write_service(service, is_first, writer.get_mut())
                .err()
            {
                warn!("Couldn't write to service state file, {}", err);
            }
            is_first = false;
        }

        // add services that are not active but are being watched for changes
        // These would include stopped persistent services or other
        // persistent services that failed to load
        for down in self.watcher
            .specs_from_watch_path()
            .unwrap()
            .values()
            .filter(|s| !persisted_idents.contains(&s.ident))
        {
            match Service::load(
                self.sys.clone(),
                down.clone(),
                self.fs_cfg.clone(),
                self.organization.as_ref().map(|org| &**org),
            ) {
                Ok(service) => {
                    if let Some(err) = self.write_service(&service, is_first, writer.get_mut())
                        .err()
                    {
                        warn!("Couldn't write to service state file, {}", err);
                    }
                    is_first = false;
                }
                Err(e) => debug!("Error loading inactive service struct: {}", e),
            }
        }

        if let Some(err) = writer.get_mut().write("]".as_bytes()).err() {
            warn!("Couldn't write to service state file, {}", err);
        }
        if let Some(err) = writer.flush().err() {
            warn!("Couldn't flush services state buffer to disk, {}", err);
        }
        if let Some(err) = fs::rename(&tmp_file, &self.fs_cfg.services_data_path).err() {
            warn!("Couldn't finalize services state on disk, {}", err);
        }
    }

    fn write_service<W: ?Sized>(
        &self,
        service: &Service,
        is_first: bool,
        writer: &mut W,
    ) -> Result<()>
    where
        W: io::Write,
    {
        if !is_first {
            writer.write(",".as_bytes())?;
        }
        serde_json::to_writer(writer, service).map_err(|e| {
            sup_error!(Error::ServiceSerializationError(e))
        })
    }

    /// Check if any elections need restarting.
    fn restart_elections(&mut self) {
        self.butterfly.restart_elections();
    }

    fn shutdown(&self) {
        outputln!("Gracefully departing from butterfly network.");
        self.butterfly.set_departed();

        let mut services = self.services.write().expect("Services lock is poisend!");

        for mut service in services.drain(..) {
            self.remove_service(&mut service, false);
        }
        release_process_lock(&self.fs_cfg);
    }

    fn start_initial_services_from_watcher(&mut self) -> Result<()> {
        for service_event in self.watcher.initial_events()? {
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

    fn update_running_services_from_watcher(&mut self) -> Result<()> {
        let mut active_specs = HashMap::new();
        for service in self.services
            .read()
            .expect("Services lock is poisoned!")
            .iter()
        {
            let spec = service.to_spec();
            active_specs.insert(spec.ident.name.clone(), spec);
        }

        for service_event in self.watcher.new_events(active_specs)? {
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

    fn remove_service_for_spec(&mut self, spec: &ServiceSpec) -> Result<()> {
        let mut services = self.services.write().expect("Services lock is poisoned");
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
        let mut service = services.remove(services_idx);
        self.remove_service(&mut service, true);
        Ok(())
    }

    /// Remove the on disk representation of the given service spec
    fn remove_spec(&self, spec: &ServiceSpec) {
        if let Err(err) = fs::remove_file(self.fs_cfg.specs_path.join(spec.file_name())) {
            outputln!(
                "Unable to cleanup service spec for transient service, {}, {}",
                spec.ident,
                err
            );
        }
    }
}

#[derive(Deserialize)]
pub struct ProcessStatus {
    #[serde(deserialize_with = "deserialize_time", rename = "state_entered")]
    pub elapsed: TimeDuration,
    pub pid: Option<u32>,
    pub state: ProcessState,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.pid {
            Some(pid) => {
                write!(
                    f,
                    "state:{}, time:{}, pid:{}",
                    self.state,
                    self.elapsed,
                    pid
                )
            }
            None => write!(f, "state:{}, time:{}", self.state, self.elapsed),
        }

    }
}

#[derive(Deserialize)]
pub struct ServiceStatus {
    pub pkg: Pkg,
    pub process: ProcessStatus,
    pub service_group: ServiceGroup,
    pub start_style: StartStyle,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, group:{}, style:{}",
            self.pkg.ident,
            self.process,
            self.service_group,
            self.start_style
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
        Err(_) => {
            match read_process_lock(&fs_cfg.proc_lock_file) {
                Ok(pid) => {
                    if process::is_alive(pid) {
                        return Err(sup_error!(Error::ProcessLocked(pid)));
                    }
                    release_process_lock(fs_cfg);
                    write_process_lock(&fs_cfg.proc_lock_file)
                }
                Err(SupError { err: Error::ProcessLockCorrupt, .. }) => {
                    release_process_lock(fs_cfg);
                    write_process_lock(&fs_cfg.proc_lock_file)
                }
                Err(err) => Err(err),
            }
        }
    }
}

fn read_process_lock<T>(lock_path: T) -> Result<u32>
where
    T: AsRef<Path>,
{
    match File::open(lock_path.as_ref()) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => {
                    match line.parse::<u32>() {
                        Ok(pid) => Ok(pid),
                        Err(_) => Err(sup_error!(Error::ProcessLockCorrupt)),
                    }
                }
                _ => Err(sup_error!(Error::ProcessLockCorrupt)),
            }
        }
        Err(err) => Err(sup_error!(
            Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err)
        )),
    }
}

fn release_process_lock(fs_cfg: &FsCfg) {
    if let Err(err) = fs::remove_file(&fs_cfg.proc_lock_file) {
        debug!("Couldn't cleanup supervisor process lock, {}", err);
    }
}

fn write_process_lock<T>(lock_path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    match OpenOptions::new().write(true).create_new(true).open(
        lock_path
            .as_ref(),
    ) {
        Ok(mut file) => {
            match write!(&mut file, "{}", process::current_pid()) {
                Ok(()) => Ok(()),
                Err(err) => {
                    Err(sup_error!(
                        Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err)
                    ))
                }
            }
        }
        Err(err) => Err(sup_error!(
            Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err)
        )),
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{Manager, ManagerConfig, STATE_PATH_PREFIX};

    #[test]
    fn manager_state_path_default() {
        let cfg = ManagerConfig::default();
        let path = Manager::state_path_from(&cfg);

        assert_eq!(
            PathBuf::from(format!("{}/default", STATE_PATH_PREFIX.to_string_lossy())),
            path
        );
    }

    #[test]
    fn manager_state_path_with_name() {
        let mut cfg = ManagerConfig::default();
        cfg.name = Some(String::from("peanuts"));
        let path = Manager::state_path_from(&cfg);

        assert_eq!(
            PathBuf::from(format!("{}/peanuts", STATE_PATH_PREFIX.to_string_lossy())),
            path
        );
    }

    #[test]
    fn manager_state_path_custom() {
        let mut cfg = ManagerConfig::default();
        cfg.custom_state_path = Some(PathBuf::from("/tmp/peanuts-and-cake"));
        let path = Manager::state_path_from(&cfg);

        assert_eq!(PathBuf::from("/tmp/peanuts-and-cake"), path);
    }

    #[test]
    fn manager_state_path_custom_beats_name() {
        let mut cfg = ManagerConfig::default();
        cfg.custom_state_path = Some(PathBuf::from("/tmp/partay"));
        cfg.name = Some(String::from("nope"));
        let path = Manager::state_path_from(&cfg);

        assert_eq!(PathBuf::from("/tmp/partay"), path);
    }
}
