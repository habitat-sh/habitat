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

mod census;
pub mod service;
mod signals;
mod service_updater;
mod spec_watcher;

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use butterfly;
use butterfly::member::Member;
use butterfly::trace::Trace;
use butterfly::server::timing::Timing;
use butterfly::server::Suitability;
use hcore::crypto::{default_cache_key_path, SymKey};
use hcore::service::ServiceGroup;
use hcore::os::process;
use serde_json;
use time::{SteadyTime, Duration as TimeDuration};
use toml;

pub use manager::service::{Service, ServiceConfig, ServiceSpec, UpdateStrategy, Topology};
use self::service::{DesiredState, StartStyle};
use self::service_updater::ServiceUpdater;
use self::spec_watcher::{SpecWatcher, SpecWatcherEvent};
use error::{Error, Result, SupError};
use config::GossipListenAddr;
use manager::census::{CensusUpdate, CensusList, CensusEntry};
use manager::signals::SignalEvent;
use http_gateway;

const STATE_PATH_PREFIX: &'static str = "/hab/sup";
const MEMBER_ID_FILE: &'static str = "MEMBER_ID";
const PROC_LOCK_FILE: &'static str = "LOCK";

static LOGKEY: &'static str = "MR";

/// FileSystem paths that the Manager uses to persist data to disk.
///
/// This is shared with the `http_gateway` and `service` modules for reading and writing
/// persistence data.
#[derive(Debug)]
pub struct FsCfg {
    data_path: PathBuf,
    pub butterfly_data_path: PathBuf,
    pub census_data_path: PathBuf,
    pub services_data_path: PathBuf,
    specs_path: PathBuf,
    proc_lock_file: PathBuf,
}

impl FsCfg {
    fn new<T>(sup_svc_root: T) -> Self
        where T: Into<PathBuf>
    {
        let sup_svc_root = sup_svc_root.into();
        let data_path = sup_svc_root.join("data");
        FsCfg {
            butterfly_data_path: data_path.join("butterfly.dat"),
            census_data_path: data_path.join("census.dat"),
            services_data_path: data_path.join("services.dat"),
            specs_path: sup_svc_root.join("specs"),
            data_path: data_path,
            proc_lock_file: sup_svc_root.join(PROC_LOCK_FILE),
        }
    }

    pub fn health_check_cache(&self, service_group: &ServiceGroup) -> PathBuf {
        self.data_path
            .join(format!("{}.health", service_group.service()))
    }
}

#[derive(Default)]
pub struct ManagerConfig {
    pub gossip_listen: GossipListenAddr,
    pub http_listen: http_gateway::ListenAddr,
    pub gossip_peers: Vec<SocketAddr>,
    pub gossip_permanent: bool,
    pub ring: Option<String>,
    pub name: Option<String>,
    custom_state_path: Option<PathBuf>,
    pub organization: Option<String>,
}

pub struct Manager {
    butterfly: butterfly::Server,
    census_list: CensusList,
    fs_cfg: Arc<FsCfg>,
    services: Arc<RwLock<Vec<Service>>>,
    updater: ServiceUpdater,
    watcher: SpecWatcher,
    gossip_listen: GossipListenAddr,
    http_listen: http_gateway::ListenAddr,
    organization: Option<String>,
}

impl Manager {
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

    pub fn load(cfg: ManagerConfig) -> Result<Manager> {
        let state_path = Self::state_path_from(&cfg);
        Self::create_state_path_dirs(&state_path)?;
        Self::clean_dirty_state(&state_path)?;
        let member = Self::load_member(&state_path)?;
        let fs_cfg = FsCfg::new(state_path);
        obtain_process_lock(&fs_cfg)?;

        Self::new(cfg, member, fs_cfg)
    }

    fn new(cfg: ManagerConfig, mut member: Member, fs_cfg: FsCfg) -> Result<Manager> {
        member.set_persistent(cfg.gossip_permanent);
        member.set_swim_port(cfg.gossip_listen.port() as i32);
        member.set_gossip_port(cfg.gossip_listen.port() as i32);

        let ring_key = match cfg.ring {
            Some(ref ring_with_revision) => {
                outputln!("Joining ring {}", ring_with_revision);
                Some(SymKey::get_pair_for(&ring_with_revision, &default_cache_key_path(None))?)
            }
            None => None,
        };

        let services = Arc::new(RwLock::new(Vec::new()));
        let server = butterfly::Server::new(&cfg.gossip_listen,
                                            &cfg.gossip_listen,
                                            member,
                                            Trace::default(),
                                            ring_key,
                                            None,
                                            Some(&fs_cfg.data_path),
                                            Box::new(SuitabilityLookup(services.clone())))?;
        outputln!("Butterfly Member ID {}", server.member_id());
        for peer_addr in &cfg.gossip_peers {
            let mut peer = Member::default();
            peer.set_address(format!("{}", peer_addr.ip()));
            peer.set_swim_port(peer_addr.port() as i32);
            peer.set_gossip_port(peer_addr.port() as i32);
            server.member_list.add_initial_member(peer);
        }
        Ok(Manager {
               updater: ServiceUpdater::new(server.clone()),
               butterfly: server,
               census_list: CensusList::new(),
               services: services,
               watcher: SpecWatcher::run(&fs_cfg.specs_path)?,
               fs_cfg: Arc::new(fs_cfg),
               gossip_listen: cfg.gossip_listen,
               http_listen: cfg.http_listen,
               organization: cfg.organization,
           })
    }

    fn load_member<T>(state_path: T) -> Result<Member>
        where T: AsRef<Path>
    {
        let mut member = Member::default();
        let file_path = state_path.as_ref().join(MEMBER_ID_FILE);
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut member_id = String::new();
                file.read_to_string(&mut member_id)
                    .map_err(|e| sup_error!(Error::BadDataFile(file_path, e)))?;
                member.set_id(member_id);
            }
            Err(_) => {
                match File::create(&file_path) {
                    Ok(mut file) => {
                        file.write(member.get_id().as_bytes())
                            .map_err(|e| sup_error!(Error::BadDataFile(file_path.clone(), e)))?;
                    }
                    Err(err) => return Err(sup_error!(Error::BadDataFile(file_path.clone(), err))),
                }
            }
        }
        Ok(member)
    }

    pub fn spec_path_for(cfg: &ManagerConfig, spec: &ServiceSpec) -> PathBuf {
        Self::specs_path(&Self::state_path_from(cfg)).join(spec.file_name())
    }

    pub fn save_spec_for(cfg: &ManagerConfig, spec: ServiceSpec) -> Result<()> {
        spec.to_file(Self::spec_path_for(cfg, &spec))
    }

    fn clean_dirty_state<T>(state_path: T) -> Result<()>
        where T: AsRef<Path>
    {
        let data_path = Self::data_path(&state_path);
        debug!("Cleaning cached health checks");
        match fs::read_dir(&data_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        match entry.path().extension().and_then(|p| p.to_str()) {
                            Some("tmp") | Some("health") => {
                                fs::remove_file(&entry.path())
                                    .map_err(|err| {
                                                 sup_error!(Error::BadDataPath(data_path.clone(),
                                                                               err))
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
        where T: AsRef<Path>
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
        where T: AsRef<Path>
    {
        state_path.as_ref().join("data")
    }

    #[inline]
    fn specs_path<T>(state_path: T) -> PathBuf
        where T: AsRef<Path>
    {
        state_path.as_ref().join("specs")
    }

    fn state_path_from(cfg: &ManagerConfig) -> PathBuf {
        match cfg.custom_state_path {
            Some(ref custom) => custom.clone(),
            None => {
                match cfg.name {
                    Some(ref name) => PathBuf::from(STATE_PATH_PREFIX).join(name),
                    None => PathBuf::from(STATE_PATH_PREFIX).join("default"),
                }
            }
        }
    }

    fn add_service(&mut self, spec: ServiceSpec) -> Result<()> {
        let service = Service::load(spec,
                                    &self.gossip_listen,
                                    &self.http_listen,
                                    self.fs_cfg.clone(),
                                    self.organization.as_ref().map(|org| &**org))?;
        service.add()?;
        self.butterfly
            .insert_service(service.to_rumor(self.butterfly.member_id()));
        if service.topology == Topology::Leader {
            // TODO: eventually, we need to deal with suitability here. The original implementation
            // didn't have this working either.
            self.butterfly
                .start_election(service.service_group.clone(), 0);
        }
        self.updater.add(&service);
        self.services
            .write()
            .expect("Services lock is poisoned!")
            .push(service);
        Ok(())
    }

    fn remove_service(&self, service: &mut Service) -> Result<()> {
        // JW TODO: Update service rumor to remove service from cluster
        service.stop();
        if service.start_style == StartStyle::Transient {
            if let Err(err) = fs::remove_file(&service.spec_file) {
                outputln!("Unable to cleanup service spec for transient service, {}, {}",
                          service,
                          err);
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        signals::init();
        self.start_initial_services_from_watcher()?;

        outputln!("Starting butterfly on {}", self.butterfly.gossip_addr());
        try!(self.butterfly.start(Timing::default()));
        debug!("butterfly server started");
        self.persist_state();
        outputln!("Starting http-gateway on {}", self.http_listen);
        try!(http_gateway::Server::new(self.fs_cfg.clone(), self.http_listen.clone()).start());
        debug!("http-gateway server started");

        let mut last_census_update = CensusUpdate::default();

        loop {
            let next_check = SteadyTime::now() + TimeDuration::milliseconds(1000);
            if self.check_for_incoming_signals() {
                self.shutdown();
                return Ok(());
            }
            self.update_running_services_from_watcher()?;
            self.check_for_updated_packages(&mut last_census_update);
            self.restart_elections();
            let (census_updated, ncu) = self.build_census(&last_census_update);
            if census_updated {
                last_census_update = ncu;
                self.persist_state();
            }
            for service in self.services
                    .write()
                    .expect("Services lock is poisoned!")
                    .iter_mut() {
                service.tick(&self.butterfly,
                             &self.census_list,
                             census_updated,
                             &mut last_census_update)
            }
            let time_to_wait = (next_check - SteadyTime::now()).num_milliseconds();
            if time_to_wait > 0 {
                thread::sleep(Duration::from_millis(time_to_wait as u64));
            }
        }
    }

    // Try and build the census from the gossip data, updating the last_census_update with
    // the resulting checkpoints. The census is our representation of the data produced
    // by Butterfly.
    fn build_census(&mut self, last_update: &CensusUpdate) -> (bool, CensusUpdate) {
        let update = CensusUpdate::new(&self.butterfly);
        if update != *last_update {
            // JW TODO: We should re-use the already allocated census list and entries instead of
            // recreating entirely new structures. We can, and should, only modify structures which
            // have had their incarnation updated.
            let mut cl = CensusList::new();
            debug!("Updating census from butterfly data");
            self.butterfly
                .service_store
                .with_keys(|(_group, rumors)| for (_member_id, service) in rumors.iter() {
                               let mut ce = CensusEntry::default();
                               ce.populate_from_service(service);
                               cl.insert(String::from(self.butterfly.member_id()), ce);
                           });
            self.butterfly
                .election_store
                .with_keys(|(_service_group, rumors)| {
                               // We know you have an election, and this is the only key in the hash
                               let election = rumors.get("election").unwrap();
                               cl.populate_from_election(election);
                           });
            self.butterfly
                .update_store
                .with_keys(|(_service_group, rumors)| {
                               // We know you have an election, and this is the only key in the hash
                               let election = rumors.get("election").unwrap();
                               cl.populate_from_update_election(election);
                           });
            self.butterfly
                .member_list
                .with_members(|member| {
                                  cl.populate_from_member(member);
                                  if let Some(health) = self.butterfly
                                         .member_list
                                         .health_of(member) {
                                      cl.populate_from_health(member, health);
                                  }
                              });
            self.census_list = cl;
            return (true, update);
        }
        (false, update)
    }

    // Takes signals passed to `hab-sup` and either shuts down all the services, or
    // passes the signals through. This functionality is totally going to need a refactor
    // when we get all the way to a single-sup-per-kernel model, since passing all random
    // signals through to all services is most certainly not what you want.
    //
    // This function returns true if we are supposed to shut the system down, false if we
    // can keep going.
    fn check_for_incoming_signals(&mut self) -> bool {
        match signals::check_for_signal() {
            Some(SignalEvent::Shutdown) => {
                for service in self.services
                        .write()
                        .expect("Services lock is poisoned!")
                        .iter_mut() {
                    outputln!("Shutting down {}", service);
                    service
                        .down()
                        .unwrap_or_else(|err| outputln!("Failed to shutdown {}: {}", service, err));
                }
                true
            }
            Some(SignalEvent::Passthrough(signal_code)) => {
                for service in self.services
                        .read()
                        .expect("Services lock is poisoned!")
                        .iter() {
                    outputln!("Forwarding signal {} to {}", signal_code, service);
                    if let Err(e) = service.send_signal(signal_code) {
                        outputln!("Failed to send signal {} to {}: {}",
                                  signal_code,
                                  service,
                                  e);
                    }
                }
                false
            }
            None => false,
        }
    }

    /// Walk each service and check if it has an updated package installed via the Update Strategy.
    /// This updates the Service to point to the new service struct, and then marks it for
    /// restarting.
    ///
    /// The run loop's last updated census is a required parameter on this function to inform the
    /// main loop that we, ourselves, updated the service counter when we updated ourselves.
    fn check_for_updated_packages(&mut self, last_update: &mut CensusUpdate) {
        let member_id = {
            self.butterfly.member_id().to_string()
        };
        for service in self.services
                .write()
                .expect("Services lock is poisoned!")
                .iter_mut() {
            if self.updater
                   .check_for_updated_package(service, &self.census_list) {
                let mut rumor = {
                    let list = self.butterfly
                        .service_store
                        .list
                        .read()
                        .expect("Rumor store lock poisoned");
                    list.get(&*service.service_group)
                        .and_then(|r| r.get(&member_id))
                        .unwrap()
                        .clone()
                };
                let incarnation = rumor.get_incarnation() + 1;
                rumor.set_pkg(service.package().to_string());
                rumor.set_incarnation(incarnation);
                service.populate(&self.census_list);
                // TODO FN: the updated toml API returns a `Result` when serializing--we should
                // handle this and not potentially panic
                match service.config.to_exported() {
                    Ok(cfg) => {
                        *rumor.mut_cfg() =
                            toml::ser::to_vec(&cfg).expect("Can't serialize to TOML bytes")
                    }
                    Err(err) => warn!("Error loading service config after update, err={}", err),
                }
                self.butterfly.insert_service(rumor);
                last_update.service_counter += 1;
            }
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
               .write(serde_json::to_string(&self.census_list)
                          .unwrap()
                          .as_bytes())
               .err() {
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
        let tmp_file = self.fs_cfg
            .butterfly_data_path
            .with_extension("dat.tmp");
        let file = match File::create(&tmp_file) {
            Ok(file) => file,
            Err(err) => {
                warn!("Couldn't open temporary butterfly state file, {}", err);
                return;
            }
        };
        let mut writer = BufWriter::new(file);
        if let Some(err) = writer
               .write(serde_json::to_string(&self.butterfly)
                          .unwrap()
                          .as_bytes())
               .err() {
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
        let services = self.services.read().expect("Services lock poisoned");
        if let Some(err) = writer
               .write(serde_json::to_string(&*services).unwrap().as_bytes())
               .err() {
            warn!("Couldn't write to services state file, {}", err);
        }
        if let Some(err) = writer.flush().err() {
            warn!("Couldn't flush services state buffer to disk, {}", err);
        }
        if let Some(err) = fs::rename(&tmp_file, &self.fs_cfg.services_data_path).err() {
            warn!("Couldn't finalize services state on disk, {}", err);
        }
    }

    /// Check if any elections need restarting.
    fn restart_elections(&mut self) {
        self.butterfly.restart_elections();
    }

    fn shutdown(&self) {
        let mut services = self.services
            .write()
            .expect("Services lock is poisend!");
        for mut service in services.drain(..) {
            if let Err(err) = self.remove_service(&mut service) {
                warn!("Couldn't cleanly shutdown service, {}, {}", service, err);
            }
        }
        release_process_lock(&self.fs_cfg);
        outputln!("Habitat thanks you - shutting down!");
    }

    fn start_initial_services_from_watcher(&mut self) -> Result<()> {
        for service_event in self.watcher.initial_events()? {
            match service_event {
                SpecWatcherEvent::AddService(spec) => {
                    if spec.desired_state == DesiredState::Up {
                        self.add_service(spec)?;
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
                .iter() {
            let spec = service.to_spec();
            active_specs.insert(spec.ident.name.clone(), spec);
        }
        for service_event in self.watcher.new_events(active_specs)? {
            match service_event {
                SpecWatcherEvent::AddService(spec) => {
                    if spec.desired_state == DesiredState::Up {
                        self.add_service(spec)?;
                    }
                }
                SpecWatcherEvent::RemoveService(spec) => self.remove_service_for_spec(&spec)?,
            }
        }
        Ok(())
    }

    fn remove_service_for_spec(&mut self, spec: &ServiceSpec) -> Result<()> {
        let mut services = self.services
            .write()
            .expect("Services lock is poisoned");
        // TODO fn: storing services as a `Vec` is a bit crazy when you have to do these
        // shenanigans--maybe we want to consider changing the data structure in the future?
        let services_idx = match services
                  .iter()
                  .position(|ref s| s.spec_ident == spec.ident) {
            Some(i) => i,
            None => {
                outputln!("Tried to remove service for {} but could not find it running, skipping",
                          &spec.ident);
                return Ok(());
            }
        };
        let mut service = services.remove(services_idx);
        self.remove_service(&mut service)?;
        Ok(())
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
    where T: AsRef<Path>
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
        Err(err) => Err(sup_error!(Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err))),
    }
}

fn release_process_lock(fs_cfg: &FsCfg) {
    if let Err(err) = fs::remove_file(&fs_cfg.proc_lock_file) {
        debug!("Couldn't cleanup supervisor process lock, {}", err);
    }
}

fn write_process_lock<T>(lock_path: T) -> Result<()>
    where T: AsRef<Path>
{
    match OpenOptions::new()
              .write(true)
              .create_new(true)
              .open(lock_path.as_ref()) {
        Ok(mut file) => {
            match write!(&mut file, "{}", process::current_pid()) {
                Ok(()) => Ok(()),
                Err(err) => {
                    Err(sup_error!(Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err)))
                }
            }
        }
        Err(err) => Err(sup_error!(Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err))),
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

        assert_eq!(PathBuf::from(format!("{}/default", STATE_PATH_PREFIX)),
                   path);
    }

    #[test]
    fn manager_state_path_with_name() {
        let mut cfg = ManagerConfig::default();
        cfg.name = Some(String::from("peanuts"));
        let path = Manager::state_path_from(&cfg);

        assert_eq!(PathBuf::from(format!("{}/peanuts", STATE_PATH_PREFIX)),
                   path);
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
