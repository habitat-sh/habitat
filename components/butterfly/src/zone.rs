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

//! Tracks zones. Contains both the `Zone` struct and the `ZoneList`.

use std::cmp::Ordering;
use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};
use std::mem;

use cast;

use error::{Error, Result};
use message::BfUuid;
use network::{Address, AddressAndPort, Network};
use protocol::{newscast, swim as proto, FromProto};
use rumor::{RumorKey, RumorPayload, RumorType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneAddress {
    pub zone_id: BfUuid,
    pub address: String,
    pub swim_port: u16,
    pub gossip_port: u16,
    pub tag: String,
}

impl From<ZoneAddress> for proto::ZoneAddress {
    fn from(value: ZoneAddress) -> Self {
        Self {
            zone_id: Some(value.zone_id.to_string()),
            address: Some(value.address),
            swim_port: Some(cast::i32(value.swim_port)),
            gossip_port: Some(cast::i32(value.gossip_port)),
            tag: Some(value.tag),
        }
    }
}

impl FromProto<proto::ZoneAddress> for ZoneAddress {
    fn from_proto(proto: proto::ZoneAddress) -> Result<Self> {
        Ok(Self {
            zone_id: proto
                .zone_id
                .ok_or(Error::ProtocolMismatch("zone_id"))?
                .parse::<BfUuid>()
                .map_err(|e| Error::InvalidField("zone_id", e.to_string()))?,
            address: proto.address.ok_or(Error::ProtocolMismatch("address"))?,
            swim_port: cast::u16(proto.swim_port.ok_or(Error::ProtocolMismatch("swim_port"))?)
                .map_err(|e| Error::InvalidField("swim_port", e.to_string()))?,
            gossip_port: cast::u16(
                proto
                    .gossip_port
                    .ok_or(Error::ProtocolMismatch("gossip_port"))?,
            ).map_err(|e| Error::InvalidField("gossip_port", e.to_string()))?,
            tag: proto.tag.ok_or(Error::ProtocolMismatch("tag"))?,
        })
    }
}

/// A zone in the swim group. Passes most of its functionality along
/// to the internal protobuf representation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    pub id: BfUuid,
    pub incarnation: u64,
    pub maintainer_id: String,
    pub parent_zone_id: Option<BfUuid>,
    pub child_zone_ids: Vec<BfUuid>,
    pub successor: Option<BfUuid>,
    pub predecessors: Vec<BfUuid>,
}

impl Zone {
    pub fn new(id: BfUuid, maintainer_id: String) -> Self {
        Self {
            id: id,
            incarnation: 0,
            maintainer_id: maintainer_id,
            parent_zone_id: None,
            child_zone_ids: Vec::new(),
            successor: None,
            predecessors: Vec::new(),
        }
    }
}

// TODO(krnowak): The Default trait implementation of the zone is
// rather pointless…
impl Default for Zone {
    fn default() -> Self {
        Self {
            id: BfUuid::generate(),
            incarnation: 0,
            maintainer_id: BfUuid::nil().to_string(),
            parent_zone_id: None,
            child_zone_ids: Vec::new(),
            successor: None,
            predecessors: Vec::new(),
        }
    }
}

impl From<Zone> for RumorKey {
    fn from(zone: Zone) -> RumorKey {
        RumorKey::new(RumorType::Zone, &zone.id, "")
    }
}

impl<'a> From<&'a Zone> for RumorKey {
    fn from(zone: &'a Zone) -> RumorKey {
        RumorKey::new(RumorType::Zone, zone.id, "")
    }
}

impl<'a> From<&'a &'a Zone> for RumorKey {
    fn from(zone: &'a &'a Zone) -> RumorKey {
        RumorKey::new(RumorType::Zone, zone.id, "")
    }
}

impl From<Zone> for proto::Zone {
    fn from(value: Zone) -> Self {
        Self {
            id: Some(value.id.to_string()),
            incarnation: Some(value.incarnation),
            maintainer_id: Some(value.maintainer_id.to_string()),
            parent_zone_id: value.parent_zone_id.map(|id| id.to_string()),
            child_zone_ids: value
                .child_zone_ids
                .iter()
                .map(|id| id.to_string())
                .collect(),
            successor: value.successor.map(|id| id.to_string()),
            predecessors: value.predecessors.iter().map(|id| id.to_string()).collect(),
        }
    }
}

impl FromProto<proto::Zone> for Zone {
    fn from_proto(proto: proto::Zone) -> Result<Self> {
        let parent_zone_id = match proto.parent_zone_id {
            Some(id) => Some(
                id.parse::<BfUuid>()
                    .map_err(|e| Error::InvalidField("parent_zone_id", e.to_string()))?,
            ),
            None => None,
        };
        let mut child_zone_ids = Vec::with_capacity(proto.child_zone_ids.len());
        for zone_id in proto.child_zone_ids {
            child_zone_ids.push(
                zone_id
                    .parse::<BfUuid>()
                    .map_err(|e| Error::InvalidField("child_zone_ids", e.to_string()))?,
            );
        }
        let successor = match proto.successor {
            Some(id) => Some(
                id.parse::<BfUuid>()
                    .map_err(|e| Error::InvalidField("successor", e.to_string()))?,
            ),
            None => None,
        };
        let mut predecessors = Vec::with_capacity(proto.predecessors.len());
        for zone_id in proto.predecessors {
            predecessors.push(
                zone_id
                    .parse::<BfUuid>()
                    .map_err(|e| Error::InvalidField("predecessors", e.to_string()))?,
            );
        }
        Ok(Self {
            id: proto
                .id
                .ok_or(Error::ProtocolMismatch("id"))?
                .parse::<BfUuid>()
                .map_err(|e| Error::InvalidField("id", e.to_string()))?,
            incarnation: proto.incarnation.unwrap_or(0),
            maintainer_id: proto
                .maintainer_id
                .ok_or(Error::ProtocolMismatch("maintainer_id"))?,
            parent_zone_id: parent_zone_id,
            child_zone_ids: child_zone_ids,
            successor: successor,
            predecessors: predecessors,
        })
    }
}

impl FromProto<newscast::Rumor> for Zone {
    fn from_proto(proto: newscast::Rumor) -> Result<Self> {
        match proto.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::Zone(zone) => Zone::from_proto(zone),
            _ => panic!("from-proto payload"),
        }
    }
}

struct ZoneAliasList {
    vecs: Vec<Vec<BfUuid>>,
    map: HashMap<BfUuid, usize>,
}

impl ZoneAliasList {
    fn new() -> Self {
        Self {
            vecs: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn ensure_id(&mut self, id: BfUuid) -> usize {
        match self.map.entry(id) {
            Entry::Occupied(oe) => *(oe.get()),
            Entry::Vacant(ve) => {
                let idx = self.vecs.len();

                self.vecs.push(vec![id]);
                ve.insert(idx);
                idx
            }
        }
    }

    fn is_alias_of(&self, id1: BfUuid, id2: BfUuid) -> bool {
        match (self.map.get(&id1), self.map.get(&id2)) {
            (Some(idx1), Some(idx2)) => idx1 == idx2,
            (_, _) => false,
        }
    }

    fn take_aliases_from(&mut self, id1: BfUuid, id2: BfUuid) {
        let idx1 = self.ensure_id(id1);
        let fixup_ids_with_idx1 = match self.map.entry(id2) {
            Entry::Occupied(oe) => {
                let idx2 = *oe.get();
                if idx1 != idx2 {
                    mem::replace(&mut self.vecs[idx2], Vec::new())
                } else {
                    Vec::new()
                }
            }
            Entry::Vacant(ve) => {
                self.vecs[idx1].push(id2);
                ve.insert(idx1);
                Vec::new()
            }
        };

        self.vecs[idx1].reserve(fixup_ids_with_idx1.len());
        for id in fixup_ids_with_idx1 {
            self.vecs[idx1].push(id);
            self.map.insert(id, idx1);
        }
    }

    fn into_max_set(self, zone_list: &ZoneList) -> HashSet<BfUuid> {
        let mut indices = self.map.values().collect::<Vec<_>>();

        indices.sort_unstable();
        indices.dedup();

        let mut set = HashSet::with_capacity(indices.len());

        for idx in indices {
            if let Some(max_id) = self.vecs[*idx].iter().max() {
                let id = if let Some(zone) = zone_list.zones.get(max_id) {
                    if let Some(successor_id) = zone.successor {
                        successor_id
                    } else {
                        *max_id
                    }
                } else {
                    *max_id
                };

                set.insert(id);
            }
        }

        return set;
    }
}

struct GatherInfoHelper {
    parents: HashSet<BfUuid>,
    children: HashSet<BfUuid>,
    aliases: HashSet<BfUuid>,
}

impl GatherInfoHelper {
    fn new() -> Self {
        Self {
            parents: HashSet::new(),
            children: HashSet::new(),
            aliases: HashSet::new(),
        }
    }

    fn fill_from_zone(&mut self, zone: &Zone) -> HashSet<BfUuid> {
        let mut new_aliases = HashSet::new();

        self.aliases.insert(zone.id);
        if let Some(successor_id) = zone.successor {
            new_aliases.insert(successor_id);
        }
        if let Some(parent_id) = zone.parent_zone_id {
            self.parents.insert(parent_id);
        }

        new_aliases.extend(zone.predecessors.iter());
        self.aliases.extend(new_aliases.iter());
        self.children.extend(zone.child_zone_ids.iter());

        new_aliases
    }
}

struct AliasesInfo {
    parents: HashSet<BfUuid>,
    children: HashSet<BfUuid>,
    aliases: HashSet<BfUuid>,
    successor: BfUuid,
    predecessors: Vec<BfUuid>,
    max_children: HashSet<BfUuid>,
    max_parents: HashSet<BfUuid>,
}

impl AliasesInfo {
    fn new_for_zone(zone_list: &ZoneList, zone: &Zone) -> Self {
        let mut helper = GatherInfoHelper::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut successor = zone.id;

        queue.extend(helper.fill_from_zone(zone));
        visited.insert(zone.id);
        while let Some(id) = queue.pop_front() {
            if visited.contains(&id) {
                continue;
            }
            visited.insert(id);
            if successor < id {
                successor = id;
            }
            if let Some(other_zone) = zone_list.zones.get(&id) {
                queue.extend(helper.fill_from_zone(other_zone));
            }
        }

        let predecessors = helper
            .aliases
            .iter()
            .filter(|i| **i < successor)
            .cloned()
            .collect();
        let max_children = Self::filter_aliases(zone_list, &helper.children);
        let max_parents = Self::filter_aliases(zone_list, &helper.parents);

        Self {
            parents: helper.parents,
            children: helper.children,
            aliases: helper.aliases,
            successor: successor,
            predecessors: predecessors,
            max_children: max_children,
            max_parents: max_parents,
        }
    }

    fn filter_aliases(zone_list: &ZoneList, zone_ids: &HashSet<BfUuid>) -> HashSet<BfUuid> {
        match zone_ids.len() {
            0 => HashSet::new(),
            1 => {
                let mut zone_alias_list = ZoneAliasList::new();

                zone_alias_list.ensure_id(*zone_ids.iter().next().unwrap());
                zone_alias_list.into_max_set(&zone_list)
            }
            len => {
                let ids = zone_ids.iter().collect::<Vec<_>>();
                let mut zone_alias_list = ZoneAliasList::new();

                zone_alias_list.ensure_id(*ids[0]);
                for first_idx in 0..(len - 1) {
                    let id1 = *ids[first_idx];

                    for second_idx in (first_idx + 1)..len {
                        let id2 = *ids[second_idx];

                        if zone_alias_list.is_alias_of(id1, id2) {
                            continue;
                        }
                        if zone_list.is_alias_of(id1, id2) {
                            zone_alias_list.take_aliases_from(id1, id2);
                        } else {
                            zone_alias_list.ensure_id(id2);
                        }
                    }
                }
                zone_alias_list.into_max_set(&zone_list)
            }
        }
    }
}

enum ChangeRelationship {
    Ourselves,
    Children(Option<usize>),
    Parent,
    Other,
}

pub enum Reachable {
    Yes,
    ThroughOtherZone(BfUuid),
    No,
}

#[derive(Debug, Serialize)]
pub struct ZoneList {
    pub zones: HashMap<BfUuid, Zone>,
    pub maintained_zone_id: Option<BfUuid>,
    pub our_zone_id: BfUuid,

    update_counter: usize,
}

impl ZoneList {
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
            maintained_zone_id: None,
            our_zone_id: BfUuid::nil(),
            update_counter: 0,
        }
    }

    pub fn get_update_counter(&self) -> usize {
        self.update_counter
    }

    pub fn insert(&mut self, zone: Zone) -> Vec<RumorKey> {
        let keys = self.insert_internal(zone);

        if !keys.is_empty() {
            self.update_counter += 1;
        }

        keys
    }

    pub fn gather_all_aliases_of(&self, id: BfUuid) -> HashSet<BfUuid> {
        let mut aliases = HashSet::new();

        aliases.insert(id);
        if let Some(zone) = self.zones.get(&id) {
            if let Some(zone_id) = zone.successor {
                aliases.insert(zone_id);
            }
            for zone_id in zone.predecessors.iter() {
                aliases.insert(*zone_id);
            }
        }

        aliases
    }

    pub fn directly_reachable(
        &self,
        our_zone_id: BfUuid,
        their_zone_id: BfUuid,
        our_zone_addresses: &[ZoneAddress],
        their_zone_addresses: &[ZoneAddress],
    ) -> Reachable {
        if our_zone_id == their_zone_id {
            return Reachable::Yes;
        }

        let our_ids = self.gather_all_aliases_of(our_zone_id);
        let their_ids = self.gather_all_aliases_of(their_zone_id);

        if !our_ids.is_disjoint(&their_ids) {
            return Reachable::Yes;
        }

        // TODO(krnowak): maybe instead of guessing which zone is
        // parent or child, take this information from the zone itself
        // (get_child_zone_ids(), get_parent_zone_id())

        // if this server is in child zone and is a gateway, and
        // member is in parent zone then this loop may catch that
        for zone_address in our_zone_addresses {
            if their_ids.contains(&zone_address.zone_id) {
                return Reachable::Yes;
            }
        }

        // if this server is in parent zone, and member is in child
        // zone and is a gateway then this loop may catch that
        for zone_address in their_zone_addresses {
            if our_ids.contains(&zone_address.zone_id) {
                return Reachable::ThroughOtherZone(zone_address.zone_id);
            }
        }

        Reachable::No
    }

    fn insert_internal(&mut self, mut zone: Zone) -> Vec<RumorKey> {
        let zone_id = zone.id;

        if zone_id.is_nil() {
            return Vec::new();
        }

        let current_zone = match self.zones.get(&zone.id).cloned() {
            Some(cz) => cz,
            None => {
                return self.make_zones_consistent(zone);
            }
        };

        match current_zone.incarnation.cmp(&zone.incarnation) {
            Ordering::Greater => Vec::new(),
            Ordering::Less => self.make_zones_consistent(zone),
            Ordering::Equal => {
                let mut predecessors = HashSet::new();
                // merge the info from current and new zone, but
                // do not increment the incarnation…
                match (zone.successor, current_zone.successor) {
                    (Some(successor_id), Some(current_successor_id)) => {
                        match successor_id.cmp(&current_successor_id) {
                            Ordering::Greater => {
                                predecessors.insert(current_successor_id);
                            }
                            Ordering::Equal => (),
                            Ordering::Less => {
                                predecessors.insert(successor_id);
                                zone.successor = Some(current_successor_id);
                            }
                        }
                    }
                    (Some(_), None) => {}
                    (None, Some(current_successor_id)) => {
                        zone.successor = Some(current_successor_id);
                    }
                    (None, None) => {}
                }

                predecessors.extend(current_zone.predecessors.iter());
                predecessors.extend(zone.predecessors.iter());
                zone.predecessors = predecessors.drain().collect();

                match (zone.parent_zone_id, current_zone.parent_zone_id) {
                    (Some(parent_id), Some(current_parent_id)) => {
                        if self.is_alias_of(parent_id, current_parent_id) {
                            if current_parent_id > parent_id {
                                zone.parent_zone_id = Some(current_parent_id);
                            }
                        } else {
                            debug!(
                                "PARENTS: looks like a new parent ({}) for zone {} is not an alias of {}",
                                parent_id,
                                zone.id,
                                current_parent_id,
                            );
                            zone.parent_zone_id = Some(current_parent_id);
                        }
                    }
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(current_parent_id)) => {
                        zone.parent_zone_id = Some(current_parent_id);
                    }
                }
                self.make_zones_consistent(zone)
            }
        }
    }

    fn is_alias_of(&self, id1: BfUuid, id2: BfUuid) -> bool {
        if id1 == id2 {
            return true;
        }

        if let Some(zone) = self.zones.get(&id1) {
            if let Some(successor_id) = zone.successor {
                if successor_id == id2 {
                    return true;
                }
            }
            if zone.predecessors.iter().any(|id| *id == id2) {
                return true;
            }
        }
        if let Some(zone) = self.zones.get(&id2) {
            if let Some(successor_id) = zone.successor {
                if successor_id == id1 {
                    return true;
                }
            }
            if zone.predecessors.iter().any(|id| *id == id1) {
                return true;
            }
        }

        false
    }

    fn make_zones_consistent(&mut self, zone: Zone) -> Vec<RumorKey> {
        let aliases_info = AliasesInfo::new_for_zone(self, &zone);
        let mut rumor_keys = Vec::new();
        let mut zones_to_insert = Vec::new();

        match self.get_change_relationship(&aliases_info) {
            ChangeRelationship::Ourselves => {
                for child_zone_id in aliases_info.max_children.iter() {
                    if let Some(child_zone) = self.zones.get(child_zone_id) {
                        let do_change = match child_zone.parent_zone_id {
                            Some(parent_id) => parent_id != aliases_info.successor,
                            None => true,
                        };

                        if do_change {
                            let mut child_zone_clone = child_zone.clone();

                            child_zone_clone.parent_zone_id = Some(aliases_info.successor);
                            zones_to_insert.push(child_zone_clone);
                        }
                    }
                }
                for parent_zone_id in aliases_info.max_parents.iter() {
                    if let Some(parent_zone) = self.zones.get(parent_zone_id) {
                        let mut found_at = None;

                        for (idx, child_id) in parent_zone.child_zone_ids.iter().enumerate() {
                            if aliases_info.aliases.contains(child_id) {
                                found_at = Some(idx);
                                break;
                            }
                        }

                        if let Some(idx) = found_at {
                            if parent_zone.child_zone_ids[idx] != aliases_info.successor {
                                let mut parent_zone_clone = parent_zone.clone();

                                parent_zone_clone.child_zone_ids[idx] = aliases_info.successor;
                                zones_to_insert.push(parent_zone_clone);
                            }
                        } else {
                            let mut parent_zone_clone = parent_zone.clone();

                            parent_zone_clone
                                .child_zone_ids
                                .push(aliases_info.successor);
                            zones_to_insert.push(parent_zone_clone);
                        }
                    }
                }
            }
            ChangeRelationship::Children(maybe_idx) => {
                if let Some(our_zone) = self.zones.get(&self.our_zone_id) {
                    match maybe_idx {
                        Some(idx) => {
                            if our_zone.child_zone_ids[idx] != aliases_info.successor {
                                let mut our_zone_clone = our_zone.clone();

                                our_zone_clone.child_zone_ids[idx] = aliases_info.successor;
                                zones_to_insert.push(our_zone_clone);
                            }
                        }
                        None => {
                            let mut our_zone_clone = our_zone.clone();

                            our_zone_clone.child_zone_ids.push(aliases_info.successor);
                            zones_to_insert.push(our_zone_clone);
                        }
                    }
                }
            }
            ChangeRelationship::Parent => {
                if let Some(our_zone) = self.zones.get(&self.our_zone_id) {
                    let do_change = match our_zone.parent_zone_id {
                        Some(parent_id) => parent_id != aliases_info.successor,
                        None => true,
                    };

                    if do_change {
                        let mut our_zone_clone = our_zone.clone();

                        our_zone_clone.parent_zone_id = Some(aliases_info.successor);
                        zones_to_insert.push(our_zone_clone);
                    }
                }
            }
            ChangeRelationship::Other => {}
        }
        rumor_keys.extend(self.make_zones_consistent_immediate_with_info(zone, aliases_info));
        for zone_to_insert in zones_to_insert {
            rumor_keys.extend(self.make_zones_consistent_immediate(zone_to_insert));
        }

        rumor_keys
    }

    fn make_zones_consistent_immediate(&mut self, zone: Zone) -> Vec<RumorKey> {
        let aliases_info = AliasesInfo::new_for_zone(self, &zone);

        self.make_zones_consistent_immediate_with_info(zone, aliases_info)
    }

    fn make_zones_consistent_immediate_with_info(
        &mut self,
        zone: Zone,
        aliases_info: AliasesInfo,
    ) -> Vec<RumorKey> {
        let parent = {
            match aliases_info.max_parents.len() {
                0 => zone.parent_zone_id,
                1 => aliases_info.max_parents.iter().cloned().next(),
                _ => {
                    debug!(
                        "PARENTS: got some unrelated parents, {:#?}, using the original one {:#?}",
                        aliases_info.max_parents, zone.parent_zone_id
                    );
                    zone.parent_zone_id
                }
            }
        };
        let mut rumor_keys = Vec::new();

        for zone_id in aliases_info.aliases.iter() {
            let mut changed = false;
            let mut new_zone = false;
            let mut other_zone = match self.zones.get(zone_id).cloned() {
                Some(mut oz) => {
                    // if this is a zone that has changed then use its
                    // incarnation value too
                    if *zone_id == zone.id && oz.incarnation != zone.incarnation {
                        oz.incarnation = zone.incarnation;
                        changed = true;
                    }
                    oz
                }
                None => {
                    if *zone_id == zone.id {
                        changed = true;
                        new_zone = true;
                        zone.clone()
                    } else {
                        continue;
                    }
                }
            };
            let mut new_parent = None;

            let new_successor = if aliases_info.successor != *zone_id {
                if let Some(other_successor_id) = other_zone.successor {
                    other_successor_id < aliases_info.successor
                } else {
                    true
                }
            } else {
                false
            };
            if new_successor {
                other_zone.successor = Some(aliases_info.successor);
                changed = true;
            }
            match (other_zone.parent_zone_id, parent) {
                (Some(other_parent_id), Some(parent_id)) => {
                    if other_parent_id < parent_id {
                        new_parent = Some(parent_id);
                    }
                }
                (Some(_), None) => {
                    debug!(
                        "PARENTS: we had one parent in {:#?}, now we are supposed to have none?",
                        other_zone
                    );
                    // eh?
                }
                (None, Some(parent_id)) => {
                    new_parent = Some(parent_id);
                }
                (None, None) => {}
            }
            if let Some(id) = new_parent {
                other_zone.parent_zone_id = Some(id);
                changed = true;
            }

            let mut filtered_predecessors = aliases_info
                .predecessors
                .iter()
                .filter(|id| **id != *zone_id)
                .cloned()
                .collect::<HashSet<_>>();
            let old_predecessors = other_zone
                .predecessors
                .iter()
                .cloned()
                .collect::<HashSet<_>>();

            // filtered predecessors is either a superset of old
            // predecessors or it's equal to it, so we can just use
            // difference instead of symmetric difference
            if filtered_predecessors
                .difference(&old_predecessors)
                .next()
                .is_some()
            {
                other_zone.predecessors = filtered_predecessors.iter().cloned().collect();
                changed = true;
            }

            let old_children = other_zone
                .child_zone_ids
                .iter()
                .cloned()
                .collect::<HashSet<_>>();

            if aliases_info
                .max_children
                .symmetric_difference(&old_children)
                .next()
                .is_some()
            {
                other_zone.child_zone_ids = aliases_info.max_children.iter().cloned().collect();
                changed = true;
            }

            if changed {
                if !new_zone && self.is_maintained_zone(other_zone.id) {
                    other_zone.incarnation += 1;
                }

                rumor_keys.push(RumorKey::from(&other_zone));
                self.zones.insert(*zone_id, other_zone);
            }
        }

        rumor_keys
    }

    fn get_change_relationship(&self, aliases_info: &AliasesInfo) -> ChangeRelationship {
        if self.our_zone_id.is_nil() {
            return ChangeRelationship::Other;
        }

        let our_zone_aliases = self.gather_all_aliases_of(self.our_zone_id);

        if aliases_info
            .aliases
            .intersection(&our_zone_aliases)
            .next()
            .is_some()
        {
            return ChangeRelationship::Ourselves;
        }

        let our_zone = if let Some(zone) = self.zones.get(&self.our_zone_id) {
            zone
        } else {
            // eh?
            return ChangeRelationship::Other;
        };

        if aliases_info
            .parents
            .intersection(&our_zone_aliases)
            .next()
            .is_some()
        {
            let mut found_at = None;

            for (idx, child_id) in our_zone.child_zone_ids.iter().enumerate() {
                if aliases_info.aliases.contains(child_id) {
                    found_at = Some(idx);
                    break;
                }
            }

            ChangeRelationship::Children(found_at)
        } else if aliases_info
            .children
            .intersection(&our_zone_aliases)
            .next()
            .is_some()
        {
            ChangeRelationship::Parent
        } else {
            ChangeRelationship::Other
        }
    }

    fn is_maintained_zone(&self, zone_id: BfUuid) -> bool {
        if let Some(ref maintained_zone_id) = self.maintained_zone_id {
            zone_id == *maintained_zone_id
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct AdditionalAddress<A: Address> {
    pub address: A,
    pub swim_port: u16,
    pub gossip_port: u16,
}

impl<A: Address> AdditionalAddress<A> {
    pub fn new(address: A, swim_port: u16, gossip_port: u16) -> Self {
        Self {
            address,
            swim_port,
            gossip_port,
        }
    }
}

pub type TaggedAddressesFromAddress<A> = HashMap<String, AdditionalAddress<A>>;
pub type TaggedAddressesFromNetwork<N> =
    TaggedAddressesFromAddress<<<N as Network>::AddressAndPort as AddressAndPort>::Address>;
