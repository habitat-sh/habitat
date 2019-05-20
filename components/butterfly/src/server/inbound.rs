//! The inbound thread.
//!
//! This module handles all the inbound SWIM messages.

use std::{net::{SocketAddr,
                UdpSocket},
          thread,
          time::Duration};

use habitat_core::util::ToI64;
use prometheus::{IntCounterVec,
                 IntGaugeVec};

use super::AckSender;
use crate::{member::Health,
            server::{outbound,
                     Server},
            swim::{Ack,
                   Ping,
                   PingReq,
                   Swim,
                   SwimKind},
            trace::TraceKind};

lazy_static! {
    static ref SWIM_MESSAGES_RECEIVED: IntCounterVec =
        register_int_counter_vec!("hab_butterfly_swim_messages_received_total",
                                  "Total number of SWIM messages received",
                                  &["type", "mode"]).unwrap();
    static ref SWIM_BYTES_RECEIVED: IntGaugeVec =
        register_int_gauge_vec!("hab_butterfly_swim_received_bytes",
                                "SWIM message size received in bytes",
                                &["type", "mode"]).unwrap();
}

/// Takes the Server and a channel to send received Acks to the outbound thread.
pub struct Inbound {
    pub server:      Server,
    pub socket:      UdpSocket,
    pub tx_outbound: AckSender,
}

impl Inbound {
    /// Create a new Inbound.
    pub fn new(server: Server, socket: UdpSocket, tx_outbound: AckSender) -> Inbound {
        Inbound { server,
                  socket,
                  tx_outbound }
    }

    /// Run the thread. Listens for messages up to 1k in size, and then processes them accordingly.
    pub fn run(&self) {
        let mut recv_buffer: Vec<u8> = vec![0; 1024];

        loop {
            habitat_common::sync::mark_thread_alive();

            if self.server.paused() {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            match self.socket.recv_from(&mut recv_buffer[..]) {
                Ok((length, addr)) => {
                    let swim_payload = match self.server.unwrap_wire(&recv_buffer[0..length]) {
                        Ok(swim_payload) => swim_payload,
                        Err(e) => {
                            // NOTE: In the future, we might want to block people who send us
                            // garbage all the time.
                            error!("Error unwrapping protocol message, {}", e);
                            let label_values = &["unwrap_wire", "failure"];
                            SWIM_BYTES_RECEIVED.with_label_values(label_values)
                                               .set(length.to_i64());
                            SWIM_MESSAGES_RECEIVED.with_label_values(label_values).inc();
                            continue;
                        }
                    };

                    let bytes_received = swim_payload.len();
                    let msg = match Swim::decode(&swim_payload) {
                        Ok(msg) => msg,
                        Err(e) => {
                            // NOTE: In the future, we might want to block people who send us
                            // garbage all the time.
                            error!("Error decoding protocol message, {}", e);
                            let label_values = &["undecodable", "failure"];
                            SWIM_BYTES_RECEIVED.with_label_values(label_values)
                                               .set(bytes_received.to_i64());
                            SWIM_MESSAGES_RECEIVED.with_label_values(label_values).inc();
                            continue;
                        }
                    };

                    // Setting a label_values variable here throws errors about moving borrowed
                    // content that I couldn't solve w/o clones. Leaving this for now. I'm sure
                    // there's a better way.
                    SWIM_BYTES_RECEIVED.with_label_values(&[msg.kind.as_str(), "success"])
                                       .set(bytes_received.to_i64());
                    SWIM_MESSAGES_RECEIVED.with_label_values(&[msg.kind.as_str(), "success"])
                                          .inc();

                    trace!("SWIM Message: {:?}", msg);
                    match msg.kind {
                        SwimKind::Ping(ping) => {
                            if self.server.is_member_blocked(&ping.from.id) {
                                debug!("Not processing message from {} - it is blocked",
                                       ping.from.id);
                                continue;
                            }
                            self.process_ping(addr, ping);
                        }
                        SwimKind::Ack(ack) => {
                            if self.server.is_member_blocked(&ack.from.id)
                               && ack.forward_to.is_none()
                            {
                                debug!("Not processing message from {} - it is blocked",
                                       ack.from.id);
                                continue;
                            }
                            self.process_ack(addr, ack);
                        }
                        SwimKind::PingReq(pingreq) => {
                            if self.server.is_member_blocked(&pingreq.from.id) {
                                debug!("Not processing message from {} - it is blocked",
                                       pingreq.from.id);
                                continue;
                            }
                            self.process_pingreq(addr, pingreq);
                        }
                    }
                }
                Err(e) => {
                    // TODO: We can't use magic numbers here because the Supervisor runs on more
                    // than one platform. I'm sure these were added as specific OS errors for Linux
                    // but we need to also handle Windows & Mac.
                    match e.raw_os_error() {
                        Some(35) | Some(11) | Some(10035) | Some(10060) => {
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
    fn process_pingreq(&self, addr: SocketAddr, mut msg: PingReq) {
        trace_it!(SWIM: &self.server, TraceKind::RecvPingReq, &msg.from.id, addr, &msg);
        msg.from.address = addr.ip().to_string();
        let id = msg.target.id.clone(); // TODO: see if we can eliminate this clone
        self.server.member_list.with_member(&id, |target| {
                                   if let Some(target) = target {
                                       // Set the route-back address to the one we received the
                                       // pingreq from
                                       outbound::ping(&self.server,
                                                      &self.socket,
                                                      target,
                                                      target.swim_socket_address(),
                                                      Some(&msg.from));
                                   } else {
                                       error!("PingReq request {:?} for invalid target", msg);
                                   }
                               });
    }

    /// Process ack messages; forwards to the outbound thread.
    fn process_ack(&self, addr: SocketAddr, mut msg: Ack) {
        trace_it!(SWIM: &self.server, TraceKind::RecvAck, &msg.from.id, addr, &msg);
        trace!("Ack from {}@{}", msg.from.id, addr);
        if msg.forward_to.is_some() && *self.server.member_id != msg.forward_to.as_ref().unwrap().id
        {
            let (forward_to_addr, from_addr) = {
                let forward_to = msg.forward_to.as_ref().unwrap();
                let forward_addr_str = format!("{}:{}", forward_to.address, forward_to.swim_port);
                let forward_to_addr = match forward_addr_str.parse() {
                    Ok(addr) => addr,
                    Err(e) => {
                        error!("Abandoning Ack forward: cannot parse member address: {}:{}, {}",
                               forward_to.address, forward_to.swim_port, e);
                        return;
                    }
                };
                trace!("Forwarding Ack from {}@{} to {}@{}",
                       msg.from.id,
                       addr,
                       forward_to.id,
                       forward_to.address,);
                (forward_to_addr, addr.ip().to_string())
            };
            msg.from.address = from_addr;
            outbound::forward_ack(&self.server, &self.socket, forward_to_addr, msg);
            return;
        }
        let memberships = msg.membership.clone();
        match self.tx_outbound.send((addr, msg)) {
            Ok(()) => {
                for membership in memberships {
                    self.server
                        .insert_member_from_rumor(membership.member, membership.health);
                }
            }
            Err(e) => panic!("Outbound thread has died - this shouldn't happen: #{:?}", e),
        }
    }

    /// Process ping messages.
    fn process_ping(&self, addr: SocketAddr, mut msg: Ping) {
        trace_it!(SWIM: &self.server, TraceKind::RecvPing, &msg.from.id, addr, &msg);
        outbound::ack(&self.server, &self.socket, &msg.from, addr, msg.forward_to);
        // Populate the member for this sender with its remote address
        msg.from.address = addr.ip().to_string();
        trace!("Ping from {}@{}", msg.from.id, addr);
        if msg.from.departed {
            self.server.insert_member(msg.from, Health::Departed);
        } else {
            self.server.insert_member(msg.from, Health::Alive);
        }
        for membership in msg.membership {
            self.server
                .insert_member_from_rumor(membership.member, membership.health);
        }
    }
}
