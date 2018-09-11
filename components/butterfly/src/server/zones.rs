// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

//! This module handles updates to zones and members.
//!
//! Used from the inbound thread.

use std::cmp::Ordering as CmpOrdering;
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::mem;

use member::{Health, Member};
use message::BfUuid;
use network::{
    Address, AddressAndPort, AddressAndPortForNetwork, AddressForNetwork, MyFromStr, Network,
};
use server::Server;
use swim::{SwimType, ZoneChange};
use zone::{Zone, ZoneAddress};

#[derive(Debug)]
pub struct HandleZoneResults {
    pub bail_out: bool,
    pub sender_has_nil_zone: bool,
    pub send_ack: bool,
    pub msgs_and_targets_for_zone_change: Vec<(ZoneChange, Member)>,
}

#[derive(Debug, Default)]
pub struct ZoneChangeDbgData {
    pub zone_found: bool,
    pub is_a_maintainer: Option<bool>,
    pub real_maintainer_found: Option<bool>,
    pub borked_successor_state: Option<bool>,
    pub available_aliases: Option<Vec<String>>,
    pub our_old_successor: Option<String>,
    pub our_new_successor: Option<String>,
    pub our_old_member_zone_id: Option<String>,
    pub our_new_member_zone_id: Option<String>,
    pub added_predecessors: Option<Vec<String>>,
    pub sent_zone_change_with_alias_to: Option<Vec<(String, String)>>,
    pub forwarded_to: Option<(String, String)>,
}

#[derive(Clone, Debug, Default)]
pub struct ZoneChangeResults {
    pub original_maintained_zone: Zone,
    pub successor_for_maintained_zone: Option<BfUuid>,
    pub predecessors_to_add_to_maintained_zone: HashSet<BfUuid>,
    pub zones_to_insert: Vec<Zone>,
    pub zone_id_for_our_member: Option<BfUuid>,
    pub aliases_to_inform: HashSet<BfUuid>,
}

#[derive(Debug)]
pub enum ZoneChangeResultsMsgOrNothing {
    Nothing,
    Msg((ZoneChange, Member)),
    Results(ZoneChangeResults),
}

#[derive(Debug, Default)]
struct HandleZoneDbgData {
    pub to_address: String,
    pub to_port: u16,
    pub host_address: String,
    pub host_port: u16,
    pub from_zone_id: String,
    pub from_address: String,
    pub from_port: u16,
    pub real_from_address: String,
    pub real_from_port: u16,
    pub scenario: String,
    pub was_settled: bool,
    pub our_old_zone_id: String,
    pub our_new_zone_id: String,
    pub sender_zone_warning: Option<String>,
    pub handle_zone_results: HandleZoneInternalResults,
    pub sender_in_the_same_zone_as_us: bool,
    pub from_kind: AddressKind,
    pub to_kind: AddressKind,
    pub parse_failures: Vec<String>,
    pub zone_change_dbg_data: Option<ZoneChangeDbgData>,
    pub additional_address_update: Option<(ZoneAddress, ZoneAddress)>,
    pub additional_address_msgs: Vec<String>,
    pub msg_and_target: Option<(ZoneChange, Member)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum AddressKind {
    Real,
    Additional,
    Unknown,
}

impl Default for AddressKind {
    fn default() -> Self {
        AddressKind::Unknown
    }
}

#[derive(Debug)]
struct HandleZoneData<'a, N: Network> {
    pub zones: &'a [Zone],
    pub from_member: &'a Member,
    pub to_member: &'a Member,
    pub addr: AddressAndPortForNetwork<N>,
    pub swim_type: SwimType,
    pub from_address_kind: AddressKind,
    pub to_address_kind: AddressKind,
    pub sender_in_the_same_zone_as_us: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum ZoneRelative {
    Child,
    Parent,
}

#[derive(Clone, Debug, Default)]
pub struct MemberOrZoneChanges {
    pub new_maintained_zone: Option<Zone>,
    pub zone_id_for_our_member: Option<BfUuid>,
    pub additional_address_for_our_member: Option<(ZoneAddress, ZoneAddress)>,
    pub call_ack: bool,
    pub sender_has_nil_zone: bool,
    pub msg_and_target: Option<(ZoneChange, Member)>,
    pub sender_relative: Option<(BfUuid, ZoneRelative)>,
}

#[derive(Clone, Debug)]
enum HandleZoneInternalResults {
    Nothing,
    UnknownSenderAddress,
    SendAck,
    // naming is hard…
    Changes(MemberOrZoneChanges),
    ZoneProcessed(ZoneChangeResults),
}

impl Default for HandleZoneInternalResults {
    fn default() -> Self {
        HandleZoneInternalResults::Nothing
    }
}

pub fn process_zone_change_internal_state(
    mut maintained_zone_clone: Zone,
    mut maybe_successor_of_maintained_zone_clone: Option<Zone>,
    mut our_zone_id: BfUuid,
    mut zone_change: ZoneChange,
    _dbg_data: &mut ZoneChangeDbgData,
) -> ZoneChangeResults {
    let mut results = ZoneChangeResults::default();
    let maintained_zone_id = maintained_zone_clone.id;
    let mut aliases_to_maybe_inform = HashMap::new();

    //let mut dbg_available_aliases = Vec::new();
    //let mut dbg_added_predecessors = Vec::new();

    results.original_maintained_zone = maintained_zone_clone.clone();
    match (
        maintained_zone_clone.successor.is_some(),
        maybe_successor_of_maintained_zone_clone.is_some(),
    ) {
        (true, true) | (false, false) => (),
        (true, false) => {
            //dbg_data.borked_successor_state = Some(true);

            error!("passed maintained zone has a successor, but the successor was not passed");
            return results;
        }
        (false, true) => {
            //dbg_data.borked_successor_state = Some(true);

            error!("passed maintained zone has no successor, but some successor was passed");
            return results;
        }
    }

    //dbg_data.borked_successor_state = Some(false);
    //dbg_data.our_old_successor = Some(maintained_zone_clone.get_successor().to_string());
    //dbg_data.our_old_member_zone_id = Some(our_zone_id.to_string());

    results.zones_to_insert = zone_change.new_aliases.clone();
    for alias_zone in zone_change.new_aliases.drain(..) {
        //dbg_available_aliases.push(alias_zone.get_id().to_string());

        let alias_id = alias_zone.id;
        let mut possible_predecessor = None;

        match alias_id.cmp(&maintained_zone_id) {
            CmpOrdering::Less => {
                possible_predecessor = Some(alias_zone);
            }
            CmpOrdering::Equal => (),
            CmpOrdering::Greater => {
                if let Some(ref successor_id) = maintained_zone_clone.successor {
                    match alias_id.cmp(&successor_id) {
                        CmpOrdering::Less => {
                            possible_predecessor = Some(alias_zone);
                        }
                        CmpOrdering::Equal => (),
                        CmpOrdering::Greater => {
                            possible_predecessor = maybe_successor_of_maintained_zone_clone;
                            maybe_successor_of_maintained_zone_clone = Some(alias_zone);
                        }
                    }
                } else {
                    maybe_successor_of_maintained_zone_clone = Some(alias_zone);
                }
            }
        }

        if let Some(ref new_successor) = &maybe_successor_of_maintained_zone_clone {
            let has_new_successor = match maintained_zone_clone.successor {
                Some(ref successor_id) => *successor_id != new_successor.id,
                None => true,
            };
            if has_new_successor {
                maintained_zone_clone.successor = Some(new_successor.id);
                results.successor_for_maintained_zone = Some(new_successor.id);
                match aliases_to_maybe_inform.entry(alias_id) {
                    Entry::Occupied(_) => (),
                    Entry::Vacant(ve) => {
                        let abridged_successor = Zone {
                            id: alias_id,
                            incarnation: 0,
                            maintainer_id: String::new(),
                            parent_zone_id: None,
                            child_zone_ids: Vec::new(),
                            successor: Some(new_successor.id),
                            predecessors: new_successor.predecessors.clone(),
                        };

                        ve.insert(abridged_successor);
                    }
                }
            }

            let successor_id = new_successor.id;

            if our_zone_id < successor_id {
                results.zone_id_for_our_member = Some(successor_id);
                our_zone_id = successor_id;
            }
        }

        if let Some(predecessor) = possible_predecessor {
            let mut found = false;

            for zone_id in maintained_zone_clone.predecessors.iter() {
                if *zone_id == predecessor.id {
                    found = true;
                    break;
                }
            }

            if !found {
                //dbg_added_predecessors.push(predecessor.get_id().to_string());

                let predecessor_id = predecessor.id;

                results
                    .predecessors_to_add_to_maintained_zone
                    .insert(predecessor_id);
                match aliases_to_maybe_inform.entry(predecessor_id) {
                    Entry::Occupied(_) => (),
                    Entry::Vacant(ve) => {
                        ve.insert(predecessor);
                    }
                };
            }
        }
    }

    //dbg_data.our_new_successor = Some(maintained_zone_clone.get_successor().to_string());
    //dbg_data.our_new_member_zone_id = Some(our_zone_id.to_string());
    //dbg_data.available_aliases = Some(dbg_available_aliases);
    //dbg_data.added_predecessors = Some(dbg_added_predecessors);

    for (zone_id, zone) in aliases_to_maybe_inform {
        if let Some(successor_id) = zone.successor {
            if successor_id == maintained_zone_clone.id {
                continue;
            }
        }

        let mut found = false;

        for predecessor_id in zone.predecessors.iter() {
            if *predecessor_id == maintained_zone_clone.id {
                found = true;
                break;
            }
        }

        if found {
            continue;
        }

        results.aliases_to_inform.insert(zone_id);
    }

    results
}

pub fn handle_zone<N: Network>(
    server: &Server<N>,
    zones: &[Zone],
    swim_type: SwimType,
    from: &Member,
    to: &Member,
    addr: N::AddressAndPort,
) -> HandleZoneResults {
    let mut bail_out = false;
    let mut send_ack = false;
    let mut sender_has_nil_zone = false;
    let mut msgs_and_targets_for_zone_change = Vec::new();

    match handle_zone_for_recipient(server, zones, swim_type, from, to, addr) {
        HandleZoneInternalResults::Nothing => (),
        HandleZoneInternalResults::UnknownSenderAddress => {
            if swim_type == SwimType::Ping {
                warn!(
                    "Sender of the PING message does not know its address {}. \
                     This shouldn't happen - this means that the sender sent a PING message to us \
                     and we are not directly reachable",
                    addr,
                );
            } else {
                warn!(
                    "Sender of the ACK message does not know its address {}. \
                     This shouldn't happen - this means that we sent a PING message to a server \
                     that is not directly reachable from us and it wasn't ignored by the receiver \
                     of the message",
                    addr,
                );
            }
            bail_out = true;
        }
        HandleZoneInternalResults::SendAck => {
            send_ack = true;
        }
        HandleZoneInternalResults::Changes(changes) => {
            if changes.sender_has_nil_zone {
                sender_has_nil_zone = true;
            }
            send_ack = changes.call_ack;
            if let Some(zone) = changes.new_maintained_zone {
                let zone_id = zone.id;
                server.insert_zone(zone);
                let mut zone_list = server.write_zone_list();
                zone_list.maintained_zone_id = Some(zone_id);
            }
            if let Some(id) = changes.zone_id_for_our_member {
                let mut zone_list = server.write_zone_list();
                zone_list.our_zone_id = id;
            }
            if let Some((sender_id, relative)) = changes.sender_relative {
                // TODO: update our zone with parent/child stuff
                // need to take aliases into account!
                let zone_id = {
                    if let Some(id) = changes.zone_id_for_our_member {
                        id
                    } else {
                        server.read_member().zone_id
                    }
                };
                let zone_to_insert = {
                    let mut zone_to_insert = None;
                    let zone_list = server.read_zone_list();

                    if let Some(zone) = zone_list.zones.get(&zone_id) {
                        match relative {
                            ZoneRelative::Child => {
                                let mut found =
                                    zone.child_zone_ids.iter().any(|id| *id == sender_id);

                                if !found {
                                    for child_zone_id in zone.child_zone_ids.iter() {
                                        if let Some(child_zone) = zone_list.zones.get(child_zone_id)
                                        {
                                            if let Some(ref successor) = child_zone.successor {
                                                if *successor == sender_id {
                                                    found = true;
                                                    break;
                                                }
                                            }

                                            if child_zone
                                                .predecessors
                                                .iter()
                                                .any(|id| *id == sender_id)
                                            {
                                                found = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                                if !found {
                                    let mut zone_clone = zone.clone();

                                    zone_clone.child_zone_ids.push(sender_id);

                                    zone_to_insert = Some(zone_clone);
                                }
                            }
                            ZoneRelative::Parent => {
                                if zone.parent_zone_id.is_none() {
                                    let mut zone_clone = zone.clone();

                                    zone_clone.parent_zone_id = Some(sender_id);

                                    zone_to_insert = Some(zone_clone);
                                }
                            }
                        }
                    }

                    zone_to_insert
                };

                if let Some(zone) = zone_to_insert {
                    server.insert_zone(zone);
                }
            }
            let member_changed = changes.zone_id_for_our_member.is_some()
                || changes.additional_address_for_our_member.is_some();
            if member_changed {
                let our_member_clone = {
                    let mut our_member = server.write_member();

                    our_member.incarnation += 1;
                    if let Some(zone_id) = changes.zone_id_for_our_member {
                        our_member.zone_id = zone_id;
                    }
                    if let Some((old, new)) = changes.additional_address_for_our_member {
                        for zone_address in our_member.additional_addresses.iter_mut() {
                            if zone_address.address != old.address {
                                continue;
                            }
                            if zone_address.swim_port != old.swim_port {
                                continue;
                            }
                            if zone_address.zone_id != old.zone_id {
                                continue;
                            }
                            zone_address.address = new.address;
                            zone_address.zone_id = new.zone_id;
                            break;
                        }
                    }

                    our_member.clone()
                };
                *server.write_zone_settled() = true;
                server.insert_member(our_member_clone, Health::Alive);
            }
            if let Some(pair) = changes.msg_and_target {
                msgs_and_targets_for_zone_change.push(pair);
            }
        }
        HandleZoneInternalResults::ZoneProcessed(mut results) => {
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
            if let Some(zone_id) = results.zone_id_for_our_member {
                let mut zone_list = server.write_zone_list();
                zone_list.our_zone_id = zone_id;
            }
            if zone_changed {
                maintained_zone.incarnation += 1;
                server.insert_zone(maintained_zone.clone());
                send_ack = true;
            }
            for zone in results.zones_to_insert.drain(..) {
                server.insert_zone(zone);
            }
            if let Some(zone_id) = results.zone_id_for_our_member {
                let our_member_clone = {
                    let mut our_member = server.write_member();

                    our_member.zone_id = zone_id;
                    our_member.incarnation += 1;

                    our_member.clone()
                };
                *server.write_zone_settled() = true;
                server.insert_member(our_member_clone, Health::Alive);
                send_ack = true;
            }

            //let mut dbg_sent_zone_change_with_alias_to = Vec::new();

            if !results.aliases_to_inform.is_empty() {
                send_ack = true;
                let mut zone_ids_and_maintainer_ids = {
                    let zone_list = server.read_zone_list();

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

                    server.member_list.with_member_list(move |members_map| {
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

                msgs_and_targets_for_zone_change.extend(msgs_and_targets);
            }
            //dbg_data.sent_zone_change_with_alias_to = dbg_sent_zone_change_with_alias_to;
        }
    }

    HandleZoneResults {
        bail_out,
        sender_has_nil_zone,
        send_ack,
        msgs_and_targets_for_zone_change,
    }
}

fn handle_zone_for_recipient<N: Network>(
    server: &Server<N>,
    zones: &[Zone],
    swim_type: SwimType,
    from: &Member,
    to: &Member,
    mut addr: N::AddressAndPort,
) -> HandleZoneInternalResults {
    let mut dbg_data = HandleZoneDbgData::default();
    let mut from_address_kind = address_kind(addr.get_address(), from, &mut dbg_data);
    let mut to_address_kind = address_kind_from_str::<AddressForNetwork<N>>(
        &to.address,
        &server.read_member(),
        &mut dbg_data,
    );
    let to_clone: Option<Member>;
    let mut to_ref = to;

    dbg_data.from_kind = from_address_kind;
    dbg_data.to_kind = to_address_kind;
    // we are dealing with several addresses here:
    //
    // real from address - can be an address of a mapping on a NAT
    // or a local one
    //
    // member from - contains a real address and additional addresses
    //
    // member to - contains an address that can be either local or
    // a mapping on a NAT
    //
    // member us - contains a local address and additional addresses
    //
    // address kinds:
    // 1. real - an address is the same as member's local address
    // 2. additional - an address is the same as one of member's
    // additional addresses
    // 3. unknown - if none of the above applies
    //
    // scenarios:
    // 1. from real to real - message sent between two servers in
    // the same zone
    //
    // 2. from real to additional - message sent from parent zone
    // to child zone
    //
    // 3. from real to unknown - message likely sent from parent
    // zone to child zone for the first time
    //
    // 4. from additional to real - message sent from child zone
    // to parent zone
    //
    // 5. from additional to additional - probably message sent
    // from a zone to a sibling zone
    //
    // 6. from additional to unknown - probably message sent from
    // a zone to a sibling zone for the first time
    //
    // 7. from unknown to real - probably message sent from child
    // zone to parent zone, but the sender either does not know
    // that it can be reached with the address the message came
    // from or it knows it can be reached, but does not know the
    // exact address (only ports). This likely should not happen -
    // if the server in child zone is not exposed in the parent
    // zone, the message should be routed through the gateway
    //
    // 8. from unknown to additional - probably message sent from
    // zone to a sibling zone, but the sender either does not know
    // that it can be reached with the address the message came
    // from. This likely should not happen - if the server in
    // child zone is not exposed in the parent zone, the message
    // should be routed through the gateway
    //
    // 9. from unknown to unknown - probably a message sent from
    // zone to a sibling zone for the first time, but the sender
    // either does not know that it can be reached with the
    // address the message came from. This likely should not
    // happen - if the server in child zone is not exposed in the
    // parent zone, the message should be routed through the
    // gateway
    let sender_in_the_same_zone_as_us;
    let mut maybe_result = None;
    debug!(
        "address kinds before fix: from {:?}, to {:?}",
        from_address_kind, to_address_kind
    );
    debug!(
        "before fix, addr: {}, from: {:#?}, to: {:#?}",
        addr, from, to_ref
    );
    match (from_address_kind, to_address_kind) {
        (AddressKind::Real, AddressKind::Real) => {
            sender_in_the_same_zone_as_us = true;
        }
        (AddressKind::Real, AddressKind::Additional) => {
            sender_in_the_same_zone_as_us = false;
        }
        (AddressKind::Additional, AddressKind::Real) => {
            sender_in_the_same_zone_as_us = false;
        }
        (AddressKind::Additional, AddressKind::Additional) => {
            sender_in_the_same_zone_as_us = false;
        }
        (AddressKind::Additional, AddressKind::Unknown) => {
            // hack for kubernetes: the host which hosts the k8s
            // cluster will likely talk to services inside the
            // cluster using a different network interface than
            // the one assigned to the IP address we detected
            sender_in_the_same_zone_as_us = false;
            let mut tc = to.clone();
            tc.address = server.host_address.to_string();
            to_clone = Some(tc);
            to_ref = to_clone.as_ref().unwrap();
            to_address_kind = AddressKind::Real;
        }
        (AddressKind::Unknown, AddressKind::Additional) => {
            sender_in_the_same_zone_as_us = false;
            // hack for kubernetes: kubernetes does the source NAT
            // on packets coming from outside the cluster to
            // inside by default.
            match AddressForNetwork::<N>::create_from_str(&from.address) {
                Ok(a) => {
                    addr = N::AddressAndPort::new_from_address_and_port(a, from.swim_port);
                    from_address_kind = AddressKind::Real;
                }
                Err(e) => {
                    error!("Could not parse from address {}: {}", from.address, e);
                    maybe_result = Some(HandleZoneInternalResults::UnknownSenderAddress);
                }
            }
        }
        (AddressKind::Unknown, AddressKind::Real) => {
            sender_in_the_same_zone_as_us = false;
            // hack for kubernetes: minikube has some weird
            // networking stuff, I send ping to one IP and get an
            // ack from different one…
            match AddressForNetwork::<N>::create_from_str(&from.address) {
                Ok(a) => {
                    addr = N::AddressAndPort::new_from_address_and_port(a, from.swim_port);
                    from_address_kind = AddressKind::Additional;
                }
                Err(e) => {
                    error!("Could not parse from address {}: {}", from.address, e);
                    maybe_result = Some(HandleZoneInternalResults::UnknownSenderAddress);
                }
            }
        }
        (_, _) => {
            sender_in_the_same_zone_as_us = false;
        }
    };
    debug!(
        "address kinds after fix: from {:?}, to {:?}",
        from_address_kind, to_address_kind
    );
    debug!(
        "after fix, addr: {}, from: {:#?}, to: {:#?}",
        addr, from, to_ref
    );

    dbg_data.to_address = to_ref.address.to_string();
    dbg_data.to_port = to_ref.swim_port;
    dbg_data.host_address = server.host_address.to_string();
    dbg_data.host_port = server.swim_port();
    dbg_data.sender_in_the_same_zone_as_us = sender_in_the_same_zone_as_us;
    dbg_data.from_address = from.address.to_string();
    dbg_data.from_port = from.swim_port;
    dbg_data.real_from_address = addr.get_address().to_string();
    dbg_data.real_from_port = addr.get_port();

    let handle_zone_results = if let Some(result) = maybe_result {
        result
    } else {
        let handle_zone_data = HandleZoneData {
            zones: zones,
            from_member: from,
            to_member: to_ref,
            addr: addr,
            swim_type: swim_type,
            from_address_kind: from_address_kind,
            to_address_kind: to_address_kind,
            sender_in_the_same_zone_as_us: sender_in_the_same_zone_as_us,
        };
        handle_zone_internal(server, handle_zone_data, &mut dbg_data)
    };
    dbg_data.handle_zone_results = handle_zone_results.clone();
    debug!(
        "=========={:?}==========\n\
         dbg:\n\
         \n\
         {:#?}\n\
         \n\
         member us: {:#?}\n\
         member from: {:#?}\n\
         \n\
         =====================",
        swim_type,
        dbg_data,
        server.read_member(),
        from,
    );
    handle_zone_results
}

fn handle_zone_internal<N: Network>(
    server: &Server<N>,
    hz_data: HandleZoneData<N>,
    dbg_data: &mut HandleZoneDbgData,
) -> HandleZoneInternalResults {
    // scenarios:
    // - 0 sender has nil zone id
    //   - 0a. i'm not settled
    //     - 0aa. sender in the same private network as me
    //       - generate my own zone
    //     - 0ab. sender in a different private network than me
    //       - generate my own zone
    //       - store the recipient address if not stored (ports
    //         should already be available)
    //   - 0b. i'm settled
    //     - 0ba. sender in the same private network as me
    //       - do nothing
    //     - 0bb. sender in a different private network than me
    //       - store the recipient address if not stored (ports
    //         should already be available)
    // - 1 sender has non-nil zone id
    //   - 1a. i'm not settled
    //     - 1aa. sender in the same private network as me
    //       - assume sender's zone
    //     - 1ab. sender in a different private network than me
    //       - generate my own zone
    //       - add sender id as a child/parent of my zone
    //       - store the recipient address if not stored (ports
    //         should already be available)
    //       - store sender zone id? what did i mean by that?
    //   - 1b. i'm settled
    //     - 1ba. sender in the same private network as me
    //       - 1ba<. senders zone id is less than mine
    //         - if message was ack then send another ack back to
    //           enlighten the sender about newer and better zone
    //       - 1ba=. senders zone id is equal to mine
    //         - do nothing
    //       - 1ba>. senders zone id is greater than mine
    //         - use process_zone_change_internal_state
    //     - 1bb. sender in a different private network than me
    //       - add sender id as a child/parent of my zone
    //       - store the recipient address if not stored (ports
    //         should already be available)
    //       - store sender zone id? what did i mean by that?
    //
    // actions:
    // - settle zone
    // - generate my own zone
    //   - new zone id for our member
    //   - new maintained zone
    //   - send an ack
    // - add sender id as a child/parent of my zone
    //   - if from/to is real/additional then sender is a parent
    //   - if from/to is additional/real then sender is a child
    // - store the additional address if not stored (ports should
    //   already be available)
    //   - if this is an ack and to zone id is nil and from is additional
    //     - send ack
    //   - this should always be an update of an existing address
    //     entry, never an addition
    //   - scenarios:
    //     - 0. nil sender zone id
    //       - search for fitting port number with no address and no zone
    //       - if there is only one then update the address, zone is still nil
    //       - otherwise ignore it
    //     - 1. non nil sender zone id
    //       - search for fitting port number with a specific zone
    //         - if found and address is the same, nothing to add
    //         - if found and address is different, continue with the other approach
    //         - if not found, continue with the other approach
    //       - search for fitting port number with a specific address
    //         - if found and zone id is the same, nothing to add (should be caught earlier, though)
    //         - if found and zone id is nil, update the zone id
    //         - if found and zone id is a child of sender - update the zone
    //         - if found and zone id is a parent of sender - no
    //           clue, do nothing, add another entry as a copy of
    //           this one? or rather warn?
    //         - if found and zone id is something else - ignore? should not happen?
    // - assume sender's zone (means that we were not settled yet)
    //   - new id for our member
    // - store sender zone id? what did i mean by that?
    // - if message was ack then send another ack back to
    //   enlighten the sender about newer and better zone
    // - use process_zone_change_internal_state
    let maybe_not_nil_sender_zone = {
        if let Some(zone) = hz_data
            .zones
            .iter()
            .find(|z| z.id == hz_data.from_member.zone_id)
            .cloned()
        {
            if zone.id.is_nil() {
                dbg_data.sender_zone_warning =
                    Some("Got a zone with a nil UUID, ignoring it".to_string());
                warn!("Got a zone with a nil UUID, ignoring it");
                None
            } else {
                Some(zone)
            }
        } else {
            let id = hz_data.from_member.zone_id;

            if !id.is_nil() {
                dbg_data.sender_zone_warning = Some(format!("Got no zone info for {}", id));
                warn!("Got no zone info for {}", id,);
            }
            None
        }
    };
    let zone_settled = *(server.read_zone_settled());
    let same_private_network = hz_data.sender_in_the_same_zone_as_us;
    let our_member_clone = server.read_member().clone();
    let (
        maybe_maintained_zone_clone,
        maybe_successor_of_maintained_zone_clone,
        maybe_our_zone_clone,
    ) = {
        let zone_list = server.read_zone_list();
        let maybe_our_zone_clone = zone_list.zones.get(&our_member_clone.zone_id).cloned();
        let zone_pair = if let &Some(ref maintained_zone_id) = &zone_list.maintained_zone_id {
            if let Some(maintained_zone) = zone_list.zones.get(maintained_zone_id) {
                if let Some(ref maintained_zone_successor) = maintained_zone.successor {
                    if let Some(successor) = zone_list.zones.get(maintained_zone_successor) {
                        (Some(maintained_zone.clone()), Some(successor.clone()))
                    } else {
                        warn!(
                            "Maintained zone {} has successor {}, \
                             but we don't have it in our zone list",
                            maintained_zone_id, maintained_zone_successor,
                        );
                        (None, None)
                    }
                } else {
                    (Some(maintained_zone.clone()), None)
                }
            } else {
                warn!(
                    "Maintained zone ID is {}, but we don't have it in our zone list",
                    maintained_zone_id
                );
                (None, None)
            }
        } else {
            (None, None)
        };

        (zone_pair.0, zone_pair.1, maybe_our_zone_clone)
    };
    let maybe_our_zone_maintainer_clone = if let Some(ref our_zone_clone) = maybe_our_zone_clone {
        let mut maybe_member = None;

        server
            .member_list
            .with_member(&our_zone_clone.maintainer_id, |maybe_maintainer| {
                maybe_member = maybe_maintainer.cloned();
            });

        maybe_member
    } else {
        None
    };

    dbg_data.was_settled = zone_settled;
    dbg_data.our_old_zone_id = our_member_clone.zone_id.to_string();

    let results = match (
        maybe_not_nil_sender_zone,
        zone_settled,
        same_private_network,
    ) {
        // 0aa.
        (None, false, true) => {
            dbg_data.scenario = "0aa".to_string();

            let mut changes = MemberOrZoneChanges::default();

            changes.sender_has_nil_zone = true;
            generate_my_own_zone(&mut changes, our_member_clone.id.clone(), dbg_data);

            HandleZoneInternalResults::Changes(changes)
        }
        // 0ab.
        (None, false, false) => {
            dbg_data.scenario = "0ab".to_string();

            let mut changes = MemberOrZoneChanges::default();

            changes.sender_has_nil_zone = true;
            generate_my_own_zone(&mut changes, our_member_clone.id.clone(), dbg_data);
            store_recipient_address_nil_sender_zone(
                &mut changes,
                hz_data.from_address_kind,
                hz_data.to_address_kind,
                &our_member_clone,
                &hz_data.to_member,
                dbg_data,
            );

            HandleZoneInternalResults::Changes(changes)
        }
        // 0ba.
        (None, true, true) => {
            dbg_data.scenario = "0ba".to_string();

            let mut changes = MemberOrZoneChanges::default();

            changes.sender_has_nil_zone = true;
            changes.call_ack = true;

            HandleZoneInternalResults::Changes(changes)
        }
        // 0bb.
        (None, true, false) => {
            dbg_data.scenario = "0bb".to_string();

            let mut changes = MemberOrZoneChanges::default();

            changes.sender_has_nil_zone = true;
            store_recipient_address_nil_sender_zone(
                &mut changes,
                hz_data.from_address_kind,
                hz_data.to_address_kind,
                &our_member_clone,
                &hz_data.to_member,
                dbg_data,
            );

            HandleZoneInternalResults::Changes(changes)
        }
        // 1aa.
        (Some(sender_zone), false, true) => {
            dbg_data.scenario = "1aa".to_string();

            let mut changes = MemberOrZoneChanges::default();

            assume_senders_zone(&mut changes, sender_zone.id, dbg_data);

            HandleZoneInternalResults::Changes(changes)
        }
        // 1ab.
        (Some(sender_zone), false, false) => {
            dbg_data.scenario = "1ab".to_string();

            let mut changes = MemberOrZoneChanges::default();

            generate_my_own_zone(&mut changes, our_member_clone.id.clone(), dbg_data);
            add_sender_zone_id_as_relative(
                &mut changes,
                hz_data.from_address_kind,
                hz_data.to_address_kind,
                sender_zone.id,
                dbg_data,
            );
            store_recipient_address_valid_sender_zone(
                &mut changes,
                hz_data.from_address_kind,
                hz_data.to_address_kind,
                &our_member_clone,
                &hz_data.to_member,
                &sender_zone,
                dbg_data,
            );

            HandleZoneInternalResults::Changes(changes)
        }
        // 1ba.
        (Some(sender_zone), true, true) => {
            dbg_data.scenario = "1ba".to_string();

            process_zone(
                our_member_clone,
                maybe_maintained_zone_clone,
                maybe_successor_of_maintained_zone_clone,
                maybe_our_zone_clone,
                maybe_our_zone_maintainer_clone,
                sender_zone,
                dbg_data,
            )
        }
        // 1bb.
        (Some(sender_zone), true, false) => {
            dbg_data.scenario = "1bb".to_string();

            let mut changes = MemberOrZoneChanges::default();

            add_sender_zone_id_as_relative(
                &mut changes,
                hz_data.from_address_kind,
                hz_data.to_address_kind,
                sender_zone.id,
                dbg_data,
            );
            store_recipient_address_valid_sender_zone(
                &mut changes,
                hz_data.from_address_kind,
                hz_data.to_address_kind,
                &our_member_clone,
                &hz_data.to_member,
                &sender_zone,
                dbg_data,
            );

            HandleZoneInternalResults::Changes(changes)
        }
    };

    results
}

fn address_kind<A: Address>(
    addr: A,
    member: &Member,
    dbg_data: &mut HandleZoneDbgData,
) -> AddressKind {
    let member_real_address = match A::create_from_str(&member.address) {
        Ok(member_addr) => member_addr,
        Err(e) => {
            let msg = format!(
                "Error parsing member {:?} address {}: {}",
                member, member.address, e
            );
            error!("{}", msg);
            dbg_data.parse_failures.push(msg);
            return AddressKind::Unknown;
        }
    };

    if member_real_address == addr {
        return AddressKind::Real;
    }

    for zone_address in member.additional_addresses.iter() {
        if let Some(ref zone_addr_str) = zone_address.address {
            let member_additional_address = match A::create_from_str(zone_addr_str) {
                Ok(zone_addr) => zone_addr,
                Err(e) => {
                    let msg = format!(
                        "Error parsing member {:?} additional address {}: {}",
                        member, zone_addr_str, e
                    );
                    error!("{}", msg);
                    dbg_data.parse_failures.push(msg);
                    continue;
                }
            };

            if member_additional_address == addr {
                return AddressKind::Additional;
            }
        }
    }

    AddressKind::Unknown
}

fn address_kind_from_str<A: Address>(
    addr: &str,
    member: &Member,
    dbg_data: &mut HandleZoneDbgData,
) -> AddressKind {
    let real_address = match A::create_from_str(addr) {
        Ok(addr) => addr,
        Err(e) => {
            error!("Error parsing address {}: {}", addr, e);
            return AddressKind::Unknown;
        }
    };

    address_kind(real_address, member, dbg_data)
}

fn generate_my_own_zone(
    changes: &mut MemberOrZoneChanges,
    maintainer_id: String,
    dbg_data: &mut HandleZoneDbgData,
) {
    let new_zone_id = BfUuid::generate();

    changes.new_maintained_zone = Some(Zone::new(new_zone_id, maintainer_id));
    changes.zone_id_for_our_member = Some(new_zone_id);
    changes.call_ack = true;

    dbg_data.our_new_zone_id = new_zone_id.to_string();
}

fn store_recipient_address_nil_sender_zone(
    changes: &mut MemberOrZoneChanges,
    from_address_kind: AddressKind,
    to_address_kind: AddressKind,
    our_member: &Member,
    to_member: &Member,
    dbg_data: &mut HandleZoneDbgData,
) {
    if from_address_kind == AddressKind::Additional && to_member.zone_id.is_nil() {
        changes.call_ack = true;
        dbg_data
            .additional_address_msgs
            .push("will send an ack".to_string());
    }
    dbg_data
        .additional_address_msgs
        .push(format!("got message on {:?} address", to_address_kind));
    if to_address_kind != AddressKind::Real {
        for zone_address in our_member.additional_addresses.iter() {
            if zone_address.swim_port != to_member.swim_port {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has swim port different than {}, skipping",
                    zone_address, to_member.swim_port
                ));
                continue;
            }

            let zone_address_id = zone_address.zone_id;

            if !zone_address_id.is_nil() {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has non-nil zone id, skipping",
                    zone_address
                ));
                continue;
            }
            if zone_address.address.is_some() {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} already has an address, skipping",
                    zone_address
                ));
                continue;
            }

            let mut new_zone_address = zone_address.clone();

            new_zone_address.address = Some(to_member.address.clone());
            changes.additional_address_for_our_member =
                Some((zone_address.clone(), new_zone_address));

            dbg_data.additional_address_update = changes.additional_address_for_our_member.clone();

            break;
        }
    }
}

fn assume_senders_zone(
    changes: &mut MemberOrZoneChanges,
    sender_zone_id: BfUuid,
    dbg_data: &mut HandleZoneDbgData,
) {
    changes.zone_id_for_our_member = Some(sender_zone_id);
    changes.call_ack = true;

    dbg_data.our_new_zone_id = sender_zone_id.to_string();
}

fn add_sender_zone_id_as_relative(
    changes: &mut MemberOrZoneChanges,
    from_address_kind: AddressKind,
    to_address_kind: AddressKind,
    sender_zone_id: BfUuid,
    _dbg_data: &mut HandleZoneDbgData,
) {
    match (from_address_kind, to_address_kind) {
        (AddressKind::Additional, AddressKind::Real) => {
            changes.sender_relative = Some((sender_zone_id, ZoneRelative::Child));
        }
        (AddressKind::Real, AddressKind::Additional) => {
            changes.sender_relative = Some((sender_zone_id, ZoneRelative::Parent));
        }
        (AddressKind::Real, AddressKind::Real) => {
            unreachable!(
                "sender was detected as being from different private network, \
                 but we got two real addresses"
            );
        }
        (AddressKind::Additional, AddressKind::Additional) => {
            unimplemented!("TODO when we implement sibling zones")
        }
        (_, _) => warn!(
            "unhandled relationship case, from {:?} to {:?}",
            from_address_kind, to_address_kind,
        ),
    }
}

// store the recipient address if not stored (ports
// should already be available)
//
// - 1. non nil sender zone id
//   - search for a zone address instance with a
//     variant-fitting zone
//     - variant-fitting zone means a variant of a
//       sender zone (successor/predecessor/itself)
//     - found and both address and port are the
//       same
//       - zone in the instance is the same as sender
//         zone or sender zone successor
//         - nothing to do
//       - zone in the instance is the same as one of
//         the sender zone's predecessors
//         - update the zone to sender's successor or
//           to sender zone itself
//     - not found
//       - continue with the other approach
//   - search for a zone address instance with a
//     relation-fitting zone
//     - relation-fitting zone means a relative of a
//       sender zone (child/parent/itself)
//     - found and both address and port are the
//       same
//       - zone in the instance is the same as sender
//         zone or parent of the sender zone
//         - do nothing (not sure about doing nothing
//           for the parent case)
//       - zone in the instance is the same as one of
//         the children of the sender zone
//         - update the zone in some way?
//     - not found
//       - continue with the other approach
//   - search for a zone address instance with a nil zone
//     - found and both address and port are the
//       same
//       - update the zone to sender zone itself
//     - not found
//       - continue with the other approach
//   - search for a zone address instance with a nil
//     zone and an unset address
//     - found and ports are the same
//       - update the zone to sender zone itself
//       - update the address
//     - not found
//       - warn
fn store_recipient_address_valid_sender_zone(
    changes: &mut MemberOrZoneChanges,
    from_address_kind: AddressKind,
    to_address_kind: AddressKind,
    our_member: &Member,
    to_member: &Member,
    sender_zone: &Zone,
    dbg_data: &mut HandleZoneDbgData,
) {
    if from_address_kind == AddressKind::Additional && to_member.zone_id.is_nil() {
        changes.call_ack = true;
        dbg_data
            .additional_address_msgs
            .push("will send an ack".to_string());
    }
    // this is to ignore messages that arrived to our
    // real address, not the additional one
    let mut done = to_address_kind == AddressKind::Real;
    dbg_data
        .additional_address_msgs
        .push(format!("got message on {:?} address", to_address_kind));

    if !done {
        dbg_data
            .additional_address_msgs
            .push("going with the variant-fitting scenario".to_string());
        for zone_address in our_member.additional_addresses.iter() {
            if let Some(ref address_str) = zone_address.address {
                if *address_str != to_member.address {
                    dbg_data.additional_address_msgs.push(format!(
                        "zone address {:#?} has different address than {}, skipping",
                        zone_address, to_member.address
                    ));
                    continue;
                }
            } else {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has no address, skipping",
                    zone_address
                ));
                continue;
            }

            if zone_address.swim_port != to_member.swim_port {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has different swim port than {}, skipping",
                    zone_address, to_member.swim_port
                ));
                continue;
            }

            let zone_address_id = zone_address.zone_id;

            if zone_address_id == sender_zone.id {
                done = true;
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has the same zone id as sender, done",
                    zone_address
                ));
                break;
            }

            if let Some(sender_successor_id) = sender_zone.successor {
                if sender_successor_id == zone_address_id {
                    dbg_data.additional_address_msgs.push(format!(
                        "zone address {:#?} has the same zone id as sender's successor, done",
                        zone_address
                    ));
                    done = true;
                    break;
                }
            }

            let mut maybe_new_zone_id = None;

            for predecessor_id in sender_zone.predecessors.iter() {
                if *predecessor_id == zone_address_id {
                    dbg_data.additional_address_msgs.push(format!(
                        "zone address {:#?} has the same zone id as sender's predecessor, done",
                        zone_address
                    ));
                    if let Some(sender_successor_id) = sender_zone.successor {
                        maybe_new_zone_id = Some(sender_successor_id);
                    } else {
                        maybe_new_zone_id = Some(sender_zone.id);
                    }
                }
            }
            done = match maybe_new_zone_id {
                Some(zone_id) => {
                    let mut new_zone_address = zone_address.clone();

                    new_zone_address.zone_id = zone_id;
                    new_zone_address.address = Some(to_member.address.clone());
                    changes.additional_address_for_our_member =
                        Some((zone_address.clone(), new_zone_address));

                    dbg_data.additional_address_update =
                        changes.additional_address_for_our_member.clone();

                    true
                }
                None => {
                    dbg_data.additional_address_msgs.push(format!(
                        "zone address {:#?} does not match the sender, skipping",
                        zone_address
                    ));
                    false
                }
            };
            if done {
                break;
            }
        }
    }
    if !done {
        dbg_data
            .additional_address_msgs
            .push("going with the relative-fitting scenario".to_string());
        // TODO: handle parent/child relationships
        // following the steps written above
        dbg_data
            .additional_address_msgs
            .push("haha not really, not implemented".to_string());
    }
    if !done {
        dbg_data
            .additional_address_msgs
            .push("going with the nil-zoned scenario".to_string());
        for zone_address in our_member.additional_addresses.iter() {
            if let Some(ref address_str) = zone_address.address {
                if *address_str != to_member.address {
                    dbg_data.additional_address_msgs.push(format!(
                        "zone address {:#?} has different address than {}, skipping",
                        zone_address, to_member.address
                    ));
                    continue;
                }
            } else {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has no address, skipping",
                    zone_address
                ));
                continue;
            }

            if zone_address.swim_port != to_member.swim_port {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has different swim port than {}, skipping",
                    zone_address, to_member.swim_port
                ));
                continue;
            }

            let zone_address_id = zone_address.zone_id;

            if !zone_address_id.is_nil() {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has non-nil zone, skipping",
                    zone_address
                ));
                continue;
            }

            let mut new_zone_address = zone_address.clone();

            new_zone_address.zone_id = sender_zone.id;
            changes.additional_address_for_our_member =
                Some((zone_address.clone(), new_zone_address));

            dbg_data.additional_address_update = changes.additional_address_for_our_member.clone();

            done = true;
            break;
        }
    }
    if !done {
        dbg_data
            .additional_address_msgs
            .push("going with the nil-zoned, address-guessing scenario".to_string());
        for zone_address in our_member.additional_addresses.iter() {
            if zone_address.address.is_some() {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} already has an address, skipping",
                    zone_address
                ));
                continue;
            }

            if zone_address.swim_port != to_member.swim_port {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has different swim port than {}, skipping",
                    zone_address, to_member.swim_port
                ));
                continue;
            }

            let zone_address_id = zone_address.zone_id;

            if !zone_address_id.is_nil() {
                dbg_data.additional_address_msgs.push(format!(
                    "zone address {:#?} has non-nil zone, skipping",
                    zone_address
                ));
                continue;
            }

            let mut new_zone_address = zone_address.clone();

            new_zone_address.zone_id = sender_zone.id;
            new_zone_address.address = Some(to_member.address.clone());
            changes.additional_address_for_our_member =
                Some((zone_address.clone(), new_zone_address));

            dbg_data.additional_address_update = changes.additional_address_for_our_member.clone();

            done = true;
            break;
        }
    }
    if !done {
        dbg_data
            .additional_address_msgs
            .push("unhandled zone address…".to_string());
        warn!("Arf")
    }
}

// - 0. we maintain a zone
//   - use process_zone_change_internal_state
// - 1. we do not maintain a zone
//   - 1a. sender's zone id is less than ours
//     - send ack back if this message was ack
//   - 1b. sender's zone id is equal to ours
//     - do nothing
//   - 1c. sender's zone id is greater than ours
//     - update our member's zone id
//     - send zone change to the maintainer of the old
//       zone id if the old zone has no info about the
//       new successor
fn process_zone(
    our_member: Member,
    maybe_maintained_zone: Option<Zone>,
    maybe_successor_of_maintained_zone: Option<Zone>,
    maybe_our_zone: Option<Zone>,
    maybe_our_zone_maintainer: Option<Member>,
    sender_zone: Zone,
    dbg_data: &mut HandleZoneDbgData,
) -> HandleZoneInternalResults {
    let our_member_zone_id = our_member.zone_id;

    if let Some(maintained_zone) = maybe_maintained_zone {
        let mut zone_change_dbg_data = ZoneChangeDbgData::default();
        let zone_change = ZoneChange {
            membership: Vec::new(),
            zones: Vec::new(),
            // TODO(krnowak): Ew.
            //
            // Setting this field is not necessary, it is
            // used only for checking in the block list
            from: Member::default(),
            zone_id: maintained_zone.id,
            new_aliases: vec![sender_zone],
        };
        let zone_change_results = process_zone_change_internal_state(
            maintained_zone,
            maybe_successor_of_maintained_zone,
            our_member_zone_id,
            zone_change,
            &mut zone_change_dbg_data,
        );

        dbg_data.zone_change_dbg_data = Some(zone_change_dbg_data);

        HandleZoneInternalResults::ZoneProcessed(zone_change_results)
    } else {
        match sender_zone.id.cmp(&our_member_zone_id) {
            CmpOrdering::Less => HandleZoneInternalResults::SendAck,
            CmpOrdering::Equal => HandleZoneInternalResults::Nothing,
            CmpOrdering::Greater => {
                let mut changes = MemberOrZoneChanges::default();
                let sender_zone_id = sender_zone.id;
                let maybe_msg_and_target = if let Some(our_zone) = maybe_our_zone {
                    let maybe_target = {
                        dbg_data
                            .parse_failures
                            .push(format!("our zone clone: {:#?}", our_zone));
                        if let Some(successor_id) = our_zone.successor {
                            dbg_data
                                .parse_failures
                                .push("our zone clone has successor".to_string());

                            if successor_id < sender_zone.id {
                                dbg_data.parse_failures.push(format!(
                                    "our zone clone successor {} is less \
                                     than sender zone {}, targetting {:#?}",
                                    successor_id, sender_zone.id, maybe_our_zone_maintainer
                                ));
                                maybe_our_zone_maintainer
                            } else {
                                dbg_data.parse_failures.push(format!(
                                    "our zone clone successor {} is NOT less than sender zone {}",
                                    successor_id, sender_zone.id
                                ));
                                None
                            }
                        } else {
                            dbg_data.parse_failures.push(format!(
                                "our zone clone has no successor, targetting {:#?}",
                                maybe_our_zone_maintainer
                            ));
                            maybe_our_zone_maintainer
                        }
                    };

                    if let Some(target) = maybe_target {
                        let zone_change = ZoneChange {
                            membership: Vec::new(),
                            zones: Vec::new(),
                            from: our_member,
                            zone_id: our_member_zone_id,
                            new_aliases: vec![sender_zone],
                        };

                        Some((zone_change, target))
                    } else {
                        None
                    }
                } else {
                    error!(
                        "We have no information about our current zone {}",
                        our_member.zone_id
                    );
                    None
                };

                changes.zone_id_for_our_member = Some(sender_zone_id);
                changes.msg_and_target = maybe_msg_and_target;

                dbg_data.our_new_zone_id = sender_zone_id.to_string();
                dbg_data.msg_and_target = changes.msg_and_target.clone();

                HandleZoneInternalResults::Changes(changes)
            }
        }
    }
}
