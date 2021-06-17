use crate::protocol;

use super::{HandleResult,
            Handler};
use crate::server::ServiceTable;

pub struct TerminateHandler;
impl Handler for TerminateHandler {
    type Message = protocol::Terminate;
    type Reply = protocol::TerminateOk;

    fn handle(msg: Self::Message, services: &mut ServiceTable) -> HandleResult<Self::Reply> {
        match services.get_mut(msg.pid as u32) {
            Some(service) => {
                debug!("Terminating: {}", service.id());
                let shutdown_method = service.kill();
                match service.wait() {
                    Ok(status) => {
                        let reply = protocol::TerminateOk { exit_code: status.code()
                                                                             .unwrap_or(0),
                                                            shutdown_method };
                        Ok(reply)
                    }
                    Err(_) => {
                        let reply = protocol::NetErr { code: protocol::ErrCode::ExecWait,
                                                       ..Default::default() };
                        Err(reply)
                    }
                }
            }
            None => {
                let reply = protocol::NetErr { code: protocol::ErrCode::NoPid,
                                               ..Default::default() };
                Err(reply)
            }
        }
    }
}
