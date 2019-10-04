use super::{HandleResult,
            Handler};
use crate::{protocol,
            server::ServiceTable};

pub struct PidHandler;

impl Handler for PidHandler {
    type Message = protocol::PidOf;
    type Reply = protocol::PidIs;

    fn handle(msg: Self::Message, services: &mut ServiceTable) -> HandleResult<Self::Reply> {
        let service_name = msg.service_name;
        let pid = services.pid_of(&service_name);
        let reply = protocol::PidIs { pid };
        Ok(reply)
    }
}
