//! The inbound thread.
//!
//! This module handles all the inbound SWIM messages.

use super::AckSender;
use crate::{member::Health,
            server::{outbound,
                     Server},
            swim::{Ack,
                   Ping,
                   PingReq,
                   Swim,
                   SwimKind}};
use habitat_common::liveliness_checker;
use habitat_core::util::ToI64;
use prometheus::{IntCounterVec,
                 IntGaugeVec};
use std::{net::{SocketAddr,
                UdpSocket},
          thread,
          time::Duration};

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

pub fn spawn_thread(name: String,
                    server: Server,
                    socket: UdpSocket,
                    tx_outbound: AckSender)
                    -> std::io::Result<()> {
    thread::Builder::new().name(name)
                          .spawn(move || -> ! { run_loop(&server, &socket, &tx_outbound) })
                          .map(|_| ())
}

/// Run the thread. Listens for messages up to 1k in size, and then processes them accordingly.
/// Takes the Server and a channel to send received Acks to the outbound thread.
pub fn run_loop(server: &Server, socket: &UdpSocket, tx_outbound: &AckSender) -> ! {
    let mut recv_buffer: Vec<u8> = vec![0; 1024];

    loop {
        liveliness_checker::mark_thread_alive().and_divergent();

        if server.paused() {
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        match socket.recv_from(&mut recv_buffer[..]) {
            Ok((length, addr)) => {
                let swim_payload = match server.unwrap_wire(&recv_buffer[0..length]) {
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
                        if server.is_member_blocked_sblr(&ping.from.id) {
                            debug!("Not processing message from {} - it is blocked",
                                   ping.from.id);
                            continue;
                        }
                        process_ping_mlw_smw_rhw(server, socket, addr, ping);
                    }
                    SwimKind::Ack(ack) => {
                        if server.is_member_blocked_sblr(&ack.from.id) && ack.forward_to.is_none() {
                            debug!("Not processing message from {} - it is blocked",
                                   ack.from.id);
                            continue;
                        }
                        process_ack_mlw_smw_rhw(server, socket, tx_outbound, addr, ack);
                    }
                    SwimKind::PingReq(pingreq) => {
                        if server.is_member_blocked_sblr(&pingreq.from.id) {
                            debug!("Not processing message from {} - it is blocked",
                                   pingreq.from.id);
                            continue;
                        }
                        process_pingreq_mlr_smr_rhw(server, socket, addr, pingreq);
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
///
/// # Locking (see locking.md)
/// * `MemberList::entries` (read)
/// * `Server::member` (read)
/// * `RumorHeat::inner` (write)
fn process_pingreq_mlr_smr_rhw(server: &Server,
                               socket: &UdpSocket,
                               addr: SocketAddr,
                               mut msg: PingReq) {
    if let Some(target) = server.member_list.get_cloned_mlr(&msg.target.id) {
        msg.from.address = addr.ip().to_string();
        let ping_msg = Ping { membership: vec![],
                              from:       server.myself.lock_smr().to_member(),
                              forward_to: Some(msg.from.clone()), };
        let swim = outbound::populate_membership_rumors_mlr_rhw(server, &target, ping_msg);
        // Set the route-back address to the one we received the
        // pingreq from
        outbound::ping(server,
                       socket,
                       target.swim_socket_address(),
                       Some(&msg.from),
                       &swim);
    } else {
        error!("PingReq request {:?} for invalid target", msg);
    }
}

/// Process ack messages; forwards to the outbound thread.
///
/// # Locking (see locking.md)
/// * `MemberList::entries` (write)
/// * `Server::member` (write)
/// * `RumorHeat::inner` (write)
fn process_ack_mlw_smw_rhw(server: &Server,
                           socket: &UdpSocket,
                           tx_outbound: &AckSender,
                           addr: SocketAddr,
                           mut msg: Ack) {
    trace!("Ack from {}@{}", msg.from.id, addr);
    if msg.forward_to.is_some() && *server.member_id != msg.forward_to.as_ref().unwrap().id {
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
        outbound::forward_ack(server, socket, forward_to_addr, msg);
        return;
    }
    let memberships = msg.membership.clone();
    match tx_outbound.send((addr, msg)) {
        Ok(()) => {
            for membership in memberships {
                server.insert_member_from_rumor_mlw_smw_rhw(membership.member, membership.health);
            }
        }
        Err(e) => panic!("Outbound thread has died - this shouldn't happen: #{:?}", e),
    }
}

/// # Locking (see locking.md)
/// * `MemberList::entries` (write)
/// * `Server::member` (write)
/// * `RumorHeat::inner` (write)
fn process_ping_mlw_smw_rhw(server: &Server, socket: &UdpSocket, addr: SocketAddr, mut msg: Ping) {
    outbound::ack_mlr_smr_rhw(server, socket, &msg.from, addr, msg.forward_to);
    // Populate the member for this sender with its remote address
    msg.from.address = addr.ip().to_string();
    trace!("Ping from {}@{}", msg.from.id, addr);
    if msg.from.departed {
        server.insert_member_mlw_rhw(msg.from, Health::Departed);
    } else {
        server.insert_member_mlw_rhw(msg.from, Health::Alive);
    }
    for membership in msg.membership {
        server.insert_member_from_rumor_mlw_smw_rhw(membership.member, membership.health);
    }
}
