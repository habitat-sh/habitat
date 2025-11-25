use super::{BindingMode,
            Topology,
            UpdateCondition,
            UpdateStrategy};
use crate::error::{Error,
                   Result};
use habitat_core::{ChannelIdent,
                   fs::atomic_write,
                   os::process::ShutdownTimeout,
                   package::{PackageIdent,
                             PackageInstall},
                   service::{HealthCheckInterval,
                             ServiceBind},
                   url::DEFAULT_BLDR_URL,
                   util};
use habitat_sup_protocol::{self,
                           net};
use log::{debug,
          warn};
use serde::{self,
            Deserialize,
            Serialize};
use std::{collections::HashSet,
          convert::TryFrom,
          fmt,
          fs::{self,
               File},
          io::{BufReader,
               Read},
          path::{Path,
                 PathBuf},
          result,
          str::FromStr};

static DEFAULT_GROUP: &str = "default";
const SPEC_FILE_EXT: &str = "spec";

#[derive(Copy,
         Clone,
         Debug,
         Default,
         Deserialize,
         Eq,
         Hash,
         PartialEq,
         Serialize)]
pub enum DesiredState {
    Down,
    #[default]
    Up,
}

impl fmt::Display for DesiredState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match *self {
            DesiredState::Down => "down",
            DesiredState::Up => "up",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for DesiredState {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "down" => Ok(DesiredState::Down),
            "up" => Ok(DesiredState::Up),
            _ => Err(Error::BadDesiredState(value.to_string())),
        }
    }
}

impl From<DesiredState> for i32 {
    fn from(other: DesiredState) -> Self {
        match other {
            DesiredState::Down => 0,
            DesiredState::Up => 1,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(default = "ServiceSpec::deserialization_base")]
pub struct ServiceSpec {
    #[serde(with = "util::serde::string")]
    pub ident:                  PackageIdent,
    pub group:                  String,
    pub bldr_url:               String,
    pub channel:                ChannelIdent,
    pub topology:               Topology,
    pub update_strategy:        UpdateStrategy,
    pub update_condition:       UpdateCondition,
    pub binds:                  Vec<ServiceBind>,
    pub binding_mode:           BindingMode,
    pub config_from:            Option<PathBuf>,
    #[serde(with = "util::serde::string")]
    pub desired_state:          DesiredState,
    pub shutdown_timeout:       Option<ShutdownTimeout>,
    pub svc_encrypted_password: Option<String>,
    // it is important that the health check interval
    // is the last field to be serialized because it
    // is serialized as a table. Individual values
    // serialized after the health check interval will
    // break the parser.
    // Note that there is an issue to ultimately fix this:
    // https://github.com/habitat-sh/habitat/issues/6469
    // and eliminate the need to keep this field last.
    pub health_check_interval:  HealthCheckInterval,
}

impl ServiceSpec {
    pub fn new(ident: PackageIdent) -> Self {
        let channel = if ident.origin == "core" {
            ChannelIdent::base()
        } else {
            ChannelIdent::stable()
        };

        Self { ident,
               group: DEFAULT_GROUP.to_string(),
               bldr_url: DEFAULT_BLDR_URL.to_string(),
               channel,
               topology: Topology::default(),
               update_strategy: UpdateStrategy::default(),
               update_condition: UpdateCondition::default(),
               binds: Vec::default(),
               binding_mode: BindingMode::Strict,
               config_from: None,
               desired_state: DesiredState::default(),
               health_check_interval: HealthCheckInterval::default(),
               svc_encrypted_password: None,
               shutdown_timeout: None }
    }

    // This should only be used to provide a default value when deserializing. We intentially do not
    // implement `Default` because a default value for `PackageIdent` does not make sense and should
    // be removed.
    fn deserialization_base() -> Self { Self::new(PackageIdent::default()) }

    fn to_toml_string(&self) -> Result<String> {
        if self.ident == PackageIdent::default() {
            return Err(Error::MissingRequiredIdent);
        }
        toml::to_string(self).map_err(Error::ServiceSpecRender)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(&path).map_err(|err| {
                                        Error::ServiceSpecFileIO(path.as_ref().to_path_buf(), err)
                                    })?;
        let mut file = BufReader::new(file);
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|err| Error::ServiceSpecFileIO(path.as_ref().to_path_buf(), err))?;
        Self::from_str(&buf)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        debug!("Writing service spec to '{}': {:?}",
               path.as_ref().display(),
               &self);
        let dst_path = path.as_ref()
                           .parent()
                           .expect("Cannot determine parent directory for service spec");
        fs::create_dir_all(dst_path).map_err(|err| {
                                        Error::ServiceSpecFileIO(path.as_ref().to_path_buf(), err)
                                    })?;
        let toml = self.to_toml_string()?;
        atomic_write(path.as_ref(), toml).map_err(|err| {
                                             Error::ServiceSpecFileIO(path.as_ref().to_path_buf(),
                                                                      err)
                                         })?;
        Ok(())
    }

    pub fn ident_file(ident: &PackageIdent) -> PathBuf {
        PathBuf::from(format!("{}.{}", ident.name, SPEC_FILE_EXT))
    }

    pub fn file(&self) -> PathBuf { Self::ident_file(&self.ident) }

    /// Validates that all required package binds are present in service binds and all remaining
    /// service binds are optional package binds.
    ///
    /// # Errors
    ///
    /// * If any required package binds are missing in service binds
    /// * If any given service binds are in neither required nor optional package binds
    pub fn validate(&self, package: &PackageInstall) -> Result<()> {
        let mut svc_binds: HashSet<&str> = self.binds.iter().map(ServiceBind::name).collect();
        let mut missing_req_binds = Vec::new();

        // Remove each service bind that matches a required package bind. If a required package
        // bind is not found, add the bind to the missing list to return an `Err`.
        let req_binds = package.binds()?;
        for req_bind in req_binds.iter().map(|b| b.service.as_str()) {
            if svc_binds.contains(req_bind) {
                svc_binds.remove(req_bind);
            } else {
                missing_req_binds.push(req_bind.to_string());
            }
        }
        // If we have missing required binds, return an `Err`.
        if !missing_req_binds.is_empty() {
            return Err(Error::MissingRequiredBind(missing_req_binds));
        }

        // Remove each service bind that matches an optional package bind.
        for opt_bind in package.binds_optional()?.iter().map(|b| b.service.as_str()) {
            if svc_binds.contains(opt_bind) {
                svc_binds.remove(opt_bind);
            }
        }
        // If we have remaining service binds then they are neither required nor optional package
        // binds. In this case, return an `Err`.
        if !svc_binds.is_empty() {
            return Err(Error::InvalidBinds(svc_binds.into_iter()
                                                    .map(str::to_string)
                                                    .collect()));
        }

        Ok(())
    }

    pub fn merge_svc_load(mut self, svc_load: habitat_sup_protocol::ctl::SvcLoad) -> Result<Self> {
        self.ident =
            svc_load.ident
                    .ok_or_else(|| net::err(net::ErrCode::BadPayload, "No ident specified"))?
                    .into();
        if let Some(group) = svc_load.group {
            self.group = group;
        }
        if let Some(bldr_url) = svc_load.bldr_url {
            self.bldr_url = bldr_url;
        }
        if let Some(channel) = svc_load.bldr_channel {
            self.channel = channel.into();
        } else {
            // Set the appropriate default channel based on origin
            self.channel = if self.ident.origin == "core" {
                ChannelIdent::base()
            } else {
                ChannelIdent::stable()
            };
        }
        if let Some(topology) = svc_load.topology {
            if let Ok(topology) = Topology::try_from(topology) {
                self.topology = topology;
            } else {
                warn!("Unable to parse topology value from SvcLoad protocol message; ignoring: {}",
                      topology);
            }
        }
        if let Some(update_strategy) = svc_load.update_strategy {
            if let Ok(update_strategy) = UpdateStrategy::try_from(update_strategy) {
                self.update_strategy = update_strategy;
            } else {
                warn!("Unable to parse update strategy value from SvcLoad protocol message; \
                       ignoring: {}",
                      update_strategy);
            }
        }
        if let Some(update_condition) = svc_load.update_condition {
            if let Ok(update_condition) = UpdateCondition::try_from(update_condition) {
                self.update_condition = update_condition;
            } else {
                warn!("Unable to parse update condition value from SvcLoad protocol message; \
                       ignoring: {}",
                      update_condition);
            }
        }
        if let Some(list) = svc_load.binds {
            self.binds = list.into();
        }
        if let Some(binding_mode) = svc_load.binding_mode {
            if let Ok(binding_mode) = BindingMode::try_from(binding_mode) {
                self.binding_mode = binding_mode;
            } else {
                warn!("Unable to parse binding mode value from SvcLoad protocol message; \
                       ignoring: {}",
                      binding_mode);
            }
        }
        if let Some(config_from) = svc_load.config_from {
            self.config_from = Some(PathBuf::from(config_from));
        }
        if let Some(svc_encrypted_password) = svc_load.svc_encrypted_password {
            self.svc_encrypted_password = Some(svc_encrypted_password);
        }
        if let Some(interval) = svc_load.health_check_interval {
            self.health_check_interval = interval.seconds.into()
        }
        if let Some(shutdown_timeout) = svc_load.shutdown_timeout {
            self.shutdown_timeout = Some(ShutdownTimeout::from(shutdown_timeout));
        }
        Ok(self)
    }

    pub fn merge_svc_update(&mut self, svc_update: habitat_sup_protocol::ctl::SvcUpdate) {
        if let Some(group) = svc_update.group {
            self.group = group;
        }
        if let Some(bldr_url) = svc_update.bldr_url {
            self.bldr_url = bldr_url;
        }
        if let Some(channel) = svc_update.bldr_channel {
            self.channel = channel.into();
        }
        if let Some(topology) = svc_update.topology {
            if let Ok(topology) = Topology::try_from(topology) {
                self.topology = topology;
            } else {
                warn!("Unable to parse topology value from SvcUpdate protocol message; ignoring: \
                       {}",
                      topology);
            }
        }
        if let Some(update_strategy) = svc_update.update_strategy {
            if let Ok(update_strategy) = UpdateStrategy::try_from(update_strategy) {
                self.update_strategy = update_strategy;
            } else {
                warn!("Unable to parse update strategy value from SvcUpdate protocol message; \
                       ignoring: {}",
                      update_strategy);
            }
        }
        if let Some(update_condition) = svc_update.update_condition {
            if let Ok(update_condition) = UpdateCondition::try_from(update_condition) {
                self.update_condition = update_condition;
            } else {
                warn!("Unable to parse update condition value from SvcUpdate protocol message; \
                       ignoring: {}",
                      update_condition);
            }
        }
        if let Some(list) = svc_update.binds {
            self.binds = list.into();
        }
        if let Some(binding_mode) = svc_update.binding_mode {
            if let Ok(binding_mode) = BindingMode::try_from(binding_mode) {
                self.binding_mode = binding_mode;
            } else {
                warn!("Unable to parse binding mode value from SvcUpdate protocol message; \
                       ignoring: {}",
                      binding_mode);
            }
        }
        if let Some(svc_encrypted_password) = svc_update.svc_encrypted_password {
            self.svc_encrypted_password = Some(svc_encrypted_password);
        }
        if let Some(interval) = svc_update.health_check_interval {
            self.health_check_interval = interval.seconds.into()
        }
        if let Some(shutdown_timeout) = svc_update.shutdown_timeout {
            self.shutdown_timeout = Some(ShutdownTimeout::from(shutdown_timeout));
        }
    }

    /// Given an `old` and a `new` spec, figure out what operations
    /// are needed in order to turn the `old` state into the `new`
    /// state.
    ///
    /// Here, `old` represents what a currently running service, while
    /// `new` represents a new version of that spec that we wish to
    /// make the currently running service. Both are `Option`s, in
    /// order to capture the scenario in which, say, nothing is
    /// currently running, but we wish to start a service, or where we
    /// are running a service, but wish to stop it.
    ///
    /// Currently, it is *assumed* that both specs (when present)
    /// refer to the same service.
    ///
    /// Returning `None` indicates that no operation is required.
    pub(crate) fn reconcile(old: Option<ServiceSpec>,
                            new: Option<ServiceSpec>)
                            -> Option<ServiceOperation> {
        // We need to compare the old spec to the new spec, taking
        // into consideration the desired state of each. While we can
        // do that via pattern matching directly, it gets a little
        // hairy. Instead, we'll just extract the data we need into
        // one unified match statement and go from there.
        use DesiredState::{Down,
                           Up};

        match (old.map(|o| (o.desired_state, o)),
               new.map(|n| (n.desired_state, n)))
        {
            // theoretically shouldn't happen, but no harm if it does.
            (None, None)
            // Somebody manually added a spec file that for some
            // reason stated the service should be down. Weird, but
            // okay....
            | (None, Some((Down, _)))
            // A stopped service's spec file was removed
            | (Some((Down, _)), None)
            // A stopped service's spec file was changed, but it is
            // still supposed to be down. This would also likely
            // require manual intervention.
            | (Some((Down, _)), Some((Down, _))) => {
                // None of these situations require us to do anything
                // in the way of starting, stopping, restarting, or
                // modifying a service.
                None
            }

            // A running service's spec file was removed (e.g., hab
            // svc unload)
            (Some((Up, old)), None)
            // A running service was told to stop (e.g., hab svc stop)
            | (Some((Up, old)), Some((Down, _))) => {
                Some(ServiceOperation::Stop(old))
            }

            // A new spec file was added (e.g., hab svc load)
            (None, Some((Up, new)))
            // A previously stopped service was started (e.g., hab svc start)
            | (Some((Down, _)), Some((Up, new))) => {
                Some(ServiceOperation::Start(new))
            }

            // The configuration of a running service was somehow changed
            (Some((Up, running_spec)), Some((Up, disk_spec))) => {
                if running_spec == disk_spec {
                    // Given how this function is called, this is
                    // unlikely to happen, but if it does, we don't
                    // have to do anything.
                    None
                } else {
                    // Destructure the entire spec so the compiler
                    // ensures that we look at everything.
                    let ServiceSpec {
                        ident,
                        group,
                        bldr_url,
                        channel,
                        topology,
                        update_strategy,
                        update_condition,
                        binds,
                        binding_mode,
                        config_from,
                        // This has to be `Up` if we're in this
                        // code. As a result, we don't care about
                        // matching or destructuring it.
                        desired_state: _,
                        shutdown_timeout,
                        svc_encrypted_password,
                        health_check_interval,
                    } = &running_spec;

                    // Currently, if any of these bits of data are
                    // different, we should restart the service. This
                    // is not to say that it will *always* be that
                    // way, however. Initially we are allowing dynamic
                    // update of update-related configuration, but
                    // nothing prevents us from making more things
                    // dynamic in the future. We are proceeding
                    // conservatively.

                    // NOTE: if the idents change in any way, you
                    // *must* restart, since that change may result in
                    // a different version of the service being run.
                    if ident != &disk_spec.ident
                        || group != &disk_spec.group
                        // TODO (CM): This *might* not need to be here
                        || topology != &disk_spec.topology
                        // TODO (CM): Bind information *may* be able
                        // to be dynamically changed, but that will
                        // need to be investigated more deeply.
                        || binds != &disk_spec.binds
                        || binding_mode != &disk_spec.binding_mode
                        || config_from != &disk_spec.config_from
                        // TODO (CM): This probably doesn't need to be here
                        || shutdown_timeout != &disk_spec.shutdown_timeout
                        || svc_encrypted_password != &disk_spec.svc_encrypted_password
                        // TODO (CM): This probably doesn't need to be here, either
                        || health_check_interval != &disk_spec.health_check_interval
                    {
                        debug!("Reconciliation: '{}' queued for restart",
                               running_spec.ident);
                        Some(ServiceOperation::Restart { to_stop:  running_spec,
                                                         to_start: disk_spec, })
                    } else {
                        let mut ops = HashSet::new();
                        if bldr_url != &disk_spec.bldr_url
                            || channel != &disk_spec.channel
                            || update_strategy != &disk_spec.update_strategy
                            || update_condition != &disk_spec.update_condition
                        {
                            ops.insert(RefreshOperation::RestartUpdater);
                        }

                        // We should have *something* to do down
                        // here, but if we don't, let's be explicit
                        // about it.
                        if ops.is_empty() {
                            warn!("No refresh operations computed for {}!", ident);
                            None
                        } else {
                            Some(ServiceOperation::Update(disk_spec, ops))
                        }
                    }
                }
            }
        }
    }
}

/// Everything we could do to a running service as a result of a
/// configuration change that does *not* require a service restart.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub(crate) enum RefreshOperation {
    /// Restart the logic for fetching updates to a service.
    ///
    /// This can happen if a user wants to change the channel a
    /// service is updating from, for instance.
    RestartUpdater,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum ServiceOperation {
    /// Start the specified service, which is not running currently.
    Start(ServiceSpec),
    /// Stop the specified service, which *is* currently running.
    Stop(ServiceSpec),
    /// Swap the given configuration into the currently running
    /// service. The service process *is not* restarted, but the
    /// specified operations are performed on it.
    Update(ServiceSpec, HashSet<RefreshOperation>),
    /// Stop the service specified by `to_stop`, and restart it with
    /// the `to_start` spec.
    ///
    /// This can be seen as a refinement of `Update` that involves
    /// stopping and starting the service process as well.
    Restart {
        to_stop:  ServiceSpec,
        to_start: ServiceSpec,
    },
}

impl FromStr for ServiceSpec {
    type Err = Error;

    fn from_str(toml: &str) -> result::Result<Self, Self::Err> {
        let spec: ServiceSpec = toml::from_str(toml).map_err(Error::ServiceSpecParse)?;
        if spec.ident == PackageIdent::default() {
            return Err(Error::MissingRequiredIdent);
        }
        Ok(spec)
    }
}

impl TryFrom<habitat_sup_protocol::ctl::SvcLoad> for ServiceSpec {
    type Error = Error;

    fn try_from(svc_load: habitat_sup_protocol::ctl::SvcLoad) -> Result<Self> {
        // We use the default `PackageIdent` here but `merge_svc_load` checks that
        // `svc_load.ident` is set so we will return an error if there is no ident.
        Self::new(PackageIdent::default()).merge_svc_load(svc_load)
    }
}

#[cfg(test)]
mod test {
    use std::{fs::{self,
                   File},
              io::{BufReader,
                   Read,
                   Write},
              iter::FromIterator,
              path::{Path,
                     PathBuf},
              str::FromStr};
    use tempfile::TempDir;

    use habitat_core::{package::PackageIdent,
                       service::HealthCheckInterval};

    use super::*;
    use crate::error::Error::*;

    use habitat_sup_protocol::ctl::SvcLoad;
    use std::convert::TryFrom;

    fn file_from_str<P: AsRef<Path>>(path: P, content: &str) {
        fs::create_dir_all(
            path.as_ref()
                .parent()
                .expect("failed to determine file's parent directory"),
        )
        .expect("failed to create parent directory recursively");
        let mut file = File::create(path).expect("failed to create file");
        file.write_all(content.as_bytes())
            .expect("failed to write content to file");
    }

    fn string_from_file<P: AsRef<Path>>(path: P) -> String {
        let file = File::open(path).expect("failed to open file");
        let mut file = BufReader::new(file);
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("cannot read file to string");
        buf
    }

    #[test]
    fn service_spec_from_str() {
        let toml = r#"
            ident = "origin/name/1.2.3/20170223130020"
            group = "jobs"
            bldr_url = "http://example.com/depot"
            topology = "leader"
            update_strategy = "rolling"
            update_condition = "latest"
            binds = ["cache:redis.cache@acmecorp", "db:postgres.app@acmecorp"]
            config_from = "/only/for/development"

            [health_check_interval]
            secs = 5
            nanos = 0
            "#;
        let spec = ServiceSpec::from_str(toml).unwrap();

        assert_eq!(spec.ident,
                   PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap());
        assert_eq!(spec.group, String::from("jobs"));
        assert_eq!(spec.bldr_url, String::from("http://example.com/depot"));
        assert_eq!(spec.topology, Topology::Leader);
        assert_eq!(spec.update_strategy, UpdateStrategy::Rolling);
        assert_eq!(spec.update_condition, UpdateCondition::Latest);
        assert_eq!(spec.binds,
                   vec![ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                        ServiceBind::from_str("db:postgres.app@acmecorp").unwrap(),]);
        assert_eq!(spec.config_from,
                   Some(PathBuf::from("/only/for/development")));
        assert_eq!(spec.health_check_interval,
                   HealthCheckInterval::from_str("5").unwrap());
    }

    #[test]
    fn service_spec_from_str_missing_ident() {
        let toml = r#""#;

        match ServiceSpec::from_str(toml) {
            Err(e) => {
                match e {
                    MissingRequiredIdent => (), // expected outcome
                    e => panic!("Unexpected error returned: {:?}", e),
                }
            }
            Ok(_) => panic!("Spec TOML should fail to parse"),
        }
    }

    #[test]
    fn service_spec_from_str_invalid_topology() {
        let toml = r#"
            ident = "origin/name/1.2.3/20170223130020"
            topology = "smartest-possible"
            "#;

        match ServiceSpec::from_str(toml) {
            Err(e) => {
                match e {
                    ServiceSpecParse(_) => (), // expected outcome
                    e => panic!("Unexpected error returned: {:?}", e),
                }
            }
            Ok(_) => panic!("Spec TOML should fail to parse"),
        }
    }

    #[test]
    fn service_spec_from_str_invalid_binds() {
        let toml = r#"
            ident = "origin/name/1.2.3/20170223130020"
            topology = "leader"
            binds = ["magic:magicness.default", "winning"]
            "#;

        match ServiceSpec::from_str(toml) {
            Err(e) => {
                match e {
                    ServiceSpecParse(_) => (), // expected outcome
                    e => panic!("Unexpected error returned: {:?}", e),
                }
            }
            Ok(_) => panic!("Spec TOML should fail to parse"),
        }
    }

    #[test]
    fn service_spec_to_toml_string() {
        let spec =
            ServiceSpec { ident:                  PackageIdent::from_str("origin/name/1.2.3/\
                                                                          20170223130020").unwrap(),
                          group:                  String::from("jobs"),
                          bldr_url:               String::from("http://example.com/depot"),
                          channel:                ChannelIdent::unstable(),
                          topology:               Topology::Leader,
                          update_strategy:        UpdateStrategy::AtOnce,
                          update_condition:       UpdateCondition::Latest,
                          binds:                  vec![ServiceBind::from_str("cache:redis.cache@\
                                                                              acmecorp").unwrap(),
                                                       ServiceBind::from_str("db:postgres.app@\
                                                                              acmecorp").unwrap(),],
                          binding_mode:           BindingMode::Relaxed,
                          health_check_interval:  HealthCheckInterval::from_str("123").unwrap(),
                          config_from:            Some(PathBuf::from("/only/for/development")),
                          desired_state:          DesiredState::Down,
                          svc_encrypted_password: None,
                          shutdown_timeout:       Some(ShutdownTimeout::from_str("10").unwrap()), };
        let toml = spec.to_toml_string().unwrap();

        assert!(toml.contains(r#"ident = "origin/name/1.2.3/20170223130020""#,));
        assert!(toml.contains(r#"group = "jobs""#));
        assert!(toml.contains(r#"bldr_url = "http://example.com/depot""#));
        assert!(toml.contains(r#"channel = "unstable""#));
        assert!(toml.contains(r#"topology = "leader""#));
        assert!(toml.contains(r#"update_strategy = "at-once""#));
        assert!(toml.contains(r#""cache:redis.cache@acmecorp""#));
        assert!(toml.contains(r#""db:postgres.app@acmecorp""#));
        assert!(toml.contains(r#"desired_state = "down""#));
        assert!(toml.contains(r#"config_from = "/only/for/development""#));
        assert!(toml.contains(r#"binding_mode = "relaxed""#));
        assert!(toml.contains(r#"[health_check_interval]"#));
        assert!(toml.contains(r#"secs = 123"#));
        assert!(toml.contains(r#"nanos = 0"#));
        assert!(toml.contains(r#"shutdown_timeout = 10"#));
    }

    #[test]
    fn service_spec_to_toml_string_invalid_ident() {
        // Remember: the default implementation of `PackageIdent` is an invalid identifier, missing
        // origin and name--we're going to exploit this here
        let spec = ServiceSpec::new(PackageIdent::default());

        match spec.to_toml_string() {
            Err(e) => {
                match e {
                    MissingRequiredIdent => (), // expected outcome,
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("Spec TOML should fail to render"),
        }
    }

    #[test]
    fn service_spec_from_file() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("name.spec");
        let toml = r#"
            ident = "origin/name/1.2.3/20170223130020"
            group = "jobs"
            bldr_url = "http://example.com/depot"
            topology = "leader"
            update_strategy = "rolling"
            binds = ["cache:redis.cache@acmecorp", "db:postgres.app@acmecorp"]
            config_from = "/only/for/development"

            [health_check_interval]
            secs = 5
            nanos = 0
            "#;
        file_from_str(&path, toml);
        let spec = ServiceSpec::from_file(path).unwrap();

        assert_eq!(spec.ident,
                   PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap());
        assert_eq!(spec.group, String::from("jobs"));
        assert_eq!(spec.bldr_url, String::from("http://example.com/depot"));
        assert_eq!(spec.topology, Topology::Leader);
        assert_eq!(spec.update_strategy, UpdateStrategy::Rolling);
        assert_eq!(spec.binds,
                   vec![ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                        ServiceBind::from_str("db:postgres.app@acmecorp").unwrap(),]);
        assert_eq!(spec.channel, ChannelIdent::stable());
        assert_eq!(spec.config_from,
                   Some(PathBuf::from("/only/for/development")));

        assert_eq!(spec.binding_mode,
                   BindingMode::Strict,
                   "Strict is the default mode, if nothing was previously specified.");
        assert_eq!(spec.health_check_interval,
                   HealthCheckInterval::from_str("5").unwrap());
    }

    #[test]
    fn service_spec_from_file_missing_healthcheck_interval() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("name.spec");
        let toml = r#"
            ident = "origin/name/1.2.3/20170223130020"
            group = "jobs"
            bldr_url = "http://example.com/depot"
            topology = "leader"
            update_strategy = "rolling"
            binds = ["cache:redis.cache@acmecorp", "db:postgres.app@acmecorp"]
            config_from = "/only/for/development"
            "#;

        file_from_str(&path, toml);
        let spec = ServiceSpec::from_file(path).unwrap();

        assert_eq!(spec.health_check_interval, HealthCheckInterval::default());
    }

    #[test]
    fn service_spec_from_file_missing() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("nope.spec");

        match ServiceSpec::from_file(&path) {
            Err(e) => {
                match e {
                    ServiceSpecFileIO(p, _) => assert_eq!(path, p),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("File should not exist for read"),
        }
    }

    #[test]
    fn service_spec_from_file_empty() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("empty.spec");
        file_from_str(&path, "");

        match ServiceSpec::from_file(&path) {
            Err(e) => {
                match e {
                    MissingRequiredIdent => (), // expected outcome,
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("File should not exist for read"),
        }
    }

    #[test]
    fn service_spec_from_file_bad_contents() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("bad.spec");
        file_from_str(&path, "You're gonna have a bad time");

        match ServiceSpec::from_file(&path) {
            Err(e) => {
                match e {
                    ServiceSpecParse(_) => (), // expected outcome,
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("File should not exist for read"),
        }
    }

    #[test]
    fn service_spec_to_file() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("name.spec");
        let spec =
            ServiceSpec { ident:                  PackageIdent::from_str("origin/name/1.2.3/\
                                                                          20170223130020").unwrap(),
                          group:                  String::from("jobs"),
                          bldr_url:               String::from("http://example.com/depot"),
                          channel:                ChannelIdent::unstable(),
                          topology:               Topology::Leader,
                          update_strategy:        UpdateStrategy::AtOnce,
                          update_condition:       UpdateCondition::Latest,
                          binds:                  vec![ServiceBind::from_str("cache:redis.cache@\
                                                                              acmecorp").unwrap(),
                                                       ServiceBind::from_str("db:postgres.app@\
                                                                              acmecorp").unwrap(),],
                          binding_mode:           BindingMode::Relaxed,
                          health_check_interval:  HealthCheckInterval::from_str("23").unwrap(),
                          config_from:            Some(PathBuf::from("/only/for/development")),
                          desired_state:          DesiredState::Down,
                          svc_encrypted_password: None,
                          shutdown_timeout:       Some(ShutdownTimeout::default()), };
        spec.to_file(&path).unwrap();
        let toml = string_from_file(path);

        assert!(toml.contains(r#"ident = "origin/name/1.2.3/20170223130020""#,));
        assert!(toml.contains(r#"group = "jobs""#));
        assert!(toml.contains(r#"bldr_url = "http://example.com/depot""#));
        assert!(toml.contains(r#"channel = "unstable""#));
        assert!(toml.contains(r#"topology = "leader""#));
        assert!(toml.contains(r#"update_strategy = "at-once""#));
        assert!(toml.contains(r#""cache:redis.cache@acmecorp""#));
        assert!(toml.contains(r#""db:postgres.app@acmecorp""#));
        assert!(toml.contains(r#"desired_state = "down""#));
        assert!(toml.contains(r#"config_from = "/only/for/development""#));
        assert!(toml.contains(r#"binding_mode = "relaxed""#));
        assert!(toml.contains(r#"[health_check_interval]"#));
        assert!(toml.contains(r#"secs = 23"#));
        assert!(toml.contains(r#"nanos = 0"#));
    }

    #[test]
    fn service_spec_to_file_invalid_ident() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("name.spec");
        // Remember: the default implementation of `PackageIdent` is an invalid identifier, missing
        // origin and name--we're going to exploit this here
        let spec = ServiceSpec::new(PackageIdent::default());

        match spec.to_file(path) {
            Err(e) => {
                match e {
                    MissingRequiredIdent => (), // expected outcome,
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("Service spec file should not have been written"),
        }
    }

    #[test]
    fn service_spec_file_name() {
        let spec = ServiceSpec::new(PackageIdent::from_str("origin/hoopa/1.2.3").unwrap());

        assert_eq!(Path::new("hoopa.spec"), spec.file());
    }

    fn testing_package_install() -> PackageInstall {
        let ident = if cfg!(target_os = "linux") {
            if cfg!(target_arch = "x86_64") {
                PackageIdent::new("test-bind",
                                  "test-bind",
                                  Some("0.1.0"),
                                  Some("20190219230309"))
            } else if cfg!(target_arch = "aarch64") {
                PackageIdent::new("test-bind-native",
                                  "test-bind-native-linux-aarch64",
                                  Some("0.1.0"),
                                  Some("20220701090436"))
            } else {
                panic!("This is being run on a platform that's not currently supported");
            }
        } else if cfg!(target_os = "windows") {
            PackageIdent::new("test-bind",
                              "test-bind-win",
                              Some("0.1.0"),
                              Some("20190219231616"))
        } else {
            panic!("This is being run on a platform that's not currently supported");
        };

        let spec = ServiceSpec::new(ident);
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests")
                                                            .join("fixtures")
                                                            .join("pkgs");

        PackageInstall::load(&spec.ident, Some(&path)).expect("PackageInstall should've loaded my \
                                                               spec, but it didn't")
    }

    #[test]
    fn service_spec_bind_present() {
        let package = testing_package_install();

        let mut spec = ServiceSpec::new(package.ident().clone());
        spec.binds = vec![ServiceBind::from_str("database:postgresql.app@acmecorp").unwrap()];
        if let Err(e) = spec.validate(&package) {
            panic!("Unexpected error returned: {:?}", e);
        }
    }

    #[test]
    fn service_spec_with_optional_bind() {
        let package = testing_package_install();

        let mut spec = ServiceSpec::new(package.ident().clone());
        spec.binds = vec![ServiceBind::from_str("database:postgresql.app@acmecorp").unwrap(),
                          ServiceBind::from_str("storage:minio.app@acmecorp").unwrap(),];
        if let Err(e) = spec.validate(&package) {
            panic!("Unexpected error returned: {:?}", e);
        }
    }

    #[test]
    /// Test when we're missing the required bind (backend)
    fn service_spec_error_missing_bind() {
        let package = testing_package_install();

        let mut spec = ServiceSpec::new(package.ident().clone());
        spec.binds = vec![];
        match spec.validate(&package) {
            Err(e) => {
                match e {
                    MissingRequiredBind(b) => assert_eq!(vec!["database".to_string()], b),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("Spec should not validate"),
        }
    }

    #[test]
    /// Test when we're asking for a bind (backend) that isn't provided by the
    /// package
    fn service_spec_error_invalid_bind() {
        let package = testing_package_install();

        let mut spec = ServiceSpec::new(package.ident().clone());
        spec.binds = vec![ServiceBind::from_str("backend:tomcat.app@acmecorp").unwrap(),
                          ServiceBind::from_str("database:postgres.app@acmecorp").unwrap(),];
        match spec.validate(&package) {
            Err(e) => {
                match e {
                    InvalidBinds(b) => assert_eq!(vec!["backend".to_string()], b),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("Spec should not validate"),
        }
    }

    /// This is to support backward compatibility with the old
    /// application/environment functionality that is being removed.
    #[test]
    fn service_spec_with_app_env_is_properly_upgraded_from_str() {
        let toml = r#"
            ident = "origin/name/1.2.3/20170223130020"
            group = "jobs"
            application_environment = "theinternet.preprod"
            bldr_url = "http://example.com/depot"
            topology = "leader"
            update_strategy = "rolling"
            binds = ["cache:app.env#redis.cache@acmecorp", "db:app.env#postgres.app@acmecorp"]
            config_from = "/only/for/development"

            [health_check_interval]
            secs = 5
            nanos = 0
            "#;

        // The presence of application_environment is fine, and is
        // basically ignored.
        let spec = ServiceSpec::from_str(toml).unwrap();

        assert_eq!(spec.ident,
                   PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap());
        assert_eq!(spec.group, String::from("jobs"));

        assert_eq!(spec.bldr_url, String::from("http://example.com/depot"));
        assert_eq!(spec.topology, Topology::Leader);
        assert_eq!(spec.update_strategy, UpdateStrategy::Rolling);

        // Any app/env in binds are removed.
        assert_eq!(spec.binds,
                   vec![ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                        ServiceBind::from_str("db:postgres.app@acmecorp").unwrap(),]);
        assert_eq!(spec.config_from,
                   Some(PathBuf::from("/only/for/development")));
        assert_eq!(spec.health_check_interval,
                   HealthCheckInterval::from_str("5").unwrap());
    }

    #[test]
    fn test_try_from_svc_load_with_core_origin_and_no_channel() {
        let mut svc_load = SvcLoad::default();

        // Set a core origin package identifier
        let ident: PackageIdent = "core/redis/1.0.0/20180701125610".parse().unwrap();
        svc_load.ident = Some(ident.into());

        // Don't set channel, let it use default

        // Convert SvcLoad to ServiceSpec using TryFrom
        let spec =
            ServiceSpec::try_from(svc_load).expect("Failed to convert SvcLoad to ServiceSpec");

        // The key assertion - core origin with no channel should default to the 'base' channel
        assert_eq!(spec.channel, ChannelIdent::base());
        assert_eq!(spec.ident.origin, "core");
        assert_eq!(spec.ident.name, "redis");
    }

    #[test]
    fn test_try_from_svc_load_with_howdy_origin_and_no_channel() {
        let mut svc_load = SvcLoad::default();

        // Set a howdy origin package identifier
        let ident: PackageIdent = "howdy/web-app/1.0.0/20180701125610".parse().unwrap();
        svc_load.ident = Some(ident.into());

        // Don't set channel, let it use default

        // Convert SvcLoad to ServiceSpec using TryFrom
        let spec =
            ServiceSpec::try_from(svc_load).expect("Failed to convert SvcLoad to ServiceSpec");

        // The key assertion - non-core origin with no channel should default to the 'stable'
        // channel
        assert_eq!(spec.channel, ChannelIdent::stable());
        assert_eq!(spec.ident.origin, "howdy");
        assert_eq!(spec.ident.name, "web-app");
    }

    #[test]
    fn test_try_from_svc_load_with_howdy_origin_and_custom_channel() {
        let mut svc_load = SvcLoad::default();

        // Set a howdy origin package identifier
        let ident: PackageIdent = "howdy/web-app/1.0.0/20180701125610".parse().unwrap();
        svc_load.ident = Some(ident.into());

        // Set channel to mychannel
        svc_load.bldr_channel = Some("mychannel".to_string());

        // Convert SvcLoad to ServiceSpec using TryFrom
        let spec =
            ServiceSpec::try_from(svc_load).expect("Failed to convert SvcLoad to ServiceSpec");

        // The key assertion - should use the explicitly specified channel
        assert_eq!(spec.channel, ChannelIdent::from("mychannel"));
        assert_eq!(spec.ident.origin, "howdy");
        assert_eq!(spec.ident.name, "web-app");
    }

    #[test]
    fn test_try_from_svc_load_with_core_origin_and_custom_channel() {
        let mut svc_load = SvcLoad::default();

        // Set a core origin package identifier
        let ident: PackageIdent = "core/redis/1.0.0/20180701125610".parse().unwrap();
        svc_load.ident = Some(ident.into());

        // Set channel to mychannel
        svc_load.bldr_channel = Some("mychannel".to_string());

        // Convert SvcLoad to ServiceSpec using TryFrom
        let spec =
            ServiceSpec::try_from(svc_load).expect("Failed to convert SvcLoad to ServiceSpec");

        // The key assertion - should use the explicitly specified channel, not base channel
        assert_eq!(spec.channel, ChannelIdent::from("mychannel"));
        assert_eq!(spec.ident.origin, "core");
        assert_eq!(spec.ident.name, "redis");
    }

    mod reconcile {
        use super::*;

        fn spec<S>(ident: S, desired_state: DesiredState) -> ServiceSpec
            where S: AsRef<str>
        {
            let mut s = ServiceSpec::new(ident.as_ref()
                                              .parse()
                                              .expect("Couldn't create a testing spec from \
                                                       given ident"));
            s.desired_state = desired_state;
            s
        }

        #[test]
        fn test_a_bunch_of_no_ops() {
            let down_spec = spec("core/blah", DesiredState::Down);

            assert_eq!(ServiceSpec::reconcile(Some(down_spec.clone()), None), None);
            assert_eq!(ServiceSpec::reconcile(None, Some(down_spec.clone())), None);
            assert_eq!(ServiceSpec::reconcile(None, Some(down_spec.clone())), None);
            assert_eq!(ServiceSpec::reconcile(Some(down_spec.clone()), Some(down_spec)),
                       None);
            assert_eq!(ServiceSpec::reconcile(None, None), None);
        }

        #[test]
        fn test_start_cases() {
            let up_spec = spec("core/starter", DesiredState::Up);
            let down_spec = spec("core/starter", DesiredState::Down);

            assert_eq!(ServiceSpec::reconcile(Some(down_spec), Some(up_spec.clone())),
                       Some(ServiceOperation::Start(up_spec.clone())));
            assert_eq!(ServiceSpec::reconcile(None, Some(up_spec.clone())),
                       Some(ServiceOperation::Start(up_spec)));
        }

        #[test]
        fn test_stop_cases() {
            let up_spec = spec("core/stopper", DesiredState::Up);
            let down_spec = spec("core/stopper", DesiredState::Down);

            assert_eq!(ServiceSpec::reconcile(Some(up_spec.clone()), Some(down_spec)),
                       Some(ServiceOperation::Stop(up_spec.clone())));
            assert_eq!(ServiceSpec::reconcile(Some(up_spec.clone()), None),
                       Some(ServiceOperation::Stop(up_spec)));
        }

        /// Edge case where we end up with identical specs;
        /// technically possible, but practically unlikely, given
        /// how the code is called.
        #[test]
        fn identical_up_specs_is_a_no_op() {
            let s = spec("core/blah", DesiredState::Up);
            assert_eq!(ServiceSpec::reconcile(Some(s.clone()), Some(s)), None);
        }

        /// Take two "up" specs that are identical except that the
        /// second one has `value` set for `field` and reconcile
        /// them. They should either trigger a restart, or an update
        /// with the given RefreshOperations.
        ///
        /// Each invocation creates an independent test case, named
        /// `test_name` (because macros can't create functions with
        /// generated names).
        macro_rules! reconcile {
            ($test_name:ident,restart, $field:ident, $value:expr) => {
                #[test]
                fn $test_name() {
                    let running = spec("core/blah", DesiredState::Up);
                    let disk = {
                        let mut s = running.clone();
                        s.$field = $value;
                        s
                    };

                    assert_eq!(ServiceSpec::reconcile(Some(running.clone()), Some(disk.clone())),
                               Some(ServiceOperation::Restart { to_stop:  running,
                                                                to_start: disk, }))
                }
            };
            ($test_name:ident,update, $field:ident, $value:expr, $ops:expr) => {
                #[test]
                fn $test_name() {
                    let running = spec("core/blah", DesiredState::Up);
                    let disk = {
                        let mut s = running.clone();
                        s.$field = $value;
                        s
                    };

                    assert_eq!(ServiceSpec::reconcile(Some(running), Some(disk.clone())),
                               Some(ServiceOperation::Update(disk, HashSet::from_iter($ops))));
                }
            };
        }

        reconcile!(ident_causes_restart,
                   restart,
                   ident,
                   "core/foo".parse().unwrap());
        reconcile!(group_causes_restart, restart, group, "prod".to_string());
        reconcile!(topology_causes_restart, restart, topology, Topology::Leader);
        reconcile!(binds_causes_restart,
                   restart,
                   binds,
                   vec![ServiceBind::new("foo", "blah.default".parse().unwrap())]);
        reconcile!(binding_mode_causes_restart,
                   restart,
                   binding_mode,
                   BindingMode::Relaxed);
        reconcile!(config_from_causes_restart,
                   restart,
                   config_from,
                   Some("blah.config".into()));
        reconcile!(shutdown_timeout_causes_restart,
                   restart,
                   shutdown_timeout,
                   Some(10.into()));
        reconcile!(svc_encrypted_password_causes_restart,
                   restart,
                   svc_encrypted_password,
                   Some("monkeys".to_string()));
        reconcile!(health_check_interval_causes_restart,
                   restart,
                   health_check_interval,
                   10000.into());

        reconcile!(bldr_url_causes_update,
                   update,
                   bldr_url,
                   "http://mybuider.company.com".to_string(),
                   vec![RefreshOperation::RestartUpdater]);
        reconcile!(channel_causes_update,
                   update,
                   channel,
                   "new_channel".into(),
                   vec![RefreshOperation::RestartUpdater]);
        reconcile!(update_strategy_causes_update,
                   update,
                   update_strategy,
                   UpdateStrategy::AtOnce,
                   vec![RefreshOperation::RestartUpdater]);
        reconcile!(update_condition_causes_update,
                   update,
                   update_condition,
                   UpdateCondition::TrackChannel,
                   vec![RefreshOperation::RestartUpdater]);
    }
}
