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

use protocol;

use super::{Handler, HandleResult};
use server::ServiceTable;

pub struct TerminateHandler;
impl Handler for TerminateHandler {
    type Message = protocol::Terminate;
    type Reply = protocol::TerminateOk;

    fn handle(msg: Self::Message, services: &mut ServiceTable) -> HandleResult<Self::Reply> {
        match services.get_mut(msg.get_pid()) {
            Some(service) => {
                debug!("Terminating: {}", service.id());
                let shutdown_method = service.kill();
                match service.wait() {
                    Ok(status) => {
                        let mut reply = protocol::TerminateOk::new();
                        reply.set_exit_code(status.code().unwrap_or(0));
                        reply.set_shutdown_method(shutdown_method);
                        Ok(reply)
                    }
                    Err(err) => Err(protocol::error(err)),
                }
            }
            None => {
                let mut reply = protocol::NetErr::new();
                reply.set_code(protocol::ErrCode::NoPID);
                Err(reply)
            }
        }
    }
}
