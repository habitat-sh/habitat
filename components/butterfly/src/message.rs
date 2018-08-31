// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::str::FromStr;

use bytes::BytesMut;
use habitat_core::crypto::SymKey;
use prost::Message;
use serde::{
    de::{Error as SerdeError, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use uuid::{ParseError, Uuid};

use error::{Error, Result};
use protocol::Wire;

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BfUuid {
    uuid: Uuid,
}

impl BfUuid {
    pub fn nil() -> Self {
        Self { uuid: Uuid::nil() }
    }

    pub fn generate() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn parse_or_nil(input: &str, what: &str) -> Self {
        match input.parse::<Self>() {
            Ok(u) => u,
            Err(e) => {
                error!("Cannot parse {} {} as UUID: {}", what, input, e);
                BfUuid::nil()
            }
        }
    }

    pub fn must_parse(input: &str) -> Self {
        match input.parse::<Self>() {
            Ok(u) => u,
            Err(e) => panic!("Cannot parse {} as UUID: {}", input, e),
        }
    }

    pub fn is_nil(&self) -> bool {
        self.uuid.is_nil()
    }
}

impl Display for BfUuid {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.uuid.simple())
    }
}

impl Debug for BfUuid {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.uuid.simple())
    }
}

impl FromStr for BfUuid {
    type Err = ParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(Self { uuid: s.parse()? })
    }
}

struct BfUuidVisitor;

impl<'de> Visitor<'de> for BfUuidVisitor {
    type Value = BfUuid;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a string with 32 hexadecimal chars representing a UUID")
    }

    fn visit_str<E: SerdeError>(self, value: &str) -> StdResult<Self::Value, E> {
        value
            .parse::<Self::Value>()
            .map_err(|e| E::custom(e.to_string()))
    }
}

impl<'de> Deserialize<'de> for BfUuid {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BfUuidVisitor)
    }
}

impl Serialize for BfUuid {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub fn generate_wire(payload: Vec<u8>, ring_key: Option<&SymKey>) -> Result<Vec<u8>> {
    let mut wire = Wire::default();
    if let Some(ring_key) = ring_key {
        wire.encrypted = Some(true);
        let (nonce, encrypted_payload) = ring_key.encrypt(&payload)?;
        wire.nonce = Some(nonce);
        wire.payload = Some(encrypted_payload);
    } else {
        wire.payload = Some(payload);
    }
    let mut buf = BytesMut::with_capacity(wire.encoded_len());
    wire.encode(&mut buf)?;
    Ok(buf.to_vec())
}

pub fn unwrap_wire(payload: &[u8], ring_key: Option<&SymKey>) -> Result<Vec<u8>> {
    let wire = Wire::decode(payload)?;
    let payload = wire
        .payload
        .ok_or(Error::ProtocolMismatch("missing payload"))?;
    if let Some(ring_key) = ring_key {
        let nonce = wire.nonce.ok_or(Error::ProtocolMismatch("missing nonce"))?;
        Ok(ring_key.decrypt(&nonce, &payload)?)
    } else {
        Ok(payload)
    }
}
