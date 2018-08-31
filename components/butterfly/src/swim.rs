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

use std::fmt;
use std::str::FromStr;

use bytes::BytesMut;
use prost::Message as ProstMessage;

use error::{Error, Result};
use member::{Health, Member, Membership};
use message::BfUuid;
pub use protocol::swim::{SwimPayload, SwimType};
use protocol::{self, swim as proto, FromProto};
use zone::Zone;

#[derive(Debug, Clone, Serialize)]
pub struct Ack {
    pub membership: Vec<Membership>,
    pub zones: Vec<Zone>,
    pub from: Member,
    pub forward_to: Option<Member>,
    pub to: Member,
}

impl FromProto<proto::Swim> for Ack {
    fn from_proto(value: proto::Swim) -> Result<Self> {
        let payload = match value.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            SwimPayload::Ack(ack) => ack,
            _ => panic!("try-from ack"),
        };
        let forward_to = if let Some(forward_to) = payload.forward_to {
            Some(Member::from_proto(forward_to)?)
        } else {
            None
        };
        let mut memberships = Vec::with_capacity(value.membership.len());
        let mut zones = Vec::with_capacity(value.zones.len());
        for membership in value.membership {
            memberships.push(Membership::from_proto(membership)?);
        }
        for zone in value.zones {
            zones.push(Zone::from_proto(zone)?);
        }
        Ok(Ack {
            membership: memberships,
            zones: zones,
            from: payload
                .from
                .ok_or(Error::ProtocolMismatch("from"))
                .and_then(Member::from_proto)?,
            forward_to: forward_to,
            to: payload
                .to
                .ok_or(Error::ProtocolMismatch("to"))
                .and_then(Member::from_proto)?,
        })
    }
}

impl protocol::Message<proto::Swim> for Ack {}

impl From<Ack> for proto::Ack {
    fn from(value: Ack) -> Self {
        proto::Ack {
            from: Some(value.from.into()),
            forward_to: value.forward_to.map(Into::into),
            to: Some(value.to.into()),
        }
    }
}

impl From<Ack> for proto::Swim {
    fn from(value: Ack) -> Self {
        proto::Swim {
            type_: SwimType::Ack as i32,
            membership: value
                .membership
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            zones: value.zones.clone().into_iter().map(Into::into).collect(),
            payload: Some(SwimPayload::Ack(value.into())),
        }
    }
}

impl From<Ack> for Swim {
    fn from(value: Ack) -> Self {
        Swim {
            type_: SwimType::Ack,
            membership: value.membership.clone(),
            zones: value.zones.clone(),
            kind: SwimKind::Ack(value),
        }
    }
}

impl FromStr for Health {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        match value.to_lowercase().as_ref() {
            "alive" => Ok(Health::Alive),
            "suspect" => Ok(Health::Suspect),
            "confirmed" => Ok(Health::Confirmed),
            "departed" => Ok(Health::Departed),
            value => panic!("No match for Health from string, {}", value),
        }
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match *self {
            Health::Alive => "alive",
            Health::Suspect => "suspect",
            Health::Confirmed => "confirmed",
            Health::Departed => "departed",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Ping {
    pub membership: Vec<Membership>,
    pub zones: Vec<Zone>,
    pub from: Member,
    pub forward_to: Option<Member>,
    pub to: Member,
}

impl FromProto<proto::Swim> for Ping {
    fn from_proto(value: proto::Swim) -> Result<Self> {
        let payload = match value.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            SwimPayload::Ping(ping) => ping,
            _ => panic!("try-from ping"),
        };
        let forward_to = if let Some(forward_to) = payload.forward_to {
            Some(Member::from_proto(forward_to)?)
        } else {
            None
        };
        let mut memberships = Vec::with_capacity(value.membership.len());
        let mut zones = Vec::with_capacity(value.zones.len());
        for membership in value.membership {
            memberships.push(Membership::from_proto(membership)?);
        }
        for zone in value.zones {
            zones.push(Zone::from_proto(zone)?);
        }
        Ok(Ping {
            membership: memberships,
            zones: zones,
            from: payload
                .from
                .ok_or(Error::ProtocolMismatch("from"))
                .and_then(Member::from_proto)?,
            forward_to: forward_to,
            to: payload
                .to
                .ok_or(Error::ProtocolMismatch("to"))
                .and_then(Member::from_proto)?,
        })
    }
}

impl protocol::Message<proto::Swim> for Ping {}

impl From<Ping> for proto::Ping {
    fn from(value: Ping) -> Self {
        proto::Ping {
            from: Some(value.from.into()),
            forward_to: value.forward_to.map(Into::into),
            to: Some(value.to.into()),
        }
    }
}

impl From<Ping> for proto::Swim {
    fn from(value: Ping) -> Self {
        proto::Swim {
            type_: SwimType::Ping as i32,
            membership: value
                .membership
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            zones: value.zones.clone().into_iter().map(Into::into).collect(),
            payload: Some(SwimPayload::Ping(value.into())),
        }
    }
}

impl From<Ping> for Swim {
    fn from(value: Ping) -> Self {
        Swim {
            type_: SwimType::Ping,
            membership: value.membership.clone(),
            zones: value.zones.clone(),
            kind: SwimKind::Ping(value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PingReq {
    pub membership: Vec<Membership>,
    pub zones: Vec<Zone>,
    pub from: Member,
    pub target: Member,
}

impl FromProto<proto::Swim> for PingReq {
    fn from_proto(value: proto::Swim) -> Result<Self> {
        let payload = match value.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            SwimPayload::Pingreq(ping) => ping,
            _ => panic!("try-from pingreq"),
        };
        let mut memberships = Vec::with_capacity(value.membership.len());
        let mut zones = Vec::with_capacity(value.zones.len());
        for membership in value.membership {
            memberships.push(Membership::from_proto(membership)?);
        }
        for zone in value.zones {
            zones.push(Zone::from_proto(zone)?);
        }
        Ok(PingReq {
            membership: memberships,
            zones: zones,
            from: payload
                .from
                .ok_or(Error::ProtocolMismatch("from"))
                .and_then(Member::from_proto)?,
            target: payload
                .target
                .ok_or(Error::ProtocolMismatch("from"))
                .and_then(Member::from_proto)?,
        })
    }
}

impl protocol::Message<proto::Swim> for PingReq {}

impl From<PingReq> for proto::PingReq {
    fn from(value: PingReq) -> Self {
        proto::PingReq {
            from: Some(value.from.into()),
            target: Some(value.target.into()),
        }
    }
}

impl From<PingReq> for proto::Swim {
    fn from(value: PingReq) -> Self {
        proto::Swim {
            type_: SwimType::Pingreq as i32,
            membership: value
                .membership
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            zones: value.zones.clone().into_iter().map(Into::into).collect(),
            payload: Some(SwimPayload::Pingreq(value.into())),
        }
    }
}

impl From<PingReq> for Swim {
    fn from(value: PingReq) -> Self {
        Swim {
            type_: SwimType::Pingreq,
            membership: value.membership.clone(),
            zones: value.zones.clone(),
            kind: SwimKind::PingReq(value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ZoneChange {
    pub membership: Vec<Membership>,
    pub zones: Vec<Zone>,
    pub from: Member,
    pub zone_id: BfUuid,
    pub new_aliases: Vec<Zone>,
}

impl FromProto<proto::Swim> for ZoneChange {
    fn from_proto(value: proto::Swim) -> Result<Self> {
        let payload = match value.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            SwimPayload::ZoneChange(zone_change) => zone_change,
            _ => panic!("try-from zonechange"),
        };
        let mut memberships = Vec::with_capacity(value.membership.len());
        let mut zones = Vec::with_capacity(value.zones.len());
        let mut new_aliases = Vec::with_capacity(payload.new_aliases.len());
        for membership in value.membership {
            memberships.push(Membership::from_proto(membership)?);
        }
        for zone in value.zones {
            zones.push(Zone::from_proto(zone)?);
        }
        for alias in payload.new_aliases {
            new_aliases.push(Zone::from_proto(alias)?);
        }
        Ok(ZoneChange {
            membership: memberships,
            zones: zones,
            from: payload
                .from
                .ok_or(Error::ProtocolMismatch("from"))
                .and_then(Member::from_proto)?,
            zone_id: payload
                .zone_id
                .ok_or(Error::ProtocolMismatch("zone_id"))?
                .parse::<BfUuid>()
                .map_err(|e| Error::InvalidField("zone_id", e.to_string()))?,
            new_aliases: new_aliases,
        })
    }
}

impl protocol::Message<proto::Swim> for ZoneChange {}

impl From<ZoneChange> for proto::ZoneChange {
    fn from(value: ZoneChange) -> Self {
        proto::ZoneChange {
            from: Some(value.from.into()),
            zone_id: Some(value.zone_id.to_string()),
            new_aliases: value
                .new_aliases
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<ZoneChange> for proto::Swim {
    fn from(value: ZoneChange) -> Self {
        proto::Swim {
            type_: SwimType::ZoneChange as i32,
            membership: value
                .membership
                .clone()
                .into_iter()
                .map(Into::into)
                .collect(),
            zones: value.zones.clone().into_iter().map(Into::into).collect(),
            payload: Some(SwimPayload::ZoneChange(value.into())),
        }
    }
}

impl From<ZoneChange> for Swim {
    fn from(value: ZoneChange) -> Self {
        Swim {
            type_: SwimType::ZoneChange,
            membership: value.membership.clone(),
            zones: value.zones.clone(),
            kind: SwimKind::ZoneChange(value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum SwimKind {
    Ping(Ping),
    Ack(Ack),
    PingReq(PingReq),
    ZoneChange(ZoneChange),
}

impl From<SwimKind> for SwimPayload {
    fn from(value: SwimKind) -> Self {
        match value {
            SwimKind::Ping(ping) => SwimPayload::Ping(ping.into()),
            SwimKind::Ack(ack) => SwimPayload::Ack(ack.into()),
            SwimKind::PingReq(pingreq) => SwimPayload::Pingreq(pingreq.into()),
            SwimKind::ZoneChange(zone_change) => SwimPayload::ZoneChange(zone_change.into()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Swim {
    pub type_: SwimType,
    pub membership: Vec<Membership>,
    pub zones: Vec<Zone>,
    pub kind: SwimKind,
}

impl Swim {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let proto = proto::Swim::decode(bytes)?;
        let type_ = SwimType::from_i32(proto.type_).ok_or(Error::ProtocolMismatch("type"))?;
        let mut memberships = Vec::with_capacity(proto.membership.len());
        for membership in proto.membership.clone() {
            memberships.push(Membership::from_proto(membership)?);
        }
        let mut zones = Vec::with_capacity(proto.zones.len());
        for zone in proto.zones.clone() {
            zones.push(Zone::from_proto(zone)?);
        }
        let kind = match type_ {
            SwimType::Ack => SwimKind::Ack(Ack::from_proto(proto)?),
            SwimType::Ping => SwimKind::Ping(Ping::from_proto(proto)?),
            SwimType::Pingreq => SwimKind::PingReq(PingReq::from_proto(proto)?),
            SwimType::ZoneChange => SwimKind::ZoneChange(ZoneChange::from_proto(proto)?),
        };
        Ok(Swim {
            type_: type_,
            membership: memberships,
            zones: zones,
            kind: kind,
        })
    }

    pub fn encode(self) -> Result<Vec<u8>> {
        let proto: proto::Swim = self.into();
        let mut buf = BytesMut::with_capacity(proto.encoded_len());
        proto.encode(&mut buf)?;
        Ok(buf.to_vec())
    }
}

impl From<Swim> for proto::Swim {
    fn from(value: Swim) -> Self {
        proto::Swim {
            type_: value.type_ as i32,
            membership: value.membership.into_iter().map(Into::into).collect(),
            zones: value.zones.into_iter().map(Into::into).collect(),
            payload: Some(value.kind.into()),
        }
    }
}
//
