//! The Service rumor.
//!
//! Service rumors declare that a given `Server` is running this Service.

use crate::{error::{Error,
                    Result},
            protocol::{self,
                       newscast,
                       FromProto},
            rumor::{Rumor,
                    RumorPayload,
                    RumorType}};
use habitat_core::{package::Identifiable,
                   service::ServiceGroup};
use serde::{ser::SerializeStruct,
            Serialize,
            Serializer};
use std::{cmp::Ordering,
          fmt,
          mem,
          result,
          str::{self,
                FromStr}};

#[derive(Debug, Clone)]
pub struct Service {
    pub member_id:       String,
    pub service_group:   ServiceGroup,
    pub incarnation:     u64,
    pub initialized:     bool,
    pub pkg:             String,
    pub pkg_incarnation: u64,
    pub cfg:             Vec<u8>,
    pub sys:             SysInfo,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "Service i/{} m/{} sg/{}",
               self.incarnation, self.member_id, self.service_group)
    }
}

// Ensures that `cfg` is rendered as a map, and not an array of bytes
impl Serialize for Service {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = serializer.serialize_struct("service", 7)?;
        let cfg: toml::value::Table =
            toml::from_str(str::from_utf8(&self.cfg).unwrap_or_default()).unwrap_or_default();
        strukt.serialize_field("member_id", &self.member_id)?;
        strukt.serialize_field("service_group", &self.service_group)?;
        strukt.serialize_field("package", &self.pkg)?;
        strukt.serialize_field("package_incarnation", &self.pkg_incarnation)?;
        strukt.serialize_field("incarnation", &self.incarnation)?;
        strukt.serialize_field("cfg", &cfg)?;
        strukt.serialize_field("sys", &self.sys)?;
        strukt.serialize_field("initialized", &self.initialized)?;
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
    pub fn new<T, U>(member_id: U,
                     package: &T,
                     service_group: ServiceGroup,
                     sys: SysInfo,
                     cfg: Option<toml::value::Table>)
                     -> Self
        where T: Identifiable,
              U: Into<String>
    {
        assert!(package.fully_qualified(),
                "Service constructor requires a fully qualified package identifier");
        assert_eq!(service_group.service(),
                   package.name(),
                   "Service constructor requires the given package name to match the service \
                    group's name");
        Service { member_id: member_id.into(),
                  service_group,
                  incarnation: 0,
                  initialized: false,
                  pkg: package.to_string(),
                  pkg_incarnation: 0,
                  sys,
                  cfg: cfg.map(|v| {
                              // Directly serializing a toml::value::Table can lead to an error
                              // Wrapping it in a toml::value::Value makes this operation safe
                              // See https://github.com/alexcrichton/toml-rs/issues/142
                              toml::ser::to_string(&toml::value::Value::Table(v))
                        .expect("Struct should serialize to toml").into_bytes()
                          })
                          .unwrap_or_default() }
    }
}

impl protocol::Message<newscast::Rumor> for Service {
    const MESSAGE_ID: &'static str = "Service";
}

impl FromProto<newscast::Rumor> for Service {
    fn from_proto(rumor: newscast::Rumor) -> Result<Self> {
        let payload = match rumor.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::Service(payload) => payload,
            _ => panic!("from-bytes service"),
        };
        Ok(Service { member_id:       payload.member_id
                                             .ok_or(Error::ProtocolMismatch("member-id"))?,
                     service_group:
                         payload.service_group
                                .ok_or(Error::ProtocolMismatch("service-group"))
                                .and_then(|s| ServiceGroup::from_str(&s).map_err(Error::from))?,
                     incarnation:     payload.incarnation.unwrap_or(0),
                     initialized:     payload.initialized.unwrap_or(false),
                     pkg:             payload.pkg.ok_or(Error::ProtocolMismatch("pkg"))?,
                     pkg_incarnation: payload.pkg_incarnation.unwrap_or(0),
                     cfg:             payload.cfg.unwrap_or_default(),
                     sys:             payload.sys
                                             .ok_or(Error::ProtocolMismatch("sys"))
                                             .and_then(SysInfo::from_proto)?, })
    }
}

impl From<Service> for newscast::Service {
    fn from(value: Service) -> Self {
        newscast::Service { member_id:       Some(value.member_id),
                            service_group:   Some(value.service_group.to_string()),
                            incarnation:     Some(value.incarnation),
                            initialized:     Some(value.initialized),
                            pkg:             Some(value.pkg),
                            pkg_incarnation: Some(value.pkg_incarnation),
                            cfg:             Some(value.cfg),
                            sys:             Some(value.sys.into()), }
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

    fn kind(&self) -> RumorType { RumorType::Service }

    fn id(&self) -> &str { &self.member_id }

    fn key(&self) -> &str { self.service_group.as_ref() }
}

#[derive(Debug, Clone, Serialize)]
pub struct SysInfo {
    pub ip:                String,
    pub hostname:          String,
    pub gossip_ip:         String,
    pub gossip_port:       u32,
    pub http_gateway_ip:   String,
    pub http_gateway_port: u32,
    pub ctl_gateway_ip:    String,
    pub ctl_gateway_port:  u32,
}

impl Default for SysInfo {
    fn default() -> Self {
        SysInfo { ip:                "127.0.0.1".to_string(),
                  hostname:          "localhost".to_string(),
                  gossip_ip:         "127.0.0.1".to_string(),
                  gossip_port:       0,
                  http_gateway_ip:   "127.0.0.1".to_string(),
                  http_gateway_port: 0,
                  ctl_gateway_ip:    "127.0.0.1".to_string(),
                  ctl_gateway_port:  0, }
    }
}

impl FromProto<newscast::SysInfo> for SysInfo {
    fn from_proto(proto: newscast::SysInfo) -> Result<Self> {
        Ok(SysInfo { ip:                proto.ip.ok_or(Error::ProtocolMismatch("ip"))?,
                     hostname:          proto.hostname
                                             .ok_or(Error::ProtocolMismatch("hostname"))?,
                     gossip_ip:         proto.gossip_ip.unwrap_or_default(),
                     gossip_port:       proto.gossip_port.unwrap_or_default(),
                     http_gateway_ip:   proto.http_gateway_ip.unwrap_or_default(),
                     http_gateway_port: proto.http_gateway_port.unwrap_or_default(),
                     ctl_gateway_ip:    proto.ctl_gateway_ip.unwrap_or_default(),
                     ctl_gateway_port:  proto.ctl_gateway_port.unwrap_or_default(), })
    }
}

impl From<SysInfo> for newscast::SysInfo {
    fn from(value: SysInfo) -> Self {
        newscast::SysInfo { ip:                Some(value.ip),
                            hostname:          Some(value.hostname),
                            gossip_ip:         Some(value.gossip_ip),
                            gossip_port:       Some(value.gossip_port),
                            http_gateway_ip:   Some(value.http_gateway_ip),
                            http_gateway_port: Some(value.http_gateway_port),
                            ctl_gateway_ip:    Some(value.ctl_gateway_ip),
                            ctl_gateway_port:  Some(value.ctl_gateway_port), }
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering,
              str::FromStr};

    use habitat_core::{package::{Identifiable,
                                 PackageIdent},
                       service::ServiceGroup};

    use super::Service;
    use crate::rumor::{service::SysInfo,
                       Rumor};

    fn create_service(member_id: &str) -> Service {
        let pkg = PackageIdent::from_str("core/neurosis/1.2.3/20161208121212").unwrap();
        let sg = ServiceGroup::new(pkg.name(), "production", None).unwrap();
        Service::new(member_id.to_string(), &pkg, sg, SysInfo::default(), None)
    }

    #[test]
    fn identical_services_are_equal() {
        // Two different objects with the same member id, service group, and incarnation are equal
        let s1 = create_service("adam");
        let s2 = create_service("adam");
        assert_eq!(s1, s2);
    }

    #[test]
    fn services_with_different_member_ids_are_not_equal() {
        let s1 = create_service("adam");
        let s2 = create_service("shanku");
        assert_ne!(s1, s2);
    }

    #[test]
    fn services_with_different_incarnations_are_not_equal() {
        let s1 = create_service("adam");
        let mut s2 = create_service("adam");
        s2.incarnation = 1;
        assert_ne!(s1, s2);
    }

    #[test]
    fn services_with_different_service_groups_are_not_equal() {
        let s1 = create_service("adam");
        let mut s2 = create_service("adam");
        s2.service_group = ServiceGroup::from_str("adam.fragile").unwrap();
        assert_ne!(s1, s2);
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
        assert!(s1.merge(s2));
        assert_eq!(s1, s2_check);
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_service("adam");
        s1.incarnation = 1;
        let s1_check = s1.clone();
        let s2 = create_service("adam");
        assert!(!s1.merge(s2));
        assert_eq!(s1, s1_check);
    }

    #[test]
    #[should_panic]
    fn service_package_name_mismatch() {
        let ident = PackageIdent::from_str("core/overwatch/1.2.3/20161208121212").unwrap();
        let sg = ServiceGroup::new("counter-strike", "times", Some("ofgrace")).unwrap();
        Service::new("bad-member".to_string(),
                     &ident,
                     sg,
                     SysInfo::default(),
                     None);
    }

    #[test]
    fn service_cfg_serialization() {
        let package: PackageIdent = "core/foo/1.0.0/20180701125610".parse().unwrap();
        let sg = ServiceGroup::new("foo", "default", None).unwrap();

        // This map contains a scalar value and a table such that the serialization order
        // would trigger a ValueAfterTable error. This test ensures we avoid it.
        // See https://github.com/habitat-sh/habitat/issues/5854
        // See https://github.com/alexcrichton/toml-rs/issues/142
        let mut map = toml::value::Table::default();
        let sub_map = toml::value::Table::default();
        map.insert("foo".into(), 5.into());
        map.insert("a".into(), toml::value::Value::Table(sub_map));
        Service::new("member_id_val", &package, sg, SysInfo::default(), Some(map));
    }
}
