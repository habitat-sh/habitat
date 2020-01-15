use super::{BindingMode,
            Topology,
            UpdateStrategy};
use crate::error::{Error,
                   Result};
use habitat_core::{fs::atomic_write,
                   os::process::ShutdownTimeout,
                   package::{PackageIdent,
                             PackageInstall},
                   service::{HealthCheckInterval,
                             ServiceBind},
                   url::DEFAULT_BLDR_URL,
                   util::serde_string,
                   ChannelIdent};
use habitat_sup_protocol::{self,
                           net};
use serde::{self,
            Deserialize};
use std::{collections::HashSet,
          convert::TryFrom,
          fmt,
          fs::{self,
               File},
          io::{BufReader,
               Read},
          iter::FromIterator,
          path::{Path,
                 PathBuf},
          result,
          str::FromStr};
use toml;

static DEFAULT_GROUP: &str = "default";
const SPEC_FILE_EXT: &str = "spec";

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DesiredState {
    Down,
    Up,
}

impl Default for DesiredState {
    fn default() -> DesiredState { DesiredState::Up }
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
    #[serde(with = "serde_string")]
    pub ident: PackageIdent,
    pub group: String,
    pub bldr_url: String,
    pub channel: ChannelIdent,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub binds: Vec<ServiceBind>,
    pub binding_mode: BindingMode,
    pub config_from: Option<PathBuf>,
    #[serde(with = "serde_string")]
    pub desired_state: DesiredState,
    pub shutdown_timeout: Option<ShutdownTimeout>,
    pub svc_encrypted_password: Option<String>,
    // it is important that the health check interval
    // is the last field to be serialized because it
    // is serialized as a table. Individual values
    // serialized after the health check interval will
    // break the parser.
    // Note that there is an issue to ultimately fix this:
    // https://github.com/habitat-sh/habitat/issues/6469
    // and eliminate the need to keep this field last.
    pub health_check_interval: HealthCheckInterval,
}

impl ServiceSpec {
    pub fn new(ident: PackageIdent) -> Self {
        Self { ident,
               group: DEFAULT_GROUP.to_string(),
               bldr_url: DEFAULT_BLDR_URL.to_string(),
               channel: ChannelIdent::stable(),
               topology: Topology::default(),
               update_strategy: UpdateStrategy::default(),
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
        let mut svc_binds: HashSet<&str> =
            HashSet::from_iter(self.binds.iter().map(ServiceBind::name));
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
        }
        if let Some(topology) = svc_load.topology {
            if let Some(topology) = Topology::from_i32(topology) {
                self.topology = topology;
            }
        }
        if let Some(update_strategy) = svc_load.update_strategy {
            if let Some(update_strategy) = UpdateStrategy::from_i32(update_strategy) {
                self.update_strategy = update_strategy;
            }
        }
        if let Some(list) = svc_load.binds {
            self.binds = list.binds
                             .into_iter()
                             .map(|pb| ServiceBind::new(&pb.name, pb.service_group.into()))
                             .collect();
        }
        if let Some(binding_mode) = svc_load.binding_mode {
            if let Some(binding_mode) = BindingMode::from_i32(binding_mode) {
                self.binding_mode = binding_mode;
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
              path::{Path,
                     PathBuf},
              str::FromStr};
    use tempfile::TempDir;

    use habitat_core::{package::PackageIdent,
                       service::HealthCheckInterval};

    use super::*;
    use crate::error::Error::*;

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
            application_environment = "theinternet.preprod"
            bldr_url = "http://example.com/depot"
            topology = "leader"
            update_strategy = "rolling"
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
            application_environment = "theinternet.preprod"
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
            application_environment = "theinternet.preprod"
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
            PackageIdent::new("test-bind",
                              "test-bind",
                              Some("0.1.0"),
                              Some("20190219230309"))
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
}
