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

use crate::protocol;

use super::{HandleResult, Handler};
use crate::server::ServiceTable;
use crate::service;

pub struct SpawnHandler;
impl Handler for SpawnHandler {
    type Message = protocol::Spawn;
    type Reply = protocol::SpawnOk;

    fn handle(msg: Self::Message, services: &mut ServiceTable) -> HandleResult<Self::Reply> {
        match service::run(msg) {
            Ok(service) => {
                let mut reply = protocol::SpawnOk::new();
                reply.set_pid(service.id().into());
                services.insert(service);
                Ok(reply)
            }
            Err(err) => Err(protocol::error(err)),
        }
    }
}
