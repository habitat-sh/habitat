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

//! The outbound thread.
//!
//! This module handles the implementation of the swim probe protocol.

use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;
use std::fmt;

use time::SteadyTime;
use protobuf::{Message, RepeatedField};

use message::swim::{Ack, Ping, PingReq, Swim, Swim_Type, Rumor_Type};
use server::Server;
use server::timing::Timing;
use member::{Member, Health};
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
pub struct Outbound<'a> {
    pub server: &'a Server,
    pub socket: UdpSocket,
    pub rx_inbound: mpsc::Receiver<(SocketAddr, Swim)>,
    pub timing: Timing,
}

impl<'a> Outbound<'a> {
    /// Creates a new Outbound struct.
    pub fn new(server: &'a Server,
               socket: UdpSocket,
               rx_inbound: mpsc::Receiver<(SocketAddr, Swim)>,
               timing: Timing)
               -> Outbound {
        Outbound {
            server: server,
            socket: socket,
            rx_inbound: rx_inbound,
            timing: timing,
        }
    }

    /// Run the outbound thread. Gets a list of members to pinmg, then walks the list, probing each
    /// member.
    ///
    /// If the probe completes before the next protocol period is scheduled, waits for the protocol
    /// period to finish before starting the next probe.
    pub fn run(&mut self) {
        let mut have_members = false;
        loop {
            if !have_members {
                if self.server.member_list.len() > 0 {
                    have_members = true;
                } else {
                    self.server.member_list.with_initial_members(|member| {
                        ping(&self.server,
                             &self.socket,
                             &member,
                             member.swim_socket_address(),
                             None);
                    });
                }
            }

            if self.server.pause.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.server.update_swim_round();

            let long_wait = self.timing.next_protocol_period();

            let check_list = self.server
                .member_list
                .check_list(self.server
                    .member
                    .read()
                    .expect("Member is poisoned")
                    .get_id());

            for member in check_list {
                if self.server.member_list.pingable(&member) {
                    // This is the timeout for the next protocol period - if we
                    // complete faster than this, we want to wait in the end
                    // until this timer expires.
                    let next_protocol_period = self.timing.next_protocol_period();

                    self.probe(member);

                    if SteadyTime::now() <= next_protocol_period {
                        let wait_time = next_protocol_period - SteadyTime::now();
                        debug!("Waiting {} until the next protocol period",
                               wait_time.num_milliseconds());
                        thread::sleep(Duration::from_millis(wait_time.num_milliseconds() as u64));
                    }
                }
            }

            if SteadyTime::now() <= long_wait {
                let wait_time = long_wait - SteadyTime::now();
                thread::sleep(Duration::from_millis(wait_time.num_milliseconds() as u64));
            }
        }
    }

    ///
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
    /// If we don't recieve anything at all in the Ping/PingReq loop, we mark the member as Suspect.
    fn probe(&mut self, member: Member) {
        let addr = member.swim_socket_address();

        trace_it!(PROBE: &self.server, TraceKind::ProbeBegin, member.get_id(), addr);

        // Ping the member, and wait for the ack.
        ping(self.server, &self.socket, &member, addr, None);
        if self.recv_ack(&member, addr, AckFrom::Ping) {
            trace_it!(PROBE: &self.server, TraceKind::ProbeAckReceived, member.get_id(), addr);
            trace_it!(PROBE: &self.server, TraceKind::ProbeComplete, member.get_id(), addr);
            return;
        }

        self.server.member_list.with_pingreq_targets(self.server.member_id(), member.get_id(), |pingreq_target| {
            trace_it!(PROBE: &self.server, TraceKind::ProbePingReq, pingreq_target.get_id(), pingreq_target.get_address());
            pingreq(self.server, &self.socket, &pingreq_target, &member);
        });
        if !self.recv_ack(&member, addr, AckFrom::PingReq) {
            // We mark as suspect when we fail to get a response from the PingReq. That moves us
            // into the suspicion phase, where anyone marked as suspect has a certain number of
            // protocol periods to recover.
            warn!("Marking {} as Suspect", member.get_id());
            trace_it!(PROBE: &self.server, TraceKind::ProbeSuspect, member.get_id(), addr);
            trace_it!(PROBE: &self.server, TraceKind::ProbeComplete, member.get_id(), addr);
            self.server.insert_member(member, Health::Suspect);
        } else {
            trace_it!(PROBE: &self.server, TraceKind::ProbeComplete, member.get_id(), addr);
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
                Ok((real_addr, mut swim)) => {
                    let mut ack_from = swim.mut_ack().take_from();
                    if member.get_id() != ack_from.get_id() {
                        error!("Discarding ack from {}@{}; expected {}",
                               ack_from.get_id(),
                               real_addr,
                               member.get_id());
                        // Keep listening, we want the ack we expected
                        continue;
                    }
                    // If this was forwarded to us, we want to retain the address of the member who
                    // sent the ack, not the one we recieved on the socket.
                    if !swim.get_ack().has_forward_to() {
                        ack_from.set_address(format!("{}", real_addr.ip()));
                    }
                    let ack_from_member: Member = ack_from.into();
                    self.server.insert_member(ack_from_member, Health::Alive);
                    // We got the ack we are looking for; return.
                    return true;
                }
                Err(mpsc::TryRecvError::Empty) => {
                    if SteadyTime::now() > timeout {
                        warn!("Timed out waiting for Ack from {}@{}",
                              member.get_id(),
                              addr);
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
    let mut membership_entries = RepeatedField::new();
    // If this isn't the first time we are communicating with this target, we want to include this
    // targets current status. This ensures that members always get a "Confirmed" rumor, before we
    // have the chance to flip it to "Alive", which helps make sure we heal from a partition.
    if server.member_list.contains_member(target.get_id()) {
        let always_target = server.member_list.membership_for(target.get_id());
        membership_entries.push(always_target);
    }
    let rumors = server.rumor_list.take_by_kind(target.get_id(), 5, Rumor_Type::Member);
    for &(ref rkey, _heat) in rumors.iter() {
        membership_entries.push(server.member_list.membership_for(&rkey.key()));
    }
    // We don't want to update the heat for rumors that we know we are sending to a target that is
    // confirmed dead; the odds are, they won't receive them. Lets spam them a little harder with
    // rumors.
    if !server.member_list.persistent_and_confirmed(target) {
        server.rumor_list.update_heat(target.get_id(), &rumors);
    }
    swim.set_membership(membership_entries);
}

/// Send a PingReq.
pub fn pingreq(server: &Server, socket: &UdpSocket, pingreq_target: &Member, target: &Member) {
    let addr = pingreq_target.swim_socket_address();
    let mut swim = Swim::new();
    swim.set_field_type(Swim_Type::PINGREQ);
    let mut pingreq = PingReq::new();
    {
        let member = server.member.read().unwrap();
        pingreq.set_from(member.proto.clone());
    }
    pingreq.set_target(target.proto.clone());
    swim.set_pingreq(pingreq);
    populate_membership_rumors(server, target, &mut swim);
    let bytes = match swim.write_to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };
    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            info!("Sent PingReq to {}@{} for {}@{}",
                  pingreq_target.get_id(),
                  addr,
                  target.get_id(),
                  target.swim_socket_address())
        }
        Err(e) => {
            error!("Failed PingReq to {}@{} for {}@{}: {}",
                   pingreq_target.get_id(),
                   addr,
                   target.get_id(),
                   target.swim_socket_address(),
                   e)
        }
    }
    trace_it!(SWIM: server,
              TraceKind::SendPingReq,
              pingreq_target.get_id(),
              addr,
              &swim);
}

/// Send a Ping.
pub fn ping(server: &Server,
            socket: &UdpSocket,
            target: &Member,
            addr: SocketAddr,
            mut forward_to: Option<Member>) {
    let mut swim = Swim::new();
    swim.set_field_type(Swim_Type::PING);
    let mut ping = Ping::new();
    {
        let member = server.member.read().unwrap();
        ping.set_from(member.proto.clone());
    }
    if forward_to.is_some() {
        let member = forward_to.take().unwrap();
        ping.set_forward_to(member.proto);
    }
    swim.set_ping(ping);
    populate_membership_rumors(server, target, &mut swim);

    let bytes = match swim.write_to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };

    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            if forward_to.is_some() {
                info!("Sent Ping to {} on behalf of {}@{}",
                      addr,
                      swim.get_ping().get_forward_to().get_id(),
                      swim.get_ping().get_forward_to().get_address());
            } else {
                info!("Sent Ping to {}", addr);
            }
        }
        Err(e) => error!("Failed Ping to {}: {}", addr, e),
    }
    trace_it!(SWIM: server,
              TraceKind::SendPing,
              target.get_id(),
              addr,
              &swim);
}

/// Forward an ack on.
pub fn forward_ack(server: &Server, socket: &UdpSocket, addr: SocketAddr, swim: Swim) {
    trace_it!(SWIM: server,
              TraceKind::SendForwardAck,
              swim.get_ack().get_from().get_id(),
              addr,
              &swim);

    let bytes = match swim.write_to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };

    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            info!("Forwarded ack to {}@{}",
                  swim.get_ack().get_from().get_id(),
                  addr)
        }
        Err(e) => {
            error!("Failed ack to {}@{}: {}",
                   swim.get_ack().get_from().get_id(),
                   addr,
                   e)
        }
    }
}

/// Send an Ack.
pub fn ack(server: &Server,
           socket: &UdpSocket,
           target: &Member,
           addr: SocketAddr,
           mut forward_to: Option<Member>) {
    let mut swim = Swim::new();
    swim.set_field_type(Swim_Type::ACK);
    let mut ack = Ack::new();
    {
        let member = server.member.read().unwrap();
        ack.set_from(member.proto.clone());
    }
    if forward_to.is_some() {
        let member = forward_to.take().unwrap();
        ack.set_forward_to(member.proto);
    }
    swim.set_ack(ack);
    populate_membership_rumors(server, target, &mut swim);

    let bytes = match swim.write_to_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };
    let payload = match server.generate_wire(bytes) {
        Ok(payload) => payload,
        Err(e) => {
            error!("Generating protobuf failed: {}", e);
            return;
        }
    };

    match socket.send_to(&payload, addr) {
        Ok(_s) => {
            info!("Sent ack to {}@{}",
                  swim.get_ack().get_from().get_id(),
                  addr)
        }
        Err(e) => {
            error!("Failed ack to {}@{}: {}",
                   swim.get_ack().get_from().get_id(),
                   addr,
                   e)
        }
    }
    trace_it!(SWIM: server,
              TraceKind::SendAck,
              target.get_id(),
              addr,
              &swim);
}
