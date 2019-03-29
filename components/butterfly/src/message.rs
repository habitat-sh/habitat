

use bytes::BytesMut;
use habitat_core::crypto::SymKey;
use prost::Message;

use crate::{error::{Error,
                    Result},
            protocol::Wire};

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
    let payload = wire.payload
                      .ok_or(Error::ProtocolMismatch("missing payload"))?;
    if let Some(ring_key) = ring_key {
        let nonce = wire.nonce.ok_or(Error::ProtocolMismatch("missing nonce"))?;
        Ok(ring_key.decrypt(&nonce, &payload)?)
    } else {
        Ok(payload)
    }
}
