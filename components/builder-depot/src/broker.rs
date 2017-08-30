// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::any::TypeId;
use std::collections::HashMap;

use hab_net::{NetError, NetResult};
use protobuf::{self, parse_from_bytes};
use protocol::Routable;
use typemap;

use error::{Error, Result};

#[allow(dead_code)]
pub struct RoutedMessages(HashMap<TypeId, Vec<u8>>);

impl RoutedMessages {
    #[allow(dead_code)]
    pub fn get<M>(&self) -> Result<M>
    where
        M: Routable,
    {
        let msg_type = &TypeId::of::<M>();
        match self.0.get(msg_type) {
            Some(msg) => {
                Ok(parse_from_bytes::<M>(msg).expect(&format!(
                    "Unable to parse {:?} message",
                    msg_type
                )))
            }
            None => Err(Error::MessageTypeNotFound),
        }
    }
}

#[derive(Default)]
pub struct TestableBroker {
    message_map: HashMap<TypeId, Vec<u8>>,
    error_map: HashMap<TypeId, NetError>,
    cached_messages: HashMap<TypeId, Vec<u8>>,
}

impl TestableBroker {
    #[allow(dead_code)]
    pub fn setup<M, R>(&mut self, response: &R)
    where
        M: Routable,
        R: protobuf::MessageStatic,
    {
        let bytes = response.write_to_bytes().unwrap();
        self.message_map.insert(TypeId::of::<M>(), bytes);
    }

    #[allow(dead_code)]
    pub fn setup_error<M>(&mut self, error: NetError)
    where
        M: Routable,
    {
        self.error_map.insert(TypeId::of::<M>(), error);
    }

    #[allow(dead_code)]
    pub fn routed_messages(&self) -> RoutedMessages {
        RoutedMessages(self.cached_messages.clone())
    }

    pub fn route<M, R>(&mut self, msg: &M) -> NetResult<R>
    where
        M: Routable,
        R: protobuf::MessageStatic,
    {
        let bytes = msg.write_to_bytes().unwrap();
        self.cached_messages.insert(TypeId::of::<M>(), bytes);
        let msg_type = &TypeId::of::<M>();
        match self.message_map.get(msg_type) {
            Some(message) => Ok(parse_from_bytes::<R>(message).unwrap()),
            None => {
                match self.error_map.get(msg_type) {
                    Some(ref error) => Err(NetError::new(error.code(), error.msg())),
                    None => panic!("Unable to find message of given type"),
                }
            }
        }
    }
}

impl typemap::Key for TestableBroker {
    type Value = Self;
}
