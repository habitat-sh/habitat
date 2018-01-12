// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

pub mod config;
pub mod depart;
pub mod file;

use std::net::SocketAddr;

use hcore::crypto::SymKey;
use butterfly::client::Client;
use butterfly::network::{GossipZmqSocket, Network, RealNetwork};

use error::{Error, Result};

fn get_client(peer: &str, ring_key: Option<SymKey>) -> Result<Client<GossipZmqSocket>> {
    let addr: SocketAddr = peer.parse().map_err(
        |e| Error::ButterflyError(format!("{}", e)),
    )?;
    let network = RealNetwork::new_for_client();
    let socket = network.get_gossip_sender(addr).map_err(|e| {
        Error::ButterflyError(format!("{}", e))
    })?;
    Ok(Client::new(socket, ring_key))
}
