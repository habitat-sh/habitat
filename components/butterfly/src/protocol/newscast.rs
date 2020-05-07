use std::fmt;

use crate::rumor::{departure::Departure as CDeparture,
                   election::{Election as CElection,
                              ElectionUpdate as CElectionUpdate},
                   service::Service as CService,
                   service_config::ServiceConfig as CServiceConfig,
                   service_file::ServiceFile as CServiceFile,
                   service_health::ServiceHealth as CServiceHealth};

include!(concat!(env!("OUT_DIR"), "/butterfly.newscast.rs"));

pub use self::rumor::{Payload as RumorPayload,
                      Type as RumorType};

impl fmt::Display for RumorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            RumorType::Member => "member",
            RumorType::Service => "service",
            RumorType::Election => "election",
            RumorType::ServiceConfig => "service-config",
            RumorType::ServiceFile => "service-file",
            RumorType::ServiceHealth => "service-health",
            RumorType::Fake => "fake",
            RumorType::Fake2 => "fake2",
            RumorType::ElectionUpdate => "election-update",
            RumorType::Departure => "departure",
        };

        write!(f, "{}", value)
    }
}

impl From<CDeparture> for Rumor {
    fn from(value: CDeparture) -> Self {
        let payload = Departure { member_id: Some(value.member_id), };
        Rumor { r#type:  RumorType::Departure as i32,
                tag:     Vec::default(),
                from_id: Some("butterflyclient".to_string()),
                payload: Some(RumorPayload::Departure(payload)), }
    }
}

impl From<CElection> for Rumor {
    fn from(value: CElection) -> Self {
        let payload = Election { member_id:     Some(value.member_id.clone()),
                                 service_group: Some(value.service_group.to_string()),
                                 term:          Some(value.term),
                                 suitability:   Some(value.suitability),
                                 status:        Some(value.status as i32),
                                 votes:         value.votes.clone(), };
        Rumor { r#type:  RumorType::Election as i32,
                tag:     Vec::default(),
                from_id: Some(value.member_id),
                payload: Some(RumorPayload::Election(payload)), }
    }
}

impl From<CElectionUpdate> for Rumor {
    fn from(value: CElectionUpdate) -> Self {
        let payload = Election { member_id:     Some(value.member_id.clone()),
                                 service_group: Some(value.service_group.to_string()),
                                 term:          Some(value.term),
                                 suitability:   Some(value.suitability),
                                 status:        Some(value.status as i32),
                                 votes:         value.votes.clone(), };
        Rumor { r#type:  RumorType::ElectionUpdate as i32,
                tag:     Vec::default(),
                from_id: Some(value.member_id.clone()),
                payload: Some(RumorPayload::Election(payload)), }
    }
}

impl From<CService> for Rumor {
    fn from(value: CService) -> Self {
        let payload = Service { member_id:     Some(value.member_id.clone()),
                                service_group: Some(value.service_group.to_string()),
                                incarnation:   Some(value.incarnation),
                                initialized:   Some(value.initialized),
                                pkg:           Some(value.pkg),
                                cfg:           Some(value.cfg),
                                sys:           Some(value.sys.into()), };
        Rumor { r#type:  RumorType::Service as i32,
                tag:     Vec::default(),
                from_id: Some(value.member_id),
                payload: Some(RumorPayload::Service(payload)), }
    }
}

impl From<CServiceConfig> for Rumor {
    fn from(value: CServiceConfig) -> Self {
        let payload = ServiceConfig { service_group: Some(value.service_group.to_string()),
                                      incarnation:   Some(value.incarnation),
                                      encrypted:     Some(value.encrypted),
                                      config:        Some(value.config), };
        Rumor { r#type:  RumorType::ServiceConfig as i32,
                tag:     Vec::default(),
                from_id: Some(value.from_id),
                payload: Some(RumorPayload::ServiceConfig(payload)), }
    }
}

impl From<CServiceFile> for Rumor {
    fn from(value: CServiceFile) -> Self {
        let payload = ServiceFile { service_group: Some(value.service_group.to_string()),
                                    incarnation:   Some(value.incarnation),
                                    encrypted:     Some(value.encrypted),
                                    filename:      Some(value.filename),
                                    body:          Some(value.body), };
        Rumor { r#type:  RumorType::ServiceFile as i32,
                tag:     Vec::default(),
                from_id: Some(value.from_id),
                payload: Some(RumorPayload::ServiceFile(payload)), }
    }
}

impl From<CServiceHealth> for Rumor {
    fn from(value: CServiceHealth) -> Self {
        Rumor { r#type:  RumorType::ServiceHealth as i32,
                tag:     Vec::default(),
                from_id: Some(value.member_id.clone()),
                payload: Some(RumorPayload::ServiceHealth(value.into())), }
    }
}
