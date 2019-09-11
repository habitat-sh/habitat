//! The outbound thread.
//!
//! This module handles the implementation of the swim probe protocol.

use super::AckReceiver;
use crate::{member::{Health,
                     Member},
            rumor::{RumorKey,
                    RumorType},
            server::{timing::Timing,
                     Server},
            swim::{Ack,
                   Ping,
                   PingReq,
                   Swim}};
use habitat_common::liveliness_checker;
use habitat_core::util::ToI64;
use prometheus::{HistogramTimer,
                 HistogramVec,
                 IntCounterVec,
                 IntGaugeVec};
use std::{fmt,
          net::{SocketAddr,
                UdpSocket},
          sync::mpsc,
          thread,
          time::Duration};
use time::SteadyTime;

/// How long to sleep between calls to `recv`.
const PING_RECV_QUEUE_EMPTY_SLEEP_MS: u64 = 10;

lazy_static! {
    static ref SWIM_MESSAGES_SENT: IntCounterVec =
        register_int_counter_vec!("hab_butterfly_swim_messages_sent_total",
                                  "Total number of SWIM messages sent",
                                  &["type"]).unwrap();
    static ref SWIM_BYTES_SENT: IntGaugeVec =
        register_int_gauge_vec!("hab_butterfly_swim_sent_bytes",
                                "SWIM message size sent in bytes",
                                &["type"]).unwrap();
    static ref SWIM_PROBES_SENT: IntCounterVec =
        register_int_counter_vec!("hab_butterfly_swim_probes_sent_total",
                                  "Total number of SWIM probes sent",
                                  &["type"]).unwrap();
    static ref SWIM_PROBE_DURATION: HistogramVec =
        register_histogram_vec!("hab_butterfly_swim_probe_duration_seconds",
                                "SWIM probe round trip time",
                                &["type"]).unwrap();
}

#[derive(Clone, Copy, Debug)]
enum AckFrom {
    Ping,
    PingReq,
}

impl fmt::Display for AckFrom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AckFrom::Ping => write!(f, "Ping"),
            AckFrom::PingReq => write!(f, "PingReq"),
        }
    }
}

pub fn spawn_thread(name: String,
                    server: Server,
                    socket: UdpSocket,
                    rx_inbound: AckReceiver,
                    timing: Timing)
                    -> std::io::Result<()> {
    thread::Builder::new().name(name)
                          .spawn(move || -> ! { run_loop(&server, &socket, &rx_inbound, &timing) })
                          .map(|_| ())
}

/// Run the outbound thread. Gets a list of members to ping, then walks the list, probing each
/// member.
///
/// If the probe completes before the next protocol period is scheduled, waits for the protocol
/// period to finish before starting the next probe.
fn run_loop(server: &Server, socket: &UdpSocket, rx_inbound: &AckReceiver, timing: &Timing) -> ! {
    let mut have_members = false;
    loop {
        liveliness_checker::mark_thread_alive().and_divergent();

        if !have_members {
            let num_initial = server.member_list.len_initial_members_imlr();
            if num_initial != 0 {
                // The minimum that's strictly more than half
                #[allow(clippy::integer_division)]
                let min_to_start = num_initial / 2 + 1;

                if server.member_list.len_mlr() >= min_to_start {
                    have_members = true;
                } else {
                    server.member_list.with_initial_members_imlr(|member| {
                                          ping_mlr_smr_rhw(&server,
                                                           &socket,
                                                           &member,
                                                           member.swim_socket_address(),
                                                           None);
                                      });
                }
            }
        }

        if server.paused() {
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        server.update_swim_round();

        let long_wait = timing.next_protocol_period();

        let check_list = server.member_list.check_list_mlr(&server.member_id);

        for member in check_list {
            if server.member_list.pingable_mlr(&member) {
                // This is the timeout for the next protocol period - if we
                // complete faster than this, we want to wait in the end
                // until this timer expires.
                let next_protocol_period = timing.next_protocol_period();

                probe_mlw_smr_rhw(&server, &socket, &rx_inbound, &timing, member);

                if SteadyTime::now() <= next_protocol_period {
                    let wait_time = (next_protocol_period - SteadyTime::now()).num_milliseconds();
                    if wait_time > 0 {
                        debug!("Waiting {} until the next protocol period", wait_time);
                        thread::sleep(Duration::from_millis(wait_time as u64));
                    }
                }
            }
        }

        if SteadyTime::now() <= long_wait {
            let wait_time = (long_wait - SteadyTime::now()).num_milliseconds();
            if wait_time > 0 {
                thread::sleep(Duration::from_millis(wait_time as u64));
            }
        }
    }
}

/// Probe Loop
///
/// First, we send the ping to the remote address. This operation never blocks - we just
/// pass the data straight on to the kernel for UDP goodness. Then we grab a timer for how
/// long we're willing to run this phase, and start listening for Ack packets from the
/// Inbound thread. If we receive an Ack that is for any Member other than the one we are
/// currently pinging, we discard it. Otherwise, we set the address for the Member whose Ack
/// we received to the one we saw on the wire, and insert it into the MemberList.
///
/// If we don't receive anything on the channel, we check if the current time has exceeded
/// our timeout. If it has, we break out of the Ping loop, and proceed to the PingReq loop.
/// If the timer has not been exceeded, we park this thread for
/// PING_RECV_QUEUE_EMPTY_SLEEP_MS, and try again.
///
/// If we don't receive anything at all in the Ping/PingReq loop, we mark the member as Suspect.
///
/// # Locking (see locking.md)
/// * `MemberList::entries` (write)
/// * `Server::member` (read)
/// * `RumorHeat::inner` (write)
fn probe_mlw_smr_rhw(server: &Server,
                     socket: &UdpSocket,
                     rx_inbound: &AckReceiver,
                     timing: &Timing,
                     member: Member) {
    let pa_timer = SWIM_PROBE_DURATION.with_label_values(&["ping/ack"])
                                      .start_timer();
    let mut pr_timer: Option<HistogramTimer> = None;
    let addr = member.swim_socket_address();

    // Ping the member, and wait for the ack.
    SWIM_PROBES_SENT.with_label_values(&["ping"]).inc();
    ping_mlr_smr_rhw(server, socket, &member, addr, None);

    if recv_ack_mlw_rhw(server, rx_inbound, timing, &member, addr, AckFrom::Ping) {
        SWIM_PROBES_SENT.with_label_values(&["ack"]).inc();
        pa_timer.observe_duration();
        return;
    }

    let pingreq_message = PingReq { membership: vec![],
                                    from:       server.myself.lock_smr().to_member(),
                                    target:     member.clone(), };
    let swim = populate_membership_rumors_mlr_rhw(server, &member, pingreq_message);

    server.member_list
          .with_pingreq_targets_mlr(server.member_id(), &member.id, |pingreq_target| {
              SWIM_PROBES_SENT.with_label_values(&["pingreq"]).inc();
              pr_timer = Some(SWIM_PROBE_DURATION.with_label_values(&["pingreq/ack"])
                                                 .start_timer());
              pingreq(server, socket, pingreq_target, &member, &swim);
          });

    if recv_ack_mlw_rhw(server, rx_inbound, timing, &member, addr, AckFrom::PingReq) {
        SWIM_PROBES_SENT.with_label_values(&["ack"]).inc();
    } else {
        // We mark as suspect when we fail to get a response from the PingReq. That moves us
        // into the suspicion phase, where anyone marked as suspect has a certain number of
        // protocol periods to recover.
        warn!("Marking {} as Suspect", &member.id);
        server.insert_member_mlw_rhw(member, Health::Suspect);
        SWIM_PROBES_SENT.with_label_values(&["pingreq/failure"])
                        .inc();
    }

    if pr_timer.is_some() {
        pr_timer.unwrap().observe_duration();
    }
}

/// Listen for an ack from the `Inbound` thread.
///
/// # Locking (see locking.md)
/// * `MemberList::entries` (write)
/// * `RumorHeat::inner` (write)
fn recv_ack_mlw_rhw(server: &Server,
                    rx_inbound: &AckReceiver,
                    timing: &Timing,
                    member: &Member,
                    addr: SocketAddr,
                    ack_from: AckFrom)
                    -> bool {
    let timeout = match ack_from {
        AckFrom::Ping => timing.ping_timeout(),
        AckFrom::PingReq => timing.pingreq_timeout(),
    };
    loop {
        match rx_inbound.try_recv() {
            Ok((real_addr, mut ack)) => {
                // If this was forwarded to us, we want to retain the address of the member who
                // sent the ack, not the one we received on the socket.
                if ack.forward_to.is_none() {
                    ack.from.address = real_addr.ip().to_string();
                }
                if member.id != ack.from.id {
                    if ack.from.departed {
                        server.insert_member_mlw_rhw(ack.from, Health::Departed);
                    } else {
                        server.insert_member_mlw_rhw(ack.from, Health::Alive);
                    }
                    // Keep listening, we want the ack we expected
                    continue;
                } else {
                    // We got the ack we are looking for; return.
                    if ack.from.departed {
                        server.insert_member_mlw_rhw(ack.from, Health::Departed);
                    } else {
                        server.insert_member_mlw_rhw(ack.from, Health::Alive);
                    }
                    return true;
                }
            }
            Err(mpsc::TryRecvError::Empty) => {
                if SteadyTime::now() > timeout {
                    warn!("Timed out waiting for Ack from {}@{}", &member.id, addr);
                    return false;
                }
                thread::sleep(Duration::from_millis(PING_RECV_QUEUE_EMPTY_SLEEP_MS));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                panic!("Outbound thread has disconnected! This is fatal.");
            }
        }
    }
}

/// Created a SWIM message from the given `message` template and populate it with rumors.
///
/// # Locking (see locking.md)
/// * `MemberList::entries` (read)
/// * `RumorHeat::inner` (write)
pub fn populate_membership_rumors_mlr_rhw(server: &Server,
                                          target: &Member,
                                          message: impl Into<Swim>)
                                          -> Swim {
    let mut swim = message.into();

    // If this isn't the first time we are communicating with this target, we want to include this
    // targets current status. This ensures that members always get a "Confirmed" rumor, before we
    // have the chance to flip it to "Alive", which helps make sure we heal from a partition.
    if server.member_list.contains_member_mlr(&target.id) {
        if let Some(always_target) = server.member_list.membership_for_mlr(&target.id) {
            swim.membership.push(always_target);
        }
    }

    // NOTE: the way this is currently implemented, this is grabbing
    // the 5 coolest (but still warm!) Member rumors.
    let rumors: Vec<RumorKey> = server
        .rumor_heat
        .currently_hot_rumors_rhr(&target.id)
        .into_iter()
        .filter(|ref r| r.kind == RumorType::Member)
        .take(5) // TODO (CM): magic number!
        .collect();

    for rkey in rumors.iter() {
        if let Some(member) = server.member_list.membership_for_mlr(&rkey.to_string()) {
            swim.membership.push(member);
        }
    }
    // We don't want to update the heat for rumors that we know we are sending to a target that is
    // confirmed dead; the odds are, they won't receive them. Lets spam them a little harder with
    // rumors.
    if !server.member_list.persistent_and_confirmed_mlr(target) {
        server.rumor_heat.cool_rumors_rhw(&target.id, &rumors);
    }

    swim
}

/// Send a PingReq: request `pingreq_target` to ping `target` on the behalf of `server` to see if
/// `target` is alive despite not being directly reachable from `server`. In other words,
/// `pingreq_target` is the proxy and `target` is the final destination.
fn pingreq(server: &Server, // TODO: eliminate this arg
           socket: &UdpSocket,
           pingreq_target: &Member,
           target: &Member,
           swim: &Swim) {
    let addr = pingreq_target.swim_socket_address();
    let bytes = match swim.clone().encode() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            let label_values = &["pingreq"];
            SWIM_MESSAGES_SENT.with_label_values(label_values).inc();
            SWIM_BYTES_SENT.with_label_values(label_values)
                           .set(payload.len().to_i64());
            trace!("Sent PingReq to {}@{} for {}@{}",
                   &pingreq_target.id,
                   addr,
                   &target.id,
                   target.swim_socket_address());
        }
        Err(e) => {
            error!("Failed PingReq to {}@{} for {}@{}: {}",
                   &pingreq_target.id,
                   addr,
                   &target.id,
                   target.swim_socket_address(),
                   e)
        }
    }
}

/// Send a Ping.
///
/// # Locking (see locking.md)
/// * `MemberList::entries` (read)
/// * `Server::member` (read)
/// * `RumorHeat::inner` (write)
pub fn ping_mlr_smr_rhw(server: &Server,
                        socket: &UdpSocket,
                        target: &Member,
                        addr: SocketAddr,
                        forward_to: Option<&Member>) {
    let ping_msg = Ping { membership: vec![],
                          from:       server.myself.lock_smr().to_member(),
                          forward_to: forward_to.cloned(), /* TODO: see if we can eliminate this
                                                            * clone */ };
    let swim = populate_membership_rumors_mlr_rhw(server, target, ping_msg);
    let bytes = match swim.clone().encode() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            let label_values = &["ping"];
            SWIM_MESSAGES_SENT.with_label_values(label_values).inc();
            SWIM_BYTES_SENT.with_label_values(label_values)
                           .set(payload.len().to_i64());
            let on_behalf_of = match forward_to {
                Some(x) => format!(" on behalf of {}@{}", x.id, x.address),
                None => "".into(),
            };
            trace!("Sent Ping to {}{}", addr, on_behalf_of);
        }
        Err(e) => error!("Failed Ping to {}: {}", addr, e),
    }
}

pub fn ping(server: &Server,
            socket: &UdpSocket,
            addr: SocketAddr,
            forward_to: Option<&Member>,
            swim: &Swim) {
    let bytes = match swim.clone().encode() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            let label_values = &["ping"];
            SWIM_MESSAGES_SENT.with_label_values(label_values).inc();
            SWIM_BYTES_SENT.with_label_values(label_values)
                           .set(payload.len().to_i64());
            let on_behalf_of = match forward_to {
                Some(x) => format!(" on behalf of {}@{}", x.id, x.address),
                None => "".into(),
            };
            trace!("Sent Ping to {}{}", addr, on_behalf_of);
        }
        Err(e) => error!("Failed Ping to {}: {}", addr, e),
    }
}

/// Forward an ack on.
pub fn forward_ack(server: &Server, socket: &UdpSocket, addr: SocketAddr, msg: Ack) {
    let member_id = msg.from.id.clone();
    let swim: Swim = msg.into();
    let bytes = match swim.encode() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    match socket.send_to(&payload, addr) {
        Ok(_s) => trace!("Forwarded ack to {}@{}", member_id, addr),
        Err(e) => error!("Failed ack to {}@{}: {}", member_id, addr, e),
    }
}

/// Send an Ack.
///
/// # Locking (see locking.md)
/// * `MemberList::entries` (read)
/// * `Server::member` (read)
/// * `RumorHeat::inner` (write)
pub fn ack_mlr_smr_rhw(server: &Server,
                       socket: &UdpSocket,
                       target: &Member,
                       addr: SocketAddr,
                       forward_to: Option<Member>) {
    let ack_msg = Ack { membership: vec![],
                        from:       server.myself.lock_smr().to_member(),
                        forward_to: forward_to.map(Member::from), };
    let member_id = ack_msg.from.id.clone();
    let swim = populate_membership_rumors_mlr_rhw(server, target, ack_msg);
    let bytes = match swim.clone().encode() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protocol message failed: {}", e);
            return;
        }
    };
    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            let label_values = &["ack"];
            SWIM_MESSAGES_SENT.with_label_values(label_values).inc();
            SWIM_BYTES_SENT.with_label_values(label_values)
                           .set(payload.len().to_i64());
            trace!("Sent ack to {}@{}", member_id, addr);
        }
        Err(e) => error!("Failed ack to {}@{}: {}", member_id, addr, e),
    }
}
