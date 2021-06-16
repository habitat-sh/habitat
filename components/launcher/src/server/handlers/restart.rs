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
                let reply = protocol::NetErr { code: protocol::ErrCode::NoPid,
                                               ..Default::default() };
                return Err(reply);
            }
        };
        service.kill();
        match service.wait() {
            Ok(_status) => {
                match service::run(service.take_args()) {
                    Ok(new_service) => {
                        let reply = protocol::SpawnOk { pid: new_service.id().into(), };
                        services.insert(new_service);
                        Ok(reply)
                    }
                    Err(err) => Err(protocol::error(err)),
                }
            }
            Err(_) => {
                let reply = protocol::NetErr { code: protocol::ErrCode::ExecWait,
                                               ..Default::default() };
                Err(reply)
            }
        }
    }
}
