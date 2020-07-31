use super::{HandleResult,
            Handler};
use crate::{protocol,
            server::ServiceTable,
            VERSION};

pub struct VersionHandler;

impl Handler for VersionHandler {
    type Message = protocol::Version;
    type Reply = protocol::VersionNumber;

    fn handle(_: Self::Message, _: &mut ServiceTable) -> HandleResult<Self::Reply> {
        // VERSION will be none if this is a cargo built binary as opposed to
        // being built by our hab plan. So in that case we will fallback to u32::MAX.
        match VERSION.unwrap_or(&u32::MAX.to_string()).parse::<u32>() {
            Ok(version) => {
                let reply = protocol::VersionNumber { version };
                Ok(reply)
            }
            Err(_) => {
                let mut reply = protocol::NetErr::default();
                reply.code = protocol::ErrCode::InvalidVersionNumber;
                Err(reply)
            }
        }
    }
}
