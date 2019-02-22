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

mod restart;
mod spawn;
mod terminate;

pub use self::{restart::*,
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
