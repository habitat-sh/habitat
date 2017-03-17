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

use std::path::PathBuf;
use std::result;
use std::str::FromStr;

use hcore::package::{PackageIdent, PackageInstall};
use hcore::service::ServiceGroup;
use hcore::url::DEFAULT_DEPOT_URL;
use hcore::util::deserialize_using_from_str;
use serde;
use toml;

use super::{Topology, UpdateStrategy};
use error::{Error, Result, SupError};

static LOGKEY: &'static str = "SS";
static DEFAULT_GROUP: &'static str = "default";

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ServiceSpec {
    #[serde(deserialize_with = "deserialize_using_from_str")]
    pub ident: PackageIdent,
    pub group: String,
    pub organization: Option<String>,
    pub depot_url: String,
    pub topology: Topology,
    pub update_strategy: UpdateStrategy,
    pub binds: Vec<ServiceBind>,
    #[serde(skip_deserializing)]
    pub config_from: Option<PathBuf>,
}

impl Default for ServiceSpec {
    fn default() -> Self {
        ServiceSpec {
            ident: PackageIdent::default(),
            group: DEFAULT_GROUP.to_string(),
            organization: None,
            depot_url: DEFAULT_DEPOT_URL.to_string(),
            topology: Topology::default(),
            update_strategy: UpdateStrategy::default(),
            binds: vec![],
            config_from: None,
        }
    }
}

impl FromStr for ServiceSpec {
    type Err = SupError;

    fn from_str(toml: &str) -> result::Result<Self, Self::Err> {
        let spec: ServiceSpec = match toml::de::from_str(toml) {
            Ok(s) => s,
            Err(e) => return Err(sup_error!(Error::ServiceSpecParsingError(e.to_string()))),
        };
        if spec.ident == PackageIdent::default() {
            return Err(sup_error!(Error::MissingRequiredIdent));
        }
        Ok(spec)
    }
}

impl ServiceSpec {
    pub fn default_for(ident: PackageIdent) -> Self {
        let mut spec = Self::default();
        spec.ident = ident;
        spec
    }

    pub fn validate(&self, package: &PackageInstall) -> Result<()> {
        self.validate_binds(package)?;
        Ok(())
    }

    fn validate_binds(&self, package: &PackageInstall) -> Result<()> {
        let missing: Vec<String> = package.binds()?
            .into_iter()
            .filter(|bind| {
                        self.binds
                            .iter()
                            .find(|ref service_bind| &bind.service == &service_bind.name)
                            .is_none()
                    })
            .map(|bind| bind.service)
            .collect();
        if !missing.is_empty() {
            return Err(sup_error!(Error::MissingRequiredBind(missing)));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServiceBind {
    pub name: String,
    pub service_group: ServiceGroup,
}

impl FromStr for ServiceBind {
    type Err = SupError;

    fn from_str(bind_str: &str) -> result::Result<Self, Self::Err> {
        let values: Vec<&str> = bind_str.splitn(2, ':').collect();
        if values.len() != 2 {
            return Err(sup_error!(Error::InvalidBinding(bind_str.to_string())));
        }

        Ok(ServiceBind {
               name: values[0].to_string(),
               service_group: ServiceGroup::from_str(values[1])?,
           })
    }
}

impl serde::Deserialize for ServiceBind {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
        where D: serde::Deserializer
    {
        deserialize_using_from_str(deserializer)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use hcore::error::Error as HError;
    use hcore::package::PackageIdent;
    use hcore::service::ServiceGroup;
    use toml;

    use super::{ServiceBind, ServiceSpec, Topology, UpdateStrategy};
    use error::Error::*;

    #[test]
    fn service_spec_from_str() {
        let toml = r#"
            ident = "origin/name/1.2.3/20170223130020"
            group = "jobs"
            organization = "acmecorp"
            depot_url = "http://example.com/depot"
            topology = "leader"
            update_strategy = "rolling"
            binds = ["cache:redis.cache@acmecorp", "db:postgres.app@acmecorp"]

            config_from = "should not be parsed"
            extra_stuff = "should be ignored"
            "#;
        let spec = ServiceSpec::from_str(toml).unwrap();

        assert_eq!(spec.ident,
                   PackageIdent::from_str("origin/name/1.2.3/20170223130020").unwrap());
        assert_eq!(spec.group, String::from("jobs"));
        assert_eq!(spec.organization, Some(String::from("acmecorp")));
        assert_eq!(spec.depot_url, String::from("http://example.com/depot"));
        assert_eq!(spec.topology, Topology::Leader);
        assert_eq!(spec.update_strategy, UpdateStrategy::Rolling);
        assert_eq!(spec.binds,
                   vec![ServiceBind::from_str("cache:redis.cache@acmecorp").unwrap(),
                        ServiceBind::from_str("db:postgres.app@acmecorp").unwrap()]);
        assert_eq!(spec.config_from, None);
    }

    #[test]
    fn service_spec_from_str_missing_ident() {
        let toml = r#""#;

        match ServiceSpec::from_str(toml) {
            Err(e) => {
                match e.err {
                    MissingRequiredIdent => assert!(true),
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
                match e.err {
                    ServiceSpecParsingError(_) => assert!(true),
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
                match e.err {
                    ServiceSpecParsingError(_) => assert!(true),
                    e => panic!("Unexpected error returned: {:?}", e),
                }
            }
            Ok(_) => panic!("Spec TOML should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str() {
        let bind_str = "name:service.group@organization";
        let bind = ServiceBind::from_str(bind_str).unwrap();

        assert_eq!(bind.name, String::from("name"));
        assert_eq!(bind.service_group,
                   ServiceGroup::from_str("service.group@organization").unwrap());
    }

    #[test]
    fn service_bind_from_str_simple() {
        let bind_str = "name:service.group";
        let bind = ServiceBind::from_str(bind_str).unwrap();

        assert_eq!(bind.name, String::from("name"));
        assert_eq!(bind.service_group,
                   ServiceGroup::from_str("service.group").unwrap());
    }

    #[test]
    fn service_bind_from_str_missing_colon() {
        let bind_str = "uhoh";

        match ServiceBind::from_str(bind_str) {
            Err(e) => {
                match e.err {
                    InvalidBinding(val) => assert_eq!("uhoh", val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str_too_many_colons() {
        let bind_str = "uhoh:this:is:bad";

        match ServiceBind::from_str(bind_str) {
            Err(e) => {
                match e.err {
                    HabitatCore(HError::InvalidServiceGroup(val)) => assert_eq!("this:is:bad", val),
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_from_str_invalid_service_group() {
        let bind_str = "uhoh:nosuchservicegroup@nope";

        match ServiceBind::from_str(bind_str) {
            Err(e) => {
                match e.err {
                    HabitatCore(HError::InvalidServiceGroup(val)) => {
                        assert_eq!("nosuchservicegroup@nope", val)
                    }
                    wrong => panic!("Unexpected error returned: {:?}", wrong),
                }
            }
            Ok(_) => panic!("String should fail to parse"),
        }
    }

    #[test]
    fn service_bind_toml_deserialize() {
        #[derive(Deserialize)]
        struct Data {
            pub key: ServiceBind,
        }
        let toml = r#"
            key = "name:service.group@organization"
            "#;
        let data: Data = toml::de::from_str(toml).unwrap();

        assert_eq!(data.key,
                   ServiceBind::from_str("name:service.group@organization").unwrap());
    }
}
