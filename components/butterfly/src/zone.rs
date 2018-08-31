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
use protocol::{newscast, swim as proto, FromProto};
use rumor::{RumorKey, RumorPayload, RumorType};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZoneAddress {
    pub zone_id: BfUuid,
    pub address: Option<String>,
    pub swim_port: u16,
    pub gossip_port: u16,
    pub tag: String,
}

impl From<ZoneAddress> for proto::ZoneAddress {
    fn from(value: ZoneAddress) -> Self {
        Self {
            zone_id: Some(value.zone_id.to_string()),
            address: value.address,
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
            address: proto.address,
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
            parent_zone_id: value.parent_zone_id.map(|uuid| uuid.to_string()),
            child_zone_ids: value
                .child_zone_ids
                .iter()
                .map(|uuid| uuid.to_string())
                .collect(),
            successor: value.successor.map(|uuid| uuid.to_string()),
            predecessors: value
                .predecessors
                .iter()
                .map(|uuid| uuid.to_string())
                .collect(),
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

    fn ensure_id(&mut self, uuid: BfUuid) -> usize {
        match self.map.entry(uuid) {
            Entry::Occupied(oe) => *(oe.get()),
            Entry::Vacant(ve) => {
                let idx = self.vecs.len();

                self.vecs.push(vec![uuid]);
                ve.insert(idx);
                idx
            }
        }
    }

    fn is_alias_of(&self, uuid1: BfUuid, uuid2: BfUuid) -> bool {
        match (self.map.get(&uuid1), self.map.get(&uuid2)) {
            (Some(idx1), Some(idx2)) => idx1 == idx2,
            (_, _) => false,
        }
    }

    fn take_aliases_from(&mut self, uuid1: BfUuid, uuid2: BfUuid) {
        let idx1 = self.ensure_id(uuid1);
        match self.map.entry(uuid2) {
            Entry::Occupied(mut oe) => {
                let idx2 = *oe.get();
                if idx1 == idx2 {
                    return;
                }
                *(oe.get_mut()) = idx1;
                let old_ids = mem::replace(&mut self.vecs[idx2], Vec::new());
                self.vecs[idx1].extend(old_ids);
            }
            Entry::Vacant(ve) => {
                self.vecs[idx1].push(uuid2);
                ve.insert(idx1);
            }
        }
    }

    fn into_max_set(self) -> HashSet<BfUuid> {
        let mut indices = self.map.values().collect::<Vec<_>>();

        indices.sort_unstable();
        indices.dedup();

        let mut set = HashSet::with_capacity(indices.len());

        for idx in indices {
            let max_uuid = self.vecs[*idx].iter().max().unwrap();

            set.insert(*max_uuid);
        }

        return set;
    }
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

    update_counter: usize,
}

impl ZoneList {
    pub fn new() -> Self {
        Self {
            zones: HashMap::new(),
            maintained_zone_id: None,
            update_counter: 0,
        }
    }

    pub fn available_zone_ids(&self) -> Vec<BfUuid> {
        self.zones.keys().cloned().collect()
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
        let zone_uuid = zone.id;

        if zone_uuid.is_nil() {
            return Vec::new();
        }

        let current_zone = match self.zones.get(&zone.id).cloned() {
            Some(cz) => cz,
            None => {
                let rk = RumorKey::from(&zone);

                self.zones.insert(zone.id, zone);

                return vec![rk];

                //return self.make_zones_consistent(zone);
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
                    (Some(successor_uuid), Some(current_successor_uuid)) => {
                        match successor_uuid.cmp(&current_successor_uuid) {
                            Ordering::Greater => {
                                predecessors.insert(current_successor_uuid);
                            }
                            Ordering::Equal => (),
                            Ordering::Less => {
                                predecessors.insert(successor_uuid);
                                zone.successor = Some(current_successor_uuid);
                            }
                        }
                    }
                    (Some(_), None) => {}
                    (None, Some(current_successor_uuid)) => {
                        zone.successor = Some(current_successor_uuid);
                    }
                    (None, None) => {}
                }

                predecessors.extend(current_zone.predecessors.iter());
                predecessors.extend(zone.predecessors.iter());
                zone.predecessors = predecessors.drain().collect();

                match (zone.parent_zone_id, current_zone.parent_zone_id) {
                    (Some(parent_uuid), Some(current_parent_uuid)) => {
                        if self.is_alias_of(parent_uuid, current_parent_uuid) {
                            if current_parent_uuid > parent_uuid {
                                zone.parent_zone_id = Some(current_parent_uuid);
                            }
                        } else {
                            debug!(
                                "PARENTS: looks like a new parent ({}) for zone {} is not an alias of {}",
                                parent_uuid,
                                zone.id,
                                current_parent_uuid,
                            );
                            zone.parent_zone_id = Some(current_parent_uuid);
                        }
                    }
                    (None, None) => (),
                    (Some(_), None) => (),
                    (None, Some(current_parent_uuid)) => {
                        zone.parent_zone_id = Some(current_parent_uuid);
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
            if let Some(successor_uuid) = zone.successor {
                if successor_uuid == id2 {
                    return true;
                }
            }
            if zone.predecessors.iter().any(|id| *id == id2) {
                return true;
            }
        }
        if let Some(zone) = self.zones.get(&id2) {
            if let Some(successor_uuid) = zone.successor {
                if successor_uuid == id1 {
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
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut aliases = HashSet::new();
        let mut original_parent = None;
        let mut parents = HashSet::new();
        let mut children = HashSet::new();

        aliases.insert(zone.id);
        if let Some(successor_uuid) = zone.successor {
            aliases.insert(successor_uuid);
        }
        aliases.extend(zone.predecessors.iter());

        if let Some(parent_uuid) = zone.parent_zone_id {
            parents.insert(parent_uuid);
            original_parent = Some(parent_uuid);
        }
        children.extend(zone.child_zone_ids.iter());
        queue.extend(aliases.iter().cloned());

        visited.insert(zone.id);
        while let Some(uuid) = queue.pop_front() {
            if visited.contains(&uuid) {
                continue;
            }
            visited.insert(uuid);
            if let Some(other_zone) = self.zones.get(&uuid) {
                aliases.insert(uuid);
                if let Some(successor_uuid) = other_zone.successor {
                    aliases.insert(successor_uuid);
                    queue.push_back(successor_uuid);
                }

                aliases.extend(other_zone.predecessors.iter());
                queue.extend(other_zone.predecessors.iter());
                children.extend(other_zone.child_zone_ids.iter());
            }
        }

        let successor = match aliases.iter().max() {
            Some(id) => *id,
            None => return vec![RumorKey::from(&zone)],
        };
        let predecessors = aliases
            .drain()
            .filter(|i| *i < successor)
            .collect::<Vec<BfUuid>>();
        let parent = {
            let mut zone_ids = self.filter_aliases(parents);

            match zone_ids.len() {
                0 => original_parent,
                1 => zone_ids.drain().next(),
                _ => {
                    debug!(
                        "PARENTS: got some unrelated parents, {:#?}, using the original one {:#?}",
                        zone_ids, original_parent
                    );
                    original_parent
                }
            }
        };
        let final_children = self.filter_aliases(children);
        let mut rumor_keys = Vec::new();

        for zone_uuid in visited.drain() {
            let mut changed = false;
            let mut other_zone = match self.zones.get(&zone_uuid).cloned() {
                Some(oz) => oz,
                None => continue,
            };
            let mut new_parent = None;

            let new_successor = if successor != zone_uuid {
                if let Some(other_successor_uuid) = other_zone.successor {
                    other_successor_uuid < successor
                } else {
                    true
                }
            } else {
                false
            };
            if new_successor {
                other_zone.successor = Some(successor);
                changed = true;
            }
            match (other_zone.parent_zone_id, parent) {
                (Some(other_parent_uuid), Some(parent_uuid)) => {
                    if other_parent_uuid < parent_uuid {
                        new_parent = Some(parent_uuid);
                    }
                }
                (Some(_), None) => {
                    debug!(
                        "PARENTS: we had one parent in {:#?}, now we are supposed to have none?",
                        other_zone
                    );
                    // eh?
                }
                (None, Some(parent_uuid)) => {
                    new_parent = Some(parent_uuid);
                }
                (None, None) => {}
            }
            if let Some(uuid) = new_parent {
                other_zone.parent_zone_id = Some(uuid);
                changed = true;
            }

            let mut filtered_predecessors = predecessors
                .iter()
                .filter(|uuid| **uuid != zone_uuid)
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

            if final_children
                .symmetric_difference(&old_children)
                .next()
                .is_some()
            {
                other_zone.child_zone_ids = final_children.iter().cloned().collect();
                changed = true;
            }

            if changed {
                if self.is_maintained_zone(other_zone.id) {
                    other_zone.incarnation += 1;
                }

                rumor_keys.push(RumorKey::from(&other_zone));
                self.zones.insert(zone_uuid, other_zone);
            }
        }

        rumor_keys
    }

    fn filter_aliases(&self, zone_uuids: HashSet<BfUuid>) -> HashSet<BfUuid> {
        match zone_uuids.len() {
            0 | 1 => zone_uuids,
            len => {
                let ids = zone_uuids.iter().collect::<Vec<_>>();
                let mut zone_alias_list = ZoneAliasList::new();

                zone_alias_list.ensure_id(*ids[0]);
                for first_idx in 0..(len - 1) {
                    let id1 = *ids[first_idx];

                    for second_idx in (first_idx + 1)..len {
                        let id2 = *ids[second_idx];

                        if zone_alias_list.is_alias_of(id1, id2) {
                            continue;
                        }
                        if self.is_alias_of(id1, id2) {
                            zone_alias_list.take_aliases_from(id1, id2);
                        } else {
                            zone_alias_list.ensure_id(id2);
                        }
                    }
                }
                zone_alias_list.into_max_set()
            }
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
