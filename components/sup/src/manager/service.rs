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
mod spawned_future;
pub mod spec;
mod supervisor;
mod terminator;

use self::{context::RenderContext,
           hooks::{HookChangeTable,
                   HookTable},
           supervisor::Supervisor};
pub use self::{health::HealthCheckResult,
               hooks::HealthCheckHook,
               spec::{DesiredState,
                      IntoServiceSpec,
                      ServiceSpec}};
use crate::{census::{CensusGroup,
                     CensusRing,
                     ElectionStatus,
                     ServiceFile},
            error::{Error,
                    Result},
            manager::{FsCfg,
                      GatewayState,
                      ShutdownConfig,
                      Sys},
            sup_futures};
use futures::{future,
              Future,
              IntoFuture};
use habitat_butterfly::rumor::service::Service as ServiceRumor;
pub use habitat_common::templating::{config::{Cfg,
                                              UserConfigPath},
                                     package::{Env,
                                               Pkg,
                                               PkgProxy}};
use habitat_common::{outputln,
                     templating::{config::CfgRenderer,
                                  hooks::Hook}};
use habitat_core::{crypto::hash,
                   fs::{atomic_write,
                        svc_hooks_path,
                        SvcDir,
                        FS_ROOT_PATH},
                   os::process::ShutdownTimeout,
                   package::{metadata::Bind,
                             PackageIdent,
                             PackageInstall},
                   service::{HealthCheckInterval,
                             ServiceBind,
                             ServiceGroup},
                   ChannelIdent};
use habitat_launcher_client::LauncherCli;
use habitat_sup_protocol::types::BindingMode;
pub use habitat_sup_protocol::types::{ProcessState,
                                      Topology,
                                      UpdateStrategy};
use prometheus::{HistogramTimer,
                 HistogramVec};
use serde::{ser::SerializeStruct,
            Serialize,
            Serializer};
use std::{self,
          collections::HashSet,
          fmt,
          fs,
          ops::Deref,
          path::{Path,
                 PathBuf},
          result,
          sync::{Arc,
                 Mutex,
                 RwLock}};
use time::Timespec;
use tokio::runtime::TaskExecutor;

static LOGKEY: &'static str = "SR";

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

#[derive(Debug)]
pub struct Service {
    pub service_group:       ServiceGroup,
    pub bldr_url:            String,
    pub channel:             ChannelIdent,
    pub desired_state:       DesiredState,
    pub spec_file:           PathBuf,
    pub spec_ident:          PackageIdent,
    pub topology:            Topology,
    pub update_strategy:     UpdateStrategy,
    pub cfg:                 Cfg,
    pub pkg:                 Pkg,
    pub sys:                 Arc<Sys>,
    pub initialized:         bool,
    pub user_config_updated: bool,
    pub shutdown_timeout:    Option<ShutdownTimeout>,
    pub needs_restart:       bool,

    config_renderer: CfgRenderer,
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
    health_check_result: Arc<Mutex<HealthCheckResult>>,
    last_election_status: ElectionStatus,
    /// The mapping of bind name to a service group, specified by the
    /// user when the service definition was loaded into the Supervisor.
    binds: Vec<ServiceBind>,
    /// The binds that the current service package declares, both
    /// required and optional. We don't differentiate because this is
    /// used to validate the user-specified bindings against the
    /// current state of the census; once you get into the actual
    /// running of the service, the distinction is immaterial.
    all_pkg_binds: Vec<Bind>,
    /// Controls how the presence or absence of bound service groups
    /// impacts the service's start-up.
    binding_mode: BindingMode,
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
    unsatisfied_binds: HashSet<ServiceBind>,
    hooks: HookTable,
    config_from: Option<PathBuf>,
    manager_fs_cfg: Arc<FsCfg>,
    supervisor: Arc<Mutex<Supervisor>>,
    svc_encrypted_password: Option<String>,
    health_check_interval: HealthCheckInterval,

    gateway_state: Arc<RwLock<GatewayState>>,

    /// A "handle" to the never-ending future that periodically runs
    /// health checks on this service. This is the means by which we
    /// can stop that future.
    health_check_handle: Option<sup_futures::FutureHandle>,
}

impl Service {
    fn new(sys: Arc<Sys>,
           package: &PackageInstall,
           spec: ServiceSpec,
           manager_fs_cfg: Arc<FsCfg>,
           organization: Option<&str>,
           gateway_state: Arc<RwLock<GatewayState>>)
           -> Result<Service> {
        spec.validate(&package)?;
        let all_pkg_binds = package.all_binds()?;
        let pkg = Pkg::from_install(&package)?;
        let spec_file = manager_fs_cfg.specs_path.join(spec.file_name());
        let service_group = ServiceGroup::new(spec.application_environment.as_ref(),
                                              &pkg.name,
                                              spec.group,
                                              organization)?;
        let config_root = Self::config_root(&pkg, spec.config_from.as_ref());
        let hooks_root = Self::hooks_root(&pkg, spec.config_from.as_ref());
        Ok(Service { sys,
                     cfg: Cfg::new(&pkg, spec.config_from.as_ref())?,
                     config_renderer: CfgRenderer::new(&config_root)?,
                     bldr_url: spec.bldr_url,
                     channel: spec.channel,
                     desired_state: spec.desired_state,
                     health_check_result: Default::default(),
                     hooks: HookTable::load(&pkg.name,
                                            &hooks_root,
                                            svc_hooks_path(&service_group.service())),
                     initialized: false,
                     last_election_status: ElectionStatus::None,
                     user_config_updated: false,
                     needs_restart: false,
                     manager_fs_cfg,
                     supervisor: Arc::new(Mutex::new(Supervisor::new(&service_group))),
                     pkg,
                     service_group,
                     binds: spec.binds,
                     all_pkg_binds,
                     unsatisfied_binds: HashSet::new(),
                     binding_mode: spec.binding_mode,
                     spec_ident: spec.ident,
                     spec_file,
                     topology: spec.topology,
                     update_strategy: spec.update_strategy,
                     config_from: spec.config_from,
                     svc_encrypted_password: spec.svc_encrypted_password,
                     health_check_interval: spec.health_check_interval,
                     gateway_state,
                     health_check_handle: None,
                     shutdown_timeout: spec.shutdown_timeout })
    }

    /// Returns the config root given the package and optional config-from path.
    fn config_root(package: &Pkg, config_from: Option<&PathBuf>) -> PathBuf {
        config_from.and_then(|p| Some(p.as_path()))
                   .unwrap_or(&package.path)
                   .join("config")
    }

    /// Returns the hooks root given the package and optional config-from path.
    fn hooks_root(package: &Pkg, config_from: Option<&PathBuf>) -> PathBuf {
        config_from.and_then(|p| Some(p.as_path()))
                   .unwrap_or(&package.path)
                   .join("hooks")
    }

    pub fn load(sys: Arc<Sys>,
                spec: ServiceSpec,
                manager_fs_cfg: Arc<FsCfg>,
                organization: Option<&str>,
                gateway_state: Arc<RwLock<GatewayState>>)
                -> Result<Service> {
        // The package for a spec should already be installed.
        let fs_root_path = Path::new(&*FS_ROOT_PATH);
        let package = PackageInstall::load(&spec.ident, Some(fs_root_path))?;
        Ok(Self::new(sys,
                     &package,
                     spec,
                     manager_fs_cfg,
                     organization,
                     gateway_state)?)
    }

    /// Create the service path for this package.
    pub fn create_svc_path(&self) -> Result<()> {
        debug!("{}, Creating svc paths", self.service_group);
        SvcDir::new(&self.pkg.name, &self.pkg.svc_user, &self.pkg.svc_group).create()?;
        Ok(())
    }

    fn start(&mut self, launcher: &LauncherCli, executor: &TaskExecutor) {
        if let Some(err) = self.supervisor
                               .lock()
                               .expect("Couldn't lock supervisor")
                               .start(&self.pkg,
                                      &self.service_group,
                                      launcher,
                                      self.svc_encrypted_password.as_ref().map(String::as_str))
                               .err()
        {
            outputln!(preamble self.service_group, "Service start failed: {}", err);
        } else {
            self.needs_restart = false;
        }

        self.start_health_checks(executor);
    }

    /// Create the state necessary for managing a repeatedly-running
    /// health check hook.
    fn health_state(&self) -> health::State {
        health::State::new(self.hooks.health_check.clone(),
                           self.service_group.clone(),
                           self.pkg.clone(),
                           self.svc_encrypted_password.clone(),
                           self.to_service_metadata(),
                           Arc::clone(&self.supervisor),
                           self.health_check_interval,
                           Arc::clone(&self.health_check_result),
                           Arc::clone(&self.gateway_state))
    }

    /// Initiate an endless future that performs periodic health
    /// checks for the service
    fn start_health_checks(&mut self, executor: &TaskExecutor) {
        let (handle, f) = sup_futures::cancelable_future(self.health_state().check_repeatedly());

        self.health_check_handle = Some(handle);

        let service_group_copy = self.service_group.clone();
        executor.spawn(f.map_err(move |err| {
                                                   if err.is_some() {
                                                       warn!("Health checking for {} \
                                                              unexpectedly errored!",
                                                             service_group_copy);
                                                   } else {
                                                       outputln!(preamble service_group_copy,
                                                                 "Health checking has been stopped");
                                                   }
                                               }));
    }

    /// Stop the endless future that performs health checks for the
    /// service.
    ///
    /// Consumes the handle to that future in the process.
    fn stop_health_checks(&mut self) {
        if let Some(h) = self.health_check_handle.take() {
            debug!("Stopping health checks for {}", self.pkg.ident);
            h.terminate();
        }
    }

    /// Any currently-running health check future will be terminated
    /// and a new one started in its place.
    ///
    /// This is mainly good for "resetting" the checks, and will
    /// initiate a new health check immediately.
    fn restart_health_checks(&mut self, executor: &TaskExecutor) {
        self.stop_health_checks();
        self.start_health_checks(executor);
    }

    /// Called when the Supervisor reattaches itself to an already
    /// running service. Use this to re-initiate any associated
    /// processes, futures, etc.
    ///
    /// This should generally be the opposite of `Service::detach`.
    fn reattach(&mut self, executor: &TaskExecutor) {
        outputln!("Reattaching to {}", self.service_group);
        self.initialized = true;
        self.restart_health_checks(executor);
    }

    /// Called when stopping the Supervisor for an update. Should
    /// *not* stop the service process itself, but should stop any
    /// associated processes, futures, etc., that would otherwise
    /// prevent the Supervisor from shutting itself down.
    ///
    /// Currently, this means stopping any associated long-running
    /// futures.
    ///
    /// See also `Service::reattach`, as these methods should
    /// generally be mirror images of each other.
    pub fn detach(&mut self) { self.stop_health_checks(); }

    /// Return a future that will shut down a service, performing any
    /// necessary cleanup, and run its post-stop hook, if any.
    pub fn stop(&mut self,
                shutdown_config: ShutdownConfig)
                -> impl Future<Item = (), Error = Error> {
        self.stop_health_checks();

        let service_group = self.service_group.clone();
        let gs = Arc::clone(&self.gateway_state);

        let f = self.supervisor
                    .lock()
                    .expect("Couldn't lock supervisor")
                    .stop(shutdown_config)
                    .and_then(move |_| {
                        gs.write()
                          .expect("GatewayState lock is poisoned")
                          .health_check_data
                          .remove(&service_group);
                        Ok(())
                    });

        // eww
        let service_group_2 = self.service_group.clone();
        match self.post_stop() {
            None => future::Either::A(f),
            Some(hook) => {
                future::Either::B(f.and_then(|_| hook.into_future().map(|_exitvalue| ())))
            }
        }.map_err(move |e| {
             outputln!(preamble service_group_2, "Service stop failed: {}", e);
             e
         })
    }

    pub fn last_state_change(&self) -> Timespec {
        self.supervisor
            .lock()
            .expect("Couldn't lock supervisor")
            .state_entered
    }

    /// Performs updates and executes hooks.
    ///
    /// Returns `true` if the service was updated.
    pub fn tick(&mut self,
                census_ring: &CensusRing,
                launcher: &LauncherCli,
                executor: &TaskExecutor)
                -> bool {
        // We may need to block the service from starting until all
        // its binds are satisfied
        if !self.initialized {
            match self.binding_mode {
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

        let (svc_updated, hook_update_table, config_change) = self.update_templates(census_ring);
        if self.update_service_files(census_ring) {
            self.file_updated();
        }

        match self.topology {
            Topology::Standalone => {
                self.execute_hooks(launcher, executor, hook_update_table, config_change);
            }
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

                            self.last_election_status = census_group.election_status
                        }
                    }
                    ElectionStatus::ElectionFinished => {
                        let leader_id = census_group.leader_id
                                                    .as_ref()
                                                    .expect("No leader with finished election");
                        if self.last_election_status != census_group.election_status {
                            outputln!(preamble self.service_group,
                                      "Executing hooks; {} is the leader",
                                      leader_id.to_string());
                            self.last_election_status = census_group.election_status;
                        }
                        self.execute_hooks(launcher, executor, hook_update_table, config_change)
                    }
                }
            }
        }
        if svc_updated {
            // The intention here is to do a health check soon after a
            // service's configuration changes, as a way to (among
            // other things) detect potential impacts when bound
            // services change exported configuration.
            //
            // TODO (CM): Rather than restarting health checks
            // immediately (and correspondingly, immediately after a
            // service starts for the first time), should we instead
            // have an initial delay of X seconds, and _then_ start
            // the "normal" sequence of checks?
            self.restart_health_checks(executor);
        }

        svc_updated
    }

    pub fn to_spec(&self) -> ServiceSpec {
        let mut spec = ServiceSpec::default_for(self.spec_ident.clone());
        spec.group = self.service_group.group().to_string();
        if let Some(appenv) = self.service_group.application_environment() {
            spec.application_environment = Some(appenv)
        }
        spec.bldr_url = self.bldr_url.clone();
        spec.channel = self.channel.clone();
        spec.topology = self.topology;
        spec.update_strategy = self.update_strategy;
        spec.binds = self.binds.clone();
        spec.binding_mode = self.binding_mode;
        spec.config_from = self.config_from.clone();
        if let Some(ref password) = self.svc_encrypted_password {
            spec.svc_encrypted_password = Some(password.clone())
        }
        spec.health_check_interval = self.health_check_interval;
        spec.shutdown_timeout = self.shutdown_timeout;
        spec
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
        for bind in self.binds.iter() {
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
    fn check_process(&mut self) -> bool {
        self.supervisor
            .lock()
            .expect("Couldn't lock supervisor")
            .check_process()
    }

    fn process_down(&self) -> bool {
        self.supervisor
            .lock()
            .expect("Couldn't lock supervisor")
            .state
        == ProcessState::Down
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
    ///
    /// TODO (DM): Comment the return type
    fn update_templates(&mut self, census_ring: &CensusRing) -> (bool, HookChangeTable, bool) {
        let census_group =
            census_ring.census_group_for(&self.service_group)
                       .expect("Service update failed; unable to find own service group");
        let cfg_updated_from_rumors = self.update_gossip(census_group);
        let cfg_changed = cfg_updated_from_rumors || self.user_config_updated;

        if self.user_config_updated {
            if let Err(e) = self.cfg.reload_user() {
                outputln!(preamble self.service_group, "Reloading user-config failed: {}", e);
            }

            self.user_config_updated = false;
        }

        // TODO (DM): Do we need to return cfg_changed? What does it actually indicate?
        // Can we just check if HookChangeTable and config_changed?
        if cfg_changed || census_ring.changed() {
            let ctx = self.render_context(census_ring);
            (cfg_changed, self.compile_hooks(&ctx), self.compile_configuration(&ctx))
        } else {
            (cfg_changed, HookChangeTable::new(), false)
        }
    }

    pub fn to_rumor(&self, incarnation: u64) -> ServiceRumor {
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
                                          self.sys.as_sys_info().clone(),
                                          exported);
        rumor.incarnation = incarnation;
        rumor
    }

    /// Run initialization hook if present.
    fn initialize(&mut self) {
        let timer = hook_timer("initialize");

        if self.initialized {
            timer.observe_duration();
            return;
        }

        outputln!(preamble self.service_group, "Initializing");
        self.initialized = true;
        if let Some(ref hook) = self.hooks.init {
            self.initialized = hook.run(&self.service_group,
                                        &self.pkg,
                                        self.svc_encrypted_password.as_ref())
        }
    }

    /// Run reconfigure hook if present.
    fn reconfigure(&mut self) {
        let _timer = hook_timer("reconfigure");

        if let Some(ref hook) = self.hooks.reconfigure {
            hook.run(&self.service_group,
                     &self.pkg,
                     self.svc_encrypted_password.as_ref());
        }
    }

    fn post_run(&mut self) {
        let _timer = hook_timer("post-run");

        if let Some(ref hook) = self.hooks.post_run {
            hook.run(&self.service_group,
                     &self.pkg,
                     self.svc_encrypted_password.as_ref());
        }
    }

    // This hook method looks different from all the others because
    // it's the only one that runs async right now.
    fn post_stop(&self) -> Option<hook_runner::HookRunner<hooks::PostStopHook>> {
        self.hooks.post_stop.as_ref().map(|hook| {
                                         hook_runner::HookRunner::new(Arc::clone(&hook),
                                                                      self.service_group.clone(),
                                                                      self.pkg.clone(),
                                                                      self.svc_encrypted_password
                                                                          .clone())
                                     })
    }

    pub fn suitability(&self) -> Option<u64> {
        let _timer = hook_timer("suitability");

        if !self.initialized {
            return None;
        }

        self.hooks.suitability.as_ref().and_then(|hook| {
                                           hook.run(&self.service_group,
                                                    &self.pkg,
                                                    self.svc_encrypted_password.as_ref())
                                       })
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
    fn compile_hooks(&self, ctx: &RenderContext<'_>) -> HookChangeTable {
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
        let svc_run = self.pkg.svc_path.join(hooks::RunHook::file_name());
        match self.hooks.run {
            Some(ref hook) => {
                fs::copy(hook.path(), &svc_run)?;
                Self::set_hook_permissions(&svc_run.to_str().unwrap())?;
            }
            None => {
                let run = self.pkg.path.join(hooks::RunHook::file_name());
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

    fn execute_hooks(&mut self,
                     launcher: &LauncherCli,
                     executor: &TaskExecutor,
                     hook_update_table: HookChangeTable,
                     config_change: bool) {
        if !self.initialized {
            if self.check_process() {
                self.reattach(executor);
                return;
            }
            self.initialize();
            if self.initialized {
                self.start(launcher, executor);
                self.post_run();
            }
        } else {
            self.check_process();
            if hook_update_table.run
               || hook_update_table.post_run
               || self.process_down()
               || (self.hooks.reconfigure.is_none() && config_change)
            {
                // TODO (DM): This flag is a hack. We have the `TaskExecutor` here. We could just
                // schedule the `stop` future, but the `Manager` wraps the `stop` future with
                // additional functionality. Can we refactor to make this flag unnecessary?
                self.needs_restart = true;
            } else if config_change || hook_update_table.reconfigure {
                self.reconfigure();
            }
        }
    }

    /// Run file-updated hook if present.
    fn file_updated(&self) -> bool {
        let _timer = hook_timer("file-updated");

        if self.initialized {
            if let Some(ref hook) = self.hooks.file_updated {
                return hook.run(&self.service_group,
                                &self.pkg,
                                self.svc_encrypted_password.as_ref());
            }
        }

        false
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
        let mut updated = false;
        for service_file in census_group.changed_service_files() {
            if self.cache_service_file(&service_file) {
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
                           self.binds
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
        let current_checksum = match hash::hash_file(&file) {
            Ok(current_checksum) => current_checksum,
            Err(err) => {
                outputln!(preamble self.service_group, "Failed to get current checksum for {}, {}",
                       file.as_ref().display(),
                       err);
                String::new()
            }
        };
        let new_checksum = hash::hash_bytes(&contents);
        if new_checksum == current_checksum {
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
        use habitat_core::{os::users,
                           util::posix_perm};

        if users::can_run_services_as_svc_user() {
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

/// This is a proxy struct to represent what information we're writing to the dat file, and
/// therefore what information gets sent out via the HTTP API. Right now, we're just wrapping the
/// actual Service struct, but this will give us something we can refactor against without
/// worrying about breaking the data returned to users.
pub struct ServiceProxy<'a> {
    service:          &'a Service,
    config_rendering: ConfigRendering,
}

impl<'a> ServiceProxy<'a> {
    pub fn new(s: &'a Service, c: ConfigRendering) -> Self {
        ServiceProxy { service:          &s,
                       config_rendering: c, }
    }
}

impl<'a> Serialize for ServiceProxy<'a> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let num_fields: usize = if self.config_rendering == ConfigRendering::Full {
            27
        } else {
            26
        };

        let s = &self.service;
        let mut strukt = serializer.serialize_struct("service", num_fields)?;
        strukt.serialize_field("all_pkg_binds", &s.all_pkg_binds)?;
        strukt.serialize_field("binding_mode", &s.binding_mode)?;
        strukt.serialize_field("binds", &s.binds)?;
        strukt.serialize_field("bldr_url", &s.bldr_url)?;

        if self.config_rendering == ConfigRendering::Full {
            strukt.serialize_field("cfg", &s.cfg)?;
        }

        strukt.serialize_field("channel", &s.channel)?;
        strukt.serialize_field("config_from", &s.config_from)?;
        strukt.serialize_field("desired_state", &s.desired_state)?;
        strukt.serialize_field("health_check", &s.health_check_result)?;
        strukt.serialize_field("hooks", &s.hooks)?;
        strukt.serialize_field("initialized", &s.initialized)?;
        strukt.serialize_field("last_election_status", &s.last_election_status)?;
        strukt.serialize_field("manager_fs_cfg", &s.manager_fs_cfg)?;

        let pkg_proxy = PkgProxy::new(&s.pkg);
        strukt.serialize_field("pkg", &pkg_proxy)?;

        strukt.serialize_field("process",
                               s.supervisor
                                .lock()
                                .expect("Couldn't lock supervisor")
                                .deref())?;
        strukt.serialize_field("service_group", &s.service_group)?;
        strukt.serialize_field("spec_file", &s.spec_file)?;
        strukt.serialize_field("spec_ident", &s.spec_ident)?;
        strukt.serialize_field("spec_identifier", &s.spec_ident.to_string())?;
        strukt.serialize_field("svc_encrypted_password", &s.svc_encrypted_password)?;
        strukt.serialize_field("health_check_interval", &s.health_check_interval)?;
        strukt.serialize_field("sys", &s.sys)?;
        strukt.serialize_field("topology", &s.topology)?;
        strukt.serialize_field("update_strategy", &s.update_strategy)?;
        strukt.serialize_field("user_config_updated", &s.user_config_updated)?;
        strukt.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;
    use habitat_common::types::{GossipListenAddr,
                                HttpListenAddr,
                                ListenCtlAddr};
    use serde_json;
    use std::str::FromStr;

    fn initialize_test_service() -> Service {
        let listen_ctl_addr =
            ListenCtlAddr::from_str("127.0.0.1:1234").expect("Can't parse IP into SocketAddr");
        let sys = Sys::new(false,
                           GossipListenAddr::default(),
                           listen_ctl_addr,
                           HttpListenAddr::default());

        let ident = if cfg!(target_os = "linux") {
            PackageIdent::new("core", "tree", Some("1.7.0"), Some("20180609045201"))
        } else if cfg!(target_os = "windows") {
            PackageIdent::new("core", "7zip", Some("16.04"), Some("20170131110814"))
        } else {
            panic!("This is being run on a platform that's not currently supported");
        };

        let spec = ServiceSpec::default_for(ident);

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                            .join("fixtures")
                                                            .join("pkgs");

        let install = PackageInstall::load(&spec.ident, Some(&path)).expect("PackageInstall \
                                                                             should've loaded my \
                                                                             spec, but it didn't");
        let asys = Arc::new(sys);
        let fscfg = FsCfg::new("/tmp");
        let afs = Arc::new(fscfg);

        let gs = Arc::new(RwLock::new(GatewayState::default()));
        Service::new(asys, &install, spec, afs, Some("haha"), gs).expect("I wanted a service to \
                                                                          load, but it didn't")
    }

    #[test]
    fn service_proxy_conforms_to_the_schema() {
        let service = initialize_test_service();

        // With config
        let proxy_with_config = ServiceProxy::new(&service, ConfigRendering::Full);
        let proxies_with_config = vec![proxy_with_config];
        let json_with_config =
            serde_json::to_string(&proxies_with_config).expect("Expected to convert \
                                                                proxies_with_config to JSON but \
                                                                failed");
        assert_valid(&json_with_config, "http_gateway_services_schema.json");

        // Without config
        let proxy_without_config = ServiceProxy::new(&service, ConfigRendering::Redacted);
        let proxies_without_config = vec![proxy_without_config];
        let json_without_config =
            serde_json::to_string(&proxies_without_config).expect("Expected to convert \
                                                                   proxies_without_config to \
                                                                   JSON but failed");
        assert_valid(&json_without_config, "http_gateway_services_schema.json");
    }
}
