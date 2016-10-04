// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

extern crate habitat_common as hcommon;
extern crate habitat_core as hcore;
#[macro_use]
extern crate habitat_sup as hsup;
#[macro_use]
extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rustc_serialize;
extern crate time;
extern crate toml;
extern crate wonder;

pub mod config;
pub mod error;
pub mod task;
pub mod controller;

pub use self::config::Config;
pub use self::error::{Error, Result};

use std::collections::HashMap;
use std::fmt;
use std::result;
use std::str::FromStr;

use hcore::package::PackageIdent;
use hcore::service::ServiceGroup;

/// ServiceDef is a combination of a PackageIdent and ServiceGroup
/// that a user has specified via config file. It represents
/// what the user wants to run, along with command line args and
/// any other params that a user can tweak via config.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceDef {
    pub ident: PackageIdent,
    pub service_group: ServiceGroup,
    pub cli_args: Option<String>,
    pub env: HashMap<String, String>,
}

impl ServiceDef {
    pub fn new(ident: PackageIdent, service_group: ServiceGroup) -> ServiceDef {
        ServiceDef {
            ident: ident,
            service_group: service_group,
            cli_args: None,
            env: HashMap::new(),
        }
    }
}

impl fmt::Display for ServiceDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}.{}.{}{}",
               &self.ident.origin,
               &self.ident.name,
               &self.service_group.group,
               &self.service_group.dotted_org_or_empty())
    }
}

impl AsRef<ServiceDef> for ServiceDef {
    fn as_ref(&self) -> &ServiceDef {
        self
    }
}

impl FromStr for ServiceDef {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let chunks: Vec<&str> = value.split(".").collect();
        let (origin, name, group, org) = match chunks.len() {
            3 => (chunks[0], chunks[1], chunks[2], None),
            4 => (chunks[0], chunks[1], chunks[2], Some(chunks[3].to_string())),
            _ => return Err(Error::DirectorError(format!("Invalid service descriptor: {}", value))),
        };

        let ident = PackageIdent::new(origin, name, None, None);
        let sg = ServiceGroup::new(name, group, org);
        let sd = ServiceDef::new(ident, sg);
        Ok(sd)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_parse_service_without_org() {
        let sd = ServiceDef::from_str("core.redis.somegroup").unwrap();
        assert_eq!("core.redis.somegroup", sd.to_string());
        assert_eq!("core", sd.ident.origin);
        assert_eq!("redis", sd.ident.name);
        assert_eq!(None, sd.ident.version);
        assert_eq!(None, sd.ident.release);
        assert_eq!("redis", sd.service_group.service);
        assert_eq!("somegroup", sd.service_group.group);
        assert_eq!(None, sd.service_group.organization);
    }

    #[test]
    fn test_parse_service_org() {
        let sd = ServiceDef::from_str("core.redis.somegroup.someorg").unwrap();
        assert_eq!("core.redis.somegroup.someorg", sd.to_string());
        assert_eq!("core", sd.ident.origin);
        assert_eq!("redis", sd.ident.name);
        assert_eq!(None, sd.ident.version);
        assert_eq!(None, sd.ident.release);
        assert_eq!("redis", sd.service_group.service);
        assert_eq!("somegroup", sd.service_group.group);
        assert_eq!(Some("someorg".to_string()), sd.service_group.organization);


    }

    #[test]
    fn test_parse_bad_service_desc() {
        assert!(ServiceDef::from_str("").is_err());
        assert!(ServiceDef::from_str("x").is_err());
        assert!(ServiceDef::from_str("x:y").is_err());
        assert!(ServiceDef::from_str("a.b").is_err());
        assert!(ServiceDef::from_str("a.b.c.d.e").is_err());
    }
}
