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

use super::{HandleResult,
            Handler};
use crate::{server::ServiceTable,
            service};

pub struct RestartHandler;
impl Handler for RestartHandler {
    type Message = protocol::Restart;
    type Reply = protocol::SpawnOk;

    fn handle(msg: Self::Message, services: &mut ServiceTable) -> HandleResult<Self::Reply> {
        let mut service = match services.remove(msg.pid as u32) {
            Some(service) => service,
            None => {
                let mut reply = protocol::NetErr::default();
                reply.code = protocol::ErrCode::NoPid;
                return Err(reply);
            }
        };
        service.kill();
        match service.wait() {
            Ok(_status) => {
                match service::run(service.take_args()) {
                    Ok(new_service) => {
                        let mut reply = protocol::SpawnOk::default();
                        reply.pid = new_service.id().into();
                        services.insert(new_service);
                        Ok(reply)
                    }
                    Err(err) => Err(protocol::error(err)),
                }
            }
            Err(_) => {
                let mut reply = protocol::NetErr::default();
                reply.code = protocol::ErrCode::ExecWait;
                Err(reply)
            }
        }
    }
}
