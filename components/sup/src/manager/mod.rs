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

pub(crate) mod action;
pub mod service;
#[macro_use]
mod debug;
pub mod commands;
mod file_watcher;
mod peer_watcher;
mod periodic;
mod self_updater;
mod service_updater;
mod spec_dir;
mod spec_watcher;
pub(crate) mod sys;
mod user_config_watcher;

use self::{action::{ShutdownSpec,
                    SupervisorAction},
           peer_watcher::PeerWatcher,
           self_updater::{SelfUpdater,
                          SUP_PKG_IDENT},
           service::{ConfigRendering,
                     DesiredState,
                     HealthCheck,
                     Service,
                     ServiceProxy,
                     ServiceSpec,
                     Topology},
           service_updater::ServiceUpdater,
           spec_dir::SpecDir,
           spec_watcher::SpecWatcher,
           sys::Sys,
           user_config_watcher::UserConfigWatcher};
use crate::{census::{CensusRing,
                     CensusRingProxy},
            config::GossipListenAddr,
            ctl_gateway::{self,
                          acceptor::CtlAcceptor,
                          CtlRequest},
            error::{Error,
                    Result,
                    SupError},
            event::{self,
                    EventCore,
                    EventStreamConfig},
            http_gateway,
            VERSION};
use cpu_time::ProcessTime;
use futures::{future,
              prelude::*,
              sync::{mpsc as fut_mpsc,
                     oneshot}};
use habitat_butterfly::{member::Member,
                        server::{timing::Timing,
                                 ServerProxy,
                                 Suitability},
                        trace::Trace};
use habitat_common::{outputln,
                     types::ListenCtlAddr,
                     FeatureFlag};
use habitat_core::{crypto::SymKey,
                   env::{self,
                         Config},
                   fs::FS_ROOT_PATH,
                   os::{process::{self,
                                  Pid,
                                  Signal},
                        signals::{self,
                                  SignalEvent}},
                   package::{Identifiable,
                             PackageIdent,
                             PackageInstall},
                   service::ServiceGroup,
                   util::ToI64,
                   ChannelIdent};
use habitat_launcher_client::{LauncherCli,
                              LAUNCHER_LOCK_CLEAN_ENV,
                              LAUNCHER_PID_ENV};
use habitat_sup_protocol;
use num_cpus;
#[cfg(unix)]
use palaver;
use prometheus::{HistogramVec,
                 IntGauge,
                 IntGaugeVec};
use rustls::{internal::pemfile,
             AllowAnyAuthenticatedClient,
             NoClientAuth,
             RootCertStore,
             ServerConfig};
use serde_json;
use std::{collections::{HashMap,
                        HashSet},
          ffi::OsStr,
          fs::{self,
               File,
               OpenOptions},
          io::{BufRead,
               BufReader,
               Read,
               Write},
          iter::IntoIterator,
          net::SocketAddr,
          path::{Path,
                 PathBuf},
          result,
          str::FromStr,
          sync::{atomic::{AtomicBool,
                          Ordering},
                 mpsc as std_mpsc,
                 Arc,
                 Condvar,
                 Mutex,
                 RwLock},
          thread,
          time::Duration as StdDuration};
use time::{self,
           Duration as TimeDuration,
           SteadyTime,
           Timespec};
use tokio::{executor,
            runtime::{Builder as RuntimeBuilder,
                      Runtime}};
#[cfg(windows)]
use winapi::{shared::minwindef::PDWORD,
             um::processthreadsapi};

const MEMBER_ID_FILE: &str = "MEMBER_ID";
pub const PROC_LOCK_FILE: &str = "LOCK";

static LOGKEY: &'static str = "MR";

lazy_static! {
    static ref RUN_LOOP_DURATION: HistogramVec =
        register_histogram_vec!("hab_sup_run_loop_duration_seconds",
                                "The time it takes for one tick of a run loop",
                                &["loop"]).unwrap();
    static ref FILE_DESCRIPTORS: IntGauge = register_int_gauge!(
        "hab_sup_open_file_descriptors_total",
        "A count of the total number of open file descriptors. Unix only"
    ).unwrap();
    static ref MEMORY_STATS: IntGaugeVec =
        register_int_gauge_vec!("hab_sup_memory_allocations_bytes",
                                "Memory allocation statistics",
                                &["category"]).unwrap();
    static ref CPU_TIME: IntGauge = register_int_gauge!("hab_sup_cpu_time_nanoseconds",
                                                        "CPU time of the supervisor process in \
                                                         nanoseconds").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
enum ServiceOperation {
    Start(ServiceSpec),
    Stop(ServiceSpec),
    Restart {
        to_stop:  ServiceSpec,
        to_start: ServiceSpec,
    },
}

/// A Supervisor can stop in a handful of ways.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ShutdownMode {
    /// When the Supervisor is shutting down for normal reasons and
    /// should take all services down with it (i.e., it's actually
    /// shutting down).
    Normal,
    /// When the Supervisor has been manually departed from the
    /// Habitat network. All services should come down, as well.
    Departed,
    /// A Supervisor is updating itself, or is otherwise simply
    /// restarting. Services _do not_ get shut down.
    Restarting,
}

/// FileSystem paths that the Manager uses to persist data to disk.
///
/// This is shared with the `http_gateway` and `service` modules for reading and writing
/// persistence data.
#[derive(Debug, Serialize)]
pub struct FsCfg {
    pub sup_root: PathBuf,

    data_path:      PathBuf,
    specs_path:     PathBuf,
    member_id_file: PathBuf,
    proc_lock_file: PathBuf,
}

impl FsCfg {
    fn new<T>(sup_root: T) -> Self
        where T: Into<PathBuf>
    {
        let sup_root = sup_root.into();
        FsCfg { specs_path: sup_root.join("specs"),
                data_path: sup_root.join("data"),
                member_id_file: sup_root.join(MEMBER_ID_FILE),
                proc_lock_file: sup_root.join(PROC_LOCK_FILE),
                sup_root }
    }
}

#[derive(Clone, Debug)]
pub struct ManagerConfig {
    pub auto_update:         bool,
    pub custom_state_path:   Option<PathBuf>,
    pub cache_key_path:      PathBuf,
    pub update_url:          String,
    pub update_channel:      ChannelIdent,
    pub gossip_listen:       GossipListenAddr,
    pub ctl_listen:          ListenCtlAddr,
    pub http_listen:         http_gateway::ListenAddr,
    pub http_disable:        bool,
    pub gossip_peers:        Vec<SocketAddr>,
    pub gossip_permanent:    bool,
    pub ring_key:            Option<SymKey>,
    pub organization:        Option<String>,
    pub watch_peer_file:     Option<String>,
    pub tls_config:          Option<TLSConfig>,
    pub feature_flags:       FeatureFlag,
    pub event_stream_config: Option<EventStreamConfig>,
}

#[derive(Clone, Debug)]
pub struct TLSConfig {
    pub cert_path:    PathBuf,
    pub key_path:     PathBuf,
    pub ca_cert_path: Option<PathBuf>,
}

impl ManagerConfig {
    pub fn sup_root(&self) -> PathBuf {
        habitat_sup_protocol::sup_root(self.custom_state_path.as_ref())
    }

    // TODO (CM): this may be able to be private after some
    // refactorings in commands.rs
    pub fn spec_path_for(&self, spec: &ServiceSpec) -> PathBuf {
        self.sup_root().join("specs").join(spec.file_name())
    }

    pub fn save_spec_for(&self, spec: &ServiceSpec) -> Result<()> {
        spec.to_file(self.spec_path_for(spec))
    }

    /// Given a `PackageIdent`, return current spec if it exists.
    pub fn spec_for_ident(&self, ident: &PackageIdent) -> Option<ServiceSpec> {
        let default_spec = ServiceSpec::default_for(ident.clone());
        let spec_file = self.spec_path_for(&default_spec);

        // JC: This mimics the logic from when we had composites.  But
        // should we check for Err ?
        ServiceSpec::from_file(&spec_file).ok()
    }
}

/// Once a formerly-busy service is no longer doing something
/// asynchronously, we mark that we should take a look at the spec
/// files on disk to ensure that we're still "in sync".
///
/// For example, a "restart" is achieved by stopping a service,
/// and then restarting it when we see that its spec file says it
/// should be up.
///
/// This flag behaves essentially the same as making an arbitrary
/// change to a file in the specs directory, in that the latter
/// provides a boolean condition on whether or not we need to
/// reexamine our spec files.
///
/// Wrapping this up into a type consolidates the logic for
/// manipulation of the signaling Boolean. In particular, all the
/// atomic ordering information resides here, meaning that we don't
/// have to scatter it throughout the code, which could lead to logic
/// errors and drift over time.
#[derive(Clone)]
struct ReconciliationFlag(Arc<AtomicBool>);

impl ReconciliationFlag {
    fn new(value: bool) -> Self { ReconciliationFlag(Arc::new(AtomicBool::new(value))) }

    /// Called after a service has finished some asynchronous
    /// operation to signal that we need to take a look at their spec
    /// file again to potentially take action.
    ///
    /// See Manager::wrap_async_service_operation for additional details.
    ///
    /// We used `Ordering::Relaxed` here because there isn't a need to
    /// sequence operations for multiple actors setting the value to
    /// `true`.
    fn set(&self) { self.0.store(true, Ordering::Relaxed); }

    fn is_set(&self) -> bool { self.0.load(Ordering::Relaxed) }

    /// Returns whether or not we need to re-examine spec files in
    /// response to some service having finished an asynchronous
    /// action.
    ///
    /// This *does* change the value of the flag back to `false` if it
    /// was `true`, so we don't have a strict CQRS-style separation of
    /// read/write responsibilities, but this is needed to avoid
    /// potential race conditions between separate check and load
    /// operations.
    ///
    /// This also allows us to use `Ordering::Relaxed`... whether we
    /// see that we need to reconcile before or after some service
    /// signals that it has finished is ultimately unimportant, since
    /// we'll just check again the next time through our supervision
    /// loop.
    ///
    /// While this is all dependent on some of the inner workings of
    /// the `Manager` right now, consolidating this "flag" logic in
    /// one place seemed the prudent choice. In the long-term, we
    /// should be able to dispense with this altogether once we're all
    /// asynchronous.
    fn toggle_if_set(&self) -> bool { self.0.compare_and_swap(true, false, Ordering::Relaxed) }
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

/// All the data that is ultimately served from the Supervisor's HTTP
/// gateway.
#[derive(Debug, Default)]
pub struct GatewayState {
    /// JSON returned by the /census endpoint
    pub census_data: String,
    /// JSON returned by the /butterfly endpoint
    pub butterfly_data: String,
    /// JSON returned by the /services endpoint
    pub services_data: String,
    /// Data returned by /services/<SERVICE_NAME>/<GROUP_NAME>/health
    /// endpoint
    pub health_check_data: HashMap<ServiceGroup, HealthCheck>,
}

pub struct Manager {
    pub state:    Arc<ManagerState>,
    butterfly:    habitat_butterfly::Server,
    census_ring:  CensusRing,
    fs_cfg:       Arc<FsCfg>,
    launcher:     LauncherCli,
    updater:      Arc<Mutex<ServiceUpdater>>,
    peer_watcher: Option<PeerWatcher>,
    spec_watcher: SpecWatcher,
    // This Arc<RwLock<>> business is a potentially temporary
    // change. Right now, in order to asynchronously shut down
    // services, we need to be able to have a safe reference to this
    // from another thread.
    //
    // Future refactorings may suggest other ways to achieve the same
    // result of being able to manipulate the config watcher from
    // other threads (e.g., maybe we subscribe to messages to change
    // the watcher)
    user_config_watcher: Arc<RwLock<UserConfigWatcher>>,
    spec_dir:            SpecDir,
    organization:        Option<String>,
    self_updater:        Option<SelfUpdater>,
    service_states:      HashMap<PackageIdent, Timespec>,
    sys:                 Arc<Sys>,
    http_disable:        bool,

    /// Collects the identifiers of all services that are currently
    /// doing something asynchronously (like shutting down, or running
    /// a lifecycle hook). We want to know which to ignore if changes
    /// in their spec files are detected while they're asynchronously
    /// doing something else. That will prevent us from getting into
    /// weird states if spec files change in the middle of us doing
    /// something else.
    // Currently, this is just going to be things that are shutting
    // down, but as more operations become asynchronous, we'll end up
    // keeping track of services doing other operations as well. At
    // that point, we might need / want to change from a HashSet to
    // something else (maybe a HashMap?) in order to cleanly manage
    // the different operations.
    busy_services: Arc<Mutex<HashSet<PackageIdent>>>,
    services_need_reconciliation: ReconciliationFlag,

    feature_flags: FeatureFlag,
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

    pub fn term(proc_lock_file: &Path) -> Result<()> {
        match read_process_lock(proc_lock_file) {
            #[cfg(unix)]
            Ok(pid) => {
                // TODO (CM): this only ever worked on Linux! It's a no-op
                // on Windows! See
                // https://github.com/habitat-sh/habitat/issues/4945
                process::signal(pid, Signal::TERM).map_err(|_| sup_error!(Error::SignalFailed))?;
                Ok(())
            }
            #[cfg(windows)]
            Ok(_) => Ok(()),
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
                Some(SelfUpdater::new(current, cfg.update_url, cfg.update_channel))
            } else {
                warn!("Supervisor version not fully qualified, unable to start self-updater");
                None
            }
        } else {
            None
        };
        let mut sys = Sys::new(cfg.gossip_permanent,
                               cfg.gossip_listen,
                               cfg.ctl_listen,
                               cfg.http_listen);
        let member = Self::load_member(&mut sys, &fs_cfg)?;
        let services = Arc::new(RwLock::new(HashMap::new()));

        let server = habitat_butterfly::Server::new(sys.gossip_listen(),
                                                    sys.gossip_listen(),
                                                    member,
                                                    Trace::default(),
                                                    cfg.ring_key,
                                                    None,
                                                    Some(&fs_cfg.data_path),
                                                    Box::new(SuitabilityLookup(services.clone())))?;
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

        if cfg.feature_flags.contains(FeatureFlag::EVENT_STREAM) {
            // Putting configuration of the stream behind a feature
            // flag for now. If the flag isn't set, just don't
            // initialize the stream; everything else will turn into a
            // no-op automatically.

            // TODO: Determine what the actual connection parameters
            // should be, and process them at some point before here.
            let es_config =
                cfg.event_stream_config
                   .expect("Config should be present if the EventStream feature is enabled");
            let ec = EventCore::new(&es_config, &sys);
            // unwrap won't fail here; if there were an issue, from_env()
            // would have already propagated an error up the stack.
            event::init_stream(es_config, ec)?;
        }

        Ok(Manager { state: Arc::new(ManagerState { cfg: cfg_static,
                                                    services,
                                                    gateway_state:
                                                        Arc::new(RwLock::new(GatewayState::default())) }),
                     self_updater,
                     updater: Arc::new(Mutex::new(ServiceUpdater::new(server.clone()))),
                     census_ring: CensusRing::new(sys.member_id.clone()),
                     butterfly: server,
                     launcher,
                     peer_watcher,
                     spec_watcher,
                     user_config_watcher: Arc::new(RwLock::new(UserConfigWatcher::new())),
                     spec_dir,
                     fs_cfg: Arc::new(fs_cfg),
                     organization: cfg.organization,
                     service_states: HashMap::new(),
                     sys: Arc::new(sys),
                     http_disable: cfg.http_disable,
                     busy_services: Arc::new(Mutex::new(HashSet::new())),
                     services_need_reconciliation: ReconciliationFlag::new(false),
                     feature_flags: cfg.feature_flags })
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
            Err(_) => {
                match File::create(&fs_cfg.member_id_file) {
                    Ok(mut file) => {
                        file.write(member.id.as_bytes()).map_err(|e| {
                        sup_error!(Error::BadDataFile(fs_cfg.member_id_file.clone(), e))
                    })?;
                    }
                    Err(err) => {
                        return Err(sup_error!(Error::BadDataFile(fs_cfg.member_id_file
                                                                       .clone(),
                                                                 err)));
                    }
                }
            }
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
                        match entry.path().extension().and_then(OsStr::to_str) {
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

        Ok(())
    }

    fn add_service(&mut self, spec: &ServiceSpec) {
        // JW TODO: This clone sucks, but our data structures are a bit messy here. What we really
        // want is the service to hold the spec and, on failure, return an error with the spec
        // back to us. Since we consume and deconstruct the spec in `Service::new()` which
        // `Service::load()` eventually delegates to we just can't have that. We should clean
        // this up in the future.
        let service = match Service::load(self.sys.clone(),
                                          spec.clone(),
                                          self.fs_cfg.clone(),
                                          self.organization.as_ref().map(|org| &**org),
                                          self.state.gateway_state.clone())
        {
            Ok(service) => {
                outputln!("Starting {} ({})", &spec.ident, service.pkg.ident);
                service
            }
            Err(err) => {
                outputln!("Unable to start {}, {}", &spec.ident, err);
                return;
            }
        };

        if let Ok(package) =
            PackageInstall::load(&service.pkg.ident, Some(Path::new(&*FS_ROOT_PATH)))
        {
            if let Err(err) = habitat_common::command::package::install::check_install_hooks(
                &mut habitat_common::ui::UI::with_sinks(),
                &package,
                Path::new(&*FS_ROOT_PATH),
            ) {
                outputln!("Failed to run install hook for {}, {}", &spec.ident, err);
                return;
            }
        }

        if let Err(e) = service.create_svc_path() {
            outputln!("Can't create directory {}: {}",
                      service.pkg.svc_path.display(),
                      e);
            outputln!("If this service is running as non-root, you'll need to create {} and give \
                       the current user write access to it",
                      service.pkg.svc_path.display());
            outputln!("{} failed to start", &spec.ident);
            return;
        }

        self.gossip_latest_service_rumor(&service);
        if service.topology == Topology::Leader {
            self.butterfly.start_election(&service.service_group, 0);
        }

        if let Err(e) = self.user_config_watcher
                            .write()
                            .expect("user-config-watcher lock is poisoned")
                            .add(&service)
        {
            outputln!("Unable to start UserConfigWatcher for {}: {}",
                      service.spec_ident,
                      e);
            return;
        }

        self.updater
            .lock()
            .expect("Updater lock poisoned")
            .add(&service);

        event::service_started(&service);

        self.state
            .services
            .write()
            .expect("Services lock is poisoned!")
            .insert(service.spec_ident.clone(), service);
    }

    // If we ever need to modify this function, it would be an excellent opportunity to
    // simplify the redundant aspects and remove this allow(clippy::cyclomatic_complexity),
    // but changing it in the absence of other necessity seems like too much risk for the
    // expected reward.
    #[allow(clippy::cyclomatic_complexity)]
    pub fn run(mut self, svc: Option<habitat_sup_protocol::ctl::SvcLoad>) -> Result<()> {
        let main_hist = RUN_LOOP_DURATION.with_label_values(&["sup"]);
        let service_hist = RUN_LOOP_DURATION.with_label_values(&["service"]);
        let mut next_cpu_measurement = SteadyTime::now();
        let mut cpu_start = ProcessTime::now();

        let mut runtime =
            RuntimeBuilder::new().name_prefix("tokio-")
                                 .core_threads(TokioThreadCount::configured_value().into())
                                 .build()
                                 .expect("Couldn't build Tokio Runtime!");

        // TODO (CM): consider bundling up these disparate channel
        // ends into a single struct that handles the communication
        // between the CtlAcceptor and this main loop.
        //
        // Well, mgr_sender needs to go into the gateway server, but
        // you get the gist.
        let (mgr_sender, mgr_receiver) = fut_mpsc::unbounded();
        let (ctl_shutdown_tx, ctl_shutdown_rx) = oneshot::channel();
        let (action_sender, action_receiver) = std_mpsc::channel();

        let ctl_handler = CtlAcceptor::new(self.state.clone(),
                                           mgr_receiver,
                                           ctl_shutdown_rx,
                                           action_sender).for_each(move |handler| {
                                                             executor::spawn(handler);
                                                             Ok(())
                                                         });
        runtime.spawn(ctl_handler);

        if let Some(svc_load) = svc {
            commands::service_load(&self.state, &mut CtlRequest::default(), &svc_load)?;
        }

        // This serves to start up any services that need starting
        // (which will be all of them at this point!)
        self.maybe_spawn_service_futures(&mut runtime);

        outputln!("Starting gossip-listener on {}",
                  self.butterfly.gossip_addr());
        self.butterfly.start(Timing::default())?;
        debug!("gossip-listener started");
        self.persist_state();
        let http_listen_addr = self.sys.http_listen();
        let ctl_listen_addr = self.sys.ctl_listen();
        let ctl_secret_key = ctl_gateway::readgen_secret_key(&self.fs_cfg.sup_root)?;
        outputln!("Starting ctl-gateway on {}", &ctl_listen_addr);
        ctl_gateway::server::run(ctl_listen_addr, ctl_secret_key, mgr_sender);
        debug!("ctl-gateway started");

        if self.http_disable {
            info!("http-gateway disabled");
        } else {
            // First let's check and see if we're going to use TLS. If so, we'll generate the
            // appropriate config here, where it's easy to propagate errors, vs in a separate
            // thread, where that process is more cumbersome.

            let tls_server_config = match &self.state.cfg.tls_config {
                Some(c) => {
                    match tls_config(c) {
                        Ok(c) => Some(c),
                        Err(e) => return Err(e),
                    }
                }
                None => None,
            };

            // Here we use a Condvar to wait on the HTTP gateway server to start up and inspect its
            // return value. Specifically, we're looking for errors when it tries to bind to the
            // listening TCP socket, so we can alert the user.
            let pair =
                Arc::new((Mutex::new(http_gateway::ServerStartup::NotStarted), Condvar::new()));

            outputln!("Starting http-gateway on {}", &http_listen_addr);
            http_gateway::Server::run(http_listen_addr,
                                      tls_server_config,
                                      self.state.gateway_state.clone(),
                                      http_gateway::GatewayAuthenticationToken::configured_value(),
                                      self.feature_flags,
                                      pair.clone());

            let &(ref lock, ref cvar) = &*pair;
            let mut started = lock.lock().expect("Control mutex is poisoned");

            // This will block the current thread until the HTTP gateway thread either starts
            // successfully or fails to bind. In practice, the wait here is so short as to not be
            // noticeable.
            loop {
                match *started {
                    http_gateway::ServerStartup::NotStarted => {
                        started = match cvar.wait_timeout(started, StdDuration::from_millis(10000))
                        {
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

        // On Windows initializng the signal handler will create a ctrl+c handler for the
        // process which will disable default windows ctrl+c behavior and allow us to
        // handle via check_for_signal. However, if the supervsor is in a long running
        // non-run hook, the below loop will not get to check_for_signal in a reasonable
        // amount of time and the supervisor will not respond to ctrl+c. On Windows, we
        // let the launcher catch ctrl+c and gracefully shut down services. ctrl+c should
        // simply halt the supervisor
        if !self.feature_flags.contains(FeatureFlag::IGNORE_SIGNALS) {
            signals::init();
        }

        // Enter the main Supervisor loop. When we break out, it'll be
        // because we've been instructed to shutdown. The value we
        // break out with governs exactly how we shut down.

        // TODO (CM): Investigate the appropriateness of capturing any
        // errors or panics generated in this loop and performing some
        // kind of controlled shutdown.
        let shutdown_mode = loop {
            // time will be recorded automatically by HistogramTimer's drop implementation when
            // this var goes out of scope
            #[allow(unused_variables)]
            let main_timer = main_hist.start_timer();

            match get_fd_count() {
                Ok(f) => FILE_DESCRIPTORS.set(f.to_i64()),
                Err(e) => error!("Error retrieving open file descriptor count: {:?}", e),
            }

            track_memory_stats();

            if self.feature_flags.contains(FeatureFlag::TEST_EXIT) {
                if let Ok(exit_file_path) = env::var("HAB_FEAT_TEST_EXIT") {
                    if let Ok(mut exit_code_file) = File::open(&exit_file_path) {
                        let mut buffer = String::new();
                        exit_code_file.read_to_string(&mut buffer)
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
                break ShutdownMode::Normal;
            }
            if self.check_for_departure() {
                break ShutdownMode::Departed;
            }

            // This formulation is gross, but it doesn't seem to compile on Windows otherwise.
            #[allow(clippy::match_bool)]
            #[allow(clippy::single_match)]
            #[cfg(unix)]
            match self.feature_flags.contains(FeatureFlag::IGNORE_SIGNALS) {
                false => {
                    if let Some(SignalEvent::Passthrough(Signal::HUP)) = signals::check_for_signal()
                    {
                        outputln!("Supervisor shutting down for signal");
                        break ShutdownMode::Restarting;
                    }
                }
                _ => {}
            }

            if let Some(package) = self.check_for_updated_supervisor() {
                outputln!("Supervisor shutting down for automatic update to {}",
                          package);
                break ShutdownMode::Restarting;
            }

            // TODO (CM): eventually, make this a future receiver
            for action in action_receiver.try_iter() {
                match action {
                    SupervisorAction::StopService { mut service_spec,
                                                    shutdown_spec, } => {
                        service_spec.desired_state = DesiredState::Down;
                        if let Err(err) = self.state.cfg.save_spec_for(&service_spec) {
                            warn!("Tried to stop '{}', but couldn't update the spec: {:?}",
                                  service_spec.ident, err);
                        }
                        if let Some(future) =
                            self.remove_service_from_state(&service_spec)
                                .map(|service| self.stop_with_spec(service, shutdown_spec))
                        {
                            runtime.spawn(future);
                        } else {
                            warn!("Tried to stop '{}', but couldn't find it in our list of \
                                   running services!",
                                  service_spec.ident);
                        }
                    }
                    SupervisorAction::UnloadService { service_spec,
                                                      shutdown_spec, } => {
                        let file = self.state.cfg.spec_path_for(&service_spec);
                        if let Err(err) = fs::remove_file(&file) {
                            warn!("Tried to unload '{}', but couldn't remove the file '{}': {:?}",
                                  service_spec.ident,
                                  file.display(),
                                  err);
                        };
                        if let Some(future) =
                            self.remove_service_from_state(&service_spec)
                                .map(|service| self.stop_with_spec(service, shutdown_spec))
                        {
                            runtime.spawn(future);
                        } else {
                            warn!("Tried to unload '{}', but couldn't find it in our list of \
                                   running services!",
                                  service_spec.ident);
                        }
                    }
                }
            }

            // Indicates if we need to examine our on-disk specfiles
            // in order to reconcile them with whatever we're
            // currently running.
            //
            // Takes into account filesystem events in the specs
            // directory, as well as whether or not we need to
            // reexamine specs after finishing some asynchronous
            // operation on a service.
            if self.spec_watcher.has_events() || self.services_need_reconciliation.is_set() {
                // This call *must* come first. If some other future
                // happens to complete before we get done spawning our
                // current batch of futures, it could set the flag to
                // true, but we wouldn't have taken another look at
                // its spec file to see if we needed to do anything
                // else. Thus, we could "lose" that signal if we
                // toggle *after* spawning these futures.
                //
                // This could mean, say, the "start" part of a service
                // restart could be greatly delayed (until some file
                // event in the specs directory is registered, or
                // another service finishes shutting down).
                self.services_need_reconciliation.toggle_if_set();
                self.maybe_spawn_service_futures(&mut runtime);
            }

            self.update_peers_from_watch_file()?;
            self.update_running_services_from_user_config_watcher();

            for f in self.stop_services_with_updates() {
                runtime.spawn(f);
            }

            self.restart_elections(self.feature_flags);
            self.census_ring
                .update_from_rumors(&self.state.cfg.cache_key_path,
                                    &self.butterfly.service_store,
                                    &self.butterfly.election_store,
                                    &self.butterfly.update_store,
                                    &self.butterfly.member_list,
                                    &self.butterfly.service_config_store,
                                    &self.butterfly.service_file_store);

            if self.check_for_changed_services() {
                self.persist_state();
            }

            if self.census_ring.changed() {
                self.persist_state();
            }

            for service in self.state
                               .services
                               .write()
                               .expect("Services lock is poisoned!")
                               .values_mut()
            {
                // time will be recorded automatically by HistogramTimer's drop implementation when
                // this var goes out of scope
                #[allow(unused_variables)]
                let service_timer = service_hist.start_timer();
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

            // Measure CPU time every second
            if SteadyTime::now() >= next_cpu_measurement {
                let cpu_duration = cpu_start.elapsed();
                let cpu_nanos =
                    cpu_duration.as_secs()
                                .checked_mul(1_000_000_000)
                                .and_then(|ns| ns.checked_add(cpu_duration.subsec_nanos().into()))
                                .expect("overflow in cpu_duration");
                CPU_TIME.set(cpu_nanos.to_i64());
                next_cpu_measurement = SteadyTime::now() + TimeDuration::seconds(1);
                cpu_start = ProcessTime::now();
            }
        }; // end main loop

        // When we make it down here, we've broken out of the main
        // Supervisor loop, which means it's time to shut down. Based
        // on the value we broke out of the loop with, we may need to
        // shut services down. We do that out here, so we can run the
        // shutdown futures directly on the reactor, and ensure
        // they're all driven to completion before we exit.

        // Stop the ctl gateway; this way we'll stop responding to
        // user commands as we're trying to shut down.
        ctl_shutdown_tx.send(()).ok();

        match shutdown_mode {
            ShutdownMode::Restarting => {}
            ShutdownMode::Normal | ShutdownMode::Departed => {
                outputln!("Gracefully departing from butterfly network.");
                self.butterfly.set_departed();

                let mut svcs = self.state
                                   .services
                                   .write()
                                   .expect("Services lock is poisoned!");

                for (_ident, svc) in svcs.drain() {
                    runtime.spawn(self.stop(svc));
                }
            }
        }

        // Allow all existing futures to run to completion.
        runtime.shutdown_on_idle()
               .wait()
               .expect("Error waiting on Tokio runtime to shutdown");

        release_process_lock(&self.fs_cfg);
        self.butterfly.persist_data();

        match shutdown_mode {
            ShutdownMode::Normal | ShutdownMode::Restarting => Ok(()),
            ShutdownMode::Departed => Err(sup_error!(Error::Departed)),
        }
    }

    fn check_for_updated_supervisor(&mut self) -> Option<PackageInstall> {
        if let Some(ref mut self_updater) = self.self_updater {
            return self_updater.updated();
        }
        None
    }

    /// Return the Services that currently have a newer package in
    /// Builder. These are removed from the internal `services` vec
    /// for further transformation into futures.
    fn take_services_with_updates(&mut self) -> Vec<Service> {
        let mut updater = self.updater.lock().expect("Updater lock poisoned");

        let mut state_services = self.state
                                     .services
                                     .write()
                                     .expect("Services lock is poisoned!");
        let idents_to_restart: Vec<_> = state_services.iter()
                                                      .filter_map(|(current_ident, service)| {
                                                          if let Some(new_ident) =
                    updater.check_for_updated_package(&service, &self.census_ring)
                {
                    outputln!("Updating from {} to {}", current_ident, new_ident);
                    Some(current_ident.clone())
                } else {
                    trace!("No update found for {}", current_ident);
                    None
                }
                                                      })
                                                      .collect();

        let mut services_to_restart = Vec::with_capacity(idents_to_restart.len());
        for current_ident in idents_to_restart {
            // unwrap is safe because we've to the write lock, and we
            // know there's a value present at this key.
            services_to_restart.push(state_services.remove(&current_ident).unwrap());
        }
        services_to_restart
    }

    /// Returns a Vec of futures for shutting down those services that
    /// need to be updated.
    // TODO (CM): In the future, when service start up is
    // future-based, we'll want to have an actual "restart"
    // future, that queues up the start future after the stop
    // future.
    //
    // Until then, we will just stop the services, and rely on the
    // our specfile reconciliation logic to catch the fact that
    // the service needs to be restarted. At that point, this function
    // can be renamed; right now, it says exactly what it's doing.
    fn stop_services_with_updates(&mut self) -> Vec<impl Future<Item = (), Error = ()>> {
        self.take_services_with_updates()
            .into_iter()
            .map(|service| self.stop(service))
            .collect()
    }

    // Creates a rumor for the specified service.
    fn gossip_latest_service_rumor(&self, service: &Service) {
        let incarnation = if let Some(rumor) = self.butterfly
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

    fn check_for_departure(&self) -> bool { self.butterfly.is_departed() }

    fn check_for_changed_services(&mut self) -> bool {
        let mut service_states = HashMap::new();
        let mut active_services = Vec::new();
        for service in self.state
                           .services
                           .write()
                           .expect("Services lock is poisoned!")
                           .values_mut()
        {
            service_states.insert(service.spec_ident.clone(), service.last_state_change());
            active_services.push(service.spec_ident.clone());
        }

        for loaded in self.spec_dir
                          .specs()
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
        let config_rendering = if self.feature_flags.contains(FeatureFlag::REDACT_HTTP) {
            ConfigRendering::Redacted
        } else {
            ConfigRendering::Full
        };

        let services = self.state
                           .services
                           .read()
                           .expect("Services lock is poisoned!");
        let existing_idents: Vec<PackageIdent> =
            services.values().map(|s| s.spec_ident.clone()).collect();

        // Services that are not active but are being watched for changes
        // These would include stopped persistent services or other
        // persistent services that failed to load
        let watched_services: Vec<Service> =
            self.spec_dir
                .specs()
                .iter()
                .filter(|spec| !existing_idents.contains(&spec.ident))
                .flat_map(|spec| {
                    Service::load(self.sys.clone(),
                                  spec.clone(),
                                  self.fs_cfg.clone(),
                                  self.organization.as_ref().map(|org| &**org),
                                  self.state.gateway_state.clone()).into_iter()
                })
                .collect();
        let watched_service_proxies: Vec<ServiceProxy<'_>> =
            watched_services.iter()
                            .map(|s| ServiceProxy::new(s, config_rendering))
                            .collect();
        let mut services_to_render: Vec<ServiceProxy<'_>> =
            services.values()
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

    /// Check if any elections need restarting.
    fn restart_elections(&mut self, feature_flags: FeatureFlag) {
        self.butterfly.restart_elections(feature_flags);
    }

    /// Create a future for stopping a Service. The Service is assumed
    /// to have been removed from the internal list of active services
    /// already (see, e.g., take_services_with_updates and
    /// remove_service_from_state).

    // NOTE: this stop / stop_with_spec division is just until
    // the parameterized shutdown is fully plumbed through everything.
    fn stop_with_spec(&self,
                      service: Service,
                      shutdown_spec: ShutdownSpec)
                      -> impl Future<Item = (), Error = ()> {
        Self::service_stop_future(service,
                                  shutdown_spec,
                                  Arc::clone(&self.user_config_watcher),
                                  Arc::clone(&self.updater),
                                  Arc::clone(&self.busy_services),
                                  self.services_need_reconciliation.clone())
    }

    fn stop(&self, service: Service) -> impl Future<Item = (), Error = ()> {
        Self::service_stop_future(service,
                                  // TODO (CM): when services can
                                  // store their shutdown
                                  // configuration in their spec file,
                                  // we can pull this data from there
                                  ShutdownSpec::default(),
                                  Arc::clone(&self.user_config_watcher),
                                  Arc::clone(&self.updater),
                                  Arc::clone(&self.busy_services),
                                  self.services_need_reconciliation.clone())
    }

    /// Remove the given service from the manager.
    fn service_stop_future(service: Service,
                           shutdown_spec: ShutdownSpec,
                           user_config_watcher: Arc<RwLock<UserConfigWatcher>>,
                           updater: Arc<Mutex<ServiceUpdater>>,
                           busy_services: Arc<Mutex<HashSet<PackageIdent>>>,
                           services_need_reconciliation: ReconciliationFlag)
                           -> impl Future<Item = (), Error = ()> {
        // JW TODO: Update service rumor to remove service from
        // cluster
        // TODO (CM): But only if we're not going down for a restart.
        let ident = service.spec_ident.clone();
        let stop_it = service.stop(shutdown_spec).then(move |_| {
                                                     event::service_stopped(&service);
                                                     user_config_watcher.write()
                                                                        .expect("Watcher lock \
                                                                                 poisoned")
                                                                        .remove(&service);
                                                     updater.lock()
                                                            .expect("Updater lock poisoned")
                                                            .remove(&service);
                                                     Ok(())
                                                 });
        Self::wrap_async_service_operation(ident,
                                           busy_services,
                                           services_need_reconciliation,
                                           stop_it)
    }

    /// Wrap a future that starts, stops, or restarts a service with
    /// logic that marks that service as "busy" for the duration of
    /// the process.
    ///
    /// This allows us to postpone taking additional action on
    /// services until they're done with what they're doing.  (For
    /// example, consider stopping a service that takes 10 seconds to
    /// shut down (including post-stop hook execution), but then
    /// executing `hab svc start SERVICE` 2 seconds into that 10
    /// seconds.)
    ///
    /// As more service operations (e.g., hooks) become asynchronous,
    /// we'll need to wrap those operations in this logic to ensure
    /// consistent operation.
    fn wrap_async_service_operation<F>(ident: PackageIdent,
                                       busy_services: Arc<Mutex<HashSet<PackageIdent>>>,
                                       services_need_reconciliation: ReconciliationFlag,
                                       fut: F)
                                       -> impl Future<Item = (), Error = ()>
        where F: IntoFuture<Item = (), Error = ()>
    {
        // TODO (CM): can't wait for the Pinning API :(
        let busy_services_2 = Arc::clone(&busy_services);
        let ident_2 = ident.clone();

        future::lazy(move || {
            trace!("Flagging '{:?}' as busy, pending an asynchronous operation",
                   ident);
            busy_services.lock()
                         .expect("busy_services lock is poisoned")
                         .insert(ident);
            Ok(())
        }).and_then(|_| fut)
          .and_then(move |_| {
              trace!("Removing 'busy' flag for '{:?}'; asynchronous operation over",
                     ident_2);
              busy_services_2.lock()
                             .expect("busy_services lock is poisoned")
                             .remove(&ident_2);
              services_need_reconciliation.set();
              Ok(())
          })
    }

    /// Determine if our on-disk spec files indicate that we should
    /// perform some action on our services (start, stop, etc.)
    ///
    /// If so, futures for those actions are spawned on the runtime.
    ///
    /// NOTE: Service start is currently synchronous, so any start
    /// operations will be performed directly as a consequence of
    /// calling this method.
    fn maybe_spawn_service_futures(&mut self, runtime: &mut Runtime) {
        let ops = self.compute_service_operations();
        for f in self.operations_into_futures(ops) {
            runtime.spawn(f);
        }
    }

    fn remove_service_from_state(&mut self, spec: &ServiceSpec) -> Option<Service> {
        self.state
            .services
            .write()
            .expect("Services lock is poisoned")
            .remove(&spec.ident)
    }

    /// Start, stop, or restart services to bring what's running in
    /// line with what our spec files say.
    ///
    /// In the future, this will simply convert `ServiceOperation`s
    /// into futures that can be later spawned. Until starting of
    /// services is made asynchronous, however, it performs a mix of
    /// operations; starts are performed synchronously, while
    /// shutdowns and restarts are turned into futures.
    fn operations_into_futures<O>(&mut self, ops: O) -> Vec<impl Future<Item = (), Error = ()>>
        where O: IntoIterator<Item = ServiceOperation>
    {
        ops.into_iter()
           .filter_map(|op| {
               match op {
                   ServiceOperation::Stop(spec)
                   | ServiceOperation::Restart { to_stop: spec, .. } => {
                       // Yes, Stop and Restart both turn into
                       // "stop"... Once we've finished stopping, we'll
                       // end up re-examining the spec file on disk; if
                       // we should be running, we'll start up again.
                       //
                       // This may change in the future, once service
                       // start can be performed asynchronously in a
                       // future; then we could just chain that future
                       // onto the end of the stop one for a *real*
                       // restart future.
                       let f = self.remove_service_from_state(&spec)
                                   .map(|service| self.stop(service));
                       if f.is_none() {
                           // We really don't expect this to happen....
                           outputln!("Tried to remove service for {} but could not find it \
                                      running, skipping",
                                     &spec.ident);
                       }
                       f
                   }
                   ServiceOperation::Start(spec) => {
                       self.add_service(&spec);
                       None // No future to return (currently synchronous!)
                   }
               }
           })
           .collect()
    }

    /// Determine what services we need to start, stop, or restart in
    /// order to be running what our on-disk spec files tell us we
    /// should be running.
    ///
    /// See `specs_to_operations` for the real logic.
    fn compute_service_operations(&mut self) -> Vec<ServiceOperation> {
        // First, figure out what's currently running.
        let services = self.state
                           .services
                           .read()
                           .expect("Services lock is poisoned");
        let currently_running_specs = services.values().map(Service::to_spec);

        // Now, figure out what we should compare against, ignoring
        // any services that are currently doing something
        // asynchronously.
        let busy_services = self.busy_services
                                .lock()
                                .expect("busy_services lock is poisoned");
        let on_disk_specs = self.spec_dir
                                .specs()
                                .into_iter()
                                .filter(|s| !busy_services.contains(&s.ident));

        Self::specs_to_operations(currently_running_specs, on_disk_specs)
    }

    /// Pure utility function to generate a list of operations to
    /// perform to bring what's currently running with what _should_ be
    /// running, based on the current on-disk spec files.
    fn specs_to_operations<C, D>(currently_running_specs: C,
                                 on_disk_specs: D)
                                 -> Vec<ServiceOperation>
        where C: IntoIterator<Item = ServiceSpec>,
              D: IntoIterator<Item = ServiceSpec>
    {
        let mut svc_states = HashMap::new();

        #[derive(Default, Debug)]
        struct ServiceState {
            running: Option<ServiceSpec>,
            disk:    Option<(DesiredState, ServiceSpec)>,
        }

        for rs in currently_running_specs {
            svc_states.insert(rs.ident.clone(),
                              ServiceState { running: Some(rs),
                                             disk:    None, });
        }

        for ds in on_disk_specs {
            let ident = ds.ident.clone();
            svc_states.entry(ident)
                      .or_insert_with(ServiceState::default)
                      .disk = Some((ds.desired_state, ds));
        }

        svc_states.into_iter()
                  .filter_map(|(ident, ss)| {
                      match ss {
                          ServiceState { disk: Some((DesiredState::Up, disk_spec)),
                                         running: None, } => {
                              debug!("Reconciliation: '{}' queued for start", ident);
                              Some(ServiceOperation::Start(disk_spec))
                          }

                          ServiceState { disk: Some((DesiredState::Up, disk_spec)),
                                         running: Some(running_spec), } => {
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
                                  Some(ServiceOperation::Restart { to_stop:  running_spec,
                                                                   to_start: disk_spec, })
                              }
                          }
                          ServiceState { disk: Some((DesiredState::Down, _)),
                                         running: Some(running_spec), } => {
                              debug!("Reconciliation: '{}' queued for stop", ident);
                              Some(ServiceOperation::Stop(running_spec))
                          }

                          ServiceState { disk: Some((DesiredState::Down, _)),
                                         running: None, } => {
                              debug!("Reconciliation: '{}' should be down, and is", ident);
                              None
                          }

                          ServiceState { disk: None,
                                         running: Some(running_spec), } => {
                              debug!("Reconciliation: '{}' queued for shutdown", ident);
                              Some(ServiceOperation::Stop(running_spec))
                          }

                          ServiceState { disk: None,
                                         running: None, } => unreachable!(),
                      }
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
        let mut services = self.state
                               .services
                               .write()
                               .expect("Services lock is poisoned");

        for service in services.values_mut() {
            if self.user_config_watcher
                   .read()
                   .expect("user_config_watcher lock is poisoned")
                   .have_events_for(service)
            {
                outputln!("user.toml changes detected for {}", &service.spec_ident);
                service.user_config_updated = true;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////

fn tls_config(config: &TLSConfig) -> Result<rustls::ServerConfig> {
    let client_auth = match &config.ca_cert_path {
        Some(path) => {
            let mut root_store = RootCertStore::empty();
            let ca_file = &mut BufReader::new(File::open(path)?);
            root_store.add_pem_file(ca_file)
                      .and_then(|(added, _)| {
                          if added < 1 {
                              Err(())
                          } else {
                              Ok(AllowAnyAuthenticatedClient::new(root_store))
                          }
                      })
                      .map_err(|_| sup_error!(Error::InvalidCertFile(path.clone())))?
        }
        None => NoClientAuth::new(),
    };

    let mut server_config = ServerConfig::new(client_auth);
    let key_file = &mut BufReader::new(File::open(&config.key_path)?);
    let cert_file = &mut BufReader::new(File::open(&config.cert_path)?);

    // Note that we must explicitly map these errors because rustls returns () as the error from
    // both pemfile::certs() as well as pemfile::rsa_private_keys() and we want to return
    // different errors for each.
    let cert_chain =
        pemfile::certs(cert_file).and_then(|c| if c.is_empty() { Err(()) } else { Ok(c) })
                                 .map_err(|_| {
                                     sup_error!(Error::InvalidCertFile(config.cert_path.clone()))
                                 })?;

    let key =
        pemfile::rsa_private_keys(key_file).and_then(|mut k| k.pop().ok_or(()))
                                           .map_err(|_| {
                                               sup_error!(Error::InvalidKeyFile(config.key_path
                                                                                      .clone()))
                                           })?;

    server_config.set_single_cert(cert_chain, key)?;
    server_config.ignore_client_order = true;
    Ok(server_config)
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
        let raw = s.parse::<usize>()
                   .map_err(|_| Error::InvalidTokioThreadCount)?;
        if raw > 0 {
            Ok(TokioThreadCount(raw))
        } else {
            Err(Error::InvalidTokioThreadCount)
        }
    }
}

impl env::Config for TokioThreadCount {
    const ENVVAR: &'static str = "HAB_TOKIO_THREAD_COUNT";
}

impl Into<usize> for TokioThreadCount {
    fn into(self) -> usize { self.0 }
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
            .and_then(Service::suitability)
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
                    release_process_lock(&fs_cfg);
                    write_process_lock(&fs_cfg.proc_lock_file)
                }
                Err(SupError { err: Error::ProcessLockCorrupt,
                               .. }) => {
                    release_process_lock(&fs_cfg);
                    write_process_lock(&fs_cfg.proc_lock_file)
                }
                Err(err) => Err(err),
            }
        }
    }
}

fn read_process_lock<T>(lock_path: T) -> Result<Pid>
    where T: AsRef<Path>
{
    match File::open(lock_path.as_ref()) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match reader.lines().next() {
                Some(Ok(line)) => {
                    match line.parse::<Pid>() {
                        Ok(pid) => Ok(pid),
                        Err(_) => Err(sup_error!(Error::ProcessLockCorrupt)),
                    }
                }
                _ => Err(sup_error!(Error::ProcessLockCorrupt)),
            }
        }
        Err(err) => {
            Err(sup_error!(Error::ProcessLockIO(lock_path.as_ref()
                                                         .to_path_buf(),
                                                err)))
        }
    }
}

fn release_process_lock(fs_cfg: &FsCfg) {
    if let Err(err) = fs::remove_file(&fs_cfg.proc_lock_file) {
        debug!("Couldn't cleanup Supervisor process lock, {}", err);
    }
}

fn write_process_lock<T>(lock_path: T) -> Result<()>
    where T: AsRef<Path>
{
    match OpenOptions::new().write(true)
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
                Err(err) => {
                    Err(sup_error!(Error::ProcessLockIO(lock_path.as_ref()
                                                                 .to_path_buf(),
                                                        err)))
                }
            }
        }
        Err(err) => {
            Err(sup_error!(Error::ProcessLockIO(lock_path.as_ref()
                                                         .to_path_buf(),
                                                err)))
        }
    }
}

#[cfg(windows)]
fn get_fd_count() -> std::io::Result<usize> {
    let mut count: u32 = 0;
    let count_ptr = &mut count as PDWORD;

    unsafe {
        let handle = processthreadsapi::GetCurrentProcess();
        match processthreadsapi::GetProcessHandleCount(handle, count_ptr) {
            // these are ints here because GetProcessHandleCount returns a BOOL which is actually
            // an i32
            1 => Ok(count as usize),
            _ => {
                Err(std::io::Error::new(std::io::ErrorKind::Other,
                                        "error getting file descriptor count"))
            }
        }
    }
}

#[cfg(unix)]
fn get_fd_count() -> std::io::Result<usize> {
    palaver::file::FdIter::new().map(palaver::file::FdIter::count)
}

#[cfg(unix)]
fn track_memory_stats() {
    // We'd like to track some memory stats, but these stats are cached and only refreshed
    // when the epoch is advanced. We manually advance it here to ensure our stats are
    // fresh.
    jemalloc_ctl::epoch().unwrap();
    MEMORY_STATS.with_label_values(&["active"])
                .set(jemalloc_ctl::stats::active().unwrap().to_i64());
    MEMORY_STATS.with_label_values(&["allocated"])
                .set(jemalloc_ctl::stats::allocated().unwrap().to_i64());
    MEMORY_STATS.with_label_values(&["mapped"])
                .set(jemalloc_ctl::stats::mapped().unwrap().to_i64());
    MEMORY_STATS.with_label_values(&["metadata"])
                .set(jemalloc_ctl::stats::metadata().unwrap().to_i64());
    MEMORY_STATS.with_label_values(&["resident"])
                .set(jemalloc_ctl::stats::resident().unwrap().to_i64());
    MEMORY_STATS.with_label_values(&["retained"])
                .set(jemalloc_ctl::stats::retained().unwrap().to_i64());
}

// This is a no-op on purpose because windows doesn't support jemalloc
#[cfg(windows)]
fn track_memory_stats() {}

#[cfg(test)]
mod test {
    use super::*;
    use habitat_common::cli::FS_ROOT;
    use habitat_core::fs::cache_key_path;
    use habitat_sup_protocol::STATE_PATH_PREFIX;
    use std::path::PathBuf;

    mod reconciliation_flag {
        use super::*;

        #[test]
        fn toggle_if_set_only_returns_true_if_previously_set() {
            let f = ReconciliationFlag::new(false);
            assert!(!f.is_set());
            assert!(!f.toggle_if_set(), "Should not be set!");
            f.set();
            assert!(f.toggle_if_set(), "Should have been toggled, but wasn't!");
            assert!(!f.toggle_if_set(),
                    "Should no longer be toggled, after having been toggled previously!");
        }
    }

    // Implementing Default in production code encourages passing the entirety of this struct
    // around when it would be better to be more targeted. However, it is very handy for test
    // code, so only implement it under test configuration.
    impl Default for ManagerConfig {
        fn default() -> Self {
            ManagerConfig { auto_update:         false,
                            custom_state_path:   None,
                            cache_key_path:      cache_key_path(Some(&*FS_ROOT)),
                            update_url:          "".to_string(),
                            update_channel:      ChannelIdent::default(),
                            gossip_listen:       GossipListenAddr::default(),
                            ctl_listen:          ListenCtlAddr::default(),
                            http_listen:         http_gateway::ListenAddr::default(),
                            http_disable:        false,
                            gossip_peers:        vec![],
                            gossip_permanent:    false,
                            ring_key:            None,
                            organization:        None,
                            watch_peer_file:     None,
                            tls_config:          None,
                            feature_flags:       FeatureFlag::empty(),
                            event_stream_config: None, }
        }
    }

    #[test]
    fn manager_state_path_default() {
        let cfg = ManagerConfig::default();
        let path = cfg.sup_root();

        assert_eq!(PathBuf::from(format!("{}/default", STATE_PATH_PREFIX.to_string_lossy())),
                   path);
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
        use habitat_common::locked_env_var;

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
        use habitat_sup_protocol::types::UpdateStrategy;

        /// Helper function for generating a basic spec from an
        /// identifier string
        fn new_spec(ident: &str) -> ServiceSpec {
            ServiceSpec::default_for(PackageIdent::from_str(ident).expect("couldn't parse ident \
                                                                           str"))
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
                ServiceOperation::Restart { to_stop: ref old,
                                            to_start: ref new, } => {
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

            let running = vec![svc_1_running.clone(),
                               svc_2_running.clone(),
                               svc_3_running.clone(),
                               svc_6_running.clone(),];

            let on_disk = vec![svc_1_on_disk.clone(),
                               svc_2_on_disk.clone(),
                               svc_3_on_disk.clone(),
                               svc_4_on_disk.clone(),
                               svc_5_on_disk.clone(),];

            let operations = Manager::specs_to_operations(running, on_disk);

            let expected_operations =
                vec![ServiceOperation::Stop(svc_2_running.clone()),
                     ServiceOperation::Restart { to_stop:  svc_3_running.clone(),
                                                 to_start: svc_3_on_disk.clone(), },
                     ServiceOperation::Start(svc_5_on_disk.clone()),
                     ServiceOperation::Stop(svc_6_running.clone()),];

            // Ideally, we'd just sort `operations` and
            // `expected_operations`, but we can't, since that would
            // mean we'd need a total ordering on `PackageIdent`,
            // which we can't do, since identifiers of different
            // packages (say, `core/foo` and `core/bar`) are not
            // comparable.
            //
            // Instead, we'll just do the verification one at a time.
            assert_eq!(operations.len(),
                       expected_operations.len(),
                       "Didn't generate the expected number of operations");
            for op in expected_operations {
                assert!(operations.contains(&op),
                        "Should have expected operation: {:?}",
                        op);
            }
        }
    }
}
