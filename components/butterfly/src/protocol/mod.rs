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

pub mod newscast;
pub mod swim;

use bytes::BytesMut;
use prost::Message as ProstMessage;
use serde::Serialize;

use crate::error::Result;

include!("../generated/butterfly.common.rs");

pub trait Message<T: ProstMessage + Default>: FromProto<T> + Clone + Into<T> + Serialize {
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
