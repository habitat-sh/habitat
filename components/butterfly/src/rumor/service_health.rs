use crate::{error::{Error,
                    Result},
            protocol::{self,
                       newscast::{self,
                                  service_health::Health,
                                  Rumor as ProtoRumor,
                                  RumorPayload},
                       FromProto},
            rumor::{Rumor,
                    RumorType}};
use habitat_core::service::ServiceGroup;
use std::{cmp::Ordering,
          fmt,
          mem,
          str::FromStr};

/// Convert numeric values from protobuf into `Health` statuses.
impl From<i32> for Health {
    fn from(value: i32) -> Self {
        match value {
            0 => Health::Ok,
            1 => Health::Warning,
            2 => Health::Critical,
            3 => Health::Unknown,
            _ => Health::Unknown,
        }
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Health::Ok => "OK",
            Health::Warning => "WARNING",
            Health::Critical => "CRITICAL",
            Health::Unknown => "UNKNOWN",
        };
        write!(f, "{}", msg)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ServiceHealth {
    pub member_id:     String,
    pub service_group: ServiceGroup,
    pub incarnation:   u64,
    pub health:        Health,
}

impl ServiceHealth {
    /// Creates a new ServiceFile.
    pub fn new<S>(member_id: S, service_group: ServiceGroup, health: Health) -> Self
        where S: Into<String>
    {
        ServiceHealth { member_id: member_id.into(),
                        service_group,
                        incarnation: 0,
                        health }
    }
}

impl fmt::Display for ServiceHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "ServiceHealth i/{} m/{} sg/{} h/{}",
               self.incarnation, self.member_id, self.service_group, self.health)
    }
}

impl PartialOrd for ServiceHealth {
    fn partial_cmp(&self, other: &ServiceHealth) -> Option<Ordering> {
        if self.member_id != other.member_id && self.service_group != other.service_group {
            None
        } else {
            Some(self.incarnation.cmp(&other.incarnation))
        }
    }
}

impl FromProto<ProtoRumor> for ServiceHealth {
    fn from_proto(rumor: ProtoRumor) -> Result<Self> {
        let payload = match rumor.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::ServiceHealth(payload) => payload,
            _ => panic!("from-bytes service-health"),
        };
        Ok(ServiceHealth { member_id:     rumor.from_id
                                               .ok_or(Error::ProtocolMismatch("from-id"))?,
                           service_group:
                               payload.service_group
                                      .ok_or(Error::ProtocolMismatch("service-group"))
                                      .and_then(|s| {
                                          ServiceGroup::from_str(&s).map_err(Error::from)
                                      })?,
                           incarnation:   payload.incarnation.unwrap_or(0),
                           health:        payload.health
                                                 .ok_or(Error::ProtocolMismatch("health"))?
                                                 .into(), })
    }
}

impl From<ServiceHealth> for newscast::ServiceHealth {
    fn from(value: ServiceHealth) -> Self {
        newscast::ServiceHealth { member_id:     Some(value.member_id),
                                  service_group: Some(value.service_group.to_string()),
                                  incarnation:   Some(value.incarnation),
                                  health:        Some(value.health.into()), }
    }
}

impl protocol::Message<ProtoRumor> for ServiceHealth {
    const MESSAGE_ID: &'static str = "ServiceHealth";
}

impl Rumor for ServiceHealth {
    /// Follows a simple pattern; if we have a newer incarnation than the one we already have, the
    /// new one wins. So far, these never change.
    fn merge(&mut self, mut other: ServiceHealth) -> bool {
        if *self >= other {
            false
        } else {
            mem::swap(self, &mut other);
            true
        }
    }

    fn kind(&self) -> RumorType { RumorType::ServiceHealth }

    fn id(&self) -> &str { &self.member_id }

    fn key(&self) -> &str { &self.service_group }
}
