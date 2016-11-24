// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

use habitat_core::crypto::SymKey;

use error::Result;
use message::swim::Wire;
use protobuf::{self, Message};

pub mod swim;

pub fn generate_wire(payload: Vec<u8>, ring_key: &Option<SymKey>) -> Result<Vec<u8>> {
    let mut wire = Wire::new();
    if let Some(ref ring_key) = *ring_key {
        wire.set_encrypted(true);
        let (nonce, encrypted_payload) = try!(ring_key.encrypt(&payload));
        wire.set_nonce(nonce);
        wire.set_payload(encrypted_payload);
    } else {
        wire.set_payload(payload);
    }
    Ok(try!(wire.write_to_bytes()))
}

pub fn unwrap_wire(payload: &[u8], ring_key: &Option<SymKey>) -> Result<Vec<u8>> {
    let mut wire: Wire = try!(protobuf::parse_from_bytes(payload));
    if let Some(ref ring_key) = *ring_key {
        Ok(try!(ring_key.decrypt(wire.get_nonce(), wire.get_payload())))
    } else {
        Ok(wire.take_payload())
    }
}
