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

use std::default::Default;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use std::fmt;

use time::{SteadyTime, Duration as TimeDuration};
use protobuf::{Message, RepeatedField};

use message::swim::{Ack, Ping, PingReq, Swim, Swim_Type};
use server::Server;
use member::{Member, Health};

/// How long to sleep between calls to `recv`.
const PING_RECV_QUEUE_EMPTY_SLEEP_MS: u64 = 10;
/// How long to wait for an Ack after we ping
const PING_TIMING_DEFAULT_MS: i64 = 1000;
/// How long to wait for an Ack after we PingReq - should be at least 2x the PING_TIMING_DEFAULT_MS
const PINGREQ_TIMING_DEFAULT_MS: i64 = 2100;
/// How many protocol periods before a suspect member is marked as confirmed.
const SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS: i64 = 3;

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

/// The timing of the outbound thread.
#[derive(Debug, Clone)]
pub struct Timing {
    pub ping_ms: i64,
    pub pingreq_ms: i64,
    pub suspicion_timeout_protocol_periods: i64,
}

impl Default for Timing {
    fn default() -> Timing {
        Timing {
            ping_ms: PING_TIMING_DEFAULT_MS,
            pingreq_ms: PINGREQ_TIMING_DEFAULT_MS,
            suspicion_timeout_protocol_periods: SUSPICION_TIMEOUT_DEFAULT_PROTOCOL_PERIODS,
        }
    }
}

impl Timing {
    /// Set up a new Timing
    pub fn new(ping_ms: i64, pingreq_ms: i64, suspicion_timeout_protocol_periods: i64) -> Timing {
        Timing {
            ping_ms: ping_ms,
            pingreq_ms: pingreq_ms,
            suspicion_timeout_protocol_periods: suspicion_timeout_protocol_periods,
        }
    }

    /// How long is a protocl period, in millis.
    pub fn protocol_period_ms(&self) -> i64 {
        self.ping_ms + self.pingreq_ms
    }

    /// When should this ping record time out?
    pub fn ping_timeout(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.ping_ms)
    }

    /// When should this pingreq timeout?
    pub fn pingreq_timeout(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.pingreq_ms)
    }

    /// How long before the next scheduled protocol period
    pub fn next_protocol_period(&self) -> SteadyTime {
        SteadyTime::now() + TimeDuration::milliseconds(self.ping_ms + self.pingreq_ms)
    }

    /// How long before this suspect entry times out
    pub fn suspicion_timeout_duration(&self) -> TimeDuration {
        TimeDuration::milliseconds(self.protocol_period_ms() *
                                   self.suspicion_timeout_protocol_periods)
    }
}

/// The outbound thread
pub struct Outbound<'a> {
    pub server: &'a Server,
    pub rx_inbound: mpsc::Receiver<(SocketAddr, Swim)>,
    pub timing: Timing,
}

impl<'a> Outbound<'a> {
    /// Creates a new Outbound struct.
    pub fn new(server: &'a Server,
               rx_inbound: mpsc::Receiver<(SocketAddr, Swim)>,
               timing: Timing)
               -> Outbound {
        Outbound {
            server: server,
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
        loop {
            if self.server.pause.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            self.server.update_round();

            let check_list = self.server
                .member_list
                .check_list(self.server
                    .member
                    .read()
                    .expect("Member is poisoned")
                    .get_id());

            for member in check_list {
                let pingable = self.server
                    .member_list
                    .member_alive_suspect_or_persistent(&member);

                if pingable {
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
        let addr = member.socket_address();

        trace_swim!(&self.server, "probe-begin", &format!("{}", addr), None);

        // Ping the member, and wait for the ack.
        ping(self.server, &member, addr, None);
        if self.recv_ack(&member, addr, AckFrom::Ping) {
            trace_swim!(&self.server,
                        "probe-ack-received",
                        &format!("{}", addr),
                        None);
            trace_swim!(&self.server, "probe-complete", &format!("{}", addr), None);
            return;
        }

        {
            let me = match self.server.member.read() {
                Ok(me) => me,
                Err(e) => panic!("The member lock is poisoned: {:?}", e),
            };
            self.server.member_list.with_pingreq_targets(&me, &member, |pingreq_target| {
                trace_swim!(&self.server,
                            "probe-pingreq",
                            pingreq_target.get_address(),
                            None);
                pingreq(self.server, pingreq_target, &member);
            });
        }
        if !self.recv_ack(&member, addr, AckFrom::PingReq) {
            // We mark as suspect when we fail to get a response from the PingReq. That moves us
            // into the suspicion phase, where anyone marked as suspect has a certain number of
            // protocol periods to recover.
            warn!("Marking {} as Suspect", member.get_id());
            trace_swim!(&self.server,
                        "probe-marked-suspect",
                        &format!("{}", addr),
                        None);
            self.server.insert_member(member, Health::Suspect);
        }
        trace_swim!(&self.server, "probe-complete", &format!("{}", addr), None);
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
                        ack_from.set_address(format!("{}", real_addr));
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
    let rumors = server.rumor_list.take_by_kind(target.get_id(), 6, "member");
    let mut membership_entries = RepeatedField::new();
    for &(ref rkey, _heat) in rumors.iter() {
        membership_entries.push(server.member_list.membership_for(&rkey.key));
    }
    server.rumor_list.update_heat(target.get_id(), &rumors);
    swim.set_membership(membership_entries);
}

/// Send a PingReq.
pub fn pingreq(server: &Server, pingreq_target: &Member, target: &Member) {
    let addr = pingreq_target.socket_address();
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
    match server.socket.send_to(&swim.write_to_bytes().unwrap(), addr) {
        Ok(_s) => {
            info!("Sent PingReq to {}@{} for {}@{}",
                  pingreq_target.get_id(),
                  addr,
                  target.get_id(),
                  target.socket_address())
        }
        Err(e) => {
            error!("Failed PingReq to {}@{} for {}@{}: {}",
                   pingreq_target.get_id(),
                   addr,
                   target.get_id(),
                   target.socket_address(),
                   e)
        }
    }
    trace_swim!(server, "send-pingreq", &format!("{}", addr), Some(&swim));
}

/// Send a Ping.
pub fn ping(server: &Server, target: &Member, addr: SocketAddr, mut forward_to: Option<Member>) {
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

    match server.socket.send_to(&swim.write_to_bytes().unwrap(), addr) {
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
    trace_swim!(server, "send-ping", &format!("{}", addr), Some(&swim));
}

/// Forward an ack on.
pub fn forward_ack(server: &Server, addr: SocketAddr, swim: Swim) {
    trace_swim!(server,
                "send-forward-ack",
                &format!("{}", addr),
                Some(&swim));
    match server.socket.send_to(&swim.write_to_bytes().unwrap(), addr) {
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
pub fn ack(server: &Server, target: &Member, addr: SocketAddr, mut forward_to: Option<Member>) {
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
    match server.socket.send_to(&swim.write_to_bytes().unwrap(), addr) {
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
    trace_swim!(server, "send-ack", &format!("{}", addr), Some(&swim));
}
