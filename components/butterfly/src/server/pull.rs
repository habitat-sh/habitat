//! The pull thread.
//!
//! This module handles pulling all the pushed rumors from every member off a ZMQ socket.

use crate::{rumor::{RumorEnvelope,
                    RumorKind},
            server::Server,
            ZMQ_CONTEXT};
use habitat_common::liveliness_checker;
use habitat_core::util::ToI64;
use prometheus::{IntCounterVec,
                 IntGaugeVec};
use std::{thread,
          time::Duration};
use zmq;

lazy_static! {
    static ref GOSSIP_MESSAGES_RECEIVED: IntCounterVec =
        register_int_counter_vec!("hab_butterfly_gossip_messages_received_total",
                                  "Total number of gossip messages received",
                                  &["type", "mode", "blocked"]).unwrap();
    static ref GOSSIP_BYTES_RECEIVED: IntGaugeVec =
        register_int_gauge_vec!("hab_butterfly_gossip_received_bytes",
                                "Gossip message size received in bytes",
                                &["type", "mode", "blocked"]).unwrap();
}

pub fn spawn_thread(name: String, server: Server) -> std::io::Result<()> {
    thread::Builder::new().name(name)
                          .spawn(move || -> ! { run_loop(&server) })
                          .map(|_| ())
}

fn run_loop(server: &Server) -> ! {
    habitat_core::env_config_int!(RecvTimeoutMillis, i32, HAB_PULL_RECV_TIMEOUT_MS, 5_000);

    let socket = (**ZMQ_CONTEXT).as_mut()
                                .socket(zmq::PULL)
                                .expect("Failure to create the ZMQ pull socket");
    socket.set_linger(0)
          .expect("Failure to set the ZMQ Pull socket to not linger");
    socket.set_tcp_keepalive(0)
          .expect("Failure to set the ZMQ Pull socket to not use keepalive");
    socket.set_rcvtimeo(RecvTimeoutMillis::configured_value().into())
          .expect("Failure to set the ZMQ Pull socket receive timeout");
    socket.bind(&format!("tcp://{}", server.gossip_addr()))
          .expect("Failure to bind the ZMQ Pull socket to the port");
    'recv: loop {
        if let Ok(-1) = socket.get_rcvtimeo() {
            trace!("Skipping thread liveliness checks due to infinite recv timeout");
        } else {
            liveliness_checker::mark_thread_alive().and_divergent();
        }

        if server.paused() {
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        let msg = match socket.recv_msg(0) {
            Ok(msg) => msg,
            Err(e) => {
                // We intentionally set a timeout above so that `mark_thread_alive` can be
                // used to show this thread is alive even when there's no data to receive.
                if e != zmq::Error::EAGAIN {
                    error!("Error receiving message: {:?}", e);
                }
                continue 'recv;
            }
        };

        let payload = match server.unwrap_wire(&msg) {
            Ok(payload) => payload,
            Err(e) => {
                // NOTE: In the future, we might want to block people who send us
                // garbage all the time.
                error!("Error parsing protocol message: {:?}", e);
                let label_values = &["unwrap_wire", "failure", "unknown"];
                GOSSIP_BYTES_RECEIVED.with_label_values(label_values)
                                     .set(msg.len().to_i64());
                GOSSIP_MESSAGES_RECEIVED.with_label_values(label_values)
                                        .inc();
                continue;
            }
        };

        let proto = match RumorEnvelope::decode(&payload) {
            Ok(proto) => proto,
            Err(e) => {
                error!("Error parsing protocol message: {:?}", e);
                let label_values = &["undecodable", "failure", "unknown"];
                GOSSIP_BYTES_RECEIVED.with_label_values(label_values)
                                     .set(payload.len().to_i64());
                GOSSIP_MESSAGES_RECEIVED.with_label_values(label_values)
                                        .inc();
                continue 'recv;
            }
        };

        let blocked = server.is_member_blocked_sblr(&proto.from_id);
        let blocked_label = if blocked { "true" } else { "false" };
        let label_values = &[&proto.r#type.to_string(), "success", blocked_label];

        GOSSIP_MESSAGES_RECEIVED.with_label_values(label_values)
                                .inc();
        GOSSIP_BYTES_RECEIVED.with_label_values(label_values)
                             .set(payload.len().to_i64());

        if blocked {
            warn!("Not processing message from {} - it is blocked",
                  proto.from_id);
            continue 'recv;
        }

        match proto.kind {
            RumorKind::Membership(membership) => {
                server.insert_member_from_rumor_mlw_smw_rhw(membership.member, membership.health);
            }
            RumorKind::Service(service) => server.insert_service_rsw_mlw_rhw(*service),
            RumorKind::ServiceConfig(service_config) => {
                server.insert_service_config_rsw_rhw(service_config);
            }
            RumorKind::ServiceFile(service_file) => {
                server.insert_service_file_rsw_rhw(service_file);
            }
            RumorKind::ServiceHealth(service_health) => {
                server.insert_service_health_rsw_rhw(service_health);
            }
            RumorKind::Election(election) => {
                server.insert_election_rsw_mlr_rhw_msr(election);
            }
            RumorKind::ElectionUpdate(election) => {
                server.insert_update_election_rsw_mlr_rhw(election);
            }
            RumorKind::Departure(departure) => {
                server.insert_departure_rsw_mlw_rhw(departure);
            }
        }
    }
}
