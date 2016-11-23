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

//! The ServiceConfig rumor.
//!
//! Holds the toml configuration injected for a service.

use std::cmp::Ordering;
use std::mem;
use std::ops::{Deref, DerefMut};

use habitat_core::crypto::{BoxKeyPair, default_cache_key_path};
use habitat_core::service::ServiceGroup;
use protobuf::Message;

use error::{Error, Result};
use message::swim::{ServiceConfig as ProtoServiceConfig, Rumor as ProtoRumor,
                    Rumor_Type as ProtoRumor_Type};
use rumor::Rumor;

/// The service rumor
#[derive(Debug, Clone, RustcEncodable)]
pub struct ServiceConfig {
    pub proto: ProtoRumor,
}

impl PartialOrd for ServiceConfig {
    fn partial_cmp(&self, other: &ServiceConfig) -> Option<Ordering> {
        if self.get_service_group() != other.get_service_group() {
            None
        } else {
            Some(self.get_incarnation().cmp(&other.get_incarnation()))
        }
    }
}

impl PartialEq for ServiceConfig {
    fn eq(&self, other: &ServiceConfig) -> bool {
        self.get_service_group() == other.get_service_group() &&
        self.get_incarnation() == other.get_incarnation() &&
        self.get_encrypted() == other.get_encrypted() &&
        self.get_config() == other.get_config()
    }
}

impl From<ProtoRumor> for ServiceConfig {
    fn from(pr: ProtoRumor) -> ServiceConfig {
        ServiceConfig { proto: pr }
    }
}

impl From<ServiceConfig> for ProtoRumor {
    fn from(service_config: ServiceConfig) -> ProtoRumor {
        service_config.proto
    }
}

impl Deref for ServiceConfig {
    type Target = ProtoServiceConfig;

    fn deref(&self) -> &ProtoServiceConfig {
        self.proto.get_service_config()
    }
}

impl DerefMut for ServiceConfig {
    fn deref_mut(&mut self) -> &mut ProtoServiceConfig {
        self.proto.mut_service_config()
    }
}

impl ServiceConfig {
    /// Creates a new ServiceConfig.
    pub fn new<S1>(member_id: S1, service_group: ServiceGroup, config: Vec<u8>) -> Self
        where S1: Into<String>
    {
        let mut rumor = ProtoRumor::new();
        let from_id = member_id.into();
        rumor.set_from_id(from_id);
        rumor.set_field_type(ProtoRumor_Type::ServiceConfig);

        let mut proto = ProtoServiceConfig::new();
        proto.set_service_group(format!("{}", service_group));
        proto.set_incarnation(0);
        proto.set_config(config);

        rumor.set_service_config(proto);
        ServiceConfig { proto: rumor }
    }

    pub fn encrypt(&mut self, user_pair: &BoxKeyPair, service_pair: &BoxKeyPair) -> Result<()> {
        let config = self.take_config();
        let encrypted_config = try!(user_pair.encrypt(&config, service_pair));
        self.set_config(encrypted_config);
        self.set_encrypted(true);
        Ok(())
    }

    pub fn config(&self) -> Result<String> {
        if self.get_encrypted() {
            let bytes = try!(BoxKeyPair::decrypt(self.get_config(), &default_cache_key_path(None)));
            let config = try!(String::from_utf8(bytes).map_err(Error::ServiceConfigNotUtf8));
            Ok(config)
        } else {
            let config = try!(String::from_utf8(self.get_config().to_vec())
                .map_err(Error::ServiceConfigNotUtf8));
            Ok(config)
        }
    }
}

impl Rumor for ServiceConfig {
    /// Follows a simple pattern; if we have a newer incarnation than the one we already have, the
    /// new one wins. So far, these never change.
    fn merge(&mut self, mut other: ServiceConfig) -> bool {
        if *self >= other {
            false
        } else {
            mem::swap(self, &mut other);
            true
        }
    }

    fn kind(&self) -> ProtoRumor_Type {
        ProtoRumor_Type::ServiceConfig
    }

    fn id(&self) -> &str {
        "service_config"
    }

    fn key(&self) -> &str {
        self.get_service_group()
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>> {
        Ok(try!(self.proto.write_to_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use habitat_core::service::ServiceGroup;

    use super::ServiceConfig;
    use rumor::Rumor;

    fn create_service_config(member_id: &str, config: &str) -> ServiceConfig {
        let config_bytes: Vec<u8> = Vec::from(config);
        ServiceConfig::new(member_id,
                           ServiceGroup::new("neurosis", "production", None),
                           config_bytes)
    }

    // Equality
    #[test]
    fn identical_service_config_are_equal() {
        let s1 = create_service_config("adam", "yep");
        let s2 = create_service_config("adam", "yep");
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn service_configs_with_different_incarnations_are_not_equal() {
        let s1 = create_service_config("adam", "yep");
        let mut s2 = create_service_config("adam", "yep");
        s2.set_incarnation(1);
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn service_configs_with_different_service_groups_are_not_equal() {
        let s1 = create_service_config("adam", "yep");
        let mut s2 = create_service_config("adam", "yep");
        s2.set_service_group(String::from("adam.fragile"));
        assert_eq!(s1, s2);
    }

    // Order
    #[test]
    fn service_configs_that_are_identical_are_equal_via_cmp() {
        let s1 = create_service_config("adam", "yep");
        let s2 = create_service_config("adam", "yep");
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Equal));
    }

    #[test]
    fn service_configs_with_different_incarnations_are_not_equal_via_cmp() {
        let s1 = create_service_config("adam", "yep");
        let mut s2 = create_service_config("adam", "yep");
        s2.set_incarnation(1);
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
        assert_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    }

    #[test]
    fn merge_chooses_the_higher_incarnation() {
        let mut s1 = create_service_config("adam", "yep");
        let mut s2 = create_service_config("adam", "yep");
        s2.set_incarnation(1);
        let s2_check = s2.clone();
        assert_eq!(s1.merge(s2), true);
        assert_eq!(s1, s2_check);
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_service_config("adam", "yep");
        s1.set_incarnation(1);
        let s1_check = s1.clone();
        let s2 = create_service_config("adam", "yep");
        assert_eq!(s1.merge(s2), false);
        assert_eq!(s1, s1_check);
    }

    #[test]
    fn config_comes_back_as_a_string() {
        let s1 = create_service_config("adam", "yep");
        assert_eq!(s1.config().unwrap(), String::from("yep"));
    }
}
