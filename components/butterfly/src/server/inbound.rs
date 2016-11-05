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

//! The inbound thread.
//!
//! This module handles all the inbound SWIM messages.

use std::sync::mpsc;
use std::sync::atomic::Ordering;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

use protobuf;

use member::{Member, Health};
use message::swim::{Swim, Swim_Type};
use server::{Server, outbound};
use trace::TraceKind;

/// Takes the Server and a channel to send recieved Acks to the outbound thread.
pub struct Inbound<'a> {
    pub server: &'a Server,
    pub socket: UdpSocket,
    pub tx_outbound: mpsc::Sender<(SocketAddr, Swim)>,
}

impl<'a> Inbound<'a> {
    /// Create a new Inbound.
    pub fn new(server: &'a Server,
               socket: UdpSocket,
               tx_outbound: mpsc::Sender<(SocketAddr, Swim)>)
               -> Inbound {
        Inbound {
            server: server,
            socket: socket,
            tx_outbound: tx_outbound,
        }
    }

    /// Run the thread. Listens for messages up to 1k in size, and then processes them accordingly.
    pub fn run(&self) {
        let mut recv_buffer: Vec<u8> = vec![0; 1024];
        loop {
            if self.server.pause.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            match self.socket.recv_from(&mut recv_buffer[..]) {
                Ok((length, addr)) => {
                    let swim_payload = match self.server.unwrap_wire(&recv_buffer[0..length]) {
                        Ok(swim_payload) => swim_payload,
                        Err(e) => {
                            // NOTE: In the future, we might want to blacklist people who send us
                            // garbage all the time.
                            error!("Error parsing protobuf: {:?}", e);
                            continue;
                        }
                    };

                    let msg: Swim = match protobuf::parse_from_bytes(&swim_payload) {
                        Ok(msg) => msg,
                        Err(e) => {
                            // NOTE: In the future, we might want to blacklist people who send us
                            // garbage all the time.
                            error!("Error parsing protobuf: {:?}", e);
                            continue;
                        }
                    };
                    debug!("SWIM Message: {:?}", msg);
                    match msg.get_field_type() {
                        Swim_Type::PING => {
                            if self.server.check_blacklist(msg.get_ping().get_from().get_id()) {
                                debug!("Not processing message from {} - it is blacklisted",
                                       msg.get_ping().get_from().get_id());
                                continue;
                            }
                            self.process_ping(addr, msg);
                        }
                        Swim_Type::ACK => {
                            if self.server.check_blacklist(msg.get_ack().get_from().get_id()) &&
                               !msg.get_ack().has_forward_to() {
                                debug!("Not processing message from {} - it is blacklisted",
                                       msg.get_ack().get_from().get_id());
                                continue;
                            }
                            self.process_ack(addr, msg);
                        }
                        Swim_Type::PINGREQ => {
                            if self.server.check_blacklist(msg.get_pingreq().get_from().get_id()) {
                                debug!("Not processing message from {} - it is blacklisted",
                                       msg.get_pingreq().get_from().get_id());
                                continue;
                            }
                            self.process_pingreq(addr, msg);
                        }
                    }
                }
                Err(e) => {
                    match e.raw_os_error() {
                        Some(35) | Some(11) => {
                            // This is the normal non-blocking result, or a timeout
                        }
                        Some(_) => {
                            error!("UDP Receive error: {}", e);
                            debug!("UDP Receive error debug: {:?}", e);
                        }
                        None => {
                            error!("UDP Receive error: {}", e);
                        }
                    }
                }
            }
        }
    }

    /// Process pingreq messages.
    fn process_pingreq(&self, addr: SocketAddr, mut msg: Swim) {
        trace_it!(SWIM: &self.server, TraceKind::RecvPingReq, msg.get_pingreq().get_from().get_id(), addr, &msg);
        // We need to get msg to be owned by the closure, so we're going to have to
        // allocate here to get the id. Kind of a bummer, but life goes on.
        let mid = String::from(msg.get_pingreq().get_target().get_id());
        self.server.member_list.with_member(&mid, |m| {
            let target = match m {
                Some(target) => target,
                None => {
                    error!("PingReq request {:?} for invalid target", msg);
                    return;
                }
            };
            // Set the route-back address to the one we received the pingreq from
            let mut from = msg.mut_pingreq().take_from();
            from.set_address(format!("{}", addr.ip()));
            outbound::ping(self.server,
                           &self.socket,
                           target,
                           target.swim_socket_address(),
                           Some(from.into()));
        });
    }

    /// Process ack messages; forwards to the outbound thread.
    fn process_ack(&self, addr: SocketAddr, mut msg: Swim) {
        trace_it!(SWIM: &self.server, TraceKind::RecvAck, msg.get_ack().get_from().get_id(), addr, &msg);
        info!("Ack from {}@{}", msg.get_ack().get_from().get_id(), addr);
        if msg.get_ack().has_forward_to() {
            if self.server.member_id() != msg.get_ack().get_forward_to().get_id() {
                let forward_addr_str = format!("{}:{}",
                                               msg.get_ack().get_forward_to().get_address(),
                                               msg.get_ack().get_forward_to().get_swim_port());
                let forward_to_addr = match forward_addr_str.parse() {
                    Ok(addr) => addr,
                    Err(e) => {
                        error!("Abandoning Ack forward: cannot parse member address: {}, {}",
                               msg.get_ack().get_forward_to().get_address(),
                               e);
                        return;
                    }
                };
                info!("Forwarding Ack from {}@{} to {}@{}",
                      msg.get_ack().get_from().get_id(),
                      addr,
                      msg.get_ack().get_forward_to().get_id(),
                      msg.get_ack().get_forward_to().get_address(),
                      );
                msg.mut_ack().mut_from().set_address(format!("{}", addr.ip()));
                outbound::forward_ack(self.server, &self.socket, forward_to_addr, msg);
                return;
            }
        }
        let membership = {
            let membership: Vec<(Member, Health)> = msg.take_membership()
                .iter()
                .map(|m| (Member::from(m.get_member()), Health::from(m.get_health())))
                .collect();
            membership
        };
        match self.tx_outbound.send((addr, msg)) {
            Ok(()) => {}
            Err(e) => panic!("Outbound thread has died - this shouldn't happen: #{:?}", e),
        }
        self.server.insert_member_from_rumors(membership);
    }

    /// Process ping messages.
    fn process_ping(&self, addr: SocketAddr, mut msg: Swim) {
        trace_it!(SWIM: &self.server, TraceKind::RecvPing, msg.get_ping().get_from().get_id(), addr, &msg);
        let target: Member = msg.get_ping().get_from().into();
        if msg.get_ping().has_forward_to() {
            outbound::ack(self.server,
                          &self.socket,
                          &target,
                          addr,
                          Some(msg.mut_ping().take_forward_to().into()));
        } else {
            outbound::ack(self.server, &self.socket, &target, addr, None);
        }
        // Populate the member for this sender with its remote address
        let from = {
            let mut ping = msg.mut_ping();
            let mut from = ping.take_from();
            from.set_address(format!("{}", addr.ip()));
            from
        };
        info!("Ping from {}@{}", from.get_id(), addr);
        self.server.insert_member(from.into(), Health::Alive);
        let membership: Vec<(Member, Health)> = msg.take_membership()
            .iter()
            .map(|m| (Member::from(m.get_member()), Health::from(m.get_health())))
            .collect();
        self.server.insert_member_from_rumors(membership);
    }
}
