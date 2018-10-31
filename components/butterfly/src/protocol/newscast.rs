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

use rumor::departure::Departure as CDeparture;
use rumor::election::{Election as CElection, ElectionUpdate as CElectionUpdate};
use rumor::service::Service as CService;
use rumor::service_config::ServiceConfig as CServiceConfig;
use rumor::service_file::ServiceFile as CServiceFile;

include!("../generated/butterfly.newscast.rs");

pub use self::{rumor::Payload as RumorPayload, rumor::Type as RumorType};

impl From<CDeparture> for Rumor {
    fn from(value: CDeparture) -> Self {
        let payload = Departure {
            member_id: Some(value.member_id),
        };
        Rumor {
            type_: RumorType::Departure as i32,
            tag: Vec::default(),
            from_id: Some("butterflyclient".to_string()),
            payload: Some(RumorPayload::Departure(payload)),
        }
    }
}

impl From<CElection> for Rumor {
    fn from(value: CElection) -> Self {
        let payload = Election {
            member_id: Some(value.member_id.clone()),
            service_group: Some(value.service_group.to_string()),
            term: Some(value.term),
            suitability: Some(value.suitability),
            status: Some(value.status as i32),
            votes: value.votes.clone(),
        };
        Rumor {
            type_: RumorType::Election as i32,
            tag: Vec::default(),
            from_id: Some(value.member_id),
            payload: Some(RumorPayload::Election(payload)),
        }
    }
}

impl From<CElectionUpdate> for Rumor {
    fn from(value: CElectionUpdate) -> Self {
        let payload = Election {
            member_id: Some(value.member_id.clone()),
            service_group: Some(value.service_group.to_string()),
            term: Some(value.term),
            suitability: Some(value.suitability),
            status: Some(value.status as i32),
            votes: value.votes.clone(),
        };
        Rumor {
            type_: RumorType::ElectionUpdate as i32,
            tag: Vec::default(),
            from_id: Some(value.member_id.clone()),
            payload: Some(RumorPayload::Election(payload)),
        }
    }
}

impl From<CService> for Rumor {
    fn from(value: CService) -> Self {
        let payload = Service {
            member_id: Some(value.member_id.clone()),
            service_group: Some(value.service_group.to_string()),
            incarnation: Some(value.incarnation),
            initialized: Some(value.initialized),
            pkg: Some(value.pkg),
            cfg: Some(value.cfg),
            sys: Some(value.sys.into()),
        };
        Rumor {
            type_: RumorType::Service as i32,
            tag: Vec::default(),
            from_id: Some(value.member_id),
            payload: Some(RumorPayload::Service(payload)),
        }
    }
}

impl From<CServiceConfig> for Rumor {
    fn from(value: CServiceConfig) -> Self {
        let payload = ServiceConfig {
            service_group: Some(value.service_group.to_string()),
            incarnation: Some(value.incarnation),
            encrypted: Some(value.encrypted),
            config: Some(value.config),
        };
        Rumor {
            type_: RumorType::ServiceConfig as i32,
            tag: Vec::default(),
            from_id: Some(value.from_id),
            payload: Some(RumorPayload::ServiceConfig(payload)),
        }
    }
}

impl From<CServiceFile> for Rumor {
    fn from(value: CServiceFile) -> Self {
        let payload = ServiceFile {
            service_group: Some(value.service_group.to_string()),
            incarnation: Some(value.incarnation),
            encrypted: Some(value.encrypted),
            filename: Some(value.filename),
            body: Some(value.body),
        };
        Rumor {
            type_: RumorType::ServiceFile as i32,
            tag: Vec::default(),
            from_id: Some(value.from_id),
            payload: Some(RumorPayload::ServiceFile(payload)),
        }
    }
}
