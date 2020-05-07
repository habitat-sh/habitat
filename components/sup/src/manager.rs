pub(crate) mod action;
pub mod service;
#[macro_use]
mod debug;
pub mod commands;
mod file_watcher;
mod peer_watcher;
mod self_updater;
mod service_updater;
mod spec_dir;
mod spec_watcher;
pub(crate) mod sys;
mod user_config_watcher;

use self::{action::{ShutdownInput,
                    SupervisorAction},
           peer_watcher::PeerWatcher,
           self_updater::{SelfUpdater,
                          SUP_PKG_IDENT},
           service::{ConfigRendering,
                     DesiredState,
                     HealthCheckResult,
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
            ctl_gateway::{self,
                          acceptor::CtlAcceptor,
                          CtlRequest},
            error::{Error,
                    Result},
            event::{self,
                    EventStreamConfig},
            http_gateway,
            util::pkg,
            VERSION};
use cpu_time::ProcessTime;
use futures::{channel::{mpsc as fut_mpsc,
                        oneshot},
              future,
              prelude::*,
              stream::FuturesUnordered};
use habitat_butterfly::{member::Member,
                        server::{timing::Timing,
                                 ServerProxy,
                                 Suitability}};
use habitat_common::{liveliness_checker,
                     outputln,
                     types::{GossipListenAddr,
                             HttpListenAddr,
                             ListenCtlAddr},
                     FeatureFlag};
#[cfg(unix)]
use habitat_core::os::{process::{ShutdownSignal,
                                 Signal},
                       signals};
use habitat_core::{crypto::SymKey,
                   env,
                   env::Config,
                   fs::FS_ROOT_PATH,
                   os::process::{self,
                                 Pid,
                                 ShutdownTimeout},
                   package::{Identifiable,
                             PackageIdent,
                             PackageInstall},
                   service::ServiceGroup,
                   util::ToI64,
                   ChannelIdent};
use habitat_launcher_client::{LauncherCli,
                              LAUNCHER_LOCK_CLEAN_ENV,
                              LAUNCHER_PID_ENV};
use habitat_sup_protocol::{self};
use parking_lot::{Mutex,
                  RwLock};
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
          iter::{FromIterator,
                 IntoIterator},
          net::{IpAddr,
                SocketAddr},
          path::{Path,
                 PathBuf},
          str::FromStr,
          sync::{atomic::{AtomicBool,
                          Ordering},
                 mpsc as std_mpsc,
                 Arc,
                 Condvar,
                 Mutex as StdMutex},
          thread,
          time::{Duration,
                 Instant,
                 SystemTime}};
use tokio;
#[cfg(windows)]
use winapi::{shared::minwindef::PDWORD,
             um::processthreadsapi};

const MEMBER_ID_FILE: &str = "MEMBER_ID";
pub const PROC_LOCK_FILE: &str = "LOCK";

static LOGKEY: &str = "MR";

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


    // The `<origin>/<name>` version of the Supervisor's package ident
    static ref THIS_SUPERVISOR_FUZZY_IDENT: PackageIdent = SUP_PKG_IDENT.parse().unwrap();

    /// Depending on the value of `VERSION` this ident may or may not be fully qualified. `VERSION`
    /// produces a fully qualified ident when built with Habitat. If it is built directly from
    /// `cargo build` no release information will be set.
    static ref THIS_SUPERVISOR_IDENT: PackageIdent =
        PackageIdent::from_str(&format!("{}/{}", SUP_PKG_IDENT, VERSION)).unwrap();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Determines whether the new pidfile-less behavior is enabled, or
/// the old behavior is used.
pub enum ServicePidSource {
    /// The "old" behavior; find out a Service's PID by reading a pidfile.
    Files,
    /// The "new" behavior; query the Launcher directly to discover a
    /// Service's PID.
    Launcher,
}

impl ServicePidSource {
    /// This check is to determine if the user is working with a
    /// Launcher that can provide service PIDs. If not, we will
    /// continue to use the old pidfile logic.
    ///
    /// You should call this function once early in the Supservisor's
    /// lifecycle and cache the results. We only want to incur the
    /// timeout hit when we check to see if the launcher can answer
    /// our query once. Otherwise, if we were using an older launcher,
    /// we would incur that hit each time we start a new service.
    fn determine_source(launcher: &LauncherCli) -> Self {
        if launcher.pid_of("fake_service.just_to_see_if_the_launcher_can_handle_this_message")
                   .is_err()
        {
            warn!("You do not appear to be running a Launcher that can provide service PIDs to \
                   the Supervisor. Using pidfiles for services instead.");
            ServicePidSource::Files
        } else {
            ServicePidSource::Launcher
        }
    }
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

#[derive(Clone, Debug, Default)]
pub struct ShutdownConfig {
    #[cfg(not(windows))]
    pub signal:  ShutdownSignal,
    pub timeout: ShutdownTimeout,
}

impl ShutdownConfig {
    fn new(shutdown_input: Option<&ShutdownInput>, service: &Service) -> Self {
        let timeout = shutdown_input.and_then(|si| si.timeout).unwrap_or_else(|| {
                                                                  service.shutdown_timeout
                                                               .unwrap_or(service.pkg
                                                                                 .shutdown_timeout)
                                                              });
        Self { timeout,
               #[cfg(not(windows))]
               signal: service.pkg.shutdown_signal }
    }
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

#[derive(Clone, Debug, PartialEq)]
pub struct ManagerConfig {
    pub auto_update:          bool,
    pub custom_state_path:    Option<PathBuf>,
    pub cache_key_path:       PathBuf,
    pub update_url:           String,
    pub update_channel:       ChannelIdent,
    pub gossip_listen:        GossipListenAddr,
    pub ctl_listen:           ListenCtlAddr,
    pub http_listen:          HttpListenAddr,
    pub http_disable:         bool,
    pub gossip_peers:         Vec<SocketAddr>,
    pub gossip_permanent:     bool,
    pub ring_key:             Option<SymKey>,
    pub organization:         Option<String>,
    pub watch_peer_file:      Option<String>,
    pub tls_config:           Option<TLSConfig>,
    pub feature_flags:        FeatureFlag,
    pub event_stream_config:  Option<EventStreamConfig>,
    /// If this field is `Some`, keep the indicated number of latest packages and uninstall all
    /// others during service start. If this field is `None`, automatic package cleanup is
    /// disabled.
    pub keep_latest_packages: Option<usize>,
    pub sys_ip:               IpAddr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TLSConfig {
    pub cert_path:    PathBuf,
    pub key_path:     PathBuf,
    pub ca_cert_path: Option<PathBuf>,
}

impl ManagerConfig {
    fn sup_root(&self) -> PathBuf {
        habitat_sup_protocol::sup_root(self.custom_state_path.as_ref())
    }

    fn spec_path_for(&self, ident: &PackageIdent) -> PathBuf {
        self.sup_root()
            .join("specs")
            .join(ServiceSpec::ident_file(ident))
    }

    pub fn save_spec_for(&self, spec: &ServiceSpec) -> Result<()> {
        spec.to_file(self.spec_path_for(&spec.ident))
    }

    /// Given a `PackageIdent`, return current spec if it exists.
    pub fn spec_for_ident(&self, ident: &PackageIdent) -> Option<ServiceSpec> {
        let spec_file = self.spec_path_for(ident);

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
    cfg:           ManagerConfig,
    services:      Arc<sync::ManagerServices>,
    gateway_state: Arc<sync::GatewayState>,
}

pub(crate) mod sync {
    use super::*;
    use habitat_common::sync::{Lock,
                               ReadGuard,
                               WriteGuard};

    pub struct GatewayStateReadGuard<'a>(ReadGuard<'a, GatewayStateInner>);

    impl<'a> GatewayStateReadGuard<'a> {
        fn new(lock: &'a Lock<GatewayStateInner>) -> Self { Self(lock.read()) }

        pub fn butterfly_data(&self) -> &str { &self.0.butterfly_data }

        pub fn census_data(&self) -> &str { &self.0.census_data }

        pub fn services_data(&self) -> &str { &self.0.services_data }

        pub fn health_of(&self, service_group: &ServiceGroup) -> Option<HealthCheckResult> {
            self.0.health_check_data.get(service_group).copied()
        }
    }

    pub struct GatewayStateWriteGuard<'a>(WriteGuard<'a, GatewayStateInner>);

    impl<'a> GatewayStateWriteGuard<'a> {
        fn new(lock: &'a Lock<GatewayStateInner>) -> Self { Self(lock.write()) }

        pub fn set_census_data(&mut self, new_data: String) { self.0.census_data = new_data }

        pub fn set_butterfly_data(&mut self, new_data: String) { self.0.butterfly_data = new_data }

        pub fn set_services_data(&mut self, new_data: String) { self.0.services_data = new_data }

        pub fn remove(&mut self, service_group: &ServiceGroup) {
            self.0.health_check_data.remove(service_group);
        }

        pub fn set_health_of(&mut self, service_group: ServiceGroup, value: HealthCheckResult) {
            self.0.health_check_data.insert(service_group, value);
        }
    }

    /// All the data that is ultimately served from the Supervisor's HTTP
    /// gateway.
    #[derive(Debug, Default)]
    pub struct GatewayState {
        inner: Lock<GatewayStateInner>,
    }

    impl GatewayState {
        #[must_use]
        pub fn lock_gsr(&self) -> GatewayStateReadGuard { GatewayStateReadGuard::new(&self.inner) }

        #[must_use]
        pub fn lock_gsw(&self) -> GatewayStateWriteGuard {
            GatewayStateWriteGuard::new(&self.inner)
        }
    }

    #[derive(Debug, Default)]
    struct GatewayStateInner {
        /// JSON returned by the /census endpoint
        census_data:       String,
        /// JSON returned by the /butterfly endpoint
        butterfly_data:    String,
        /// JSON returned by the /services endpoint
        services_data:     String,
        /// Data returned by /services/<SERVICE_NAME>/<GROUP_NAME>/health
        /// endpoint
        health_check_data: HashMap<ServiceGroup, HealthCheckResult>,
    }

    type ManagerServicesInner = HashMap<PackageIdent, Service>;

    pub struct ManagerServicesReadGuard<'a>(ReadGuard<'a, ManagerServicesInner>);

    impl<'a> ManagerServicesReadGuard<'a> {
        fn new(lock: &'a Lock<ManagerServicesInner>) -> Self { Self(lock.read()) }

        pub fn services(&self) -> impl Iterator<Item = &Service> { self.0.values() }
    }

    pub struct ManagerServicesWriteGuard<'a>(WriteGuard<'a, ManagerServicesInner>);

    impl<'a> ManagerServicesWriteGuard<'a> {
        fn new(lock: &'a Lock<ManagerServicesInner>) -> Self { Self(lock.write()) }

        pub fn iter(&self) -> impl Iterator<Item = (&PackageIdent, &Service)> { self.0.iter() }

        pub fn insert(&mut self, key: PackageIdent, value: Service) { self.0.insert(key, value); }

        pub fn remove(&mut self, key: &PackageIdent) -> Option<Service> { self.0.remove(key) }

        pub fn services(&mut self) -> impl Iterator<Item = &mut Service> { self.0.values_mut() }

        pub fn drain_services(&mut self) -> DrainServices<'_> {
            DrainServices { base: self.0.drain(), }
        }
    }

    pub struct DrainServices<'a> {
        base: std::collections::hash_map::Drain<'a, PackageIdent, Service>,
    }

    impl<'a> Iterator for DrainServices<'a> {
        type Item = Service;

        fn next(&mut self) -> Option<Service> { self.base.next().map(|(_ident, service)| service) }
    }

    #[derive(Debug, Default)]
    pub struct ManagerServices {
        inner: Lock<ManagerServicesInner>,
    }

    impl ManagerServices {
        #[must_use]
        pub fn lock_msr(&self) -> ManagerServicesReadGuard {
            ManagerServicesReadGuard::new(&self.inner)
        }

        #[must_use]
        pub fn lock_msw(&self) -> ManagerServicesWriteGuard {
            ManagerServicesWriteGuard::new(&self.inner)
        }
    }

    impl Suitability for ManagerServices {
        /// # Locking (see locking.md)
        /// * `ManagerServices::inner` (read)
        fn suitability_for_msr(&self, service_group: &str) -> u64 {
            self.lock_msr()
                .services()
                .find(|svc| svc.service_group.as_ref() == service_group)
                .and_then(Service::suitability)
                .unwrap_or_else(u64::min_value)
        }
    }
}

pub struct Manager {
    pub state:           Arc<ManagerState>,
    butterfly:           habitat_butterfly::Server,
    census_ring:         Arc<RwLock<CensusRing>>,
    fs_cfg:              Arc<FsCfg>,
    launcher:            LauncherCli,
    service_updater:     Arc<Mutex<ServiceUpdater>>,
    peer_watcher:        Option<PeerWatcher>,
    spec_watcher:        SpecWatcher,
    // This Arc<RwLock<>> business is a potentially temporary
    // change. Right now, in order to asynchronously shut down
    // services, we need to be able to have a safe reference to this
    // from another thread.
    //
    // Future refactorings may suggest other ways to achieve the same
    // result of being able to manipulate the config watcher from
    // other threads (e.g., maybe we subscribe to messages to change
    // the watcher)
    user_config_watcher: UserConfigWatcher,
    spec_dir:            SpecDir,
    organization:        Option<String>,
    self_updater:        Option<SelfUpdater>,
    sys:                 Arc<Sys>,
    http_disable:        bool,
    /// Though it is a `HashMap`, `service_states` not really used as
    /// a `HashMap`. The values are there to act as a kind of
    /// "snapshot marker"... if any of those time markers change
    /// between service checks, that means that something has happened
    /// to one of the services (it was up, but now it's down; it was
    /// up, then down, then up; etc).
    ///
    /// Feel free to refactor to something different!
    service_states:      HashMap<PackageIdent, SystemTime>,

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
    busy_services:                Arc<Mutex<HashSet<PackageIdent>>>,
    services_need_reconciliation: ReconciliationFlag,

    feature_flags: FeatureFlag,
    pid_source:    ServicePidSource,
}

impl Manager {
    /// Load a Manager with the given configuration.
    ///
    /// The returned Manager will be pre-populated with any cached data from disk from a previous
    /// run if available.
    ///
    /// # Locking (see locking.md)
    /// * `MemberList::initial_members` (write)
    pub async fn load_imlw(cfg: ManagerConfig, launcher: LauncherCli) -> Result<Manager> {
        let state_path = cfg.sup_root();
        let fs_cfg = FsCfg::new(state_path);
        Self::create_state_path_dirs(&fs_cfg)?;
        Self::clean_dirty_state(&fs_cfg)?;
        if env::var(LAUNCHER_LOCK_CLEAN_ENV).is_ok() {
            release_process_lock(&fs_cfg);
        }
        obtain_process_lock(&fs_cfg)?;

        Self::new_imlw(cfg, fs_cfg, launcher).await
    }

    pub fn term(proc_lock_file: &Path) -> Result<()> {
        match read_process_lock(proc_lock_file) {
            Ok(pid) => {
                #[cfg(unix)]
                process::signal(pid, Signal::TERM).map_err(|_| Error::SignalFailed)?;
                #[cfg(windows)]
                process::terminate(pid)?;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// # Locking (see locking.md)
    /// * `MemberList::initial_members` (write)
    async fn new_imlw(cfg: ManagerConfig, fs_cfg: FsCfg, launcher: LauncherCli) -> Result<Manager> {
        debug!("new(cfg: {:?}, fs_cfg: {:?}", cfg, fs_cfg);
        outputln!("{} ({})", SUP_PKG_IDENT, *THIS_SUPERVISOR_IDENT);
        let cfg_static = cfg.clone();
        let self_updater = if cfg.auto_update {
            if THIS_SUPERVISOR_IDENT.fully_qualified() {
                Some(SelfUpdater::new(&*THIS_SUPERVISOR_IDENT, cfg.update_url, cfg.update_channel))
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
                               cfg.http_listen,
                               cfg.sys_ip);
        let member = Self::load_member(&mut sys, &fs_cfg)?;
        let services = Arc::default();
        let suitability_lookup = Arc::clone(&services) as Arc<dyn Suitability>;

        let server = habitat_butterfly::Server::new(sys.gossip_listen(),
                                                    sys.gossip_listen(),
                                                    member,
                                                    cfg.ring_key,
                                                    None,
                                                    Some(&fs_cfg.data_path),
                                                    suitability_lookup)?;
        outputln!("Supervisor Member-ID {}", sys.member_id);
        for peer_addr in &cfg.gossip_peers {
            let mut peer = Member::default();
            peer.address = format!("{}", peer_addr.ip());
            peer.swim_port = peer_addr.port();
            peer.gossip_port = peer_addr.port();
            server.member_list.add_initial_member_imlw(peer);
        }

        let peer_watcher = if let Some(path) = cfg.watch_peer_file {
            Some(PeerWatcher::run(path)?)
        } else {
            None
        };

        let spec_dir = SpecDir::new(&fs_cfg.specs_path)?;
        spec_dir.migrate_specs();

        let spec_watcher = SpecWatcher::run(&spec_dir)?;

        if let Some(config) = cfg.event_stream_config {
            // Collect the FQDN of the running machine
            let fqdn = habitat_core::os::net::fqdn().unwrap_or_else(|| sys.hostname.clone());
            outputln!("Event FQDN {}", fqdn);

            event::init(&sys, fqdn, config).await?;
        }

        let pid_source = ServicePidSource::determine_source(&launcher);

        let census_ring = Arc::new(RwLock::new(CensusRing::new(sys.member_id.clone())));
        Ok(Manager { state: Arc::new(ManagerState { cfg: cfg_static,
                                                    services,
                                                    gateway_state: Arc::default() }),
                     self_updater,
                     service_updater:
                         Arc::new(Mutex::new(ServiceUpdater::new(server.clone(),
                                                                 Arc::clone(&census_ring)))),
                     census_ring,
                     butterfly: server,
                     launcher,
                     peer_watcher,
                     spec_watcher,
                     user_config_watcher: UserConfigWatcher::new(),
                     spec_dir,
                     fs_cfg: Arc::new(fs_cfg),
                     organization: cfg.organization,
                     service_states: HashMap::new(),
                     sys: Arc::new(sys),
                     http_disable: cfg.http_disable,
                     busy_services: Arc::default(),
                     services_need_reconciliation: ReconciliationFlag::new(false),
                     feature_flags: cfg.feature_flags,
                     pid_source })
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
                                                        Error::BadDataFile(fs_cfg.member_id_file
                                                                                 .clone(),
                                                                           e)
                                                    })?;
                member.id = member_id;
            }
            Err(_) => {
                match File::create(&fs_cfg.member_id_file) {
                    Ok(mut file) => {
                        file.write(member.id.as_bytes())
                            .map_err(|e| Error::BadDataFile(fs_cfg.member_id_file.clone(), e))?;
                    }
                    Err(err) => {
                        return Err(Error::BadDataFile(fs_cfg.member_id_file.clone(), err));
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
                                    Error::BadDataPath(data_path.clone(), err)
                                })?;
                            }
                            _ => continue,
                        }
                    }
                }
                Ok(())
            }
            Err(err) => Err(Error::BadDataPath(data_path.clone(), err)),
        }
    }

    fn create_state_path_dirs(fs_cfg: &FsCfg) -> Result<()> {
        let data_path = &fs_cfg.data_path;
        debug!("Creating data directory: {}", data_path.display());
        if let Some(err) = fs::create_dir_all(&data_path).err() {
            return Err(Error::BadDataPath(data_path.clone(), err));
        }
        let specs_path = &fs_cfg.specs_path;
        debug!("Creating specs directory: {}", specs_path.display());
        if let Some(err) = fs::create_dir_all(&specs_path).err() {
            return Err(Error::BadSpecsPath(specs_path.clone(), err));
        }

        Ok(())
    }

    async fn maybe_uninstall_old_packages(&self, ident: &PackageIdent) {
        if let Some(number_latest_to_keep) = self.state.cfg.keep_latest_packages {
            match pkg::uninstall_all_but_latest(ident, number_latest_to_keep).await {
                Ok(uninstalled) => {
                    info!("Uninstalled '{}' '{}' packages keeping the '{}' latest",
                          uninstalled, ident, number_latest_to_keep)
                }
                Err(e) => {
                    error!("Failed to uninstall '{}' packages keeping the '{}' latest, err: {}",
                           ident, number_latest_to_keep, e)
                }
            }
        }
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (read)
    async fn add_service_rsw_mlw_rhw_msr(&mut self, spec: ServiceSpec) {
        let ident = spec.ident.clone();
        let service = match Service::new(self.sys.clone(),
                                         spec,
                                         self.fs_cfg.clone(),
                                         self.organization.as_ref().map(String::as_str),
                                         self.state.gateway_state.clone(),
                                         self.pid_source,
                                         self.feature_flags).await
        {
            Ok(service) => {
                outputln!("Starting {} ({})", ident, service.pkg.ident);
                service
            }
            Err(err) => {
                outputln!("Unable to start {}, {}", ident, err);
                // Remove the spec file so it does not look like this service is loaded.
                self.remove_spec_file(&ident).ok();
                return;
            }
        };

        if let Ok(package) =
            PackageInstall::load(service.pkg.ident.as_ref(), Some(Path::new(&*FS_ROOT_PATH)))
        {
            if let Err(err) = habitat_common::command::package::install::check_install_hooks(
                &mut habitat_common::ui::UI::with_sinks(),
                &package,
                Path::new(&*FS_ROOT_PATH),
            ).await {
                outputln!("Failed to run install hook for {}, {}", ident, err);
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
            outputln!("{} failed to start", ident);
            return;
        }

        self.gossip_latest_service_rumor_rsw_mlw_rhw(&service);
        if service.topology == Topology::Leader {
            self.butterfly
                .start_election_rsw_mlr_rhw_msr(&service.service_group, 0);
        }

        if let Err(e) = self.user_config_watcher.add(&service) {
            outputln!("Unable to start UserConfigWatcher for {}: {}",
                      service.spec_ident,
                      e);
            return;
        }

        self.maybe_uninstall_old_packages(&ident).await;

        self.service_updater.lock().add(&service);

        event::service_started(&service);

        self.state
            .services
            .lock_msw()
            .insert(service.spec_ident.clone(), service);
    }

    // If we ever need to modify this function, it would be an excellent opportunity to
    // simplify the redundant aspects and remove this allow(clippy::cognitive_complexity),
    // but changing it in the absence of other necessity seems like too much risk for the
    // expected reward.
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::initial_members` (write)
    /// * `MemberList::entries` (write)
    /// * `GatewayState::inner` (write)
    /// * `Server::member` (write)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (write)
    #[allow(clippy::cognitive_complexity)]
    pub async fn run_rsw_imlw_mlw_gsw_smw_rhw_msw(mut self,
                                                  svc: Option<habitat_sup_protocol::ctl::SvcLoad>)
                                                  -> Result<()> {
        let main_hist = RUN_LOOP_DURATION.with_label_values(&["sup"]);
        let service_hist = RUN_LOOP_DURATION.with_label_values(&["service"]);
        let mut next_cpu_measurement = Instant::now();
        let mut cpu_start = ProcessTime::now();

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
                                                             tokio::spawn(handler);
                                                             future::ready(())
                                                         });
        tokio::spawn(ctl_handler);

        if let Some(svc_load) = svc {
            commands::service_load(&self.state, &mut CtlRequest::default(), svc_load).await?;
        }

        // This serves to start up any services that need starting
        // (which will be all of them at this point!)
        self.maybe_spawn_service_futures_rsw_mlw_gsw_rhw_msw().await;

        outputln!("Starting gossip-listener on {}",
                  self.butterfly.gossip_addr());
        self.butterfly
            .start_rsw_mlw_smw_rhw_msr(&Timing::default())?;
        debug!("gossip-listener started");
        self.persist_state_rsr_mlr_gsw_msr().await;
        let http_listen_addr = self.sys.http_listen();
        let ctl_listen_addr = self.sys.ctl_listen();
        let ctl_secret_key = ctl_gateway::readgen_secret_key(&self.fs_cfg.sup_root)?;
        outputln!("Starting ctl-gateway on {}", &ctl_listen_addr);
        tokio::spawn(ctl_gateway::server::run(ctl_listen_addr, ctl_secret_key, mgr_sender));
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
                Arc::new((StdMutex::new(http_gateway::ServerStartup::NotStarted), Condvar::new()));

            outputln!("Starting http-gateway on {}", &http_listen_addr);
            http_gateway::Server::run(http_listen_addr,
                                      tls_server_config,
                                      self.state.gateway_state.clone(),
                                      http_gateway::GatewayAuthenticationToken::configured_value(),
                                      self.feature_flags,
                                      pair.clone());

            // Only cleanup supervisor packages if we are running the latest installed version. It
            // is possible to have no versions installed if a development build is being run.
            if let Some(latest) = pkg::installed(&*THIS_SUPERVISOR_FUZZY_IDENT) {
                if *THIS_SUPERVISOR_IDENT == latest.ident {
                    self.maybe_uninstall_old_packages(&*THIS_SUPERVISOR_FUZZY_IDENT)
                        .await;
                }
            }

            let &(ref lock, ref cvar) = &*pair;
            let mut started = lock.lock().expect("Control mutex is poisoned");

            // This will block the current thread until the HTTP gateway thread either starts
            // successfully or fails to bind. In practice, the wait here is so short as to not be
            // noticeable.
            loop {
                match *started {
                    http_gateway::ServerStartup::NotStarted => {
                        started = match cvar.wait_timeout(started, Duration::from_secs(10)) {
                            Ok((mutex, timeout_result)) => {
                                if timeout_result.timed_out() {
                                    return Err(Error::BindTimeout(http_listen_addr.to_string()));
                                } else {
                                    mutex
                                }
                            }
                            Err(e) => {
                                error!("Mutex for the HTTP gateway was poisoned. e = {:?}", e);
                                return Err(Error::LockPoisoned);
                            }
                        };
                    }
                    http_gateway::ServerStartup::BindFailed => {
                        return Err(Error::BadAddress(http_listen_addr.to_string()));
                    }
                    http_gateway::ServerStartup::Started => break,
                }
            }

            debug!("http-gateway started");
        }

        // Enter the main Supervisor loop. When we break out, it'll be
        // because we've been instructed to shutdown. The value we
        // break out with governs exactly how we shut down.

        // TODO (CM): Investigate the appropriateness of capturing any
        // errors or panics generated in this loop and performing some
        // kind of controlled shutdown.
        let shutdown_mode = loop {
            // This particular loop isn't truly divergent, but since we're in the main loop
            // if the supervisor process, and everything that comes after is expected to complete
            // in a timely manner, we forgo unregistering with the liveliness checker so that
            // getting stuck after exiting this loop generates warnings. Ideally, there would be
            // additional mark_thread_alive calls in any subsequent code which has the potential to
            // loop or wait (including futures), but we don't have that capability yet.
            liveliness_checker::mark_thread_alive().and_divergent();

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

            let next_check = Instant::now() + Duration::from_secs(1);
            if self.launcher.is_stopping() {
                break ShutdownMode::Normal;
            }
            if self.check_for_departure() {
                break ShutdownMode::Departed;
            }

            #[cfg(unix)]
            {
                if signals::pending_sighup() {
                    outputln!("Supervisor shutting down for signal");
                    break ShutdownMode::Restarting;
                }
            }

            if let Some(package) = self.check_for_updated_supervisor().await {
                outputln!("Supervisor shutting down for automatic update to {}",
                          package);
                break ShutdownMode::Restarting;
            }

            // TODO (CM): eventually, make this a future receiver
            for action in action_receiver.try_iter() {
                match action {
                    SupervisorAction::StopService { mut service_spec,
                                                    shutdown_input, } => {
                        service_spec.desired_state = DesiredState::Down;
                        if let Err(err) = self.state.cfg.save_spec_for(&service_spec) {
                            warn!("Tried to stop '{}', but couldn't update the spec: {:?}",
                                  service_spec.ident, err);
                        }
                        self.stop_service_gsw_msw(&service_spec.ident, &shutdown_input);
                    }
                    SupervisorAction::UnloadService { service_spec,
                                                      shutdown_input, } => {
                        self.remove_spec_file(&service_spec.ident).ok();
                        self.stop_service_gsw_msw(&service_spec.ident, &shutdown_input);
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
                self.maybe_spawn_service_futures_rsw_mlw_gsw_rhw_msw().await;
            }

            self.update_peers_from_watch_file_mlr_imlw()?;
            self.update_running_services_from_user_config_watcher_msw();

            // Restart all services that need it
            self.restart_services_rsw_mlr_rhw_msw();

            self.restart_elections_rsw_mlr_rhw_msr(self.feature_flags);
            self.census_ring
                .write()
                .update_from_rumors_rsr_mlr(&self.state.cfg.cache_key_path,
                                            &self.butterfly.service_store,
                                            &self.butterfly.election_store,
                                            &self.butterfly.update_store,
                                            &self.butterfly.member_list,
                                            &self.butterfly.service_config_store,
                                            &self.butterfly.service_file_store);

            if self.check_for_changed_services_msr() || self.census_ring.read().changed() {
                self.persist_state_rsr_mlr_gsw_msr().await;
            }

            for service in self.state.services.lock_msw().services() {
                // time will be recorded automatically by HistogramTimer's drop implementation when
                // this var goes out of scope
                #[allow(unused_variables)]
                let service_timer = service_hist.start_timer();
                if service.tick(&self.census_ring.read(), &self.launcher) {
                    self.gossip_latest_service_rumor_rsw_mlw_rhw(&service);
                }
            }

            // This is really only needed until everything is running
            // in futures.
            let now = Instant::now();
            if now < next_check {
                let time_to_wait = next_check - now;
                thread::sleep(time_to_wait);
            }

            // Measure CPU time every second
            if Instant::now() >= next_cpu_measurement {
                let cpu_duration = cpu_start.elapsed();
                let cpu_nanos =
                    cpu_duration.as_secs()
                                .checked_mul(1_000_000_000)
                                .and_then(|ns| ns.checked_add(cpu_duration.subsec_nanos().into()))
                                .expect("overflow in cpu_duration");
                CPU_TIME.set(cpu_nanos.to_i64());
                next_cpu_measurement = Instant::now() + Duration::from_secs(1);
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
            ShutdownMode::Restarting => {
                outputln!("Preparing services for Supervisor restart");
                for svc in self.state.services.lock_msw().services() {
                    svc.detach();
                }
            }
            ShutdownMode::Normal | ShutdownMode::Departed => {
                outputln!("Gracefully departing from butterfly network.");
                self.butterfly.set_departed_mlw_smw_rhw();

                let service_stop_futures =
                    FuturesUnordered::from_iter(self.state
                                                    .services
                                                    .lock_msw()
                                                    .drain_services()
                                                    .map(|svc| {
                                                        self.stop_service_future_gsw(svc, None,
                                                                                     None)
                                                    }));
                // Wait while all services are stopped
                service_stop_futures.collect::<Vec<_>>().await;
            }
        }

        release_process_lock(&self.fs_cfg);
        self.butterfly.persist_data_rsr_mlr();

        match shutdown_mode {
            ShutdownMode::Normal | ShutdownMode::Restarting => Ok(()),
            ShutdownMode::Departed => Err(Error::Departed),
        }
    }

    async fn check_for_updated_supervisor(&mut self) -> Option<PackageInstall> {
        if let Some(ref mut self_updater) = self.self_updater {
            return self_updater.updated().await;
        }
        None
    }

    /// Restart the Services that have an update or have set their `needs_restart` flag set.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (read)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (write)
    fn restart_services_rsw_mlr_rhw_msw(&mut self) {
        let service_updater = self.service_updater.lock();

        let mut state_services = self.state.services.lock_msw();
        let mut idents_to_restart_and_latest_desired_on_restart = Vec::new();
        for (ident, service) in state_services.iter() {
            if let Some(new_ident) = service_updater.has_update(&service.service_group) {
                outputln!("Restarting {} with package {}", ident, new_ident);
                event::service_update_started(&service, &new_ident);
                // The supervisor always runs the latest package on disk. When we have an update
                // ensure that the lastest package on disk is the package we updated to.
                idents_to_restart_and_latest_desired_on_restart.push((ident.clone(),
                                                                      Some(new_ident)));
            } else if service.needs_restart {
                idents_to_restart_and_latest_desired_on_restart.push((ident.clone(), None));
            } else {
                trace!("No restart required for {}", ident);
            };
        }

        for (ident, latest_desired_on_restart) in idents_to_restart_and_latest_desired_on_restart {
            // unwrap is safe because we've to the write lock, and we
            // know there's a value present at this key.
            let service = state_services.remove(&ident).unwrap();
            // TODO (CM): In the future, when service start up is
            // future-based, we'll want to have an actual "restart"
            // future, that queues up the start future after the stop
            // future.
            //
            // Until then, we will just stop the services, and rely on the
            // our specfile reconciliation logic to catch the fact that
            // the service needs to be restarted. At that point, this function
            // can be renamed; right now, it says exactly what it's doing.
            tokio::spawn(self.stop_service_future_gsw(service, latest_desired_on_restart, None));
        }
    }

    // Creates a rumor for the specified service.
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `RumorHeat::inner` (write)
    fn gossip_latest_service_rumor_rsw_mlw_rhw(&self, service: &Service) {
        let incarnation = self.butterfly
                              .service_store
                              .lock_rsr()
                              .service_group(&service.service_group)
                              .map_rumor(&self.sys.member_id, |rumor| rumor.incarnation + 1)
                              .unwrap_or(1);

        self.butterfly
            .insert_service_rsw_mlw_rhw(service.to_service_rumor(incarnation));
    }

    // Creates a rumor for the specified service's health.
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `RumorHeat::inner` (write)
    fn gossip_latest_service_health_rumor_rsw_mlw_rhw(&self, service: &Service) {
        let incarnation = self.butterfly
                              .service_health_store
                              .lock_rsr()
                              .service_group(&service.service_group)
                              .map_rumor(&self.sys.member_id, |rumor| rumor.incarnation + 1)
                              .unwrap_or(1);

        self.butterfly
            .insert_service_health_rsw_rhw(service.to_service_health_rumor(incarnation));
    }

    fn check_for_departure(&self) -> bool { self.butterfly.is_departed() }

    /// # Locking (see locking.md)
    /// * `ManagerServices::inner` (read)
    fn check_for_changed_services_msr(&mut self) -> bool {
        let mut service_states = HashMap::new();
        let mut active_services = Vec::new();
        for service in self.state.services.lock_msr().services() {
            service_states.insert(service.spec_ident.clone(), service.last_state_change());
            active_services.push(service.spec_ident.clone());
        }

        for loaded in self.spec_dir
                          .specs()
                          .iter()
                          .filter(|s| !active_services.contains(&s.ident))
        {
            // These are loaded but not-running services. As such,
            // we'll use the Epoch as a "default" time marker that
            // won't change.
            //
            // TODO (CM): why do we bother tracking loaded but not
            // running services at all?
            service_states.insert(loaded.ident.clone(), SystemTime::UNIX_EPOCH);
        }

        if service_states != self.service_states {
            self.service_states = service_states;
            true
        } else {
            false
        }
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    /// * `GatewayState::inner` (write)
    /// * `ManagerServices::inner` (read)
    async fn persist_state_rsr_mlr_gsw_msr(&mut self) {
        debug!("Updating census state");
        self.persist_census_state_gsw();
        debug!("Updating butterfly state");
        self.persist_butterfly_state_rsr_mlr_gsw();
        debug!("Updating services state");
        self.persist_services_state_gsw_msr().await;
    }

    /// # Locking (see locking.md)
    /// * `GatewayState::inner` (write)
    fn persist_census_state_gsw(&self) {
        let census_ring = &self.census_ring.read();
        let crp = CensusRingProxy::new(census_ring);
        let json = serde_json::to_string(&crp).expect("CensusRingProxy::serialize failure");
        self.state.gateway_state.lock_gsw().set_census_data(json);
    }

    /// # Locking (see locking.md)
    /// * `RumorStore::list` (read)
    /// * `MemberList::entries` (read)
    /// * `GatewayState::inner` (write)
    fn persist_butterfly_state_rsr_mlr_gsw(&self) {
        let bs = ServerProxy::new(&self.butterfly);
        let json = serde_json::to_string(&bs).expect("ServerProxy::serialize failure");
        self.state.gateway_state.lock_gsw().set_butterfly_data(json);
    }

    /// # Locking (see locking.md)
    /// * `GatewayState::inner` (write)
    /// * `ManagerServices::inner` (read)
    async fn persist_services_state_gsw_msr(&self) {
        let config_rendering = if self.feature_flags.contains(FeatureFlag::REDACT_HTTP) {
            ConfigRendering::Redacted
        } else {
            ConfigRendering::Full
        };

        let service_map = self.state.services.lock_msr();
        let existing_idents: Vec<PackageIdent> = service_map.services()
                                                            .map(|s| s.spec_ident.clone())
                                                            .collect();

        // Services that are not active but are being watched for changes
        // These would include stopped persistent services or other
        // persistent services that failed to load
        // We cannot use `filter_map` here because futures cannot be awaited in a closure.
        let mut watched_services = Vec::new();
        for spec in self.spec_dir.specs() {
            if !existing_idents.contains(&spec.ident) {
                let ident = spec.ident.clone();
                let result = Service::new(self.sys.clone(),
                                          spec,
                                          self.fs_cfg.clone(),
                                          self.organization.as_ref().map(String::as_str),
                                          self.state.gateway_state.clone(),
                                          self.pid_source,
                                          self.feature_flags).await;
                match result {
                    Ok(result) => watched_services.push(result),
                    Err(ref e) => warn!("Failed to create service '{}' from spec: {:?}", ident, e),
                }
            }
        }
        let watched_service_proxies: Vec<ServiceProxy<'_>> =
            watched_services.iter()
                            .map(|s| ServiceProxy::new(s, config_rendering))
                            .collect();
        let mut services_to_render: Vec<ServiceProxy<'_>> =
            service_map.services()
                       .map(|s| ServiceProxy::new(s, config_rendering))
                       .collect();

        services_to_render.extend(watched_service_proxies);

        let json =
            serde_json::to_string(&services_to_render).expect("ServiceProxy::serialize failure");
        self.state.gateway_state.lock_gsw().set_services_data(json);
    }

    /// Check if any elections need restarting.
    ///
    /// # Locking (see locking.md)
    /// * `MemberList::entries` (read)
    /// * `RumorStore::list` (write)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (read)
    fn restart_elections_rsw_mlr_rhw_msr(&mut self, feature_flags: FeatureFlag) {
        self.butterfly
            .restart_elections_rsw_mlr_rhw_msr(feature_flags);
    }

    /// # Locking (see locking.md)
    /// * `GatewayState::inner` (write)
    /// * `ManagerServices::inner` (write)
    fn stop_service_gsw_msw(&mut self, ident: &PackageIdent, shutdown_input: &ShutdownInput) {
        if let Some(service) = self.remove_service_from_state_msw(&ident) {
            let future = self.stop_service_future_gsw(service, None, Some(shutdown_input));
            tokio::spawn(future);
        } else {
            warn!("Tried to stop '{}', but couldn't find it in our list of running services!",
                  ident);
        }
    }

    /// Create a future for stopping a Service removing it from the manager. The Service is assumed
    /// to have been removed from the internal list of active services already (see, e.g.,
    /// restart_services_rsw_mlr_rhw_msw and remove_service_from_state).
    /// # Locking for the returned Future (see locking.md)
    /// * `GatewayState::inner` (write)
    fn stop_service_future_gsw(&self,
                               mut service: Service,
                               latest_desired_on_restart: Option<PackageIdent>,
                               shutdown_input: Option<&ShutdownInput>)
                               -> impl Future<Output = ()> {
        let mut user_config_watcher = self.user_config_watcher.clone();
        let service_updater = Arc::clone(&self.service_updater);
        let busy_services = Arc::clone(&self.busy_services);
        let services_need_reconciliation = self.services_need_reconciliation.clone();
        let shutdown_config = ShutdownConfig::new(shutdown_input, &service);

        // JW TODO: Update service rumor to remove service from
        // cluster
        // TODO (CM): But only if we're not going down for a restart.
        let ident = service.spec_ident.clone();
        let stop_it = async move {
            service.stop_gsw(shutdown_config).await;
            event::service_stopped(&service);
            user_config_watcher.remove(&service);
            service_updater.lock().remove(&service.service_group);
            // At this point the service process is stopped but the package is still loaded by the
            // Supervisor.
            if let Some(latest_desired_ident) = latest_desired_on_restart {
                Self::uninstall_newer_packages(&service.spec_ident, &latest_desired_ident).await;
            }
        };
        Self::wrap_async_service_operation(ident,
                                           busy_services,
                                           services_need_reconciliation,
                                           stop_it)
    }

    /// Uninstall packages that are newer than the specified ident.
    ///
    /// This can be used to guarantee that when a service restarts it starts with the desired
    /// package.
    async fn uninstall_newer_packages(install_ident: &PackageIdent,
                                      latest_desired_ident: &PackageIdent) {
        while let Some(latest_installed) = pkg::installed(install_ident) {
            let latest_ident = latest_installed.ident;
            if latest_ident > *latest_desired_ident {
                info!("Uninstalling '{}' inorder to ensure '{}' is the latest installed package",
                      latest_ident, latest_desired_ident);
                if let Err(e) = pkg::uninstall_even_if_loaded(&latest_ident).await {
                    error!("Failed to uninstall '{}' unable to ensure '{}' is the latest \
                            installed package. On restart, service will start with the wrong \
                            package. err: {}",
                           latest_ident, latest_desired_ident, e);
                }
            } else {
                break;
            }
        }
    }

    fn remove_spec_file(&self, ident: &PackageIdent) -> std::io::Result<()> {
        let file = self.state.cfg.spec_path_for(ident);
        let result = fs::remove_file(&file);
        if let Err(ref err) = result {
            warn!("Tried to remove spec file '{}' for '{}': {:?}",
                  file.display(),
                  ident,
                  err);
        };
        result
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
    async fn wrap_async_service_operation<F>(ident: PackageIdent,
                                             busy_services: Arc<Mutex<HashSet<PackageIdent>>>,
                                             services_need_reconciliation: ReconciliationFlag,
                                             fut: F)
        where F: Future<Output = ()>
    {
        trace!("Flagging '{:?}' as busy, pending an asynchronous operation",
               ident);
        busy_services.lock().insert(ident.clone());
        fut.await;
        trace!("Removing 'busy' flag for '{:?}'; asynchronous operation over",
               ident);
        busy_services.lock().remove(&ident);
        services_need_reconciliation.set();
    }

    /// Determine if our on-disk spec files indicate that we should
    /// perform some action on our services (start, stop, etc.)
    ///
    /// If so, futures for those actions are spawned on the runtime.
    ///
    /// NOTE: Service start is currently synchronous, so any start
    /// operations will be performed directly as a consequence of
    /// calling this method.
    ///
    /// # Locking (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `GatewayState::inner` (write)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (write)
    async fn maybe_spawn_service_futures_rsw_mlw_gsw_rhw_msw(&mut self) {
        let ops = self.compute_service_operations_msr();
        self.spawn_futures_from_operations_rsw_mlw_gsw_rhw_msw(ops)
            .await;
    }

    /// # Locking (see locking.md)
    /// * `ManagerServices::inner` (write)
    fn remove_service_from_state_msw(&mut self, ident: &PackageIdent) -> Option<Service> {
        self.state.services.lock_msw().remove(&ident)
    }

    /// Start, stop, or restart services to bring what's running in
    /// line with what our spec files say.
    ///
    /// In the future, this will simply convert `ServiceOperation`s
    /// into futures that can be later spawned. Until starting of
    /// services is made asynchronous, however, it performs a mix of
    /// operations; starts are performed synchronously, while
    /// shutdowns and restarts are turned into futures.
    ///
    /// # Locking for the returned Futures (see locking.md)
    /// * `RumorStore::list` (write)
    /// * `MemberList::entries` (write)
    /// * `GatewayState::inner` (write)
    /// * `RumorHeat::inner` (write)
    /// * `ManagerServices::inner` (write)
    async fn spawn_futures_from_operations_rsw_mlw_gsw_rhw_msw<O>(&mut self, ops: O)
        where O: IntoIterator<Item = ServiceOperation>
    {
        for op in ops.into_iter() {
            match op {
                ServiceOperation::Stop(spec) | ServiceOperation::Restart { to_stop: spec, .. } => {
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
                    if let Some(service) = self.remove_service_from_state_msw(&spec.ident) {
                        tokio::spawn(self.stop_service_future_gsw(service, None, None));
                    } else {
                        // We really don't expect this to happen....
                        outputln!("Tried to remove service for {} but could not find it running, \
                                   skipping",
                                  &spec.ident);
                    }
                }
                ServiceOperation::Start(spec) => {
                    // Execute the future synchronously
                    self.add_service_rsw_mlw_rhw_msr(spec).await;
                }
            }
        }
    }

    /// Determine what services we need to start, stop, or restart in
    /// order to be running what our on-disk spec files tell us we
    /// should be running.
    ///
    /// See `specs_to_operations` for the real logic.
    /// # Locking (see locking.md)
    /// * `ManagerServices::inner` (read)
    fn compute_service_operations_msr(&mut self) -> Vec<ServiceOperation> {
        // First, figure out what's currently running.
        let service_map = self.state.services.lock_msr();
        let currently_running_specs = service_map.services().map(Service::to_spec);

        // Now, figure out what we should compare against, ignoring
        // any services that are currently doing something
        // asynchronously.
        let busy_services = self.busy_services.lock();
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

    /// # Locking (see locking.md)
    /// * `MemberList::entries` (read)
    /// * `MemberList::initial_members` (write)
    fn update_peers_from_watch_file_mlr_imlw(&mut self) -> Result<()> {
        if !self.butterfly.need_peer_seeding_mlr() {
            return Ok(());
        }
        match self.peer_watcher {
            None => Ok(()),
            Some(ref watcher) => {
                if watcher.has_fs_events() {
                    let members = watcher.get_members()?;
                    self.butterfly.member_list.set_initial_members_imlw(members);
                }
                Ok(())
            }
        }
    }

    /// # Locking (see locking.md)
    /// * `ManagerServices::inner` (write)
    fn update_running_services_from_user_config_watcher_msw(&mut self) {
        for service in self.state.services.lock_msw().services() {
            if self.user_config_watcher.have_events_for(service) {
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
                      .map_err(|_| Error::InvalidCertFile(path.clone()))?
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
                                 .map_err(|_| Error::InvalidCertFile(config.cert_path.clone()))?;

    let key = pemfile::rsa_private_keys(key_file).and_then(|mut k| k.pop().ok_or(()))
                                                 .map_err(|_| {
                                                     Error::InvalidKeyFile(config.key_path.clone())
                                                 })?;

    server_config.set_single_cert(cert_chain, key)?;
    server_config.ignore_client_order = true;
    Ok(server_config)
}

fn obtain_process_lock(fs_cfg: &FsCfg) -> Result<()> {
    match write_process_lock(&fs_cfg.proc_lock_file) {
        Ok(()) => Ok(()),
        Err(_) => {
            match read_process_lock(&fs_cfg.proc_lock_file) {
                Ok(pid) => {
                    if process::is_alive(pid) {
                        return Err(Error::ProcessLocked(pid));
                    }
                    release_process_lock(&fs_cfg);
                    write_process_lock(&fs_cfg.proc_lock_file)
                }
                Err(Error::ProcessLockCorrupt) => {
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
                        Ok(pid) if pid == 0 => {
                            error!(target: "pidfile_tracing", "Read PID of 0 from process lock file {}!",
                                   lock_path.as_ref().display());
                            // Treat this the same as a corrupt pid
                            // file, because that's basically what it
                            // is.
                            //
                            // This *should* be an impossible situation.
                            Err(Error::ProcessLockCorrupt)
                        }
                        Ok(pid) => Ok(pid),
                        Err(_) => Err(Error::ProcessLockCorrupt),
                    }
                }
                _ => Err(Error::ProcessLockCorrupt),
            }
        }
        Err(err) => Err(Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err)),
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
                Err(err) => Err(Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err)),
            }
        }
        Err(err) => Err(Error::ProcessLockIO(lock_path.as_ref().to_path_buf(), err)),
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
    // Assume we have /proc/self/fd unless we know we don't
    #[cfg(not(any(target_os = "freebsd", target_os = "macos", target_os = "ios")))]
    const FD_DIR: &str = "/proc/self/fd";
    #[cfg(any(target_os = "freebsd", target_os = "macos", target_os = "ios"))]
    const FD_DIR: &str = "/dev/fd";

    Ok(fs::read_dir(FD_DIR)?.count())
}

#[cfg(unix)]
fn track_memory_stats() {
    // We'd like to track some memory stats, but these stats are cached and only refreshed
    // when the epoch is advanced. We manually advance it here to ensure our stats are
    // fresh.
    jemalloc_ctl::epoch::mib().unwrap().advance().unwrap();

    MEMORY_STATS.with_label_values(&["active"])
                .set(jemalloc_ctl::stats::active::mib().unwrap()
                                                       .read()
                                                       .unwrap()
                                                       .to_i64());
    MEMORY_STATS.with_label_values(&["allocated"])
                .set(jemalloc_ctl::stats::allocated::mib().unwrap()
                                                          .read()
                                                          .unwrap()
                                                          .to_i64());
    MEMORY_STATS.with_label_values(&["mapped"])
                .set(jemalloc_ctl::stats::mapped::mib().unwrap()
                                                       .read()
                                                       .unwrap()
                                                       .to_i64());
    MEMORY_STATS.with_label_values(&["metadata"])
                .set(jemalloc_ctl::stats::metadata::mib().unwrap()
                                                         .read()
                                                         .unwrap()
                                                         .to_i64());
    MEMORY_STATS.with_label_values(&["resident"])
                .set(jemalloc_ctl::stats::resident::mib().unwrap()
                                                         .read()
                                                         .unwrap()
                                                         .to_i64());
    MEMORY_STATS.with_label_values(&["retained"])
                .set(jemalloc_ctl::stats::retained::mib().unwrap()
                                                         .read()
                                                         .unwrap()
                                                         .to_i64());
}

// This is a no-op on purpose because windows doesn't support jemalloc
#[cfg(windows)]
fn track_memory_stats() {}

#[cfg(test)]
mod test {
    use super::*;
    use habitat_core::fs::CACHE_KEY_PATH;
    use habitat_sup_protocol::STATE_PATH_PREFIX;
    use std::{net::Ipv4Addr,
              path::PathBuf};

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
            ManagerConfig { auto_update:          false,
                            custom_state_path:    None,
                            cache_key_path:       (&*CACHE_KEY_PATH).to_path_buf(),
                            update_url:           "".to_string(),
                            update_channel:       ChannelIdent::default(),
                            gossip_listen:        GossipListenAddr::default(),
                            ctl_listen:           ListenCtlAddr::default(),
                            http_listen:          HttpListenAddr::default(),
                            http_disable:         false,
                            gossip_peers:         vec![],
                            gossip_permanent:     false,
                            ring_key:             None,
                            organization:         None,
                            watch_peer_file:      None,
                            tls_config:           None,
                            feature_flags:        FeatureFlag::empty(),
                            event_stream_config:  None,
                            keep_latest_packages: None,
                            sys_ip:               IpAddr::V4(Ipv4Addr::LOCALHOST), }
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

    mod specs_to_operations {
        //! Testing out the reconciliation of on-disk spec files with
        //! what is currently running.

        use super::super::*;
        use habitat_sup_protocol::types::UpdateStrategy;

        /// Helper function for generating a basic spec from an
        /// identifier string
        fn new_spec(ident: &str) -> ServiceSpec {
            ServiceSpec::new(PackageIdent::from_str(ident).expect("couldn't parse ident str"))
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
            assert!(Manager::specs_to_operations(specs.clone(), specs).is_empty());
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

            let running = vec![svc_1_running,
                               svc_2_running.clone(),
                               svc_3_running.clone(),
                               svc_6_running.clone(),];

            let on_disk = vec![svc_1_on_disk,
                               svc_2_on_disk,
                               svc_3_on_disk.clone(),
                               svc_4_on_disk,
                               svc_5_on_disk.clone(),];

            let operations = Manager::specs_to_operations(running, on_disk);

            let expected_operations = vec![ServiceOperation::Stop(svc_2_running),
                                           ServiceOperation::Restart { to_stop:  svc_3_running,
                                                                       to_start: svc_3_on_disk, },
                                           ServiceOperation::Start(svc_5_on_disk),
                                           ServiceOperation::Stop(svc_6_running),];

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
