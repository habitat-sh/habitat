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

use std::thread;
use std::time;

use butterfly::client::Client;
use common::ui::{Status, UI};
use hcore::crypto::SymKey;

use error::{Error, Result};

pub fn run(
    ui: &mut UI,
    member_id: &str,
    peers: Vec<String>,
    ring_key: Option<SymKey>,
) -> Result<()> {
    ui.begin(
        format!("Permanently marking {} as departed", member_id),
    )?;
    ui.status(
        Status::Creating,
        format!("service configuration"),
    )?;
    for peer in peers.into_iter() {
        ui.status(Status::Applying, format!("to peer {}", peer))?;
        let mut client = Client::new(peer, ring_key.clone()).map_err(|e| {
            Error::ButterflyError(e.to_string())
        })?;
        client.send_departure(member_id).map_err(|e| {
            Error::ButterflyError(e.to_string())
        })?;
        // please take a moment to weep over the following line
        // of code. We must sleep to allow messages to be sent
        // before freeing the socket to prevent loss.
        // see https://github.com/zeromq/libzmq/issues/1264
        thread::sleep(time::Duration::from_millis(100));
    }
    ui.end("Departure recorded.")?;
    Ok(())
}
