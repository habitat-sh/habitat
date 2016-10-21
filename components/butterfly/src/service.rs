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

//! The Service rumor.
//!
//! Service rumors declare that a given `Server` is running this Service.

use std::cmp::Ordering;
use std::mem;
use std::ops::{Deref, DerefMut};

use habitat_core::service::ServiceGroup;
use protobuf::Message;

use error::Result;
use message::swim::{Service as ProtoService, Rumor as ProtoRumor, Rumor_Type as ProtoRumor_Type};
use rumor::Rumor;

/// The service rumor
#[derive(Debug, Clone)]
pub struct Service {
    pub proto: ProtoRumor,
}

impl PartialOrd for Service {
    fn partial_cmp(&self, other: &Service) -> Option<Ordering> {
        if self.get_member_id() != other.get_member_id() ||
           self.get_service_group() != other.get_service_group() {
            None
        } else {
            Some(self.get_incarnation().cmp(&other.get_incarnation()))
        }
    }
}

impl PartialEq for Service {
    fn eq(&self, other: &Service) -> bool {
        self.get_member_id() == other.get_member_id() &&
        self.get_service_group() == other.get_service_group() &&
        self.get_incarnation() == other.get_incarnation()
    }
}

impl From<ProtoRumor> for Service {
    fn from(pr: ProtoRumor) -> Service {
        Service { proto: pr }
    }
}

impl From<Service> for ProtoRumor {
    fn from(service: Service) -> ProtoRumor {
        service.proto
    }
}

impl Deref for Service {
    type Target = ProtoService;

    fn deref(&self) -> &ProtoService {
        self.proto.get_service()
    }
}

impl DerefMut for Service {
    fn deref_mut(&mut self) -> &mut ProtoService {
        self.proto.mut_service()
    }
}

impl Service {
    /// Creates a new Service.
    pub fn new<S1, S2, S3>(member_id: S1,
                           service_group: ServiceGroup,
                           hostname: S2,
                           ip: S3,
                           exposes: Vec<u32>)
                           -> Self
        where S1: Into<String>,
              S2: Into<String>,
              S3: Into<String>
    {
        let mut rumor = ProtoRumor::new();
        let from_id = member_id.into();
        let real_member_id = from_id.clone();
        rumor.set_from_id(from_id);
        rumor.set_field_type(ProtoRumor_Type::Service);

        let mut proto = ProtoService::new();
        proto.set_member_id(real_member_id);
        proto.set_service_group(format!("{}", service_group));
        proto.set_incarnation(0);
        proto.set_hostname(hostname.into());
        proto.set_ip(ip.into());
        if let Some(port) = exposes.get(0) {
            proto.set_port(*port);
        }
        proto.set_exposes(exposes);
        rumor.set_service(proto);
        Service { proto: rumor }
    }
}

impl Rumor for Service {
    /// Follows a simple pattern; if we have a newer incarnation than the one we already have, the
    /// new one wins. So far, these never change.
    fn merge(&mut self, mut other: Service) -> bool {
        if *self >= other {
            false
        } else {
            mem::swap(self, &mut other);
            true
        }
    }

    fn kind(&self) -> ProtoRumor_Type {
        ProtoRumor_Type::Service
    }

    fn id(&self) -> &str {
        self.get_member_id()
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

    use super::Service;
    use rumor::Rumor;

    fn create_service(member_id: &str) -> Service {
        Service::new(member_id,
                     ServiceGroup::new("neurosis", "production", None),
                     "fire.beyond",
                     "127.0.0.1",
                     vec![9090, 9091])
    }

    // Equality
    #[test]
    fn identical_services_are_equal() {
        // Two different objects with the same member id, service group, and incarnation are equal
        let s1 = create_service("adam");
        let s2 = create_service("adam");
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn services_with_different_member_ids_are_not_equal() {
        let s1 = create_service("adam");
        let s2 = create_service("shanku");
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn services_with_different_incarnations_are_not_equal() {
        let s1 = create_service("adam");
        let mut s2 = create_service("adam");
        s2.set_incarnation(1);
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn services_with_different_service_groups_are_not_equal() {
        let s1 = create_service("adam");
        let mut s2 = create_service("adam");
        s2.set_service_group(String::from("adam.fragile"));
        assert_eq!(s1, s2);
    }

    // Order
    #[test]
    fn services_that_are_identical_are_equal_via_cmp() {
        let s1 = create_service("adam");
        let s2 = create_service("adam");
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Equal));
    }

    #[test]
    fn services_with_different_incarnations_are_not_equal_via_cmp() {
        let s1 = create_service("adam");
        let mut s2 = create_service("adam");
        s2.set_incarnation(1);
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Less));
        assert_eq!(s2.partial_cmp(&s1), Some(Ordering::Greater));
    }

    #[test]
    fn services_of_different_members_and_groups_cannot_be_compared() {
        let s1 = create_service("adam");
        let s2 = create_service("neurosis");
        assert_eq!(s1.partial_cmp(&s2), None);
    }

    #[test]
    fn merge_chooses_the_higher_incarnation() {
        let mut s1 = create_service("adam");
        let mut s2 = create_service("adam");
        s2.set_incarnation(1);
        let s2_check = s2.clone();
        assert_eq!(s1.merge(s2), true);
        assert_eq!(s1, s2_check);
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_service("adam");
        s1.set_incarnation(1);
        let s1_check = s1.clone();
        let s2 = create_service("adam");
        assert_eq!(s1.merge(s2), false);
        assert_eq!(s1, s1_check);
    }
}
