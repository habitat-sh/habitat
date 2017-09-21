// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

mod handlers;

use std::collections::HashMap;

use hab_net::{ErrCode, NetError};
use hab_net::time;
use protocol::message::{Message, Protocol};
use protocol::routesrv::PING_INTERVAL_MS;
use protocol::sharding::{ShardId, SHARD_COUNT};
use rand::{self, Rng};
use zmq;

use config::Config;
use conn::{ConnErr, ConnEvent, SrvConn};
use error::{Error, Result};

const SERVER_TTL: i64 = PING_INTERVAL_MS + 5_000;

pub struct Server {
    /// Server's configuration.
    config: Config,
    /// ZeroMQ socket Context. Use this when creating new sockets if you wish them to be able to
    /// participate in the server's main thread.
    context: zmq::Context,
    /// Random seed used in calculating shard destinations.
    rng: rand::ThreadRng,
    /// Map of all registered servers and, if applicable, the shards they are hosting.
    servers: ServerMap,
}

impl Server {
    fn new(config: Config) -> Self {
        Server {
            config: config,
            context: zmq::Context::new(),
            rng: rand::thread_rng(),
            servers: ServerMap::default(),
        }
    }

    /// Forward a protocol message containing a transaction reply to the originator.
    fn forward_reply(&self, conn: &SrvConn, message: &mut Message) {
        message.identities.remove(0);
        trace!("route-message, transaction reply, {:?}", message);
        if let Err(err) = conn.forward_reply(message) {
            error!("{}", err);
        }
    }

    /// Handle a protocol message intended for this RouteSrv.
    fn handle_message(&mut self, conn: &SrvConn, message: &mut Message) -> Result<()> {
        debug!("handle-message, {}", message);
        let handler = match message.message_id() {
            "Disconnect" => handlers::on_disconnect,
            "Heartbeat" => handlers::on_heartbeat,
            "Registration" => handlers::on_registration,
            message_id => {
                warn!("handle-message, recv unknown message, {}", message_id);
                return Ok(());
            }
        };
        handler(conn, message, &mut self.servers)
    }

    /// Handle routing of a protocol message to a connected service or delegate to `handle_message`
    /// if the message was intended for this RouteSrv.
    fn route_message(&mut self, conn: &SrvConn, message: &mut Message) {
        match message.route_info().map(|r| r.protocol()) {
            Some(Protocol::RouteSrv) => {
                if let Err(err) = self.handle_message(&conn, message) {
                    error!("{}", err);
                }
            }
            Some(Protocol::Net) => warn!("route-message, unroutable message, {}", message),
            Some(_) => {
                if let Some(identity) = self.select_shard(message) {
                    if let Err(err) = conn.forward(message, identity.to_vec()) {
                        error!("{}", err);
                    }
                    return;
                }
                let err = NetError::new(ErrCode::NO_SHARD, "rt:route:2");
                error!("{}", err);
                message.populate_reply(&*err).unwrap();
                if let Err(err) = conn.forward_reply(message) {
                    error!("{}", err);
                }
            }
            None => warn!("route-message, no route-info, {}", message),
        }
    }

    /// Run the server blocking the calling thread until the server shuts down.
    fn run(&mut self) -> Result<()> {
        let mut conn = SrvConn::new(&mut self.context, &self.config)?;
        let mut message = Message::default();
        conn.bind(&self.config.addr())?;
        println!("Listening on ({})", self.config.addr());
        info!("builder-router is ready to go.");
        loop {
            message.reset();
            trace!("waiting for message");
            match conn.wait_recv(&mut message, self.wait_timeout()) {
                Ok(ConnEvent::OnMessage) => {
                    debug!("OnMessage, {}", message);
                    if message.completed_txn() {
                        self.forward_reply(&conn, &mut message);
                    } else {
                        self.route_message(&conn, &mut message);
                    }
                }
                Ok(ConnEvent::OnConnect) => debug!("OnConnect, {}", message.sender_str().unwrap()),
                Err(ConnErr::Shutdown(signal)) => {
                    info!("received shutdown signal ({}), shutting down...", signal);
                    break;
                }
                Err(err @ ConnErr::Socket(_)) => {
                    return Err(Error::from(err));
                }
                Err(ConnErr::Timeout) => (),
                Err(err) => error!("{}", err),
            }
            self.servers.expire();
        }
        Ok(())
    }

    /// Returns `Some` with the net identity of the server registered for the given protocol
    /// hosting the shard for which the given protocol message was intended for. Returns `None`
    /// if there is no server hosting the shard for the given protocol.
    fn select_shard(&mut self, message: &Message) -> Option<&[u8]> {
        let shard_id = match message.route_info().and_then(|m| m.hash()) {
            Some(hash) => (hash % SHARD_COUNT as u64) as u32,
            None => (self.rng.gen::<u64>() % SHARD_COUNT as u64) as u32,
        };
        self.servers.get(
            &message.route_info().unwrap().protocol(),
            &shard_id,
        )
    }

    /// A tickless timer for determining how long to wait between each server tick. This value is
    /// variable depending upon when the next registration expiration would occur. The default
    /// wait time is `30_000` milliseconds.
    fn wait_timeout(&self) -> i64 {
        self.servers.next_expiration().unwrap_or(30_000)
    }
}

#[derive(Debug, Default)]
pub struct ServerMap {
    reg: HashMap<Protocol, HashMap<ShardId, Vec<u8>>>,
    timestamps: HashMap<Vec<u8>, i64>,
}

impl ServerMap {
    pub fn add(&mut self, protocol: Protocol, net_ident: Vec<u8>, shards: Vec<ShardId>) -> bool {
        if !self.reg.contains_key(&protocol) {
            self.reg.insert(protocol, HashMap::default());
        }
        let registrations = self.reg.get_mut(&protocol).unwrap();
        for shard in shards.iter() {
            if let Some(reg) = registrations.get(&shard) {
                if reg != &net_ident {
                    return false;
                }
            }
        }
        for shard in shards {
            registrations.insert(shard, net_ident.clone());
        }
        self.timestamps.insert(net_ident, time::clock_time());
        true
    }

    pub fn drop(&mut self, target: &[u8]) {
        for map in self.reg.values_mut() {
            map.retain(|_, net_ident| net_ident.as_slice() != target);
        }
        self.timestamps.retain(
            |net_ident, _| net_ident.as_slice() != target,
        );
    }

    pub fn expire(&mut self) {
        let now = time::clock_time();
        let mut expired = vec![];
        self.timestamps.retain(
            |id, last| if (*last + SERVER_TTL) <= now {
                info!(
                    "expiring server registration, {:?}",
                    String::from_utf8_lossy(&id)
                );
                expired.push(id.clone());
                false
            } else {
                true
            },
        );
        for net_ident in expired.iter() {
            self.drop(net_ident);
        }
    }

    pub fn get(&self, protocol: &Protocol, shard: &ShardId) -> Option<&[u8]> {
        self.reg
            .get(protocol)
            .and_then(|shards| shards.get(shard))
            .and_then(|s| Some(s.as_slice()))
    }

    pub fn next_expiration(&self) -> Option<i64> {
        let mut timestamps = self.timestamps.values().collect::<Vec<&i64>>();
        timestamps.sort_by(|av, bv| bv.cmp(av));
        timestamps.first().map(|v| **v)
    }

    pub fn renew(&mut self, target: &[u8]) -> bool {
        if let Some(time) = self.timestamps.get_mut(target) {
            trace!("registration renew, {}", String::from_utf8_lossy(target));
            *time = time::clock_time() + SERVER_TTL;
            return true;
        }
        false
    }
}

pub fn run(config: Config) -> Result<()> {
    Server::new(config).run()
}
