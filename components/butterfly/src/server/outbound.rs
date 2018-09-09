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

//! The outbound thread.
//!
//! This module handles the implementation of the swim probe protocol.

use std::fmt;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use time::SteadyTime;

use super::AckReceiver;
use member::{Health, Member};
use rumor::{RumorKey, RumorType};
use server::timing::Timing;
use server::Server;
use swim::{Ack, Ping, PingReq, Swim};
use trace::TraceKind;

/// How long to sleep between calls to `recv`.
const PING_RECV_QUEUE_EMPTY_SLEEP_MS: u64 = 10;

/// Where an Ack came from; either Ping or PingReq.
#[derive(Debug)]
enum AckFrom {
    Ping,
    PingReq,
}

impl fmt::Display for AckFrom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &AckFrom::Ping => write!(f, "Ping"),
            &AckFrom::PingReq => write!(f, "PingReq"),
        }
    }
}

/// The outbound thread
pub struct Outbound {
    pub server: Server,
    pub socket: UdpSocket,
    pub rx_inbound: AckReceiver,
    pub timing: Timing,
}

impl Outbound {
    /// Creates a new Outbound struct.
    pub fn new(
        server: Server,
        socket: UdpSocket,
        rx_inbound: AckReceiver,
        timing: Timing,
    ) -> Outbound {
        Outbound {
            server: server,
            socket: socket,
            rx_inbound: rx_inbound,
            timing: timing,
        }
    }

    /// Run the outbound thread. Gets a list of members to ping, then walks the list, probing each
    /// member.
    ///
    /// If the probe completes before the next protocol period is scheduled, waits for the protocol
    /// period to finish before starting the next probe.
    pub fn run(&mut self) {
        let mut have_members = false;
        loop {
            if !have_members {
                let num_initial = self.server.member_list.len_initial_members();
                if num_initial != 0 {
                    // The minimum that's strictly more than half
                    let min_to_start = num_initial / 2 + 1;

                    if self.server.member_list.len() >= min_to_start {
                        have_members = true;
                    } else {
                        self.server.member_list.with_initial_members(|member| {
                            ping(
                                &self.server,
                                &self.socket,
                                &member,
                                member.swim_socket_address(),
                                None,
                            );
                        });
                    }
                }
            }

            if self.server.pause.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.server.update_swim_round();

            let long_wait = self.timing.next_protocol_period();

            let check_list = self.server.member_list.check_list(
                &self
                    .server
                    .member
                    .read()
                    .expect("Member is poisoned")
                    .as_ref()
                    .id,
            );

            for member in check_list {
                if self.server.member_list.pingable(&member) {
                    // This is the timeout for the next protocol period - if we
                    // complete faster than this, we want to wait in the end
                    // until this timer expires.
                    let next_protocol_period = self.timing.next_protocol_period();

                    self.probe(member);

                    if SteadyTime::now() <= next_protocol_period {
                        let wait_time =
                            (next_protocol_period - SteadyTime::now()).num_milliseconds();
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
    fn probe(&mut self, member: Member) {
        let addr = member.swim_socket_address();

        trace_it!(PROBE: &self.server, TraceKind::ProbeBegin, &member.id, addr);

        // Ping the member, and wait for the ack.
        ping(&self.server, &self.socket, &member, addr, None);
        if self.recv_ack(&member, addr, AckFrom::Ping) {
            trace_it!(PROBE: &self.server, TraceKind::ProbeAckReceived, &member.id, addr);
            trace_it!(PROBE: &self.server, TraceKind::ProbeComplete, &member.id, addr);
            return;
        }

        self.server.member_list.with_pingreq_targets(
            self.server.member_id(),
            &member.id,
            |pingreq_target| {
                trace_it!(PROBE: &self.server,
                          TraceKind::ProbePingReq,
                          &pingreq_target.id,
                          &pingreq_target.address);
                pingreq(&self.server, &self.socket, &pingreq_target, &member);
            },
        );
        if !self.recv_ack(&member, addr, AckFrom::PingReq) {
            // We mark as suspect when we fail to get a response from the PingReq. That moves us
            // into the suspicion phase, where anyone marked as suspect has a certain number of
            // protocol periods to recover.
            warn!("Marking {} as Suspect", &member.id);
            trace_it!(PROBE: &self.server, TraceKind::ProbeSuspect, &member.id, addr);
            trace_it!(PROBE: &self.server, TraceKind::ProbeComplete, &member.id, addr);
            self.server.insert_member(member, Health::Suspect);
        } else {
            trace_it!(PROBE: &self.server, TraceKind::ProbeComplete, &member.id, addr);
        }
    }

    /// Listen for an ack from the `Inbound` thread.
    fn recv_ack(&mut self, member: &Member, addr: SocketAddr, ack_from: AckFrom) -> bool {
        let timeout = match ack_from {
            AckFrom::Ping => self.timing.ping_timeout(),
            AckFrom::PingReq => self.timing.pingreq_timeout(),
        };
        loop {
            match self.rx_inbound.try_recv() {
                Ok((real_addr, mut ack)) => {
                    // If this was forwarded to us, we want to retain the address of the member who
                    // sent the ack, not the one we received on the socket.
                    if ack.forward_to.is_none() {
                        ack.from.address = real_addr.ip().to_string();
                    }
                    if member.id != ack.from.id {
                        if ack.from.departed {
                            self.server.insert_member(ack.from, Health::Departed);
                        } else {
                            self.server.insert_member(ack.from, Health::Alive);
                        }
                        // Keep listening, we want the ack we expected
                        continue;
                    } else {
                        // We got the ack we are looking for; return.
                        if ack.from.departed {
                            self.server.insert_member(ack.from, Health::Departed);
                        } else {
                            self.server.insert_member(ack.from, Health::Alive);
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
}

/// Populate a SWIM message with rumors.
pub fn populate_membership_rumors(server: &Server, target: &Member, swim: &mut Swim) {
    // If this isn't the first time we are communicating with this target, we want to include this
    // targets current status. This ensures that members always get a "Confirmed" rumor, before we
    // have the chance to flip it to "Alive", which helps make sure we heal from a partition.
    if server.member_list.contains_member(&target.id) {
        if let Some(always_target) = server.member_list.membership_for(&target.id) {
            swim.membership.push(always_target);
        }
    }

    // NOTE: the way this is currently implemented, this is grabbing
    // the 5 coolest (but still warm!) Member rumors.
    let rumors: Vec<RumorKey> = server.rumor_heat
        .currently_hot_rumors(&target.id)
        .into_iter()
        .filter(|ref r| r.kind == RumorType::Member)
        .take(5) // TODO (CM): magic number!
        .collect();

    for ref rkey in rumors.iter() {
        if let Some(member) = server.member_list.membership_for(&rkey.key()) {
            swim.membership.push(member);
        }
    }
    // We don't want to update the heat for rumors that we know we are sending to a target that is
    // confirmed dead; the odds are, they won't receive them. Lets spam them a little harder with
    // rumors.
    if !server.member_list.persistent_and_confirmed(target) {
        server.rumor_heat.cool_rumors(&target.id, &rumors);
    }
}

/// Send a PingReq.
pub fn pingreq(server: &Server, socket: &UdpSocket, pingreq_target: &Member, target: &Member) {
    let pingreq = PingReq {
        membership: vec![],
        from: server.member.read().unwrap().as_ref().clone(),
        target: target.clone(),
    };
    let mut swim: Swim = pingreq.into();
    let addr = pingreq_target.swim_socket_address();
    populate_membership_rumors(server, target, &mut swim);
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
        Ok(_s) => trace!(
            "Sent PingReq to {}@{} for {}@{}",
            &pingreq_target.id,
            addr,
            &target.id,
            target.swim_socket_address()
        ),
        Err(e) => error!(
            "Failed PingReq to {}@{} for {}@{}: {}",
            &pingreq_target.id,
            addr,
            &target.id,
            target.swim_socket_address(),
            e
        ),
    }
    trace_it!(
        SWIM: server,
        TraceKind::SendPingReq,
        &pingreq_target.id,
        addr,
        &swim
    );
}

/// Send a Ping.
pub fn ping(
    server: &Server,
    socket: &UdpSocket,
    target: &Member,
    addr: SocketAddr,
    forward_to: Option<Member>,
) {
    let forward_addr = if let Some(ref forward_to) = forward_to {
        Some(format!("{}@{}", forward_to.id, forward_to.address))
    } else {
        None
    };
    let ping = Ping {
        membership: vec![],
        from: server.member.read().unwrap().as_ref().clone(),
        forward_to: forward_to,
    };
    let mut swim: Swim = ping.into();
    populate_membership_rumors(server, target, &mut swim);
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
            if let Some(forward_addr) = forward_addr {
                trace!("Sent Ping to {} on behalf of {}", addr, forward_addr);
            } else {
                trace!("Sent Ping to {}", addr);
            }
        }
        Err(e) => error!("Failed Ping to {}: {}", addr, e),
    }
    trace_it!(SWIM: server, TraceKind::SendPing, &target.id, addr, &swim);
}

/// Forward an ack on.
pub fn forward_ack(server: &Server, socket: &UdpSocket, addr: SocketAddr, msg: Ack) {
    trace_it!(
        SWIM: server,
        TraceKind::SendForwardAck,
        &msg.from.id,
        addr,
        &msg
    );
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
pub fn ack(
    server: &Server,
    socket: &UdpSocket,
    target: &Member,
    addr: SocketAddr,
    forward_to: Option<Member>,
) {
    let ack = Ack {
        membership: vec![],
        from: server.member.read().unwrap().as_ref().clone(),
        forward_to: forward_to.map(Into::into),
    };
    let member_id = ack.from.id.clone();
    let mut swim: Swim = ack.into();
    populate_membership_rumors(server, target, &mut swim);
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
        Ok(_s) => trace!("Sent ack to {}@{}", member_id, addr),
        Err(e) => error!("Failed ack to {}@{}: {}", member_id, addr, e),
    }
    trace_it!(SWIM: server, TraceKind::SendAck, &target.id, addr, &swim);
}
