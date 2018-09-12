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

//! The inbound thread.
//!
//! This module handles all the inbound SWIM messages.

use std::mem;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use super::AckSender;
use error::Error;
use member::{Health, Member};
use network::{AddressAndPort, AddressForNetwork, MyFromStr, Network, SwimReceiver};
use server::{
    outbound,
    zones::{self, ZoneChangeDbgData, ZoneChangeResultsMsgOrNothing},
    Server,
};
use swim::{Ack, Ping, PingReq, Swim, SwimKind, SwimType, ZoneChange};
use trace::TraceKind;
use zone::Zone;

/// Takes the Server and a channel to send received Acks to the outbound thread.
pub struct Inbound<N: Network> {
    pub server: Server<N>,
    pub swim_receiver: N::SwimReceiver,
    pub swim_sender: N::SwimSender,
    pub tx_outbound: AckSender<N::AddressAndPort>,
}

impl<N: Network> Inbound<N> {
    /// Create a new Inbound.
    pub fn new(
        server: Server<N>,
        swim_receiver: N::SwimReceiver,
        swim_sender: N::SwimSender,
        tx_outbound: AckSender<N::AddressAndPort>,
    ) -> Self {
        Self {
            server: server,
            swim_receiver: swim_receiver,
            swim_sender: swim_sender,
            tx_outbound: tx_outbound,
        }
    }

    /// Run the thread. Listens for messages up to 4k in size, and then processes them accordingly.
    pub fn run(&self) {
        let mut recv_buffer: Vec<u8> = vec![0; 4096];
        loop {
            if self.server.pause.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            match self.swim_receiver.receive(&mut recv_buffer[..]) {
                Ok((length, addr)) => {
                    let swim_payload = match self.server.unwrap_wire(&recv_buffer[0..length]) {
                        Ok(swim_payload) => swim_payload,
                        Err(e) => {
                            // NOTE: In the future, we might want to block people who send us
                            // garbage all the time.
                            error!("Error unwrapping protocol message, {}", e);
                            continue;
                        }
                    };
                    let msg = match Swim::decode(&swim_payload) {
                        Ok(msg) => msg,
                        Err(e) => {
                            // NOTE: In the future, we might want to block people who send us
                            // garbage all the time.
                            error!("Error decoding protocol message, {}", e);
                            continue;
                        }
                    };
                    trace!("SWIM Message: {:?}", msg);
                    match msg.kind {
                        SwimKind::Ping(ping) => {
                            if self.server.is_member_blocked(&ping.from.id) {
                                debug!(
                                    "Not processing message from {} - it is blocked",
                                    ping.from.id
                                );
                                continue;
                            }
                            self.process_ping(addr, ping);
                        }
                        SwimKind::Ack(ack) => {
                            if self.server.is_member_blocked(&ack.from.id)
                                && ack.forward_to.is_none()
                            {
                                debug!(
                                    "Not processing message from {} - it is blocked",
                                    ack.from.id
                                );
                                continue;
                            }
                            self.process_ack(addr, ack);
                        }
                        SwimKind::PingReq(pingreq) => {
                            if self.server.is_member_blocked(&pingreq.from.id) {
                                debug!(
                                    "Not processing message from {} - it is blocked",
                                    pingreq.from.id
                                );
                                continue;
                            }
                            self.process_pingreq(addr, pingreq);
                        }
                        SwimKind::ZoneChange(zone_change) => {
                            if self.server.is_member_blocked(&zone_change.from.id) {
                                debug!(
                                    "Not processing message from {} - it is blocked",
                                    zone_change.from.id
                                );
                                continue;
                            }
                            self.process_zone_change(addr, zone_change);
                        }
                    }
                }
                Err(Error::SwimReceiveIOError(e)) => {
                    // TODO: We can't use magic numbers here because the Supervisor runs on more
                    // than one platform. I'm sure these were added as specific OS errors for Linux
                    // but we need to also handle Windows & Mac.
                    match e.raw_os_error() {
                        Some(35) | Some(11) | Some(10035) | Some(10060) => {
                            // This is the normal non-blocking result, or a timeout
                        }
                        Some(_) => {
                            error!("SWIM Receive error: {}", e);
                            debug!("SWIM Receive error debug: {:?}", e);
                        }
                        None => {
                            error!("SWIM Receive error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("SWIM Receive error: {}", e);
                }
            }
        }
    }

    /// Process pingreq messages.
    fn process_pingreq(&self, addr: N::AddressAndPort, mut msg: PingReq) {
        trace_it!(SWIM: &self.server, TraceKind::RecvPingReq, &msg.from.id, addr, &msg);
        msg.from.address = addr.get_address().to_string();
        match self
            .server
            .member_list
            .members
            .read()
            .expect("Member list lock poisoned")
            .get(&msg.target.id)
        {
            Some(target) => {
                // Set the route-back address to the one we received the pingreq from
                outbound::ping(
                    &self.server,
                    &self.swim_sender,
                    target,
                    target.swim_socket_address(),
                    Some(msg.from),
                );
            }
            None => {
                error!("PingReq request {:?} for invalid target", msg);
                return;
            }
        }
    }

    /// Process ack messages; forwards to the outbound thread.
    fn process_ack(&self, addr: N::AddressAndPort, mut msg: Ack) {
        let results = zones::handle_zone(
            &self.server,
            &msg.zones,
            SwimType::Ack,
            &msg.from,
            &msg.to,
            addr,
        );
        if results.bail_out {
            return;
        }
        for (msg, target) in results.msgs_and_targets_for_zone_change {
            outbound::zone_change(&self.server, &self.swim_sender, &target, msg);
        }
        if results.sender_has_nil_zone {
            warn!("Supervisor {} sent an Ack with a nil zone ID", msg.from.id);
        }
        trace_it!(SWIM: &self.server, TraceKind::RecvAck, &msg.from.id, addr, &msg);
        trace!("Ack from {}@{}", msg.from.id, addr);
        if msg.forward_to.is_some() {
            if *self.server.member_id != msg.forward_to.as_ref().unwrap().id {
                let (forward_to_addr, from_addr) = {
                    let forward_to = msg.forward_to.as_ref().unwrap();
                    let forward_to_addr =
                        match AddressForNetwork::<N>::create_from_str(&forward_to.address) {
                            Ok(addr) => addr,
                            Err(e) => {
                                error!(
                                    "Abandoning Ack forward: cannot parse member address: {}, {}",
                                    forward_to.address, e
                                );
                                return;
                            }
                        };
                    let forward_to_addr_and_port = N::AddressAndPort::new_from_address_and_port(
                        forward_to_addr,
                        forward_to.swim_port as u16,
                    );
                    trace!(
                        "Forwarding Ack from {}@{} to {}@{}",
                        msg.from.id,
                        addr,
                        forward_to.id,
                        forward_to.address,
                    );
                    (forward_to_addr_and_port, addr.get_address().to_string())
                };
                msg.from.address = from_addr;
                outbound::forward_ack(&self.server, &self.swim_sender, forward_to_addr, msg);
                return;
            }
        }
        let memberships = msg.membership.clone();
        let zones = msg.zones.clone();
        let from = msg.from.clone();
        match self.tx_outbound.send((addr, msg)) {
            Ok(()) => {
                for membership in memberships {
                    self.server
                        .insert_member_from_rumor(membership.member, membership.health);
                }
                self.server.insert_zones_from_rumors(zones);
            }
            Err(e) => panic!("Outbound thread has died - this shouldn't happen: #{:?}", e),
        }
        if results.send_ack {
            outbound::ack(&self.server, &self.swim_sender, &from, addr, None);
        }
    }

    /// Process ping messages.
    fn process_ping(&self, addr: N::AddressAndPort, mut msg: Ping) {
        let results = zones::handle_zone(
            &self.server,
            &msg.zones,
            SwimType::Ping,
            &msg.from,
            &msg.to,
            addr,
        );
        if results.bail_out {
            return;
        }
        for (msg, target) in results.msgs_and_targets_for_zone_change {
            outbound::zone_change(&self.server, &self.swim_sender, &target, msg);
        }
        trace_it!(SWIM: &self.server, TraceKind::RecvPing, &msg.from.id, addr, &msg);
        // TODO: do it after filling membership and zones
        outbound::ack(
            &self.server,
            &self.swim_sender,
            &msg.from,
            addr,
            msg.forward_to,
        );
        // Populate the member for this sender with its remote address
        msg.from.address = addr.get_address().to_string();
        trace!("Ping from {}@{}", msg.from.id, addr);
        if !results.sender_has_nil_zone {
            if msg.from.departed {
                self.server.insert_member(msg.from, Health::Departed);
            } else {
                self.server.insert_member(msg.from, Health::Alive);
            }
            for membership in msg.membership {
                self.server
                    .insert_member_from_rumor(membership.member, membership.health);
            }
            self.server.insert_zones_from_rumors(msg.zones.clone());
        }
    }

    fn process_zone_change(&self, addr: N::AddressAndPort, msg: ZoneChange) {
        trace_it!(SWIM: &self.server,
                  TraceKind::RecvZoneChange,
                  &msg.from.id,
                  addr,
                  &msg);
        trace!("Zone change from {}@{}", msg.from.id, addr);

        let mut dbg_data = ZoneChangeDbgData::default();
        let from = msg.from.clone();
        let results_msg_or_nothing = self.process_zone_change_internal(msg, &mut dbg_data);

        match results_msg_or_nothing {
            ZoneChangeResultsMsgOrNothing::Nothing => (),
            ZoneChangeResultsMsgOrNothing::Msg((zone_change, target)) => {
                outbound::zone_change(&self.server, &self.swim_sender, &target, zone_change);
            }
            ZoneChangeResultsMsgOrNothing::Results(mut results) => {
                let zone_changed = results.successor_for_maintained_zone.is_some()
                    || !results.predecessors_to_add_to_maintained_zone.is_empty();
                let mut maintained_zone = Zone::default();

                mem::swap(&mut maintained_zone, &mut results.original_maintained_zone);

                if let Some(successor_id) = results.successor_for_maintained_zone.take() {
                    maintained_zone.successor = Some(successor_id);
                }
                for predecessor_id in results.predecessors_to_add_to_maintained_zone {
                    maintained_zone.predecessors.push(predecessor_id);
                }
                if zone_changed {
                    maintained_zone.incarnation += 1;
                    self.server.insert_zone(maintained_zone.clone());
                }
                for zone in results.zones_to_insert.drain(..) {
                    self.server.insert_zone(zone);
                }
                if let Some(zone_id) = results.zone_id_for_our_member {
                    let our_member_clone = {
                        let mut our_member = self.server.write_member();
                        our_member.zone_id = zone_id;
                        our_member.incarnation += 1;

                        our_member.clone()
                    };

                    *self.server.write_zone_settled() = true;
                    self.server.insert_member(our_member_clone, Health::Alive);
                }

                if !results.aliases_to_inform.is_empty() {
                    let mut zone_ids_and_maintainer_ids = {
                        let zone_list = self.server.read_zone_list();

                        results
                            .aliases_to_inform
                            .iter()
                            .filter_map(|zone_id| {
                                zone_list
                                    .zones
                                    .get(&zone_id)
                                    .map(|zone| (zone_id, zone.maintainer_id.clone()))
                            })
                            .collect::<Vec<_>>()
                    };

                    let mut msgs_and_targets = Vec::new();

                    {
                        let mut msgs_and_targets = &mut msgs_and_targets;
                        let mut zone_ids_and_maintainer_ids = &mut zone_ids_and_maintainer_ids;

                        self.server.member_list.with_member_list(|members_map| {
                            for (zone_id, maintainer_id) in zone_ids_and_maintainer_ids.drain(..) {
                                if let Some(maintainer) = members_map.get(&maintainer_id) {
                                    let zone_change = ZoneChange {
                                        membership: Vec::new(),
                                        zones: Vec::new(),
                                        // TODO(krnowak): Ew.
                                        from: Member::default(),
                                        zone_id: *zone_id,
                                        new_aliases: vec![maintained_zone.clone()],
                                    };

                                    msgs_and_targets.push((zone_change, maintainer.clone()));
                                }
                            }
                        });
                    }

                    for (msg, target) in msgs_and_targets {
                        outbound::zone_change(&self.server, &self.swim_sender, &target, msg);
                    }
                }
            }
        }
        outbound::ack(&self.server, &self.swim_sender, &from, addr, None);
        debug!(
            "===========ZONE CHANGE=========\n\
             dbg:\n\
             \n\
             {:#?}\n\
             \n\
             ===============================",
            dbg_data,
        );
    }

    fn process_zone_change_internal(
        &self,
        zone_change: ZoneChange,
        dbg_data: &mut ZoneChangeDbgData,
    ) -> ZoneChangeResultsMsgOrNothing {
        enum MaintainershipStatus {
            ImTheMaintainerOf(Zone),
            MaintainerIs(String),
        }

        let maintainership_status = {
            let zone_list = self.server.read_zone_list();
            let maybe_maintained_zone = zone_list.zones.get(&zone_change.zone_id);

            dbg_data.zone_found = maybe_maintained_zone.is_some();

            if let Some(maintained_zone) = maybe_maintained_zone {
                let im_a_maintainer = maintained_zone.maintainer_id == self.server.member_id();

                dbg_data.is_a_maintainer = Some(im_a_maintainer);

                if im_a_maintainer {
                    MaintainershipStatus::ImTheMaintainerOf(maintained_zone.clone())
                } else {
                    MaintainershipStatus::MaintainerIs(maintained_zone.maintainer_id.clone())
                }
            } else {
                return ZoneChangeResultsMsgOrNothing::Nothing;
            }
        };
        let maintained_zone_clone = {
            match maintainership_status {
                MaintainershipStatus::ImTheMaintainerOf(zone) => zone,
                MaintainershipStatus::MaintainerIs(id) => {
                    let mut maybe_maintainer_clone = None;

                    self.server
                        .member_list
                        .with_member(&id, |maybe_maintainer| {
                            maybe_maintainer_clone = maybe_maintainer.cloned()
                        });

                    dbg_data.real_maintainer_found = Some(maybe_maintainer_clone.is_some());

                    if let Some(maintainer_clone) = maybe_maintainer_clone {
                        let addr: N::AddressAndPort = maintainer_clone.swim_socket_address();

                        dbg_data.forwarded_to =
                            Some((maintainer_clone.id.to_string(), addr.to_string()));

                        return ZoneChangeResultsMsgOrNothing::Msg((zone_change, maintainer_clone));
                    }

                    return ZoneChangeResultsMsgOrNothing::Nothing;
                }
            }
        };

        let maybe_successor_clone = if let Some(id) = maintained_zone_clone.successor {
            self.server.read_zone_list().zones.get(&id).cloned()
        } else {
            None
        };
        let our_member_id = self.server.read_member().zone_id;

        ZoneChangeResultsMsgOrNothing::Results(zones::process_zone_change_internal_state(
            maintained_zone_clone,
            maybe_successor_clone,
            our_member_id,
            zone_change,
            dbg_data,
        ))
    }
}
