pub mod newscast;
pub mod swim;

use bytes::BytesMut;
use prost::Message as ProstMessage;
use serde::Serialize;

use crate::error::Result;

include!("../generated/butterfly.common.rs");

pub trait Message<T: ProstMessage + Default>: FromProto<T> + Clone + Into<T> + Serialize {
    const MESSAGE_ID: &'static str;

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let decoded = T::decode(bytes)?;
        Self::from_proto(decoded)
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>> {
        let envelope = self.clone().into();
        let mut buf = BytesMut::with_capacity(envelope.encoded_len());
        envelope.encode(&mut buf)?;
        Ok(buf.to_vec())
    }
}

pub trait FromProto<T>: Sized {
    fn from_proto(value: T) -> Result<Self>;
}
