mod pid;
mod restart;
mod spawn;
mod terminate;

pub use self::{pid::*,
               restart::*,
               spawn::*,
               terminate::*};

use crate::protocol;

use super::{Sender,
            ServiceTable};

type HandleResult<T> = Result<T, protocol::NetErr>;

pub trait Handler {
    type Message: protocol::LauncherMessage;
    type Reply: protocol::LauncherMessage;

    fn handle(msg: Self::Message, services: &mut ServiceTable) -> HandleResult<Self::Reply>;

    fn run(tx: &Sender, txn: protocol::NetTxn, services: &mut ServiceTable) {
        let msg = match txn.decode::<Self::Message>() {
            Ok(msg) => msg,
            Err(err) => {
                error!("{}: decoding, {}", txn.message_id(), err);
                return;
            }
        };
        trace!("{}, {:?}, {:?}", txn.message_id(), msg, services);
        match Self::handle(msg, services) {
            Ok(reply) => {
                if let Err(err) = super::send(tx, &reply) {
                    error!("{}: replying, {}", txn.message_id(), err);
                }
            }
            Err(reply) => {
                if let Err(err) = super::send(tx, &reply) {
                    error!("{}: replying, {}", txn.message_id(), err);
                }
            }
        }
    }
}
