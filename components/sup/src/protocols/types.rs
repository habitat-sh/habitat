// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! Generic types shared between the Supervisor and other components in the Habitat software
//! ecosystem.
//!
//! Note: See `protocols/types.proto` for type level documentation for generated types.
//! JW TODO: These types should be moved to the _core crate_ and where they will replace their
//!          vanilla Rust type counterparts that we define there.

use std::fmt;
use std::str::FromStr;

use hcore;
use hcore::package::{self, Identifiable};
use hcore::util::deserialize_using_from_str;
use serde;

pub use super::generated::types::*;
use error::{Error, SupError};
use manager;

static LOGKEY: &'static str = "PT";

impl fmt::Display for ApplicationEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.get_application(), self.get_environment())
    }
}

impl fmt::Display for PackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_version() && self.has_release() {
            write!(
                f,
                "{}/{}/{}/{}",
                self.get_origin(),
                self.get_name(),
                self.get_version(),
                self.get_release()
            )
        } else if self.has_version() {
            write!(
                f,
                "{}/{}/{}",
                self.get_origin(),
                self.get_name(),
                self.get_version()
            )
        } else {
            write!(f, "{}/{}", self.get_origin(), self.get_name())
        }
    }
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match *self {
            ProcessState::Down => "down",
            ProcessState::Up => "up",
        };
        write!(f, "{}", state)
    }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut value = format!("{}.{}", self.get_service(), self.get_group());
        if self.has_application_environment() {
            value.insert_str(0, &format!("{}#", self.get_application_environment()));
        }
        if self.has_organization() {
            value.push_str(&format!("@{}", self.get_organization()));
        }
        write!(f, "{}", value)
    }
}

// JW TODO: These trait implementations to provide translations between protocol messages and
// concrete Rust types defined in the core crate will go away eventually. We need to put the
// core crate back into the Supervisor's repository and untangle our dependency hell before
// that can happen.

impl From<hcore::service::ApplicationEnvironment> for ApplicationEnvironment {
    fn from(app_env: hcore::service::ApplicationEnvironment) -> Self {
        let mut proto = ApplicationEnvironment::new();
        proto.set_application(app_env.application().to_string());
        proto.set_environment(app_env.environment().to_string());
        proto
    }
}

impl From<package::PackageIdent> for PackageIdent {
    fn from(ident: package::PackageIdent) -> Self {
        let mut proto = PackageIdent::new();
        proto.set_origin(ident.origin);
        proto.set_name(ident.name);
        if let Some(version) = ident.version {
            proto.set_version(version);
        }
        if let Some(release) = ident.release {
            proto.set_release(release);
        }
        proto
    }
}

impl Into<package::PackageIdent> for PackageIdent {
    fn into(mut self) -> package::PackageIdent {
        let version = if self.has_version() {
            Some(self.take_version())
        } else {
            None
        };
        let release = if self.has_release() {
            Some(self.take_release())
        } else {
            None
        };
        package::PackageIdent::new(self.take_origin(), self.take_name(), version, release)
    }
}

impl From<manager::service::ProcessState> for ProcessState {
    fn from(other: manager::service::ProcessState) -> Self {
        match other {
            manager::service::ProcessState::Down => ProcessState::Down,
            manager::service::ProcessState::Up => ProcessState::Up,
        }
    }
}

impl From<manager::ProcessStatus> for ProcessStatus {
    fn from(other: manager::ProcessStatus) -> Self {
        let mut proto = ProcessStatus::new();
        proto.set_elapsed(other.elapsed.num_seconds());
        proto.set_state(other.state.into());
        if let Some(pid) = other.pid {
            proto.set_pid(pid);
        }
        proto
    }
}

impl From<manager::service::ServiceBind> for ServiceBind {
    fn from(bind: manager::service::ServiceBind) -> Self {
        let mut proto = ServiceBind::new();
        proto.set_name(bind.name);
        proto.set_service_group(bind.service_group.into());
        proto
    }
}

impl From<hcore::service::ServiceGroup> for ServiceGroup {
    fn from(service_group: hcore::service::ServiceGroup) -> Self {
        let mut proto = ServiceGroup::new();
        if let Some(app_env) = service_group.application_environment() {
            proto.set_application_environment(app_env.into());
        }
        proto.set_group(service_group.group().to_string());
        proto.set_service(service_group.service().to_string());
        if let Some(organization) = service_group.org() {
            proto.set_organization(organization.to_string());
        }
        proto
    }
}

impl From<manager::ServiceStatus> for ServiceStatus {
    fn from(other: manager::ServiceStatus) -> Self {
        let mut proto = ServiceStatus::new();
        proto.set_ident(other.pkg.ident.into());
        proto.set_process(other.process.into());
        proto.set_service_group(other.service_group.into());
        if let Some(composite) = other.composite {
            proto.set_composite(composite);
        }
        proto
    }
}

impl Into<manager::service::ServiceBind> for ServiceBind {
    fn into(mut self) -> manager::service::ServiceBind {
        let service_name = if self.has_service_name() {
            Some(self.take_service_name())
        } else {
            None
        };
        manager::service::ServiceBind {
            name: self.take_name(),
            service_group: self.take_service_group().into(),
            service_name: service_name,
        }
    }
}

impl Into<hcore::service::ServiceGroup> for ServiceGroup {
    fn into(mut self) -> hcore::service::ServiceGroup {
        let app_env = if self.has_application_environment() {
            Some(
                hcore::service::ApplicationEnvironment::new(
                    self.get_application_environment().get_application(),
                    self.get_application_environment().get_environment(),
                ).unwrap(),
            )
        } else {
            None
        };
        let service = self.take_service();
        let group = self.take_group();
        let organization = if self.has_organization() {
            Some(self.get_organization())
        } else {
            None
        };
        hcore::service::ServiceGroup::new(app_env.as_ref(), service, group, organization).unwrap()
    }
}

impl Identifiable for PackageIdent {
    fn origin(&self) -> &str {
        self.get_origin()
    }

    fn name(&self) -> &str {
        self.get_name()
    }

    fn version(&self) -> Option<&str> {
        if self.has_version() {
            Some(self.get_version())
        } else {
            None
        }
    }

    fn release(&self) -> Option<&str> {
        if self.has_release() {
            Some(self.get_release())
        } else {
            None
        }
    }
}

impl Topology {
    fn as_str(&self) -> &str {
        match *self {
            Topology::Leader => "leader",
            Topology::Standalone => "standalone",
        }
    }
}

impl FromStr for Topology {
    type Err = SupError;

    fn from_str(topology: &str) -> Result<Self, Self::Err> {
        match topology {
            "leader" => Ok(Topology::Leader),
            "standalone" => Ok(Topology::Standalone),
            _ => Err(sup_error!(Error::InvalidTopology(String::from(topology)))),
        }
    }
}

impl fmt::Display for Topology {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Topology {
    fn default() -> Topology {
        Topology::Standalone
    }
}

impl<'de> serde::Deserialize<'de> for Topology {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize_using_from_str(deserializer)
    }
}

impl serde::Serialize for Topology {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl UpdateStrategy {
    fn as_str(&self) -> &str {
        match *self {
            UpdateStrategy::None => "none",
            UpdateStrategy::AtOnce => "at-once",
            UpdateStrategy::Rolling => "rolling",
        }
    }
}

impl FromStr for UpdateStrategy {
    type Err = SupError;

    fn from_str(strategy: &str) -> Result<Self, Self::Err> {
        match strategy {
            "none" => Ok(UpdateStrategy::None),
            "at-once" => Ok(UpdateStrategy::AtOnce),
            "rolling" => Ok(UpdateStrategy::Rolling),
            _ => Err(sup_error!(
                Error::InvalidUpdateStrategy(String::from(strategy))
            )),
        }
    }
}

impl fmt::Display for UpdateStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for UpdateStrategy {
    fn default() -> UpdateStrategy {
        UpdateStrategy::None
    }
}

impl<'de> serde::Deserialize<'de> for UpdateStrategy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize_using_from_str(deserializer)
    }
}

impl serde::Serialize for UpdateStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use toml;

    use super::{Topology, UpdateStrategy};
    use error::Error::*;

    #[test]
    fn topology_default() {
        // This should always be the default topology, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(Topology::default(), Topology::Standalone);
    }

    #[test]
    fn topology_from_str() {
        assert_eq!(Topology::from_str("leader").unwrap(), Topology::Leader);
        assert_eq!(
            Topology::from_str("standalone").unwrap(),
            Topology::Standalone
        );
    }

    #[test]
    fn topology_from_str_invalid() {
        let topology_str = "dope";

        match Topology::from_str(topology_str) {
            Err(e) => {
                match e.err {
                    InvalidTopology(s) => assert_eq!("dope", s),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),

        }
    }

    #[test]
    fn topology_to_string() {
        assert_eq!("standalone", Topology::Standalone.to_string());
        assert_eq!("leader", Topology::Leader.to_string());
    }

    #[test]
    fn topology_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: Topology,
        }
        let toml = r#"
            key = "leader"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, Topology::Leader);
    }

    #[test]
    fn topology_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: Topology,
        }
        let data = Data { key: Topology::Leader };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "leader""#))
    }

    #[test]
    fn update_strategy_default() {
        // This should always be the default update strategy, if this default gets changed, we have
        // a failing test to confirm we changed our minds
        assert_eq!(UpdateStrategy::default(), UpdateStrategy::None);
    }

    #[test]
    fn update_strategy_from_str() {
        let strategy_str = "at-once";
        let strategy = UpdateStrategy::from_str(strategy_str).unwrap();

        assert_eq!(strategy, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_from_str_invalid() {
        let strategy_str = "dope";

        match UpdateStrategy::from_str(strategy_str) {
            Err(e) => {
                match e.err {
                    InvalidUpdateStrategy(s) => assert_eq!("dope", s),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),

        }
    }

    #[test]
    fn update_strategy_to_string() {
        let strategy = UpdateStrategy::AtOnce;

        assert_eq!("at-once", strategy.to_string())
    }

    #[test]
    fn update_strategy_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let toml = r#"
            key = "at-once"
            "#;
        let data: Data = toml::from_str(toml).unwrap();

        assert_eq!(data.key, UpdateStrategy::AtOnce);
    }

    #[test]
    fn update_strategy_toml_serialize() {
        #[derive(Serialize)]
        struct Data {
            key: UpdateStrategy,
        }
        let data = Data { key: UpdateStrategy::AtOnce };
        let toml = toml::to_string(&data).unwrap();

        assert!(toml.starts_with(r#"key = "at-once""#));
    }
}
