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

use super::{BindingMode, Topology, UpdateStrategy};
use crate::error::{Error, Result, SupError};
use crate::hcore::channel::STABLE_CHANNEL;
use crate::hcore::package::{PackageIdent, PackageInstall};
use crate::hcore::service::{ApplicationEnvironment, HealthCheckInterval, ServiceGroup};
use crate::hcore::url::DEFAULT_BLDR_URL;
use crate::hcore::util::{deserialize_using_from_str, serialize_using_to_string};
use crate::hcore::ChannelIdent;
use crate::protocol;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{self, Deserialize};
use std::collections::HashSet;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, Read, Write};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;
use std::time::Duration;
use toml;

static LOGKEY: &str = "SS";
static DEFAULT_GROUP: &str = "default";
const SPEC_FILE_EXT: &str = "spec";

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum DesiredState {
    Down,
    Up,
}

impl Default for DesiredState {
    fn default() -> DesiredState {
        DesiredState::Up
    }
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
    type Err = SupError;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "down" => Ok(DesiredState::Down),
            "up" => Ok(DesiredState::Up),
            _ => Err(sup_error!(Error::BadDesiredState(value.to_string()))),
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

pub fn deserialize_application_environment<'de, D>(
    d: D,
) -> result::Result<Option<ApplicationEnvironment>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(d)?;
    if let Some(s) = s {
        Ok(Some(
            FromStr::from_str(&s).map_err(serde::de::Error::custom)?,
        ))
    } else {
        Ok(None)
    }
}

pub trait IntoServiceSpec {
    fn into_spec(&self, spec: &mut ServiceSpec);
}

impl IntoServiceSpec for protocol::ctl::SvcLoad {
    fn into_spec(&self, spec: &mut ServiceSpec) {
        spec.ident = self.ident.clone().unwrap().into();
        spec.group = self
            .group
            .clone()
            .unwrap_or_else(|| DEFAULT_GROUP.to_string());
        if let Some(ref app_env) = self.application_environment {
            spec.application_environment = Some(app_env.clone().into());
        }
        if let Some(ref bldr_url) = self.bldr_url {
            spec.bldr_url = bldr_url.to_string();
        }
        if let Some(ref channel) = self.bldr_channel {
            spec.channel = channel.to_string();
        }
        if let Some(topology) = self.topology {
            spec.topology = Topology::from_i32(topology).unwrap_or_default();
        }
        if let Some(update_strategy) = self.update_strategy {
            spec.update_strategy = UpdateStrategy::from_i32(update_strategy).unwrap_or_default();
        }
        if let Some(ref list) = self.binds {
            spec.binds = list.binds.clone().into_iter().map(Into::into).collect();
        }
        if let Some(binding_mode) = self.binding_mode {
            spec.binding_mode = BindingMode::from_i32(binding_mode).unwrap_or_default();
        }
        if let Some(ref config_from) = self.config_from {
            spec.config_from = Some(PathBuf::from(config_from));
        }
        if let Some(ref svc_encrypted_password) = self.svc_encrypted_password {
            spec.svc_encrypted_password = Some(svc_encrypted_password.to_string());
        }
        if let Some(ref interval) = self.health_check_interval {
            spec.health_check_interval = Duration::from_secs(interval.seconds).into()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(default)]
pub struct ServiceSpec {
    #[serde(
        deserialize_with = "deserialize_using_from_str",
        serialize_with = "serialize_using_to_string"
    )]
    pub ident: PackageIdent,
    pub group: String,
    #[serde(
        deserialize_with = "deserialize_application_environment",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_environment: Option<ApplicationEnvironment>,
    pub bldr_url: String,
    pub channel: String,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub binds: Vec<ServiceBind>,
    pub binding_mode: BindingMode,
    pub config_from: Option<PathBuf>,
    #[serde(
        deserialize_with = "deserialize_using_from_str",
        serialize_with = "serialize_using_to_string"
    )]
    pub desired_state: DesiredState,
    pub health_check_interval: HealthCheckInterval,
    pub svc_encrypted_password: Option<String>,
}

impl ServiceSpec {
    pub fn default_for(ident: PackageIdent) -> Self {
        let mut spec = Self::default();
        spec.ident = ident;
        spec
    }

    fn to_toml_string(&self) -> Result<String> {
        if self.ident == PackageIdent::default() {
            return Err(sup_error!(Error::MissingRequiredIdent));
        }
        toml::to_string(self).map_err(|err| sup_error!(Error::ServiceSpecRender(err)))
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(&path).map_err(|err| {
            sup_error!(Error::ServiceSpecFileIO(path.as_ref().to_path_buf(), err))
        })?;
        let mut file = BufReader::new(file);
        let mut buf = String::new();
        file.read_to_string(&mut buf).map_err(|err| {
            sup_error!(Error::ServiceSpecFileIO(path.as_ref().to_path_buf(), err))
        })?;
        Self::from_str(&buf)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        debug!(
            "Writing service spec to '{}': {:?}",
            path.as_ref().display(),
            &self
        );
        let dst_path = path
            .as_ref()
            .parent()
            .expect("Cannot determine parent directory for service spec");
        let tmpfile = path.as_ref().with_extension(
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(8)
                .collect::<String>(),
        );
        fs::create_dir_all(dst_path).map_err(|err| {
            sup_error!(Error::ServiceSpecFileIO(path.as_ref().to_path_buf(), err))
        })?;
        // Release the write file handle before the end of the function since we're done
        {
            let mut file = File::create(&tmpfile)
                .map_err(|err| sup_error!(Error::ServiceSpecFileIO(tmpfile.to_path_buf(), err)))?;
            let toml = self.to_toml_string()?;
            file.write_all(toml.as_bytes())
                .map_err(|err| sup_error!(Error::ServiceSpecFileIO(tmpfile.to_path_buf(), err)))?;
        }
        fs::rename(&tmpfile, path.as_ref()).map_err(|err| {
            sup_error!(Error::ServiceSpecFileIO(path.as_ref().to_path_buf(), err))
        })?;

        Ok(())
    }

    pub fn file_name(&self) -> String {
        format!("{}.{}", &self.ident.name, SPEC_FILE_EXT)
    }

    pub fn validate(&self, package: &PackageInstall) -> Result<()> {
        self.validate_binds(package)?;
        Ok(())
    }

    /// Validates that all required package binds are present in service binds and all remaining
    /// service binds are optional package binds.
    ///
    /// # Errors
    ///
    /// * If any required required package binds are missing in service binds
    /// * If any given service binds are in neither required nor optional package binds
    fn validate_binds(&self, package: &PackageInstall) -> Result<()> {
        let mut svc_binds: HashSet<String> =
            HashSet::from_iter(self.binds.iter().cloned().map(|b| b.name));

        let mut missing_req_binds = Vec::new();
        // Remove each service bind that matches a required package bind. If a required package
        // bind is not found, add the bind to the missing list to return an `Err`.
        for req_bind in package.binds()?.iter().map(|b| &b.service) {
            if svc_binds.contains(req_bind) {
                svc_binds.remove(req_bind);
            } else {
                missing_req_binds.push(req_bind.clone());
            }
        }
        // If we have missing required binds, return an `Err`.
        if !missing_req_binds.is_empty() {
            return Err(sup_error!(Error::MissingRequiredBind(missing_req_binds)));
        }

        // Remove each service bind that matches an optional package bind.
        for opt_bind in package.binds_optional()?.iter().map(|b| &b.service) {
            if svc_binds.contains(opt_bind) {
                svc_binds.remove(opt_bind);
            }
        }
        // If we have remaining service binds then they are neither required nor optional package
        // binds. In this case, return an `Err`.
        if !svc_binds.is_empty() {
            return Err(sup_error!(Error::InvalidBinds(
                svc_binds.into_iter().collect()
            )));
        }

        Ok(())
    }
}

impl Default for ServiceSpec {
    fn default() -> Self {
        ServiceSpec {
            ident: PackageIdent::default(),
            group: DEFAULT_GROUP.to_string(),
            application_environment: None,
            bldr_url: DEFAULT_BLDR_URL.to_string(),
            channel: ChannelIdent::stable().to_string(),
            topology: Topology::default(),
            update_strategy: UpdateStrategy::default(),
            binds: Vec::default(),
            binding_mode: BindingMode::Strict,
            config_from: None,
            desired_state: DesiredState::default(),
            health_check_interval: HealthCheckInterval::default(),
            svc_encrypted_password: None,
        }
    }
}

impl FromStr for ServiceSpec {
    type Err = SupError;

    fn from_str(toml: &str) -> result::Result<Self, Self::Err> {
        let spec: ServiceSpec =
            toml::from_str(toml).map_err(|e| sup_error!(Error::ServiceSpecParse(e)))?;
        if spec.ident == PackageIdent::default() {
            return Err(sup_error!(Error::MissingRequiredIdent));
        }
        Ok(spec)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ServiceBind {
    pub name: String,
    pub service_group: ServiceGroup,
}

impl FromStr for ServiceBind {
    type Err = SupError;

    fn from_str(bind_str: &str) -> result::Result<Self, Self::Err> {
        let values: Vec<&str> = bind_str.split(':').collect();
        if values.len() == 2 {
            Ok(ServiceBind {
                name: values[0].to_string(),
                service_group: ServiceGroup::from_str(values[1])?,
            })
        } else {
            Err(sup_error!(Error::InvalidBinding(bind_str.to_string())))
        }
    }
}

impl fmt::Display for ServiceBind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.service_group)
    }
}

impl<'de> serde::Deserialize<'de> for ServiceBind {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize_using_from_str(deserializer)
    }
}

impl serde::Serialize for ServiceBind {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<ServiceBind> for protocol::types::ServiceBind {
    fn from(bind: ServiceBind) -> Self {
        let mut proto = protocol::types::ServiceBind::default();
        proto.name = bind.name;
        proto.service_group = bind.service_group.into();
        proto
    }
}

impl Into<ServiceBind> for protocol::types::ServiceBind {
    fn into(self) -> ServiceBind {
        ServiceBind {
            name: self.name,
            service_group: self.service_group.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use std::io::{BufReader, Read, Write};
    use std::path::{Path, PathBuf};
    use std::str::FromStr;

    use crate::hcore::error::Error as HError;
    use crate::hcore::package::PackageIdent;
    use crate::hcore::service::{ApplicationEnvironment, HealthCheckInterval, ServiceGroup};
    use tempfile::TempDir;
    use toml;

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

        assert_eq!(
            spec.ident,
            PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap()
        );
        assert_eq!(spec.group, String::from("jobs"));
        assert_eq!(
            spec.application_environment,
            Some(ApplicationEnvironment::from_str("theinternet.preprod").unwrap(),)
        );
        assert_eq!(spec.bldr_url, String::from("http://example.com/depot"));
        assert_eq!(spec.topology, Topology::Leader);
        assert_eq!(spec.update_strategy, UpdateStrategy::Rolling);
        assert_eq!(
            spec.binds,
            vec![
                ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                ServiceBind::from_str("db:postgres.app@acmecorp").unwrap(),
            ]
        );
        assert_eq!(
            spec.config_from,
            Some(PathBuf::from("/only/for/development"))
        );
        assert_eq!(
            spec.health_check_interval,
            HealthCheckInterval::from_str("5").unwrap()
        );
    }

    #[test]
    fn service_spec_from_str_missing_ident() {
        let toml = r#""#;

        match ServiceSpec::from_str(toml) {
            Err(e) => match e.err {
                MissingRequiredIdent => assert!(true),
                e => panic!("Unexpected error returned: {:?}", e),
            },
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
            Err(e) => match e.err {
                ServiceSpecParse(_) => assert!(true),
                e => panic!("Unexpected error returned: {:?}", e),
            },
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
            Err(e) => match e.err {
                ServiceSpecParse(_) => assert!(true),
                e => panic!("Unexpected error returned: {:?}", e),
            },
            Ok(_) => panic!("Spec TOML should fail to parse"),
        }
    }

    #[test]
    fn service_spec_to_toml_string() {
        let spec = ServiceSpec {
            ident: PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap(),
            group: String::from("jobs"),
            application_environment: Some(
                ApplicationEnvironment::from_str("theinternet.preprod").unwrap(),
            ),
            bldr_url: String::from("http://example.com/depot"),
            channel: String::from("unstable"),
            topology: Topology::Leader,
            update_strategy: UpdateStrategy::AtOnce,
            binds: vec![
                ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                ServiceBind::from_str("db:postgres.app@acmecorp").unwrap(),
            ],
            binding_mode: BindingMode::Relaxed,
            health_check_interval: HealthCheckInterval::from_str("123").unwrap(),
            config_from: Some(PathBuf::from("/only/for/development")),
            desired_state: DesiredState::Down,
            svc_encrypted_password: None,
        };
        let toml = spec.to_toml_string().unwrap();

        assert!(toml.contains(r#"ident = "origin/name/1.2.3/20170223130020""#,));
        assert!(toml.contains(r#"group = "jobs""#));
        assert!(toml.contains(r#"application_environment = "theinternet.preprod""#,));
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
    }

    #[test]
    fn service_spec_to_toml_string_invalid_ident() {
        // Remember: the default implementation of `PackageIdent` is an invalid identifier, missing
        // origin and name--we're going to exploit this here
        let spec = ServiceSpec::default();

        match spec.to_toml_string() {
            Err(e) => match e.err {
                MissingRequiredIdent => assert!(true),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
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

        assert_eq!(
            spec.ident,
            PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap()
        );
        assert_eq!(spec.group, String::from("jobs"));
        assert_eq!(spec.bldr_url, String::from("http://example.com/depot"));
        assert_eq!(spec.topology, Topology::Leader);
        assert_eq!(
            spec.application_environment,
            Some(ApplicationEnvironment::from_str("theinternet.preprod").unwrap(),)
        );
        assert_eq!(spec.update_strategy, UpdateStrategy::Rolling);
        assert_eq!(
            spec.binds,
            vec![
                ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                ServiceBind::from_str("db:postgres.app@acmecorp").unwrap(),
            ]
        );
        assert_eq!(&spec.channel, "stable");
        assert_eq!(
            spec.config_from,
            Some(PathBuf::from("/only/for/development"))
        );

        assert_eq!(
            spec.binding_mode,
            BindingMode::Strict,
            "Strict is the default mode, if nothing was previously specified."
        );
        assert_eq!(
            spec.health_check_interval,
            HealthCheckInterval::from_str("5").unwrap()
        );
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
            Err(e) => match e.err {
                ServiceSpecFileIO(p, _) => assert_eq!(path, p),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("File should not exist for read"),
        }
    }

    #[test]
    fn service_spec_from_file_empty() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("empty.spec");
        file_from_str(&path, "");

        match ServiceSpec::from_file(&path) {
            Err(e) => match e.err {
                MissingRequiredIdent => assert!(true),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("File should not exist for read"),
        }
    }

    #[test]
    fn service_spec_from_file_bad_contents() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("bad.spec");
        file_from_str(&path, "You're gonna have a bad time");

        match ServiceSpec::from_file(&path) {
            Err(e) => match e.err {
                ServiceSpecParse(_) => assert!(true),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("File should not exist for read"),
        }
    }

    #[test]
    fn service_spec_to_file() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("name.spec");
        let spec = ServiceSpec {
            ident: PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap(),
            group: String::from("jobs"),
            application_environment: Some(
                ApplicationEnvironment::from_str("theinternet.preprod").unwrap(),
            ),
            bldr_url: String::from("http://example.com/depot"),
            channel: String::from("unstable"),
            topology: Topology::Leader,
            update_strategy: UpdateStrategy::AtOnce,
            binds: vec![
                ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                ServiceBind::from_str("db:postgres.app@acmecorp").unwrap(),
            ],
            binding_mode: BindingMode::Relaxed,
            health_check_interval: HealthCheckInterval::from_str("23").unwrap(),
            config_from: Some(PathBuf::from("/only/for/development")),
            desired_state: DesiredState::Down,
            svc_encrypted_password: None,
        };
        spec.to_file(&path).unwrap();
        let toml = string_from_file(path);

        assert!(toml.contains(r#"ident = "origin/name/1.2.3/20170223130020""#,));
        assert!(toml.contains(r#"group = "jobs""#));
        assert!(toml.contains(r#"application_environment = "theinternet.preprod""#,));
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
        let spec = ServiceSpec::default();

        match spec.to_file(path) {
            Err(e) => match e.err {
                MissingRequiredIdent => assert!(true),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("Service spec file should not have been written"),
        }
    }

    #[test]
    fn service_spec_file_name() {
        let spec = ServiceSpec::default_for(PackageIdent::from_str("origin/hoopa/1.2.3").unwrap());

        assert_eq!(String::from("hoopa.spec"), spec.file_name());
    }

    #[test]
    fn service_bind_from_str() {
        let bind_str = "name:app.env#service.group@organization";
        let bind = ServiceBind::from_str(bind_str).unwrap();

        assert_eq!(bind.name, String::from("name"));
        assert_eq!(
            bind.service_group,
            ServiceGroup::from_str("app.env#service.group@organization").unwrap()
        );
    }

    #[test]
    fn service_bind_from_str_simple() {
        let bind_str = "name:service.group";
        let bind = ServiceBind::from_str(bind_str).unwrap();

        assert_eq!(bind.name, String::from("name"));
        assert_eq!(
            bind.service_group,
            ServiceGroup::from_str("service.group").unwrap()
        );
    }

    #[test]
    fn service_bind_from_str_missing_colon() {
        let bind_str = "uhoh";

        match ServiceBind::from_str(bind_str) {
            Err(e) => match e.err {
                InvalidBinding(val) => assert_eq!("uhoh", val),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str_too_many_colons() {
        let bind_str = "uhoh:this:is:bad";

        match ServiceBind::from_str(bind_str) {
            Err(e) => match e.err {
                InvalidBinding(val) => assert_eq!("uhoh:this:is:bad", val),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str_invalid_service_group() {
        let bind_str = "uhoh:nosuchservicegroup@nope";

        match ServiceBind::from_str(bind_str) {
            Err(e) => match e.err {
                HabitatCore(HError::InvalidServiceGroup(val)) => {
                    assert_eq!("nosuchservicegroup@nope", val)
                }
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_to_string() {
        let bind = ServiceBind {
            name: String::from("name"),
            service_group: ServiceGroup::from_str("service.group").unwrap(),
        };

        assert_eq!("name:service.group", bind.to_string());
    }

    #[test]
    fn service_bind_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: ServiceBind,
        }
        let toml = r#"
            key = "name:app.env#service.group@organization"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(
            data.key,
            ServiceBind::from_str("name:app.env#service.group@organization").unwrap()
        );
    }

    #[test]
    fn service_bind_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: ServiceBind,
        }
        let data = Data {
            key: ServiceBind {
                name: String::from("name"),
                service_group: ServiceGroup::from_str("service.group").unwrap(),
            },
        };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "name:service.group""#));
    }
}
