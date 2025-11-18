// TODO (CM): Take another look at the public exports of this module
// (specifically, `pub mod spec`, and the various `pub use`
// statements. Playing fast-and-loose with our imports has led to code
// that's more confusing that it probably needs to be.

// TODO (CM): Take a deeper look at the direct consumption of
// Prost-generated types (habitat_sup_protocol::types::*) in
// here. Ideally, those would exist only at the periphery of the
// system, and we'd use separate internal types for our core logic.

mod context;
mod health;
mod hook_runner;
mod hooks;
#[cfg(windows)]
mod pipe_hook_client;
pub mod spec;
mod supervisor;
mod terminator;

use self::{context::RenderContext,
           hook_runner::HookRunner,
           hooks::{HookCompileTable,
                   HookTable,
                   HookTableQueryModel},
           supervisor::{PidUpdate,
                        SupervisedProcessQueryModel,
                        Supervisor}};
pub use self::{health::{HealthCheckBundle,
                        HealthCheckHookStatus,
                        HealthCheckResult},
               hooks::{HealthCheckHook,
                       ProcessOutput,
                       StandardStreams},
               spec::{DesiredState,
                      ServiceSpec}};
use crate::{census::{CensusGroup,
                     CensusRing,
                     ElectionStatus,
                     ServiceFile},
            error::{Error,
                    Result},
            manager::{FsCfg,
                      ServicePidSource,
                      ShutdownConfig,
                      Sys,
                      event,
                      sync::GatewayState}};
use futures::future::{self,
                      AbortHandle};
use habitat_butterfly::rumor::service::Service as ServiceRumor;
#[cfg(windows)]
use habitat_common::templating::package::DEFAULT_USER;
pub use habitat_common::templating::{config::{Cfg,
                                              UserConfigPath},
                                     package::{Env,
                                               Pkg}};
use habitat_common::{FeatureFlag,
                     outputln,
                     templating::{config::CfgRenderer,
                                  hooks::Hook,
                                  package::PkgQueryModel}};
#[cfg(windows)]
use habitat_core::os::users;
use habitat_core::{ChannelIdent,
                   crypto::Blake2bHash,
                   flowcontrol::Backoff,
                   fs::{FS_ROOT_PATH,
                        SvcDir,
                        atomic_write,
                        svc_hooks_path},
                   os::process::{Pid,
                                 ShutdownTimeout},
                   package::{PackageIdent,
                             PackageInstall,
                             metadata::Bind},
                   service::{HealthCheckInterval,
                             ServiceBind,
                             ServiceGroup}};
use habitat_launcher_client::LauncherCli;
use habitat_sup_protocol::types::BindingMode;
pub use habitat_sup_protocol::types::{ProcessState,
                                      Topology,
                                      UpdateCondition,
                                      UpdateStrategy};
use log::{debug,
          trace};
use parking_lot::RwLock;
use prometheus::{HistogramTimer,
                 HistogramVec,
                 register_histogram_vec};
use serde::{Deserialize,
            Serialize,
            Serializer,
            ser::{Error as _,
                  SerializeStruct}};
use std::{self,
          collections::HashSet,
          convert::TryFrom,
          fmt,
          fs,
          ops::Deref,
          path::{Path,
                 PathBuf},
          result,
          sync::{Arc,
                 Mutex},
          time::{Duration,
                 SystemTime}};

use super::ServiceRestartConfig;
use lazy_static::lazy_static;

static LOGKEY: &str = "SR";

#[cfg(not(windows))]
pub const GOSSIP_FILE_PERMISSIONS: u32 = 0o640;

lazy_static! {
    static ref HOOK_DURATION: HistogramVec =
        register_histogram_vec!("hab_sup_hook_duration_seconds",
                                "The time it takes for a hook to run",
                                &["hook"]).unwrap();
}

/// When evaluating whether a particular service group can satisfy a
/// bind of the Service, there are several states it can be
/// in. Depending on which point in the lifecycle of the Service we
/// are in, we may want to take different actions depending on the
/// current status.
enum BindStatus<'a> {
    /// The bound group is not present in the census
    NotPresent,
    /// The bound group is present in the census, but has no active
    /// members.
    Empty,
    /// The bound group is present in the census, has active members,
    /// but does not satisfy the contract of the bind; the set of
    /// unsatisfied exports is returned.
    Unsatisfied(HashSet<&'a String>),
    /// The bound group is present, has active members, and fully
    /// satisfies the contract of the bind.
    Satisfied,
    /// An error was encountered determining the status
    Unknown(Error),
}

/// Encapsulate changes to `/hooks` and `/config`.
#[derive(Default)]
struct TemplateUpdate {
    hooks:                 HookCompileTable,
    config_changed:        bool,
    have_reconfigure_hook: bool,
}

impl TemplateUpdate {
    fn new(hooks: HookCompileTable, config_changed: bool, have_reconfigure_hook: bool) -> Self {
        Self { hooks,
               config_changed,
               have_reconfigure_hook }
    }

    /// Returns `true` if the service needs to be restarted.
    ///
    /// A restart is needed under the following conditions:
    /// 1. the `init`, `run` or `post-run` hooks have changed. A restart is limited to these hooks
    ///    because they are the only hooks that can impact the execution of the service.
    /// 2. `/config` changed and there is no `reconfigure` hook
    fn needs_restart(&self) -> Option<ProcessTerminationReason> {
        if self.hooks.init_changed() {
            Some(ProcessTerminationReason::InitHookUpdated)
        } else if self.hooks.run_changed() {
            Some(ProcessTerminationReason::RunHookUpdated)
        } else if self.hooks.post_run_changed() {
            Some(ProcessTerminationReason::PostRunHookUpdated)
        } else if !self.have_reconfigure_hook && self.config_changed {
            Some(ProcessTerminationReason::AppConfigUpdated)
        } else {
            None
        }
    }

    /// Returns `true` if the service needs to be reconfigured.
    ///
    /// A reconfigure is needed if `/config` or the `reconfigure` hook changed.
    fn needs_reconfigure(&self) -> bool { self.config_changed || self.hooks.reconfigure_changed() }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum InitializationState {
    Uninitialized,
    Initializing,
    InitializerFailed(SystemTime),
    InitializerFinished,
    Initialized,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub enum RestartState {
    #[default]
    None,
    NeedsRestart,
    NeedsImmediateRestart,
    Restarting,
    RestartingImmediately,
    Restarted,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessTerminationReason {
    #[serde(rename = "package_updated")]
    PackageUpdated,
    #[serde(rename = "init_hook_failed")]
    InitHookFailed,
    #[serde(rename = "run_hook_failed")]
    RunHookFailed,
    #[serde(rename = "app_config_updated")]
    AppConfigUpdated,
    #[serde(rename = "init_hook_updated")]
    InitHookUpdated,
    #[serde(rename = "run_hook_updated")]
    RunHookUpdated,
    #[serde(rename = "post_run_hook_updated")]
    PostRunHookUpdated,
}

#[derive(Debug, Clone)]
pub struct LastProcessState {
    pub pid:                Option<Pid>,
    pub termination_reason: ProcessTerminationReason,
    // TODO: Create a new type for SystemTime so we don't have to rely on
    // a custom serialize implementation for the whole type
    pub terminated_at:      SystemTime,
}

impl Serialize for LastProcessState {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("last_process_state", 3)?;
        strukt.serialize_field("pid", &self.pid)?;
        strukt.serialize_field("termination_reason", &self.termination_reason)?;
        strukt.serialize_field("terminated_at",
                               &self.terminated_at
                                    .duration_since(SystemTime::UNIX_EPOCH)
                                    .map_err(|err| {
                                        S::Error::custom(format!("System time should ALWAYS be \
                                                                  after the UNIX Epoch: {:?}",
                                                                 err))
                                    })?
                                    .as_secs())?;
        strukt.end()
    }
}

#[derive(Debug, Clone)]
pub struct ServiceRunState {
    pub restart_count:      u64,
    pub restart_config:     ServiceRestartConfig,
    pub last_process_state: Option<LastProcessState>,
    current_pid:            Option<Pid>,
    restart_state:          RestartState,
    restart_backoff:        Backoff,
    last_updated_at:        SystemTime,
}

impl ServiceRunState {
    pub fn new(restart_config: &ServiceRestartConfig) -> ServiceRunState {
        ServiceRunState { restart_count:      0,
                          restart_config:     restart_config.clone(),
                          last_process_state: None,
                          current_pid:        None,
                          restart_state:      RestartState::None,
                          restart_backoff:    Backoff::new(restart_config.min_backoff_period,
                                                           restart_config.max_backoff_period,
                                                           3f64),
                          last_updated_at:    SystemTime::now(), }
    }

    pub fn mark_for_restart(&mut self,
                            old_pid: Option<Pid>,
                            reason: ProcessTerminationReason,
                            timestamp: SystemTime) {
        self.restart_state = RestartState::NeedsRestart;
        self.last_process_state = Some(LastProcessState { pid:                old_pid,
                                                          terminated_at:      timestamp,
                                                          termination_reason: reason, });
        self.last_updated_at = timestamp;
    }

    pub fn mark_for_immediate_restart(&mut self,
                                      old_pid: Option<Pid>,
                                      reason: ProcessTerminationReason,
                                      timestamp: SystemTime) {
        self.restart_state = RestartState::NeedsImmediateRestart;
        self.last_process_state = Some(LastProcessState { pid:                old_pid,
                                                          terminated_at:      timestamp,
                                                          termination_reason: reason, });
        // Immediate restarts wipe out the restart out
        self.restart_count = 0;
        self.restart_backoff.reset();
        self.last_updated_at = timestamp;
    }

    pub fn reset_backoff(&mut self) {
        self.restart_backoff.reset();
        self.last_updated_at = SystemTime::now();
    }
}

#[derive(Debug)]
pub struct PersistentServiceWrapper {
    run_state: ServiceRunState,
    inner:     Option<Service>,
}

impl PersistentServiceWrapper {
    pub fn new(service: Service,
               restart_config: &ServiceRestartConfig)
               -> PersistentServiceWrapper {
        PersistentServiceWrapper { run_state: ServiceRunState::new(restart_config),
                                   inner:     Some(service), }
    }

    /// Takes ownership of a service from another wrapper
    pub fn take_service(&mut self, mut other: PersistentServiceWrapper) {
        self.inner = other.inner.take();
    }

    /// Get the run state of a service
    pub fn service_run_state(&self) -> &ServiceRunState { &self.run_state }

    pub fn service_run_state_mut(&mut self) -> &ServiceRunState { &mut self.run_state }

    /// Mark this service for an immediate restart
    pub fn mark_for_restart_due_to_update(&mut self, timestamp: SystemTime) {
        self.run_state
            .mark_for_immediate_restart(self.run_state.current_pid,
                                        ProcessTerminationReason::PackageUpdated,
                                        timestamp);
    }

    pub fn service(&self) -> Option<&Service> { self.inner.as_ref() }

    pub fn service_mut(&mut self) -> Option<&mut Service> { self.inner.as_mut() }

    /// Returns the last time the run state or the service's process state
    /// has changed. This is used to determine when to write the changed state to
    /// the gateway.
    pub fn last_state_change(&self) -> SystemTime {
        if let Some(service) = self.inner.as_ref() {
            service.last_state_change()
                   .max(self.run_state.last_updated_at)
        } else {
            self.run_state.last_updated_at
        }
    }

    pub fn start(&mut self) {
        if let Some(service) = self.inner.as_ref() {
            self.run_state.restart_state = match self.run_state.restart_state {
                RestartState::None => RestartState::None,
                RestartState::NeedsRestart | RestartState::NeedsImmediateRestart => {
                    panic!("Start called on service which was not ready to be restarted");
                }
                RestartState::RestartingImmediately => {
                    outputln!(preamble service.service_group, "Restarted");
                    RestartState::Restarted
                }
                RestartState::Restarting => {
                    outputln!(preamble service.service_group, "Restarted");
                    self.run_state.restart_backoff.record_attempt_end();
                    RestartState::Restarted
                }
                RestartState::Restarted => {
                    panic!("Start called on service which was already restarted")
                }
            };
        }
    }

    pub fn shutdown(&mut self, is_restart: bool) -> Option<Service> {
        if let Some(service) = &self.inner {
            if is_restart {
                self.run_state.restart_state = match self.run_state.restart_state {
                    RestartState::None => {
                        panic!("Shutdown called on service which did not need restarting")
                    }
                    RestartState::NeedsRestart => {
                        let restart_duration = self.run_state
                                                   .restart_backoff
                                                   .record_attempt_start()
                                                   .unwrap_or_default();
                        self.run_state.restart_count += 1;
                        if restart_duration == Duration::from_secs(0) {
                            outputln!(preamble service.service_group, "Stopping service, will restart immediately");
                        } else {
                            outputln!(preamble service.service_group, "Stopping service, will restart after {:.2} secs", restart_duration.as_secs_f32());
                        }
                        RestartState::Restarting
                    }
                    RestartState::NeedsImmediateRestart => {
                        outputln!(preamble service.service_group, "Stopping service, will restart immediately");
                        RestartState::RestartingImmediately
                    }
                    RestartState::Restarting | RestartState::RestartingImmediately => {
                        panic!("Shutdown called on service which was already restarting")
                    }
                    RestartState::Restarted => {
                        panic!("Shutdown called on service not requiring restart")
                    }
                };
            }
            self.inner.take()
        } else {
            None
        }
    }

    pub fn should_shutdown_for_restart(&self) -> bool {
        match self.run_state.restart_state {
            RestartState::NeedsRestart | RestartState::NeedsImmediateRestart => true,
            RestartState::None
            | RestartState::Restarting
            | RestartState::RestartingImmediately
            | RestartState::Restarted => false,
        }
    }

    pub fn is_ready_for_restart(&self) -> bool {
        self.run_state.restart_state == RestartState::RestartingImmediately
        || (self.run_state.restart_state == RestartState::Restarting
            && self.run_state
                   .restart_backoff
                   .duration_until_next_attempt_start()
                   .is_none())
    }

    pub fn tick(&mut self, census_ring: &CensusRing, launcher: &LauncherCli) -> bool {
        match &mut self.inner {
            Some(service) => {
                trace!("Starting service tick with persistent state: {:?}",
                       self.run_state);
                service.tick(&mut self.run_state, census_ring, launcher)
            }
            None => false,
        }
    }
}

#[derive(Debug)]
pub struct Service {
    spec:                    ServiceSpec,
    pub service_group:       ServiceGroup,
    // TODO: `spec_file` is only used for serialization; unsure if
    // that's even useful, given that it's always the same value for a
    // given service.
    spec_file:               PathBuf,
    pub cfg:                 Cfg,
    pub pkg:                 Pkg,
    pub sys:                 Arc<Sys>,
    pub user_config_updated: bool,
    // TODO (DM): The need to track initialization state across ticks would be removed if we
    // migrated away from the event loop architecture to an architecture that had a top level
    // `Service` future. See https://github.com/habitat-sh/habitat/issues/7112
    initialization_state:    Arc<RwLock<InitializationState>>,

    config_renderer:      CfgRenderer,
    // Note: This field is really only needed for serializing a
    // Service in the gateway (see ServiceProxy's Serialize
    // implementation). Ideally, we could get rid of this, since we're
    // *also* storing the health check result directly (see
    // manager::GatewayState#health_check_data), but because of how
    // the data is currently rendered, this is a little complicated.
    //
    // In order to access this field in an asynchronous health check
    // hook, we need to wrap some Arc<Mutex<_>> protection around it
    // :(
    health_check_result:  Arc<Mutex<HealthCheckResult>>,
    last_election_status: ElectionStatus,
    /// The binds that the current service package declares, both
    /// required and optional. We don't differentiate because this is
    /// used to validate the user-specified bindings against the
    /// current state of the census; once you get into the actual
    /// running of the service, the distinction is immaterial.
    all_pkg_binds:        Vec<Bind>,
    /// Binds specified by the user that are currently mapped to
    /// service groups that do _not_ satisfy the bind's contract, as
    /// defined in the service's current package.
    ///
    /// They may not satisfy them because they do not have the
    /// requisite exports, because no live members of the group exist,
    /// or because the group itself does not exist in the census.
    ///
    /// We don't serialize because this is purely runtime information
    /// that should be reconciled against the current state of the
    /// census.
    unsatisfied_binds:    HashSet<ServiceBind>,
    hooks:                HookTable,
    manager_fs_cfg:       Arc<FsCfg>,
    supervisor:           Arc<Mutex<Supervisor>>,

    gateway_state: Arc<GatewayState>,

    /// A "handle" to the never-ending future that periodically runs
    /// health checks on this service. This is the means by which we
    /// can stop that future.
    health_check_handle: Option<AbortHandle>,
    post_run_handle:     Option<AbortHandle>,
    initialize_handle:   Option<AbortHandle>,
}

impl Service {
    pub(crate) fn bldr_url(&self) -> String { self.spec.bldr_url.clone() }

    pub(crate) fn channel(&self) -> ChannelIdent { self.spec.channel.clone() }

    pub(crate) fn spec_ident(&self) -> PackageIdent { self.spec.ident.clone() }

    pub(crate) fn topology(&self) -> Topology { self.spec.topology }

    pub(crate) fn update_strategy(&self) -> UpdateStrategy { self.spec.update_strategy }

    pub(crate) fn update_condition(&self) -> UpdateCondition { self.spec.update_condition }

    pub(crate) fn shutdown_timeout(&self) -> Option<ShutdownTimeout> { self.spec.shutdown_timeout }

    pub(crate) fn spec(&self) -> ServiceSpec { self.spec.clone() }

    pub(crate) fn set_spec(&mut self, spec: ServiceSpec) {
        trace!("Setting spec for {}: {:?}", self.spec.ident, spec);
        self.spec = spec
    }

    #[allow(clippy::too_many_arguments)]
    async fn with_package(sys: Arc<Sys>,
                          package: &PackageInstall,
                          spec: ServiceSpec,
                          manager_fs_cfg: Arc<FsCfg>,
                          organization: Option<&str>,
                          census_ring: Arc<RwLock<CensusRing>>,
                          gateway_state: Arc<GatewayState>,
                          pid_source: ServicePidSource,
                          feature_flags: FeatureFlag)
                          -> Result<Service> {
        spec.validate(package)?;
        let all_pkg_binds = package.all_binds()?;
        let mut pkg = Self::resolve_pkg(package, &spec).await?;
        if let Some(timeout) = spec.shutdown_timeout {
            pkg.shutdown_timeout = timeout;
        }
        let spec_file = manager_fs_cfg.specs_path.join(spec.file());
        let service_group = ServiceGroup::new(&pkg.name, &spec.group, organization)?;
        let config_root = Self::config_root(&pkg, spec.config_from.as_ref());
        let hooks_root = Self::hooks_root(&pkg, spec.config_from.as_ref());
        let cfg = Cfg::new(&pkg, spec.config_from.as_ref())?;
        let mut service =
            Service { spec,
                      sys,
                      cfg,
                      config_renderer: CfgRenderer::new(config_root)?,
                      health_check_result: Arc::new(Mutex::new(HealthCheckResult::Unknown)),
                      hooks: HookTable::load(&pkg.name,
                                             hooks_root,
                                             svc_hooks_path(service_group.service()),
                                             feature_flags),
                      last_election_status: ElectionStatus::None,
                      user_config_updated: false,
                      initialization_state:
                          Arc::new(RwLock::new(InitializationState::Uninitialized)),
                      manager_fs_cfg,
                      supervisor: Arc::new(Mutex::new(Supervisor::new(&service_group,
                                                                      pid_source))),
                      pkg,
                      service_group,
                      all_pkg_binds,
                      unsatisfied_binds: HashSet::new(),
                      spec_file,
                      gateway_state,
                      health_check_handle: None,
                      post_run_handle: None,
                      initialize_handle: None };

        // Update the service gossip from census data.
        // We do this to ensure that the data rendered out via the HTTP API through the ServiceProxy
        // class always reflects the service configuration data stored within the gossip
        // store.
        if let Some(census_group) = census_ring.read().census_group_for(&service.service_group) {
            service.update_gossip(census_group);
        }
        Ok(service)
    }

    // And now prepare yourself for a little horribleness...Ready?
    // In releases 0.88.0 and prior, we would run hooks under
    // the hab user account on windows if it existed and no other
    // svc_user was specified just like we do on linux. That is problematic
    // and not a ubiquitous pattern for windows. The default user is now
    // always the current user. However, packages built on those older
    // versions included a SVC_USER metafile with the 'hab' user by default.
    // So to protect for scenarios where a user has launched an older package,
    // is on windows and has a 'hab' account on the system BUT never intended
    // to run hooks under that account and therefore has not passed a
    // '--password' argument to 'hab svc load', we will revert the user to
    // the current user.
    #[cfg(windows)]
    async fn resolve_pkg(package: &PackageInstall, spec: &ServiceSpec) -> Result<Pkg> {
        let mut pkg = Pkg::from_install(package).await?;
        if spec.svc_encrypted_password.is_none() && pkg.svc_user == DEFAULT_USER {
            if let Some(user) = users::get_current_username()? {
                pkg.svc_user = user;
            }
        }
        Ok(pkg)
    }

    #[cfg(unix)]
    async fn resolve_pkg(package: &PackageInstall, _spec: &ServiceSpec) -> Result<Pkg> {
        Ok(Pkg::from_install(package).await?)
    }

    /// Returns the config root given the package and optional config-from path.
    fn config_root(package: &Pkg, config_from: Option<&PathBuf>) -> PathBuf {
        config_from.map(PathBuf::as_path)
                   .unwrap_or(&package.path)
                   .join("config")
    }

    /// Returns the hooks root given the package and optional config-from path.
    fn hooks_root(package: &Pkg, config_from: Option<&PathBuf>) -> PathBuf {
        config_from.map(PathBuf::as_path)
                   .unwrap_or(&package.path)
                   .join("hooks")
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn new(sys: Arc<Sys>,
                     spec: ServiceSpec,
                     manager_fs_cfg: Arc<FsCfg>,
                     organization: Option<&str>,
                     census_ring: Arc<RwLock<CensusRing>>,
                     gateway_state: Arc<GatewayState>,
                     pid_source: ServicePidSource,
                     feature_flags: FeatureFlag)
                     -> Result<Service> {
        // The package for a spec should already be installed.
        let fs_root_path = Path::new(&*FS_ROOT_PATH);
        let package = PackageInstall::load(&spec.ident, Some(fs_root_path))?;
        Self::with_package(sys,
                           &package,
                           spec,
                           manager_fs_cfg,
                           organization,
                           census_ring,
                           gateway_state,
                           pid_source,
                           feature_flags).await
    }

    /// Create the service path for this package.
    pub fn create_svc_path(&self) -> Result<()> {
        debug!("{}, Creating svc paths", self.service_group);
        SvcDir::new(&self.pkg.name, &self.pkg.svc_user, &self.pkg.svc_group).create()?;
        Ok(())
    }

    fn start(&mut self, launcher: &LauncherCli) {
        debug!("Starting service {}", self.pkg.ident);
        let result = self.supervisor
                         .lock()
                         .expect("Couldn't lock supervisor")
                         .start(&self.pkg,
                                &self.service_group,
                                launcher,
                                self.spec.svc_encrypted_password.as_deref());
        match result {
            Ok(_) => {
                self.start_health_checks();
            }
            Err(e) => {
                outputln!(preamble self.service_group, "Service start failed: {}", e);
            }
        }
    }

    fn initialized(&self) -> bool {
        *self.initialization_state.read() == InitializationState::Initialized
    }

    /// Initiate an endless task that performs periodic health checks for the service and takes
    /// appropriate actions upon receiving the results of a health check. The actions taken are:
    ///
    /// * Cache the health check result for this service
    /// * Set the health check result for this service in the gateway state
    /// * Send a `HealthCheckEvent` over the event stream
    fn start_health_checks(&mut self) {
        debug!("Starting health checks for {}", self.pkg.ident);
        let mut rx = health::check_repeatedly(Arc::clone(&self.supervisor),
                                              self.hooks.health_check.clone(),
                                              self.spec.health_check_interval,
                                              self.service_group.clone(),
                                              self.pkg.clone(),
                                              self.spec.svc_encrypted_password.clone());

        let service_group = self.service_group.clone();
        let service_event_metadata = self.to_service_metadata();
        let service_health_result = Arc::clone(&self.health_check_result);
        let gateway_state = Arc::clone(&self.gateway_state);
        let f = async move {
            while let Some(HealthCheckBundle { status,
                                               result,
                                               interval, }) = rx.recv().await
            {
                debug!("Caching HealthCheckResult = '{}' for '{}'",
                       result, service_group);
                *service_health_result.lock()
                                      .expect("Could not unlock service_health_result") = result;

                gateway_state.lock_gsw()
                             .get_services_data_mut()
                             .iter_mut()
                             .for_each(|service| {
                                 if service.service_group == service_group {
                                     service.health_check = result;
                                 }
                             });

                event::health_check(service_event_metadata.clone(), result, status, interval);
            }
        };
        let (f, handle) = future::abortable(f);
        self.health_check_handle = Some(handle);
        tokio::spawn(f);
    }

    /// Stop the endless future that performs health checks for the
    /// service.
    fn stop_health_checks(&mut self) {
        if let Some(h) = self.health_check_handle.take() {
            debug!("Stopping health checks for {}", self.pkg.ident);
            h.abort();
        }
    }

    /// Any currently-running health check future will be terminated
    /// and a new one started in its place.
    ///
    /// This is mainly good for "resetting" the checks, and will
    /// initiate a new health check immediately.
    fn restart_health_checks(&mut self) {
        debug!("Restarting health checks for {}", self.pkg.ident);
        self.stop_health_checks();
        self.start_health_checks();
    }

    /// Called when the Supervisor reattaches itself to an already
    /// running service. Use this to re-initiate any associated
    /// processes, futures, etc.
    ///
    /// This should generally be the opposite of `Service::detach`.
    fn reattach(&mut self) {
        outputln!("Reattaching to {}", self.service_group);
        *self.initialization_state.write() = InitializationState::Initialized;
        self.restart_health_checks();
        // We intentionally do not restart the `post_run` retry future. Currently, there is not
        // a way to track if `post_run` ran successfully following a Supervisor restart.
        // See https://github.com/habitat-sh/habitat/issues/6739
    }

    /// Called when stopping the Supervisor for an update and
    /// before stopping a service. Should *not* stop the service
    /// process itself, but should stop any associated processes,
    /// futures, etc., that would otherwise prevent the Supervisor
    /// from shutting itself down.
    ///
    /// Currently, this means stopping any associated long-running
    /// futures.
    ///
    /// See also `Service::reattach`, as these methods should
    /// generally be mirror images of each other.
    pub fn detach(&mut self) {
        debug!("Detaching service {}", self.pkg.ident);
        self.stop_initialize();
        self.stop_post_run();
        self.stop_health_checks();
    }

    /// Return a future that will shut down a service, performing any
    /// necessary cleanup, and run its post-stop hook, if any.
    /// # Locking for the returned Future (see locking.md)
    /// * `GatewayState::inner` (write)
    pub async fn stop_gsw(&mut self, shutdown_config: ShutdownConfig) {
        debug!("Stopping service {}", self.pkg.ident);
        self.detach();

        let service_group = self.service_group.clone();

        self.supervisor
            .lock()
            .expect("Couldn't lock supervisor")
            .stop(shutdown_config);

        if let Some(hook) = self.post_stop()
           && let Err(e) = hook.into_future().await
        {
            outputln!(preamble service_group, "Service stop failed: {}", e);
        }
    }

    /// Only used as a way to see if anything has happened to this
    /// service since the last time we might have checked
    pub fn last_state_change(&self) -> SystemTime {
        self.supervisor
            .lock()
            .expect("Couldn't lock supervisor")
            .state_entered()
    }

    /// Performs updates and executes hooks.
    ///
    /// Returns `true` if the service was marked to be restarted or reconfigured.
    fn tick(&mut self,
            run_state: &mut ServiceRunState,
            census_ring: &CensusRing,
            launcher: &LauncherCli)
            -> bool {
        // We may need to block the service from starting until all
        // its binds are satisfied
        if !self.initialized() {
            match self.spec.binding_mode {
                BindingMode::Relaxed => (),
                BindingMode::Strict => {
                    self.validate_binds(census_ring);
                    if !self.unsatisfied_binds.is_empty() {
                        outputln!(preamble self.service_group, "Waiting for service binds...");
                        return false;
                    }
                }
            }
        }

        // Binds may become unsatisfied as a service is running (e.g.,
        // service members disappear, etc.) This can affect the data
        // we pass to templates, so we must account for it here.
        if census_ring.changed() {
            self.validate_binds(census_ring);
        }

        // TODO (DM): As a temporary fix, we return this `template_data_changed` boolean which does
        // not account for changes in the census ring. This is needed because when we restart a
        // service, we do not correctly produce the initial gossip message.
        let (template_data_changed, template_update) = self.update_templates(census_ring);
        if self.update_service_files(census_ring) {
            self.file_updated();
        }

        match self.spec.topology {
            Topology::Standalone => self.execute_hooks(run_state, launcher, &template_update),
            Topology::Leader => {
                let census_group =
                    census_ring.census_group_for(&self.service_group)
                               .expect("Service Group's census entry missing from list!");
                match census_group.election_status {
                    ElectionStatus::None => {
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; election hasn't started");
                            self.last_election_status = census_group.election_status;
                        }
                    }
                    ElectionStatus::ElectionInProgress => {
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; election in progress.");
                            self.last_election_status = census_group.election_status;
                        }
                    }
                    ElectionStatus::ElectionNoQuorum => {
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Waiting to execute hooks; election in progress, \
                                      and we have no quorum.");

                            self.last_election_status = census_group.election_status;
                        }
                    }
                    ElectionStatus::ElectionFinished => {
                        let leader_id = census_group.leader_id
                                                    .as_ref()
                                                    .expect("No leader with finished election");
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Executing hooks; {} is the leader",
                                      leader_id);
                            self.last_election_status = census_group.election_status;
                        }
                        self.execute_hooks(run_state, launcher, &template_update)
                    }
                }
            }
        };
        template_data_changed
    }

    /// Iterate through all the service binds, marking any that are
    /// unsatisfied in `self.unsatisfied_binds`.
    ///
    /// When starting with a "strict" binding mode, the presence of
    /// any unsatisfied binds will block service startup.
    ///
    /// Thereafter, if binds become unsatisfied during the running of
    /// the service, those binds will be removed from the rendering
    /// context, allowing services to take appropriate action.
    fn validate_binds(&mut self, census_ring: &CensusRing) {
        for bind in self.spec.binds.iter() {
            let mut bind_is_unsatisfied = true;

            match self.current_bind_status(census_ring, bind) {
                BindStatus::NotPresent => {
                    outputln!(preamble self.service_group,
                                  "The specified service group '{}' for binding '{}' is not (yet?) present \
                                   in the census data.",
                                  bind.service_group(),
                                  bind.name());
                }
                BindStatus::Empty => {
                    outputln!(preamble self.service_group,
                                  "The specified service group '{}' for binding '{}' is present in the \
                                   census, but currently has no active members.",
                                  bind.service_group(),
                                  bind.name());
                }
                BindStatus::Unsatisfied(ref unsatisfied) => {
                    outputln!(preamble self.service_group,
                                  "The group '{}' cannot satisfy the `{}` bind because it does not export \
                                   the following required fields: {:?}",
                                  bind.service_group(),
                                  bind.name(),
                                  unsatisfied);
                }
                BindStatus::Satisfied => {
                    // Since this function is currently called any
                    // time the census changes, and this is the
                    // expected steady-state of a properly running
                    // service, we won't log anything here. Otherwise
                    // we'd just spam the logs. Instead, log only on a
                    // state change (see below).
                    bind_is_unsatisfied = false;
                }
                BindStatus::Unknown(ref e) => {
                    outputln!(preamble self.service_group,
                                  "Error validating bind for {}=>{}: {}",
                                  bind.name(),
                                  bind.service_group(),
                                  e);
                }
            };

            if bind_is_unsatisfied {
                // TODO (CM): use Entry API to clone only when necessary
                self.unsatisfied_binds.insert((bind).clone())
            } else if self.unsatisfied_binds.remove(bind) {
                // We'll log if the bind was previously
                // unsatisfied, but now it is satisfied.
                outputln!(preamble self.service_group,
                              "The group '{}' satisfies the `{}` bind",
                              bind.service_group(),
                              bind.name());
                true
            } else {
                false
            };
        }
    }

    /// Evaluate the suitability of the given `ServiceBind` based on
    /// current census information.
    fn current_bind_status<'a>(&'a self,
                               census_ring: &'a CensusRing,
                               service_bind: &'a ServiceBind)
                               -> BindStatus<'a> {
        match census_ring.census_group_for(service_bind.service_group()) {
            None => BindStatus::NotPresent,
            Some(group) => {
                if group.active_members().count() == 0 {
                    BindStatus::Empty
                } else {
                    match self.unsatisfied_bind_exports(group, service_bind.name()) {
                        Ok(unsatisfied) => {
                            if unsatisfied.is_empty() {
                                BindStatus::Satisfied
                            } else {
                                BindStatus::Unsatisfied(unsatisfied)
                            }
                        }
                        Err(e) => BindStatus::Unknown(e),
                    }
                }
            }
        }
    }

    /// Does the service we've bound to actually satisfy the bind's
    /// contract (i.e., does it export everything we need)?
    ///
    /// Returns the set of unsatisfied exports. If everything is
    /// present, though, you get an empty set.
    ///
    /// Can return `Error::NoSuchBind` if there's not a bind with the
    /// given name.
    /// Can return `Error::NoActiveMembers` if there are no active members
    /// of the group.
    fn unsatisfied_bind_exports<'a>(&'a self,
                                    group: &'a CensusGroup,
                                    bind_name: &'a str)
                                    -> Result<HashSet<&'a String>> {
        let exports = self.exports_required_for_bind(bind_name)?;
        let group_exports = group.group_exports()?;

        let diff: HashSet<&String> = exports
            .difference(&group_exports)
            .cloned() // &&String -> &String
            .collect();

        Ok(diff)
    }

    /// Returns the list of exported values a given bind requires
    ///
    /// Returns Err if there is no bind by the given name... by the
    /// time we get to this code, though, that shouldn't happen.
    fn exports_required_for_bind<'a>(&'a self, binding_name: &str) -> Result<HashSet<&'a String>> {
        // TODO (CM): Really, we want a HashMap of name => HashSet instead of a
        // Vec<Bind>... this finding is for the birds
        self.all_pkg_binds
            .iter()
            .find(|b| b.service == binding_name)
            .ok_or_else(|| Error::NoSuchBind(binding_name.to_string()))
            .map(|b| b.exports.iter().collect())
    }

    /// Updates the process state of the service's supervisor
    fn update_process_state(&mut self, launcher: &LauncherCli) -> PidUpdate {
        self.supervisor
            .lock()
            .expect("Couldn't lock supervisor")
            .update_process_state(launcher)
    }

    /// Updates the service configuration with data from a census group if the census group has
    /// newer data than the current configuration.
    ///
    /// Returns `true` if the configuration was updated.
    fn update_gossip(&mut self, census_group: &CensusGroup) -> bool {
        match census_group.service_config {
            Some(ref config) => {
                if config.incarnation <= self.cfg.gossip_incarnation {
                    return false;
                }
                self.cfg
                    .set_gossip(config.incarnation, config.value.clone());
                true
            }
            None => false,
        }
    }

    /// Compares the current state of the service to the current state of the census ring and the
    /// user-config, and re-renders all templatable content to disk.
    fn update_templates(&mut self, census_ring: &CensusRing) -> (bool, TemplateUpdate) {
        let census_group =
            census_ring.census_group_for(&self.service_group)
                       .expect("Service update failed; unable to find own service group");
        let cfg_updated_from_rumors = self.update_gossip(census_group);
        let template_data_changed = cfg_updated_from_rumors || self.user_config_updated;

        if self.user_config_updated {
            if let Err(e) = self.cfg.reload_user() {
                outputln!(preamble self.service_group, "Reloading user-config failed: {}", e);
            }

            self.user_config_updated = false;
        }

        let template_update = if template_data_changed || census_ring.changed() {
            let ctx = self.render_context(census_ring);
            TemplateUpdate::new(self.compile_hooks(&ctx),
                                self.compile_configuration(&ctx),
                                self.hooks.reconfigure.is_some())
        } else {
            TemplateUpdate::default()
        };
        (template_data_changed, template_update)
    }

    pub fn to_rumor(&self, incarnation: u64, pkg_incarnation: u64) -> ServiceRumor {
        let exported = match self.cfg.to_exported(&self.pkg) {
            Ok(exported) => Some(exported),
            Err(err) => {
                outputln!(preamble self.service_group,
                          "Failed to generate exported cfg for service rumor: {}",
                           err);
                None
            }
        };
        let mut rumor = ServiceRumor::new(self.sys.member_id.as_str(),
                                          &self.pkg.ident,
                                          self.service_group.clone(),
                                          self.sys.as_sys_info(),
                                          exported);
        rumor.incarnation = incarnation;
        rumor.pkg_incarnation = pkg_incarnation;
        rumor
    }

    /// Run initialization hook if present.
    fn initialize(&mut self) {
        outputln!(preamble self.service_group, "Initializing");
        *self.initialization_state.write() = InitializationState::Initializing;
        if let Some(ref hook) = self.hooks.init {
            let hook_runner = HookRunner::new(Arc::clone(hook),
                                              self.service_group.clone(),
                                              self.pkg.clone(),
                                              self.spec.svc_encrypted_password.clone());
            // These clones are unfortunate. async/await will make this much better.
            let service_group = self.service_group.clone();
            let initialization_state = Arc::clone(&self.initialization_state);
            let initialization_state_for_err = Arc::clone(&self.initialization_state);
            let f = async move {
                match hook_runner.into_future().await {
                    Ok((exit_value, _)) => {
                        *initialization_state.write() = if exit_value {
                            InitializationState::InitializerFinished
                        } else {
                            InitializationState::InitializerFailed(SystemTime::now())
                        };
                    }
                    Err(e) => {
                        outputln!(preamble service_group, "Service initialization failed: {}", e);
                        *initialization_state_for_err.write() =
                            InitializationState::InitializerFailed(SystemTime::now());
                    }
                }
            };
            let (f, handle) = future::abortable(f);
            self.initialize_handle = Some(handle);
            tokio::spawn(f);
        } else {
            *self.initialization_state.write() = InitializationState::InitializerFinished;
        }
    }

    fn stop_initialize(&mut self) {
        if let Some(h) = self.initialize_handle.take() {
            h.abort();
        }
    }

    /// Run reconfigure hook if present.
    fn reconfigure(&mut self) {
        let _timer = hook_timer("reconfigure");

        if let Some(ref hook) = self.hooks.reconfigure {
            hook.run(&self.service_group,
                     &self.pkg,
                     self.spec.svc_encrypted_password.as_ref())
                .ok();
            // The intention here is to do a health check soon after a service's configuration
            // changes, as a way to (among other things) detect potential impacts when bound
            // services change exported configuration.
            self.restart_health_checks();
        }
    }

    fn post_run(&mut self) {
        if let Some(ref hook) = self.hooks.post_run {
            let hook_runner = HookRunner::new(Arc::clone(hook),
                                              self.service_group.clone(),
                                              self.pkg.clone(),
                                              self.spec.svc_encrypted_password.clone());
            let f = HookRunner::retryable_future(hook_runner);
            let (f, handle) = future::abortable(f);
            self.post_run_handle = Some(handle);
            tokio::spawn(f);
        }
    }

    /// Stop the `post-run` retry future. This will stop this retry loop regardless of `post-run`'s
    /// exit code.
    fn stop_post_run(&mut self) {
        if let Some(h) = self.post_run_handle.take() {
            h.abort();
        }
    }

    fn post_stop(&self) -> Option<HookRunner<hooks::PostStopHook>> {
        self.hooks.post_stop.as_ref().map(|hook| {
                                         HookRunner::new(Arc::clone(hook),
                                                         self.service_group.clone(),
                                                         self.pkg.clone(),
                                                         self.spec.svc_encrypted_password.clone())
                                     })
    }

    pub fn suitability(&self) -> Option<u64> {
        let _timer = hook_timer("suitability");

        if !self.initialized() {
            return None;
        }

        self.hooks
            .suitability
            .as_ref()
            .and_then(|hook| {
                hook.run(&self.service_group,
                         &self.pkg,
                         self.spec.svc_encrypted_password.as_ref())
                    .ok()
            })
            .unwrap_or(None)
    }

    /// Helper for compiling configuration templates into configuration files.
    ///
    /// Returns `true` if the configuration has changed.
    fn compile_configuration(&self, ctx: &RenderContext) -> bool {
        match self.config_renderer.compile(&ctx.service_group_name(),
                                           &self.pkg,
                                           &self.pkg.svc_config_path,
                                           ctx)
        {
            Ok(true) => true,
            Ok(false) => false,
            Err(e) => {
                outputln!(preamble self.service_group,
                          "Failed to compile configuration: {}",
                          e);
                false
            }
        }
    }

    /// Helper for compiling hook templates into hooks.
    ///
    /// This function will also perform any necessary post-compilation tasks.
    fn compile_hooks(&self, ctx: &RenderContext<'_>) -> HookCompileTable {
        let hook_update_table = self.hooks.compile(&self.service_group, ctx);
        if let Some(err) = self.copy_run().err() {
            outputln!(preamble self.service_group, "Failed to copy run hook: {}", err);
        }
        if hook_update_table.changed() {
            outputln!(preamble self.service_group, "Hooks recompiled");
        }
        hook_update_table
    }

    // Copy the "run" file to the svc path.
    fn copy_run(&self) -> Result<()> {
        let svc_run = self.pkg.svc_path.join(hooks::RunHook::FILE_NAME);
        match self.hooks.run {
            Some(ref hook) => {
                fs::copy(hook.path(), &svc_run)?;
                Self::set_hook_permissions(svc_run.to_str().unwrap())?;
            }
            None => {
                let run = self.pkg.path.join(hooks::RunHook::FILE_NAME);
                match fs::metadata(&run) {
                    Ok(_) => {
                        fs::copy(&run, &svc_run)?;
                        Self::set_hook_permissions(&svc_run)?;
                    }
                    Err(err) => {
                        outputln!(preamble self.service_group, "Error finding run file: {}", err);
                    }
                }
            }
        }
        Ok(())
    }

    #[cfg(not(windows))]
    fn set_hook_permissions<T: AsRef<Path>>(path: T) -> habitat_core::error::Result<()> {
        use habitat_common::templating::hooks::HOOK_PERMISSIONS;
        use habitat_core::util::posix_perm;

        posix_perm::set_permissions(path.as_ref(), HOOK_PERMISSIONS)
    }

    #[cfg(windows)]
    fn set_hook_permissions<T: AsRef<Path>>(path: T) -> habitat_core::error::Result<()> {
        use habitat_core::util::win_perm;

        win_perm::harden_path(path.as_ref())
    }

    /// Returns `true` if the service was marked to be restarted or reconfigured.
    fn execute_hooks(&mut self,
                     run_state: &mut ServiceRunState,
                     launcher: &LauncherCli,
                     template_update: &TemplateUpdate) {
        let pid_update = self.update_process_state(launcher);
        // We copy the current process id to the run state to avoid
        // having to lock the supervisor for this information.
        run_state.current_pid = pid_update.new_pid;

        // It is ok that we do not hold this lock while we are performing the match. If we
        // transistion states while we are matching, we will catch the new state on the next tick.
        let initialization_state = self.initialization_state.read().clone();
        match initialization_state {
            InitializationState::Uninitialized => {
                // If the service is not initialized and the process is still running, the
                // Supervisor was restarted and we just have to reattach to the
                // process.
                if pid_update.is_running() {
                    self.reattach();
                } else {
                    self.initialize();
                }
            }
            InitializationState::InitializerFailed(failed_at) => {
                run_state.mark_for_restart(None,
                                           ProcessTerminationReason::InitHookFailed,
                                           failed_at);
            }
            InitializationState::Initializing => {
                // Wait until the initializer finishes running
            }
            InitializationState::InitializerFinished => {
                self.start(launcher);
                self.post_run();
                *self.initialization_state.write() = InitializationState::Initialized;
            }
            InitializationState::Initialized => {
                let restart_cooldown_period_expired =
                    run_state.restart_backoff
                             .duration_elapsed_since_last_attempt_ended()
                             .map(|duration| -> bool {
                                 duration > run_state.restart_config.cooldown_period
                             })
                             .unwrap_or(false);
                // If the service is initialized and the process is not running, the process
                // unexpectedly died and needs to be restarted.
                if !pid_update.is_running() {
                    run_state.mark_for_restart(pid_update.old_pid,
                                               ProcessTerminationReason::RunHookFailed,
                                               pid_update.timestamp.expect("Process update time \
                                                                            should be present"));
                } else if let Some(termination_reason) = template_update.needs_restart() {
                    run_state.mark_for_immediate_restart(pid_update.new_pid,
                                                         termination_reason,
                                                         SystemTime::now());
                } else if template_update.needs_reconfigure() {
                    // Only reconfigure if we did NOT restart the service
                    self.reconfigure();
                } else if run_state.restart_state == RestartState::Restarted
                          && pid_update.is_running()
                          && restart_cooldown_period_expired
                {
                    // If the service was not restarted for the duration of the cooldown period
                    // since the last attempt ended we wipe all state associated with restarting.
                    run_state.reset_backoff();
                }
            }
        };
    }

    /// Run file-updated hook if present.
    fn file_updated(&self) -> bool {
        let _timer = hook_timer("file-updated");

        if self.initialized()
           && let Some(ref hook) = self.hooks.file_updated
        {
            return hook.run(&self.service_group,
                            &self.pkg,
                            self.spec.svc_encrypted_password.as_ref())
                       .unwrap_or(false);
        }

        false
    }

    /// Writes out all service files for a service.
    ///
    /// Must be called before a loaded service starts (even before any
    /// init hook, since the operation of the hook may depend on the
    /// presence of service files).
    ///
    /// Doesn't return a boolean (cf. `update_service_files` below)
    /// because we don't particularly care in this case.
    pub fn write_initial_service_files(&mut self, census_ring: &CensusRing) {
        // In this case, a service group not being found is fine; this
        // may be a non-peered Supervisor running this service for the
        // first time, for instance.
        if let Some(census_group) = census_ring.census_group_for(&self.service_group) {
            self.write_service_files(census_group, CensusGroup::service_files);
        }
    }

    /// Write service files from gossip data to disk under
    /// [`svc_files_path()`](../../fs/fn.svc_files_path.html).
    ///
    /// Returns `true` if a file was changed, added, or removed, and
    /// `false` if there were no updates.
    fn update_service_files(&mut self, census_ring: &CensusRing) -> bool {
        let census_group =
            census_ring.census_group_for(&self.service_group)
                       .expect("Service update service files failed; unable to find own service \
                                group");
        self.write_service_files(census_group, CensusGroup::changed_service_files)
    }

    /// Abstracts the logic for writing out service files for a
    /// service.
    ///
    /// The key bit here is `file_fn`, which returns the list of files
    /// to write out. In practice, this will be either
    /// `CensusGroup::service_files`, to write _all_ files to disk, or
    /// `CensusGroup::changed_service_files`, to write out only the
    /// files that have had recent gossip activity.
    ///
    /// Returns `true` if any service files were written to disk.
    fn write_service_files<'a, F, I>(&mut self, census_group: &'a CensusGroup, file_fn: F) -> bool
        where F: Fn(&'a CensusGroup) -> I,
              I: IntoIterator<Item = &'a ServiceFile>
    {
        let mut updated = false;
        for service_file in file_fn(census_group) {
            if self.cache_service_file(service_file) {
                outputln!(preamble self.service_group, "Service file updated, {}",
                          service_file.filename);
                updated = true;
            }
        }
        updated
    }

    /// Helper for constructing a new render context for the service.
    fn render_context<'a>(&'a self, census: &'a CensusRing) -> RenderContext<'a> {
        // Unsatisfied binds are filtered out; you only get bind
        // information in the render context if they actually satisfy
        // the contract!
        RenderContext::new(&self.service_group,
                           &self.sys,
                           &self.pkg,
                           &self.cfg,
                           census,
                           self.spec
                               .binds
                               .iter()
                               .filter(|b| !self.unsatisfied_binds.contains(b)))
    }

    // Returns `false` if the write fails.
    fn cache_service_file(&mut self, service_file: &ServiceFile) -> bool {
        let file = self.pkg.svc_files_path.join(&service_file.filename);
        self.write_cache_file(file, &service_file.body)
    }

    // Returns `false` if the write fails.
    fn write_cache_file<T>(&self, file: T, contents: &[u8]) -> bool
        where T: AsRef<Path>
    {
        let current_checksum = match Blake2bHash::from_file(&file) {
            Ok(current_checksum) => Some(current_checksum),
            Err(err) => {
                outputln!(preamble self.service_group, "Failed to get current checksum for {}, {}",
                       file.as_ref().display(),
                       err);
                None
            }
        };
        let new_checksum = Blake2bHash::from_bytes(contents);

        if let Some(current_checksum) = current_checksum
           && new_checksum == current_checksum
        {
            return false;
        }

        if let Err(e) = atomic_write(file.as_ref(), contents) {
            outputln!(preamble self.service_group,
                      "Failed to write to cache file {}, {}",
                      file.as_ref().display(), e);
            return false;
        }

        self.set_gossip_permissions(&file)
    }

    #[cfg(not(windows))]
    fn set_gossip_permissions<T: AsRef<Path>>(&self, path: T) -> bool {
        use habitat_core::{os::process,
                           util::posix_perm};

        if process::can_run_services_as_svc_user() {
            let result =
                posix_perm::set_owner(path.as_ref(), &self.pkg.svc_user, &self.pkg.svc_group);
            if let Err(e) = result {
                outputln!(preamble self.service_group,
                          "Failed to set ownership of cache file {}, {}",
                          path.as_ref().display(), e);
                return false;
            }
        }

        if let Err(e) = posix_perm::set_permissions(path.as_ref(), GOSSIP_FILE_PERMISSIONS) {
            outputln!(preamble self.service_group,
                      "Failed to set permissions on cache file {}, {}",
                      path.as_ref().display(), e);
            return false;
        }
        true
    }

    #[cfg(windows)]
    fn set_gossip_permissions<T: AsRef<Path>>(&self, path: T) -> bool {
        use habitat_core::util::win_perm;

        if let Err(e) = win_perm::harden_path(path.as_ref()) {
            outputln!(preamble self.service_group,
                      "Failed to set permissions on cache file {}, {}",
                      path.as_ref().display(), e);
            return false;
        }
        true
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.service_group, self.pkg.ident)
    }
}

// This returns a HistogramTimer that we can use to track how long hooks take to execute. Note that
// times will get tracked automatically when the HistogramTimer goes out of scope.
fn hook_timer(name: &str) -> HistogramTimer {
    HOOK_DURATION.with_label_values(&[name]).start_timer()
}

/// This enum represents whether or not we want to render config information when we serialize this
/// service via the ServiceProxy struct below. Choosing ConfigRendering::Full will render the
/// config, and choosing ConfigRendering::Redacted will not render it. This matches up to the
/// feature flag we have in place to redact config information from a service's serialized output,
/// which shows up in the supervisor's HTTP API responses.
///
/// Please note that this enum derives the Copy trait, so that it behaves more like the boolean
/// that it is, and so that we don't have to clone() it everywhere. Adding anything to this enum
/// that consumes a large amount of memory would be a bad idea (without removing Copy first)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigRendering {
    Full,
    Redacted,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnixTimestamp(u64);

impl TryFrom<SystemTime> for UnixTimestamp {
    type Error = std::time::SystemTimeError;

    fn try_from(timestamp: SystemTime) -> result::Result<Self, Self::Error> {
        Ok(UnixTimestamp(timestamp.duration_since(SystemTime::UNIX_EPOCH)?.as_secs()))
    }
}

/// This is a queryable representation of a service. It allows us to separate the
/// structure of the data returned through APIs from the actual internal representation.
/// Importantly this allows us to refactor the API and internal models without
/// worrying about breaking the data returned to users.
#[derive(Debug, Clone, Serialize)]
pub struct ServiceQueryModel {
    pub all_pkg_binds:          Vec<Bind>,
    pub binding_mode:           BindingMode,
    pub binds:                  Vec<ServiceBind>,
    pub bldr_url:               String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg:                    Option<Cfg>,
    pub channel:                ChannelIdent,
    pub config_from:            Option<PathBuf>,
    pub desired_state:          DesiredState,
    pub health_check:           HealthCheckResult,
    pub hooks:                  HookTableQueryModel,
    pub initialized:            bool,
    pub last_election_status:   ElectionStatus,
    pub manager_fs_cfg:         Arc<FsCfg>,
    pub pkg:                    PkgQueryModel,
    pub process:                SupervisedProcessQueryModel,
    pub last_process_state:     Option<LastProcessState>,
    pub next_restart_at:        Option<UnixTimestamp>,
    pub restart_count:          u64,
    pub restart_config:         ServiceRestartConfig,
    pub service_group:          ServiceGroup,
    pub spec_file:              PathBuf,
    pub spec_ident:             PackageIdent,
    pub spec_identifier:        String,
    pub svc_encrypted_password: Option<String>,
    pub health_check_interval:  HealthCheckInterval,
    pub sys:                    Arc<Sys>,
    pub topology:               Topology,
    pub update_strategy:        UpdateStrategy,
    pub update_condition:       UpdateCondition,
    pub user_config_updated:    bool,
}

impl ServiceQueryModel {
    pub fn new(service: &Service,
               service_run_state: &ServiceRunState,
               config_rendering: ConfigRendering)
               -> Self {
        ServiceQueryModel { all_pkg_binds:          service.all_pkg_binds.clone(),
                            binding_mode:           service.spec.binding_mode,
                            binds:                  service.spec.binds.clone(),
                            bldr_url:               service.spec.bldr_url.clone(),
                            cfg:                    match config_rendering {
                                ConfigRendering::Full => Some(service.cfg.clone()),
                                ConfigRendering::Redacted => None,
                            },
                            channel:                service.spec.channel.clone(),
                            config_from:            service.spec.config_from.clone(),
                            desired_state:          service.spec.desired_state,
                            health_check:
                                (*service.health_check_result
                                         .lock()
                                         .expect("Couldn't lock health check result for \
                                                  serialization")),
                            hooks:                  HookTableQueryModel::new(&service.hooks),
                            initialized:            service.initialized(),
                            last_election_status:   service.last_election_status,
                            manager_fs_cfg:         service.manager_fs_cfg.clone(),
                            pkg:                    PkgQueryModel::new(&service.pkg),
                            process:
                                SupervisedProcessQueryModel::new(service.supervisor
                                                                        .lock()
                                                                        .expect("Couldn't lock \
                                                                                 supervisor for \
                                                                                 serialization")
                                                                        .deref()),
                            last_process_state:     service_run_state.last_process_state.clone(),
                            next_restart_at:
                                service_run_state.restart_backoff
                                                 .duration_until_next_attempt_start()
                                                 .and_then(|duration| {
                                                     SystemTime::now().checked_add(duration)
                                                 })
                                                 .and_then(|timestamp| {
                                                     UnixTimestamp::try_from(timestamp).ok()
                                                 }),
                            restart_count:          service_run_state.restart_count,
                            restart_config:         service_run_state.restart_config.clone(),
                            service_group:          service.service_group.clone(),
                            spec_file:              service.spec_file.clone(),
                            spec_ident:             service.spec.ident.clone(),
                            spec_identifier:        service.spec.ident.to_string(),
                            svc_encrypted_password: service.spec.svc_encrypted_password.clone(),
                            health_check_interval:  service.spec.health_check_interval,
                            sys:                    service.sys.clone(),
                            topology:               service.spec.topology,
                            update_strategy:        service.spec.update_strategy,
                            update_condition:       service.spec.update_condition,
                            user_config_updated:    service.user_config_updated, }
    }
}

impl From<&ServiceQueryModel> for habitat_sup_protocol::types::ServiceStatus {
    fn from(service: &ServiceQueryModel) -> Self {
        Self { ident:         (*service.pkg.ident.as_ref()).clone().into(),
               process:       Some((&service.process).into()),
               service_group: service.service_group.clone().into(),
               desired_state: Some(service.desired_state.into()), }
    }
}

#[cfg(test)]
#[cfg(any(all(target_os = "linux", target_arch = "x86_64"),
          all(target_os = "windows", target_arch = "x86_64"),))]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use habitat_common::types::{GossipListenAddr,
                                HttpListenAddr,
                                ListenCtlAddr};
    use std::{net::{IpAddr,
                    Ipv4Addr},
              str::FromStr};

    async fn initialize_test_service() -> PersistentServiceWrapper {
        let listen_ctl_addr =
            ListenCtlAddr::from_str("127.0.0.1:1234").expect("Can't parse IP into SocketAddr");
        let sys = Sys::new(false,
                           GossipListenAddr::default(),
                           listen_ctl_addr,
                           HttpListenAddr::default(),
                           IpAddr::V4(Ipv4Addr::LOCALHOST));

        let ident = if cfg!(target_os = "linux") {
            PackageIdent::new("core", "tree", Some("1.7.0"), Some("20180609045201"))
        } else if cfg!(target_os = "windows") {
            PackageIdent::new("core", "7zip", Some("16.04"), Some("20170131110814"))
        } else {
            panic!("This is being run on a platform that's not currently supported");
        };

        let spec = ServiceSpec::new(ident);

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                            .join("fixtures")
                                                            .join("pkgs");

        let install = PackageInstall::load(&spec.ident, Some(&path)).expect("PackageInstall \
                                                                             should've loaded my \
                                                                             spec, but it didn't");
        let asys = Arc::new(sys);
        let fscfg = FsCfg::new("/tmp");
        let afs = Arc::new(fscfg);
        let census_ring = Arc::new(RwLock::new(CensusRing::new(asys.member_id.clone())));
        let gs = Arc::default();
        PersistentServiceWrapper::new(Service::with_package(asys,
                              &install,
                              spec,
                              afs,
                              Some("haha"),
                              census_ring,
                              gs,
                              ServicePidSource::Launcher,
                              FeatureFlag::empty()).await
                                                   .expect("I wanted a service to load, but it \
                                                            didn't"), &ServiceRestartConfig::default())
    }

    // We only run this test case for x86 platforms as it is not worth the effort
    // to test it on other platforms as the schema of a API response is likely to be the same
    // across all non-x86_64 unix platforms.
    #[tokio::test]
    async fn service_proxy_conforms_to_the_schema() {
        let service_wrapper = initialize_test_service().await;

        // With config
        let proxy_with_config = ServiceQueryModel::new(service_wrapper.service().unwrap(),
                                                       service_wrapper.service_run_state(),
                                                       ConfigRendering::Full);
        let proxies_with_config = vec![proxy_with_config];
        let json_with_config =
            serde_json::to_string(&proxies_with_config).expect("Expected to convert \
                                                                proxies_with_config to JSON but \
                                                                failed");
        assert_valid(&json_with_config, "http_gateway_services_schema.json");

        // Without config
        let proxy_without_config = ServiceQueryModel::new(service_wrapper.service().unwrap(),
                                                          service_wrapper.service_run_state(),
                                                          ConfigRendering::Redacted);
        let proxies_without_config = vec![proxy_without_config];
        let json_without_config =
            serde_json::to_string(&proxies_without_config).expect("Expected  to convert \
                                                                   proxies_without_config to \
                                                                   JSON but failed");
        assert_valid(&json_without_config, "http_gateway_services_schema.json");
    }
}
