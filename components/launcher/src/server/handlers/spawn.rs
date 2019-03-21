use crate::protocol;

use super::{HandleResult,
            Handler};
use crate::{server::ServiceTable,
            service};

pub struct SpawnHandler;
impl Handler for SpawnHandler {
    type Message = protocol::Spawn;
    type Reply = protocol::SpawnOk;

    fn handle(msg: Self::Message, services: &mut ServiceTable) -> HandleResult<Self::Reply> {
        match service::run(msg) {
            Ok(service) => {
                let mut reply = protocol::SpawnOk::default();
                reply.pid = service.id().into();
                services.insert(service);
                Ok(reply)
            }
            Err(err) => Err(protocol::error(err)),
        }
    }
}
