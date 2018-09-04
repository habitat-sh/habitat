// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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
use std::collections::HashMap;
use std::mem;
use std::result;
use std::str::FromStr;

use cast;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use toml;

use habitat_core::package::Identifiable;
use habitat_core::service::ServiceGroup;

use error::{Error, Result};
use protocol::{self, newscast, FromProto};
use rumor::{Rumor, RumorPayload, RumorType};

pub type Tag = String;
pub type PortName = String;
pub type TaggedPorts = HashMap<Tag, HashMap<PortName, u16>>;

#[derive(Debug, Clone)]
pub struct Service {
    pub member_id: String,
    pub service_group: ServiceGroup,
    pub incarnation: u64,
    pub initialized: bool,
    pub pkg: String,
    pub cfg: Vec<u8>,
    pub sys: SysInfo,
    pub tagged_ports: TaggedPorts,
}

// Ensures that `cfg` is rendered as a map, and not an array of bytes
impl Serialize for Service {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("service", 8)?;
        let cfg = toml::from_slice(&self.cfg).unwrap_or(toml::value::Table::default());
        strukt.serialize_field("member_id", &self.member_id)?;
        strukt.serialize_field("service_group", &self.service_group)?;
        strukt.serialize_field("package", &self.pkg)?;
        strukt.serialize_field("incarnation", &self.incarnation)?;
        strukt.serialize_field("cfg", &cfg)?;
        strukt.serialize_field("sys", &self.sys)?;
        strukt.serialize_field("initialized", &self.initialized)?;
        strukt.serialize_field("tagged_ports", &self.tagged_ports)?;
        strukt.end()
    }
}

impl PartialOrd for Service {
    fn partial_cmp(&self, other: &Service) -> Option<Ordering> {
        if self.member_id != other.member_id || self.service_group != other.service_group {
            None
        } else {
            Some(self.incarnation.cmp(&other.incarnation))
        }
    }
}

impl PartialEq for Service {
    fn eq(&self, other: &Service) -> bool {
        self.member_id == other.member_id
            && self.service_group == other.service_group
            && self.incarnation == other.incarnation
    }
}

impl Service {
    /// Creates a new Service.
    pub fn new<T, U>(
        member_id: U,
        package: &T,
        service_group: ServiceGroup,
        sys: SysInfo,
        cfg: Option<&toml::value::Table>,
        tagged_ports: TaggedPorts,
    ) -> Self
    where
        T: Identifiable,
        U: Into<String>,
    {
        assert!(
            package.fully_qualified(),
            "Service constructor requires a fully qualified package identifier"
        );
        assert_eq!(
            service_group.service(),
            package.name(),
            "Service constructor requires the given package name to match the service \
             group's name"
        );
        Service {
            member_id: member_id.into(),
            service_group: service_group,
            incarnation: 0,
            initialized: false,
            pkg: package.to_string(),
            sys: sys,
            // TODO FN: Can we really expect this all the time, should we return a `Result<Self>`
            // in this constructor?
            cfg: cfg
                .map(|v| toml::ser::to_vec(v).expect("Struct should serialize to bytes"))
                .unwrap_or_default(),
            tagged_ports: tagged_ports,
        }
    }
}

impl protocol::Message<newscast::Rumor> for Service {}

impl FromProto<newscast::Rumor> for Service {
    fn from_proto(rumor: newscast::Rumor) -> Result<Self> {
        let mut payload = match rumor.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::Service(payload) => payload,
            _ => panic!("from-bytes service"),
        };
        let mut tagged_ports = HashMap::with_capacity(payload.ports.len());
        for mut ports in payload.ports.drain(..) {
            let mut named_ports: HashMap<String, u16> =
                HashMap::with_capacity(ports.named_ports.len());

            for named_port in ports.named_ports.drain(..) {
                named_ports.insert(
                    named_port
                        .name
                        .ok_or(Error::ProtocolMismatch("named-port.name"))?,
                    cast::u16(
                        named_port
                            .port
                            .ok_or(Error::ProtocolMismatch("named-port.port"))?,
                    ).map_err(|e| Error::InvalidField("named-port.port", e.to_string()))?,
                );
            }

            tagged_ports.insert(
                ports.tag.ok_or(Error::ProtocolMismatch("ports.tag"))?,
                named_ports,
            );
        }
        Ok(Service {
            member_id: payload
                .member_id
                .ok_or(Error::ProtocolMismatch("member-id"))?,
            service_group: payload
                .service_group
                .ok_or(Error::ProtocolMismatch("service-group"))
                .and_then(|s| ServiceGroup::from_str(&s).map_err(Error::from))?,
            incarnation: payload.incarnation.unwrap_or(0),
            initialized: payload.initialized.unwrap_or(false),
            pkg: payload.pkg.ok_or(Error::ProtocolMismatch("pkg"))?,
            cfg: payload.cfg.unwrap_or_default(),
            sys: payload
                .sys
                .ok_or(Error::ProtocolMismatch("sys"))
                .and_then(SysInfo::from_proto)?,
            tagged_ports: tagged_ports,
        })
    }
}

impl From<Service> for newscast::Service {
    fn from(mut value: Service) -> Self {
        newscast::Service {
            member_id: Some(value.member_id),
            service_group: Some(value.service_group.to_string()),
            incarnation: Some(value.incarnation),
            initialized: Some(value.initialized),
            pkg: Some(value.pkg),
            cfg: Some(value.cfg),
            sys: Some(value.sys.into()),
            ports: value
                .tagged_ports
                .drain()
                .map(|(tag, mut ports)| newscast::Ports {
                    tag: Some(tag),
                    named_ports: ports
                        .drain()
                        .map(|(name, port)| newscast::NamedPort {
                            name: Some(name),
                            port: Some(cast::i32(port)),
                        })
                        .collect(),
                })
                .collect(),
        }
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

    fn kind(&self) -> RumorType {
        RumorType::Service
    }

    fn id(&self) -> &str {
        &self.member_id
    }

    fn key(&self) -> &str {
        self.service_group.as_ref()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SysInfo {
    pub ip: String,
    pub hostname: String,
    pub gossip_ip: String,
    pub gossip_port: u32,
    pub http_gateway_ip: String,
    pub http_gateway_port: u32,
    pub ctl_gateway_ip: String,
    pub ctl_gateway_port: u32,
}

impl Default for SysInfo {
    fn default() -> Self {
        SysInfo {
            ip: "127.0.0.1".to_string(),
            hostname: "localhost".to_string(),
            gossip_ip: "127.0.0.1".to_string(),
            gossip_port: 0,
            http_gateway_ip: "127.0.0.1".to_string(),
            http_gateway_port: 0,
            ctl_gateway_ip: "127.0.0.1".to_string(),
            ctl_gateway_port: 0,
        }
    }
}

impl FromProto<newscast::SysInfo> for SysInfo {
    fn from_proto(proto: newscast::SysInfo) -> Result<Self> {
        Ok(SysInfo {
            ip: proto.ip.ok_or(Error::ProtocolMismatch("ip"))?,
            hostname: proto.hostname.ok_or(Error::ProtocolMismatch("hostname"))?,
            gossip_ip: proto.gossip_ip.unwrap_or_default(),
            gossip_port: proto.gossip_port.unwrap_or_default(),
            http_gateway_ip: proto.http_gateway_ip.unwrap_or_default(),
            http_gateway_port: proto.http_gateway_port.unwrap_or_default(),
            ctl_gateway_ip: proto.ctl_gateway_ip.unwrap_or_default(),
            ctl_gateway_port: proto.ctl_gateway_port.unwrap_or_default(),
        })
    }
}

impl From<SysInfo> for newscast::SysInfo {
    fn from(value: SysInfo) -> Self {
        newscast::SysInfo {
            ip: Some(value.ip),
            hostname: Some(value.hostname),
            gossip_ip: Some(value.gossip_ip),
            gossip_port: Some(value.gossip_port),
            http_gateway_ip: Some(value.http_gateway_ip),
            http_gateway_port: Some(value.http_gateway_port),
            ctl_gateway_ip: Some(value.ctl_gateway_ip),
            ctl_gateway_port: Some(value.ctl_gateway_port),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::str::FromStr;

    use habitat_core::package::{Identifiable, PackageIdent};
    use habitat_core::service::ServiceGroup;

    use super::Service;
    use rumor::service::SysInfo;
    use rumor::Rumor;

    fn create_service(member_id: &str) -> Service {
        let pkg = PackageIdent::from_str("core/neurosis/1.2.3/20161208121212").unwrap();
        let sg = ServiceGroup::new(None, pkg.name(), "production", None).unwrap();
        Service::new(
            member_id.to_string(),
            &pkg,
            sg,
            SysInfo::default(),
            None,
            HashMap::new(),
        )
    }

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
        s2.incarnation = 1;
        assert_eq!(s1, s2);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn services_with_different_service_groups_are_not_equal() {
        let s1 = create_service("adam");
        let mut s2 = create_service("adam");
        s2.service_group = ServiceGroup::from_str("adam.fragile").unwrap();
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
        s2.incarnation = 1;
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
        s2.incarnation = 1;
        let s2_check = s2.clone();
        assert_eq!(s1.merge(s2), true);
        assert_eq!(s1, s2_check);
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_service("adam");
        s1.incarnation = 1;
        let s1_check = s1.clone();
        let s2 = create_service("adam");
        assert_eq!(s1.merge(s2), false);
        assert_eq!(s1, s1_check);
    }

    #[test]
    #[should_panic]
    fn service_package_name_mismatch() {
        let ident = PackageIdent::from_str("core/overwatch/1.2.3/20161208121212").unwrap();
        let sg = ServiceGroup::new(None, "counter-strike", "times", Some("ofgrace")).unwrap();
        Service::new(
            "bad-member".to_string(),
            &ident,
            sg,
            SysInfo::default(),
            None,
            HashMap::new(),
        );
    }
}
