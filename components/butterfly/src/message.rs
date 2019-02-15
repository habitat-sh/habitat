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

use bytes::BytesMut;
use habitat_core::crypto::SymKey;
use prost::Message;

use crate::{
    error::{Error, Result},
    protocol::Wire,
};

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
