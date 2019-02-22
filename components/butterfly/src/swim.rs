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

use std::{fmt,
          str::FromStr};

use bytes::BytesMut;
use prost::Message as ProstMessage;

pub use crate::protocol::swim::{SwimPayload,
                                SwimType};
use crate::{error::{Error,
                    Result},
            member::{Health,
                     Member,
                     Membership},
            protocol::{self,
                       swim as proto,
                       FromProto}};

#[derive(Debug, Clone, Serialize)]
pub struct Ack {
    pub membership: Vec<Membership>,
    pub from: Member,
    pub forward_to: Option<Member>,
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
        for membership in value.membership {
            memberships.push(Membership::from_proto(membership)?);
        }
        Ok(Ack {
            membership: memberships,
            from: payload
                .from
                .ok_or(Error::ProtocolMismatch("from"))
                .and_then(Member::from_proto)?,
            forward_to,
        })
    }
}

impl protocol::Message<proto::Swim> for Ack {}

impl From<Ack> for proto::Ack {
    fn from(value: Ack) -> Self {
        proto::Ack {
            from: Some(value.from.into()),
            forward_to: value.forward_to.map(Into::into),
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
            payload: Some(SwimPayload::Ack(value.into())),
        }
    }
}

impl From<Ack> for Swim {
    fn from(value: Ack) -> Self {
        Swim {
            type_: SwimType::Ack,
            membership: value.membership.clone(),
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub from: Member,
    pub forward_to: Option<Member>,
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
        for membership in value.membership {
            memberships.push(Membership::from_proto(membership)?);
        }
        Ok(Ping {
            membership: memberships,
            from: payload
                .from
                .ok_or(Error::ProtocolMismatch("from"))
                .and_then(Member::from_proto)?,
            forward_to,
        })
    }
}

impl protocol::Message<proto::Swim> for Ping {}

impl From<Ping> for proto::Ping {
    fn from(value: Ping) -> Self {
        proto::Ping {
            from: Some(value.from.into()),
            forward_to: value.forward_to.map(Into::into),
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
            payload: Some(SwimPayload::Ping(value.into())),
        }
    }
}

impl From<Ping> for Swim {
    fn from(value: Ping) -> Self {
        Swim {
            type_: SwimType::Ping,
            membership: value.membership.clone(),
            kind: SwimKind::Ping(value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PingReq {
    pub membership: Vec<Membership>,
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
        for membership in value.membership {
            memberships.push(Membership::from_proto(membership)?);
        }
        Ok(PingReq {
            membership: memberships,
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
            payload: Some(SwimPayload::Pingreq(value.into())),
        }
    }
}

impl From<PingReq> for Swim {
    fn from(value: PingReq) -> Self {
        Swim {
            type_: SwimType::Pingreq,
            membership: value.membership.clone(),
            kind: SwimKind::PingReq(value),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum SwimKind {
    Ping(Ping),
    Ack(Ack),
    PingReq(PingReq),
}

impl From<SwimKind> for SwimPayload {
    fn from(value: SwimKind) -> Self {
        match value {
            SwimKind::Ping(ping) => SwimPayload::Ping(ping.into()),
            SwimKind::Ack(ack) => SwimPayload::Ack(ack.into()),
            SwimKind::PingReq(pingreq) => SwimPayload::Pingreq(pingreq.into()),
        }
    }
}

impl fmt::Display for SwimKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.as_str();
        write!(f, "{}", value)
    }
}

impl SwimKind {
    pub fn as_str(&self) -> &str {
        match *self {
            SwimKind::Ping(_) => "ping",
            SwimKind::Ack(_) => "ack",
            SwimKind::PingReq(_) => "pingreq",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Swim {
    pub type_: SwimType,
    pub membership: Vec<Membership>,
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
        let kind = match type_ {
            SwimType::Ack => SwimKind::Ack(Ack::from_proto(proto)?),
            SwimType::Ping => SwimKind::Ping(Ping::from_proto(proto)?),
            SwimType::Pingreq => SwimKind::PingReq(PingReq::from_proto(proto)?),
        };
        Ok(Swim {
            type_,
            membership: memberships,
            kind,
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
            payload: Some(value.kind.into()),
        }
    }
}
//
