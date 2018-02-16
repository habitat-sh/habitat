mod nat;

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::iter;
use std::path::PathBuf;
use std::result::Result as StdResult;
use std::str::FromStr;
use std::sync::mpsc::{self, Receiver, SendError, Sender};
use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::time::Duration;
use std::u8;

use habitat_butterfly::error::{Error, Result};
use habitat_butterfly::member::{Health, Member};
use habitat_butterfly::message::{self, BfUuid};
use habitat_butterfly::network::{
    Address, AddressAndPort, GossipReceiver, GossipSender, MyFromStr, Network, SwimReceiver,
    SwimSender,
};
use habitat_butterfly::rumor::RumorEnvelope;
use habitat_butterfly::server::timing::Timing;
use habitat_butterfly::server::{Server, Suitability};
use habitat_butterfly::swim::Swim;
use habitat_butterfly::trace::Trace;
use habitat_butterfly::zone::{AdditionalAddress, TaggedAddressesFromAddress};
use habitat_core::crypto::SymKey;
use habitat_core::service::ServiceGroup;

struct DbgVec {
    v: Vec<String>,
}

impl DbgVec {
    fn new(header: String) -> Self {
        Self {
            v: vec![
                "==============================".to_string(),
                header,
                "==============================".to_string(),
            ],
        }
    }

    fn push<S: AsRef<str>>(&mut self, s: S) {
        for line in s.as_ref().split('\n') {
            self.v.push(line.to_string())
        }
    }
}

impl Drop for DbgVec {
    fn drop(&mut self) {
        self.push("==============================".to_string());
        debug!("{:#?}", self.v);
    }
}

// ZoneID is a number that identifies a zone. Within a zone all the
// supervisors can talk to each other. For the interzone
// communication, a parent-child relationship needs to be established
// first, then supervisors in the child zone can talk to supervisors
// in the parent zone, but not the other way around.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct ZoneID(u8);

impl ZoneID {
    pub fn new(raw_id: u8) -> Self {
        assert!(
            Self::is_raw_valid(raw_id),
            "zone IDs must be greater than zero"
        );
        ZoneID(raw_id)
    }

    pub fn raw(&self) -> u8 {
        self.0
    }

    pub fn is_raw_valid(raw: u8) -> bool {
        raw > 0
    }
}

impl Display for ZoneID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.raw())
    }
}

impl FromStr for ZoneID {
    type Err = String;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let raw_self = s
            .parse()
            .map_err(|e| format!("'{}' is not a u8: {}", s, e))?;
        if !Self::is_raw_valid(raw_self) {
            return Err(format!("{} is not a valid ZoneID", raw_self));
        }

        Ok(Self::new(raw_self))
    }
}

// ZoneInfo stores the relationship information of a zone.
#[derive(Debug, Default, Clone)]
struct ZoneInfo {
    parent: Option<ZoneID>,
    children: HashSet<ZoneID>,
}

struct ZoneMap(HashMap<ZoneID, Mutex<ZoneInfo>>);

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    ParentToChild,
    ChildToParent,
}

struct DijkstraData {
    info: ZoneInfo,
    distance: usize,
}

impl DijkstraData {
    pub fn new_with_max_distance(info: &ZoneInfo) -> Self {
        Self::new(info, usize::max_value())
    }

    pub fn new_with_zero_distance(info: &ZoneInfo) -> Self {
        Self::new(info, 0)
    }

    fn new(info: &ZoneInfo, distance: usize) -> Self {
        let info = info.clone();
        Self { info, distance }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct TraversalInfo {
    direction: Direction,
    from: ZoneID,
    to: ZoneID,
}

impl TraversalInfo {
    pub fn new(direction: Direction, from: ZoneID, to: ZoneID) -> Self {
        Self {
            direction,
            from,
            to,
        }
    }
}

#[derive(Eq, PartialEq)]
struct DijkstraState {
    cost: usize,
    id: ZoneID,
    route: Vec<TraversalInfo>,
}

impl DijkstraState {
    pub fn new_start(start_id: ZoneID) -> Self {
        Self::new(0, start_id, Vec::new())
    }

    pub fn new_incremental(old: &Self, new_id: ZoneID, direction: Direction) -> Self {
        let mut new_route = old.route.clone();

        new_route.push(TraversalInfo::new(direction, old.id, new_id));
        Self::new(old.cost + 1, new_id, new_route)
    }

    pub fn steal_route(self) -> Vec<TraversalInfo> {
        self.route
    }

    fn new(cost: usize, id: ZoneID, route: Vec<TraversalInfo>) -> Self {
        Self { cost, id, route }
    }
}

// This is to make BinaryHeap a min-heap instead of max-heap.
impl Ord for DijkstraState {
    fn cmp(&self, other: &DijkstraState) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &DijkstraState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ZoneMap {
    pub fn setup_zone_relationship(&mut self, parent_id: ZoneID, child_id: ZoneID) {
        assert_ne!(parent_id, child_id);
        self.ensure_zone(parent_id);
        self.ensure_zone(child_id);
        assert!(!self.is_zone_descendant_of_mut(parent_id, child_id));
        {
            let parent_zone = self.get_zone_mut(parent_id);
            parent_zone.children.insert(child_id);
        }
        {
            let child_zone = self.get_zone_mut(child_id);
            assert!(child_zone.parent.is_none());
            child_zone.parent = Some(parent_id);
        }
    }

    pub fn is_zone_child_of(&self, child_id: ZoneID, parent_id: ZoneID) -> bool {
        self.get_zone_guard(parent_id).children.contains(&child_id)
    }

    pub fn is_zone_descendant_of_mut(
        &mut self,
        descendant_id: ZoneID,
        ancestor_id: ZoneID,
    ) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(ancestor_id);
        while let Some(id) = queue.pop_front() {
            let zone = self.get_zone_mut(id);
            if zone.children.contains(&descendant_id) {
                return true;
            }
            queue.extend(&zone.children);
        }
        false
    }

    // Dijkstra, basically.
    pub fn get_route(&self, source_id: ZoneID, target_id: ZoneID) -> Option<Vec<TraversalInfo>> {
        if source_id == target_id {
            return Some(Vec::new());
        }

        let mut dd_map = HashMap::with_capacity(self.0.len());
        let mut heap = BinaryHeap::new();

        for (zone_id, info_lock) in &self.0 {
            let info = info_lock
                .lock()
                .expect(&format!("Zone {} lock is poisoned", zone_id));
            let dd = if *zone_id == source_id {
                DijkstraData::new_with_zero_distance(&info)
            } else {
                DijkstraData::new_with_max_distance(&info)
            };
            dd_map.insert(*zone_id, dd);
        }
        heap.push(DijkstraState::new_start(source_id));

        while let Some(ds) = heap.pop() {
            if ds.id == target_id {
                return Some(ds.steal_route());
            }

            let (parent, children) = {
                let dd = Self::get_dijkstra_data(&dd_map, ds.id);
                if ds.cost > dd.distance {
                    continue;
                }

                (dd.info.parent.clone(), dd.info.children.clone())
            };

            if let Some(parent_id) = parent {
                Self::dijkstra_step(
                    &mut dd_map,
                    parent_id,
                    &mut heap,
                    &ds,
                    Direction::ChildToParent,
                );
            }

            for child_id in children {
                Self::dijkstra_step(
                    &mut dd_map,
                    child_id,
                    &mut heap,
                    &ds,
                    Direction::ParentToChild,
                );
            }
        }

        None
    }

    fn dijkstra_step(
        dd_map: &mut HashMap<ZoneID, DijkstraData>,
        id: ZoneID,
        heap: &mut BinaryHeap<DijkstraState>,
        old_ds: &DijkstraState,
        direction: Direction,
    ) {
        let dd = Self::get_dijkstra_data_mut(dd_map, id);
        if old_ds.cost + 1 < dd.distance {
            let new_ds = DijkstraState::new_incremental(&old_ds, id, direction);

            dd.distance = new_ds.cost;
            heap.push(new_ds);
        }
    }

    fn get_dijkstra_data<'a>(
        map: &'a HashMap<ZoneID, DijkstraData>,
        id: ZoneID,
    ) -> &'a DijkstraData {
        map.get(&id)
            .expect(&format!("zone {} exists in dijkstra data map", id))
    }

    fn get_dijkstra_data_mut<'a>(
        map: &'a mut HashMap<ZoneID, DijkstraData>,
        id: ZoneID,
    ) -> &'a mut DijkstraData {
        map.get_mut(&id)
            .expect(&format!("zone {} exists in dijkstra data map", id))
    }

    fn ensure_zone(&mut self, zone_id: ZoneID) {
        if let Entry::Vacant(v) = self.0.entry(zone_id) {
            v.insert(Mutex::new(ZoneInfo::default()));
        }
    }

    fn get_zone_guard(&self, zone_id: ZoneID) -> MutexGuard<ZoneInfo> {
        self.0
            .get(&zone_id)
            .expect(&format!("Zone {} not in zone map", zone_id))
            .lock()
            .expect(&format!("Zone {} lock is poisoned", zone_id))
    }

    fn get_zone_mut(&mut self, zone_id: ZoneID) -> &mut ZoneInfo {
        self.0
            .get_mut(&zone_id)
            .expect(&format!("Zone {} not in zone map", zone_id))
            .get_mut()
            .expect(&format!("Zone {} lock is poisoned", zone_id))
    }
}

#[derive(Debug)]
struct TestAddrParseError {
    failed_string: String,
    reason: String,
    idx: usize,
}

impl TestAddrParseError {
    fn new<T1, T2>(failed_string: T1, reason: T2, idx: usize) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        Self {
            failed_string: failed_string.into(),
            reason: reason.into(),
            idx: idx,
        }
    }
}

impl Display for TestAddrParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let spaces = iter::repeat(' ').take(self.idx).collect::<String>();
        write!(
            f,
            "failed to parse TestAddr at idx {}: {}\n\
             {}\n\
             {}^",
            self.idx, self.reason, self.failed_string, spaces,
        )
    }
}

impl StdError for TestAddrParseError {
    fn description(&self) -> &str {
        "failed to parse TestAddr from some string at some index for some reason"
    }
}

#[derive(Debug)]
struct TestAddrParts {
    address_type: String,
    fields: Vec<(String, String)>,
    port: Option<u16>,
}

impl FromStr for TestAddrParts {
    type Err = TestAddrParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        #[derive(PartialEq)]
        enum State {
            Start,
            OpenBracket,
            AddressType,
            CloseBracket,
            OpenBrace,
            FieldName,
            FieldSeparator,
            FieldValue,
            CloseBrace,
            Colon,
            Port,
        };
        let mut state = State::Start;
        let final_states = vec![State::CloseBrace, State::Port];
        let mut address_type = String::new();
        let mut fields = Vec::new();
        let mut field_name = String::new();
        let mut field_value = String::new();
        let mut maybe_port = None;

        for (idx, c) in s.chars().enumerate() {
            match state {
                State::Start => match c {
                    '[' => state = State::OpenBracket,
                    _ => return Err(Self::Err::new(s, "expected an opening bracket", idx)),
                },
                State::OpenBracket => match c {
                    'a' ... 'z' | '-' => {
                        address_type.push(c);
                        state = State::AddressType;
                    }
                    _ => return Err(Self::Err::new(s, "expected an alphabetic ASCII char or a dash for the address type", idx)),
                }
                State::AddressType => match c {
                    'a' ... 'z' | '-' => {
                        address_type.push(c);
                    }
                    ']' => state = State::CloseBracket,
                    _ => return Err(Self::Err::new(s, "expected an alphabetic ASCII char or a dash for the address type, or a closing bracket", idx)),
                }
                State::CloseBracket => match c {
                    '{' => state = State::OpenBrace,
                    _ => return Err(Self::Err::new(s, "expected an opening brace for address contents", idx)),
                }
                State::OpenBrace => match c {
                    'a' ... 'z' | '0' ... '9' | '-' | '_' => {
                        field_name.push(c);
                        state = State::FieldName;
                    }
                    '}' => state = State::CloseBrace,
                    _ => return Err(Self::Err::new(s, "expected either a closing brace or ASCII alphanumeric char for a field", idx))
                }
                State::FieldName => match c {
                    'a' ... 'z' | '0' ... '9' | '-' | '_' => field_name.push(c),
                    ':' => state = State::FieldSeparator,
                    _ => return Err(Self::Err::new(s, "expected field name or a colon", idx)),
                }
                State::FieldSeparator => match c {
                    'a' ... 'z' | '0' ... '9' => {
                        field_value.push(c);
                        state = State::FieldValue;
                    }
                    '}' => {
                        fields.push((field_name, field_value));
                        field_name = String::new();
                        field_value = String::new();
                        state = State::CloseBrace;
                    }
                    _ => return Err(Self::Err::new(s, "expected field name or closing brace", idx)),
                }
                State::FieldValue => match c {
                    'a' ... 'z' | '0' ... '9' => field_value.push(c),
                    ',' => {
                        fields.push((field_name, field_value));
                        field_name = String::new();
                        field_value = String::new();
                        state = State::FieldName;
                    }
                    '}' => {
                        fields.push((field_name, field_value));
                        field_name = String::new();
                        field_value = String::new();
                        state = State::CloseBrace;
                    }
                    _ => return Err(Self::Err::new(s, "expected field value, field separator (,) or closing brace", idx)),
                }
                State::CloseBrace => match c {
                    ':' => state = State::Colon,
                    _ => return Err(Self::Err::new(s, "expected a colon after the closing brace", idx)),
                }
                State::Colon => match c {
                    '0' ... '9' => {
                        let mut port_str = String::new();

                        port_str.push(c);
                        maybe_port = Some(port_str);
                        state = State::Port;
                    }
                    _ => return Err(Self::Err::new(s, "expected a number after the colon", idx)),
                }
                State::Port => match c {
                    '0' ... '9' => maybe_port.as_mut().unwrap().push(c),
                    _ => return Err(Self::Err::new(s, "expected a number for a port", idx))
                }
            }
        }

        if !final_states.contains(&state) {
            let mut len = s.len();

            if len > 0 {
                len -= 1;
            }
            return Err(Self::Err::new(s, "premature end of address string", len));
        }

        let port = match maybe_port {
            Some(port_str) => {
                let parsed_port = match port_str.parse::<u16>() {
                    Ok(port) => Ok(port),
                    Err(e) => {
                        let mut len = s.len();

                        if len > 0 {
                            len -= 1;
                        }
                        Err(Self::Err::new(
                            s,
                            format!("still failed to parse port into a u16 number: {}", e),
                            len,
                        ))
                    }
                }?;
                Some(parsed_port)
            }
            None => None,
        };

        Ok(Self {
            address_type,
            fields,
            port,
        })
    }
}

fn field_name_check(actual: &str, expected: &str, idx: &str) -> StdResult<(), String> {
    if actual != expected {
        Err(format!(
            "expected {} field to be '{}', got {}",
            idx, expected, actual
        ))
    } else {
        Ok(())
    }
}

/*
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TestPublicAddr {
    zone_id: ZoneID,
    idx: u8,
}

impl TestPublicAddr {
    fn new(zone_id: ZoneID, idx: u8) -> Self {
        Self { zone_id, idx }
    }

    fn from_parts(parts: TestAddrParts) -> StdResult<Self, String> {
        if parts.address_type != "public" {
            return Err(format!(
                "expected 'public' address type, got '{}'",
                parts.address_type
            ));
        }
        if let Some(port) = parts.port {
            return Err(format!("expected no port information, got '{}'", port));
        }

        if parts.fields.len() != 2 {
            return Err(format!(
                "expected exactly 2 fields, got {}",
                parts.fields.len()
            ));
        }

        field_name_check(&parts.fields[0].0, "zone-id", "first")?;
        let zone_id = parts.fields[0]
            .1
            .parse()
            .map_err(|e| format!("failed to get zone ID from first field: {}", e))?;
        field_name_check(&parts.fields[1].0, "idx", "second")?;
        let idx = parts.fields[1]
            .1
            .parse()
            .map_err(|e| format!("failed to get index from second field: {}", e))?;

        Ok(Self::new(zone_id, idx))
    }

    fn get_zone_id(&self) -> ZoneID {
        self.zone_id
    }

    fn get_valid_port() -> u16 {
        42
    }

    fn validate_port(port: u16) -> StdResult<u16, String> {
        let valid_port = Self::get_valid_port();
        if port == valid_port {
            Ok(valid_port)
        } else {
            Err(format!(
                "expected port for public address to be '{}', got '{}'",
                valid_port, port
            ))
        }
    }
}

impl Display for TestPublicAddr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "[public]{{zone-id:{},idx:{}}}", self.zone_id, self.idx)
    }
}

impl FromStr for TestPublicAddr {
    type Err = TestAddrParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let parts = s.parse::<TestAddrParts>()?;

        Self::from_parts(parts)
            .map_err(|e| Self::Err::new(s, format!("badly formed public address: {}", e), 0))
    }
}

impl MyFromStr for TestPublicAddr {
    type MyErr = <Self as FromStr>::Err;
}
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TestLocalAddr {
    zone_id: ZoneID,
    idx: u8,
}

impl TestLocalAddr {
    fn new(zone_id: ZoneID, idx: u8) -> Self {
        Self { zone_id, idx }
    }

    fn from_parts(parts: TestAddrParts) -> StdResult<Self, String> {
        if parts.address_type != "local" {
            return Err(format!(
                "expected 'local' address type, got '{}'",
                parts.address_type
            ));
        }
        if let Some(port) = parts.port {
            return Err(format!("expected no port information, got '{}'", port));
        }

        if parts.fields.len() != 2 {
            return Err(format!(
                "expected exactly 2 fields, got {}",
                parts.fields.len()
            ));
        }

        field_name_check(&parts.fields[0].0, "zone-id", "first")?;
        let zone_id = parts.fields[0]
            .1
            .parse()
            .map_err(|e| format!("failed to get zone ID from first field: {}", e))?;
        field_name_check(&parts.fields[1].0, "idx", "second")?;
        let idx = parts.fields[1]
            .1
            .parse()
            .map_err(|e| format!("failed to get index from second field: {}", e))?;

        Ok(Self::new(zone_id, idx))
    }

    fn get_zone_id(&self) -> ZoneID {
        self.zone_id
    }

    fn get_valid_port() -> u16 {
        85
    }

    fn validate_port(port: u16) -> StdResult<u16, String> {
        let valid_port = Self::get_valid_port();
        if port == valid_port {
            Ok(valid_port)
        } else {
            Err(format!(
                "expected port for local address to be '{}', got '{}'",
                valid_port, port
            ))
        }
    }
}

impl Display for TestLocalAddr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "[local]{{zone-id:{},idx:{}}}", self.zone_id, self.idx)
    }
}

impl FromStr for TestLocalAddr {
    type Err = TestAddrParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let parts = s.parse::<TestAddrParts>()?;

        Self::from_parts(parts)
            .map_err(|e| Self::Err::new(s, format!("badly formed local address: {}", e), 0))
    }
}

impl MyFromStr for TestLocalAddr {
    type MyErr = <Self as FromStr>::Err;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TestPersistentMappingAddr {
    parent_zone_id: ZoneID,
    child_zone_id: ZoneID,
}

impl TestPersistentMappingAddr {
    fn new(parent_zone_id: ZoneID, child_zone_id: ZoneID) -> Self {
        Self {
            parent_zone_id,
            child_zone_id,
        }
    }

    fn from_parts(parts: TestAddrParts) -> StdResult<Self, String> {
        if parts.address_type != "perm-map" {
            return Err(format!(
                "expected 'perm-map' address type, got '{}'",
                parts.address_type
            ));
        }
        if let Some(port) = parts.port {
            return Err(format!("expected no port information, got '{}'", port));
        }

        if parts.fields.len() != 2 {
            return Err(format!(
                "expected exactly 2 fields, got {}",
                parts.fields.len()
            ));
        }

        field_name_check(&parts.fields[0].0, "parent-zone-id", "first")?;
        let parent_zone_id = parts.fields[0]
            .1
            .parse()
            .map_err(|e| format!("failed to get parent zone ID from first field: {}", e))?;
        field_name_check(&parts.fields[1].0, "child-zone-id", "second")?;
        let child_zone_id = parts.fields[1]
            .1
            .parse()
            .map_err(|e| format!("failed to get child zone ID from second field: {}", e))?;

        Ok(Self::new(parent_zone_id, child_zone_id))
    }

    fn get_parent_zone_id(&self) -> ZoneID {
        self.parent_zone_id
    }

    fn get_child_zone_id(&self) -> ZoneID {
        self.child_zone_id
    }

    fn validate_port(port: u16) -> StdResult<u16, String> {
        if port <= u8::MAX.into() {
            Ok(port)
        } else {
            Err(format!("expected port to fit u8, got {}", port))
        }
    }
}

impl Display for TestPersistentMappingAddr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "[perm-map]{{parent-zone-id:{},child-zone-id:{}}}",
            self.parent_zone_id, self.child_zone_id
        )
    }
}

impl FromStr for TestPersistentMappingAddr {
    type Err = TestAddrParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let parts = s.parse::<TestAddrParts>()?;

        Self::from_parts(parts).map_err(|e| {
            Self::Err::new(
                s,
                format!("badly formed persistent mapping address: {}", e),
                0,
            )
        })
    }
}

impl MyFromStr for TestPersistentMappingAddr {
    type MyErr = <Self as FromStr>::Err;
}

/*
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct TestTemporaryMappingAddr {
    parent_zone_id: ZoneID,
    parent_server_idx: u8,
    child_zone_id: ZoneID,
    child_server_idx: u8,
    random_value: u16,
}

impl TestTemporaryMappingAddr {
    fn new(
        parent_zone_id: ZoneID,
        parent_server_idx: u8,
        child_zone_id: ZoneID,
        child_server_idx: u8,
        random_value: u16,
    ) -> Self {
        Self {
            parent_zone_id,
            parent_server_idx,
            child_zone_id,
            child_server_idx,
            random_value,
        }
    }

    fn from_parts(parts: TestAddrParts) -> StdResult<Self, String> {
        if parts.address_type != "temp-map" {
            return Err(format!(
                "expected 'temp-map' address type, got '{}'",
                parts.address_type
            ));
        }
        if let Some(port) = parts.port {
            return Err(format!("expected no port information, got '{}'", port));
        }

        if parts.fields.len() != 5 {
            return Err(format!(
                "expected exactly 5 fields, got {}",
                parts.fields.len()
            ));
        }

        field_name_check(&parts.fields[0].0, "parent-zone-id", "first")?;
        let parent_zone_id = parts.fields[0]
            .1
            .parse()
            .map_err(|e| format!("failed to get parent zone ID from first field: {}", e))?;
        field_name_check(&parts.fields[1].0, "parent-server-idx", "second")?;
        let parent_server_idx = parts.fields[1]
            .1
            .parse()
            .map_err(|e| format!("failed to get parent server index from second field: {}", e))?;
        field_name_check(&parts.fields[2].0, "child-zone-id", "third")?;
        let child_zone_id = parts.fields[2]
            .1
            .parse()
            .map_err(|e| format!("failed to get child zone ID from third field: {}", e))?;
        field_name_check(&parts.fields[3].0, "child-server-idx", "fourth")?;
        let child_server_idx = parts.fields[3]
            .1
            .parse()
            .map_err(|e| format!("failed to get child server index from fourth field: {}", e))?;
        field_name_check(&parts.fields[4].0, "random-value", "fifth")?;
        let random_value = parts.fields[4]
            .1
            .parse()
            .map_err(|e| format!("failed to get random u16 value from fifth field: {}", e))?;

        Ok(Self::new(
            parent_zone_id,
            parent_server_idx,
            child_zone_id,
            child_server_idx,
            random_value,
        ))
    }

    fn get_parent_zone_id(&self) -> ZoneID {
        self.parent_zone_id
    }

    fn validate_port(port: u16) -> StdResult<u16, String> {
        Ok(port)
    }
}

impl Display for TestTemporaryMappingAddr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "[temp-map]{{parent-zone-id:{},parent-server-idx:{},child-zone-id:{},child-server-idx:{},random-value:{}}}",
            self.parent_zone_id,
            self.parent_server_idx,
            self.child_zone_id,
            self.child_server_idx,
            self.random_value
        )
    }
}

impl FromStr for TestTemporaryMappingAddr {
    type Err = TestAddrParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let parts = s.parse::<TestAddrParts>()?;

        Self::from_parts(parts).map_err(|e| {
            Self::Err::new(
                s,
                format!("badly formed temporary mapping address: {}", e),
                0,
            )
        })
    }
}

impl MyFromStr for TestTemporaryMappingAddr {
    type MyErr = <Self as FromStr>::Err;
}
*/

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum TestAddr {
    //Public(TestPublicAddr),
    Local(TestLocalAddr),
    PersistentMapping(TestPersistentMappingAddr),
    //TemporaryMapping(TestTemporaryMappingAddr),
}

impl TestAddr {
    fn from_parts(parts: TestAddrParts) -> StdResult<Self, String> {
        match parts.address_type.as_str() {
            //"public" => TestPublicAddr::from_parts(parts).map(|a| TestAddr::Public(a)),
            "local" => TestLocalAddr::from_parts(parts).map(|a| TestAddr::Local(a)),
            "perm-map" => {
                TestPersistentMappingAddr::from_parts(parts).map(|a| TestAddr::PersistentMapping(a))
            }
            /*"temp-map" => {
                TestTemporaryMappingAddr::from_parts(parts).map(|a| TestAddr::TemporaryMapping(a))
            }*/
            _ => Err(format!("unknown address type '{}'", parts.address_type)),
        }
    }

    fn get_zone_id(&self) -> ZoneID {
        match self {
            //&TestAddr::Public(ref pip) => pip.get_zone_id(),
            &TestAddr::Local(ref lip) => lip.get_zone_id(),
            &TestAddr::PersistentMapping(ref pmip) => pmip.get_parent_zone_id(),
            //&TestAddr::TemporaryMapping(ref tmip) => tmip.get_parent_zone_id(),
        }
    }

    fn validate_port(&self, port: u16) -> StdResult<u16, String> {
        match self {
            //TestAddr::Public(_) => TestPublicAddr::validate_port(port),
            TestAddr::Local(_) => TestLocalAddr::validate_port(port),
            TestAddr::PersistentMapping(_) => TestPersistentMappingAddr::validate_port(port),
            //TestAddr::TemporaryMapping(_) => TestTemporaryMappingAddr::validate_port(port),
        }
    }
}

impl Display for TestAddr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let disp: &Display = match self {
            //TestAddr::Public(ref addr) => addr,
            TestAddr::Local(ref addr) => addr,
            TestAddr::PersistentMapping(ref addr) => addr,
            //TestAddr::TemporaryMapping(ref addr) => addr,
        };

        disp.fmt(f)
    }
}

impl FromStr for TestAddr {
    type Err = TestAddrParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let parts = s.parse::<TestAddrParts>()?;
        let address_type = parts.address_type.clone();

        Ok(Self::from_parts(parts).map_err(|e| {
            Self::Err::new(
                s,
                format!("badly formed address of type '{}': {}", address_type, e),
                0,
            )
        })?)
    }
}

impl MyFromStr for TestAddr {
    type MyErr = <Self as FromStr>::Err;
}

impl Address for TestAddr {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct TestAddrAndPort {
    addr: TestAddr,
    port: u16,
}

impl TestAddrAndPort {
    fn new(addr: TestAddr, port: u16) -> Self {
        assert!(addr.validate_port(port).is_ok());

        Self { addr, port }
    }

    pub fn get_zone_id(&self) -> ZoneID {
        self.addr.get_zone_id()
    }

    fn from_parts(mut parts: TestAddrParts) -> StdResult<Self, String> {
        let maybe_port = parts.port.take();
        let addr = TestAddr::from_parts(parts)?;
        let port = match maybe_port {
            Some(port) => addr.validate_port(port),
            None => Err(format!("missing port information")),
        }?;

        Ok(Self { addr, port })
    }
}

impl Display for TestAddrAndPort {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}:{}", self.addr, self.port)
    }
}

impl FromStr for TestAddrAndPort {
    type Err = TestAddrParseError;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        let parts = s.parse::<TestAddrParts>()?;
        let address_type = parts.address_type.clone();

        Ok(Self::from_parts(parts).map_err(|e| {
            Self::Err::new(
                s,
                format!(
                    "badly formed address and port of type '{}': {}",
                    address_type, e
                ),
                0,
            )
        })?)
    }
}

impl MyFromStr for TestAddrAndPort {
    type MyErr = <Self as FromStr>::Err;
}

impl AddressAndPort for TestAddrAndPort {
    type Address = TestAddr;

    fn new_from_address_and_port(addr: TestAddr, port: u16) -> Self {
        Self { addr, port }
    }

    fn get_address(&self) -> TestAddr {
        self.addr
    }

    fn get_port(&self) -> u16 {
        self.port
    }
}

fn create_member_from_addr(addr: TestAddrAndPort) -> Member {
    let mut member = Member::default();
    let port = addr.get_port();
    member.address = format!("{}", addr.get_address());
    member.swim_port = port;
    member.gossip_port = port;
    member
}

// TalkTarget is a trait used for types that can be talked to. It is
// basically about establishing a ring with SWIM messages.
trait TalkTarget {
    fn create_member_info(&self) -> Member;
}

// TestServer is a (thin) wrapper around the butterfly server.
#[derive(Clone)]
struct TestServer {
    butterfly: Server<TestNetwork>,
    addr: TestAddrAndPort,
    idx: usize,
    zone_id: ZoneID,
}

impl TestServer {
    pub fn talk_to(&self, talk_targets: &[&TalkTarget]) {
        let members = talk_targets
            .iter()
            .map(|tt| tt.create_member_info())
            .collect();

        self.butterfly.member_list.set_initial_members(members);
    }
}

impl TalkTarget for TestServer {
    fn create_member_info(&self) -> Member {
        let addr = self.butterfly.read_network().get_swim_addr();
        create_member_from_addr(addr)
    }
}

type ZoneToCountMap = HashMap<ZoneID, u8>;

struct Addresses {
    server_map: ZoneToCountMap,
    mapping_map: ZoneToCountMap,
}

impl Addresses {
    pub fn new() -> Self {
        Self {
            server_map: HashMap::new(),
            mapping_map: HashMap::new(),
        }
    }

    /*
    pub fn generate_public_address_for_server(&mut self, zone_id: ZoneID) -> TestAddrAndPort {
        let idx = self.get_server_idx(zone_id);
        let addr = TestAddr::Public(TestPublicAddr::new(zone_id, idx));

        TestAddrAndPort::new(addr, TestPublicAddr::get_valid_port())
    }
    */

    pub fn generate_address_for_server(&mut self, zone_id: ZoneID) -> TestAddrAndPort {
        let idx = self.get_server_idx(zone_id);
        let addr = TestAddr::Local(TestLocalAddr::new(zone_id, idx));

        TestAddrAndPort::new(addr, TestLocalAddr::get_valid_port())
    }

    pub fn generate_persistent_mapping_address(
        &mut self,
        parent_zone_id: ZoneID,
        child_zone_id: ZoneID,
    ) -> TestAddrAndPort {
        let idx = self.get_mapping_idx(parent_zone_id);
        let addr = TestAddr::PersistentMapping(TestPersistentMappingAddr::new(
            parent_zone_id,
            child_zone_id,
        ));

        TestAddrAndPort::new(addr, idx.into())
    }

    fn get_server_idx(&mut self, zone_id: ZoneID) -> u8 {
        Self::get_next_idx_for_zone(&mut self.server_map, zone_id)
    }

    fn get_mapping_idx(&mut self, zone_id: ZoneID) -> u8 {
        Self::get_next_idx_for_zone(&mut self.mapping_map, zone_id)
    }

    fn get_next_idx_for_zone(map: &mut ZoneToCountMap, zone_id: ZoneID) -> u8 {
        match map.entry(zone_id) {
            Entry::Vacant(v) => {
                v.insert(1);
                1
            }
            Entry::Occupied(mut o) => {
                let value = o.get_mut();
                *value += 1;
                *value
            }
        }
    }
}

// TestMessage is a wrapper around the SWIM or gossip message sent by
// a butterfly server. Contains source and destination addresses used
// to determine a routing.
struct TestMessage {
    source: TestAddrAndPort,
    target: TestAddrAndPort,
    bytes: Vec<u8>,
    dbg_key: Option<SymKey>,
    channel_type: ChannelType,
}

impl Debug for TestMessage {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut debug_struct = f.debug_struct("TestMessage");

        match message::unwrap_wire(&self.bytes, self.dbg_key.as_ref()) {
            Ok(payload) => match self.channel_type {
                ChannelType::SWIM => match Swim::decode(&payload) {
                    Ok(parsed) => debug_struct.field("protobuf", &parsed),
                    Err(e) => debug_struct
                        .field("parse_error", &e)
                        .field("invalid-payload", &payload),
                },
                ChannelType::Gossip => match RumorEnvelope::decode(&payload) {
                    Ok(parsed) => debug_struct.field("protobuf", &parsed),
                    Err(e) => debug_struct
                        .field("parse_error", &e)
                        .field("invalid-payload", &payload),
                },
            },
            Err(e) => debug_struct
                .field("decrypt_error", &e)
                .field("bytes", &self.bytes),
        }.field("source", &self.source)
            .field("target", &self.target)
            .field("channel_type", &self.channel_type)
            .finish()
    }
}

// LockedSender is a convenience struct to make mpsc::Sender fulfill
// the Send + Sync traits.
#[derive(Debug)]
struct LockedSender<T> {
    sender: Mutex<Sender<T>>,
}

impl<T> LockedSender<T> {
    pub fn new(sender: Sender<T>) -> Self {
        Self {
            sender: Mutex::new(sender),
        }
    }

    pub fn send(&self, t: T) -> StdResult<(), SendError<T>> {
        self.get_sender_guard().send(t)
    }

    pub fn cloned_sender(&self) -> Sender<T> {
        self.get_sender_guard().clone()
    }

    fn get_sender_guard(&self) -> MutexGuard<Sender<T>> {
        self.sender.lock().expect("Sender lock is poisoned")
    }
}

// ChannelMap is a mapping from IP address to an mpsc::Sender.
type ChannelMap = HashMap<TestAddrAndPort, LockedSender<TestMessage>>;

#[derive(Copy, Clone, Debug)]
enum ChannelType {
    SWIM,
    Gossip,
}

type AddrToAddrMap = HashMap<TestAddrAndPort, TestAddrAndPort>;

struct Mappings {
    hole_to_internal: AddrToAddrMap,
    internal_to_hole: AddrToAddrMap,
}

impl Mappings {
    pub fn new() -> Self {
        Self {
            hole_to_internal: HashMap::new(),
            internal_to_hole: HashMap::new(),
        }
    }

    pub fn insert_both_ways(&mut self, hole: TestAddrAndPort, internal: TestAddrAndPort) {
        match self.hole_to_internal.entry(hole) {
            Entry::Vacant(v) => {
                v.insert(internal);
            }
            Entry::Occupied(_) => {
                unreachable!("mapping for hole {:?} already taken", hole);
            }
        };
        match self.internal_to_hole.entry(internal) {
            Entry::Vacant(v) => {
                v.insert(hole);
            }
            Entry::Occupied(_) => {
                unreachable!("mapping for internal {:?} already taken", internal);
            }
        };
    }

    pub fn hole_to_internal(&self, hole: TestAddrAndPort) -> Option<TestAddrAndPort> {
        Self::get_from_map(&self.hole_to_internal, hole)
    }

    pub fn internal_to_hole(&self, internal: TestAddrAndPort) -> Option<TestAddrAndPort> {
        Self::get_from_map(&self.internal_to_hole, internal)
    }

    fn get_from_map(map: &AddrToAddrMap, addr: TestAddrAndPort) -> Option<TestAddrAndPort> {
        map.get(&addr).cloned()
    }
}

#[derive(Copy, Clone)]
struct NatHole {
    addr: TestAddrAndPort,
}

impl NatHole {
    pub fn new(addr: TestAddrAndPort) -> Self {
        Self { addr }
    }

    pub fn into_additional_address(self) -> AdditionalAddress<TestAddr> {
        AdditionalAddress {
            address: Some(self.addr.addr),
            swim_port: self.addr.port,
            gossip_port: self.addr.port,
        }
    }
}

impl TalkTarget for NatHole {
    fn create_member_info(&self) -> Member {
        create_member_from_addr(self.addr)
    }
}

#[derive(Clone)]
struct TestNat {
    parent_id: ZoneID,
    child_id: ZoneID,
    addresses: Arc<Mutex<Addresses>>,
    mappings: Arc<RwLock<Mappings>>,
}

impl TestNat {
    pub fn new(parent_id: ZoneID, child_id: ZoneID, addresses: Arc<Mutex<Addresses>>) -> Self {
        let mappings = Arc::new(RwLock::new(Mappings::new()));
        Self {
            parent_id,
            child_id,
            addresses,
            mappings,
        }
    }

    pub fn punch_hole(&mut self) -> NatHole {
        NatHole::new(
            self.get_addresses_guard()
                .generate_persistent_mapping_address(self.parent_id, self.child_id),
        )
    }

    pub fn make_route(&mut self, hole: NatHole, internal: TestAddrAndPort) {
        assert_eq!(hole.addr.get_zone_id(), self.parent_id);
        assert_eq!(internal.get_zone_id(), self.child_id);

        self.write_mappings().insert_both_ways(hole.addr, internal);
    }

    pub fn can_route(&self, msg: &mut TestMessage, ti: &TraversalInfo) -> bool {
        if ti.direction == Direction::ParentToChild {
            return false;
        }

        {
            let mappings = self.read_mappings();
            if let Some(hole) = mappings.internal_to_hole(msg.source) {
                msg.source = hole;
                return true;
            }
        }

        return false;
    }

    pub fn route(&self, msg: &mut TestMessage) -> bool {
        let mappings = self.read_mappings();
        if let Some(internal) = mappings.hole_to_internal(msg.target) {
            msg.target = internal;
            return true;
        }

        return false;
    }

    fn get_addresses_guard(&self) -> MutexGuard<Addresses> {
        self.addresses.lock().expect("Addresses lock is poisoned")
    }

    fn write_mappings(&self) -> RwLockWriteGuard<Mappings> {
        self.mappings.write().expect("Mappings lock is poisoned")
    }

    fn read_mappings(&self) -> RwLockReadGuard<Mappings> {
        self.mappings.read().expect("Mappings lock is poisoned")
    }
}

#[derive(Debug)]
struct TestSuitability(u64);
impl Suitability for TestSuitability {
    fn get(&self, _service_group: &ServiceGroup) -> u64 {
        self.0
    }
}

#[derive(PartialEq, Eq, Hash)]
struct NatsKey {
    first: ZoneID,
    second: ZoneID,
}

impl NatsKey {
    pub fn new(z1: ZoneID, z2: ZoneID) -> Self {
        let (first, second) = if z1.raw() < z2.raw() {
            (z1, z2)
        } else {
            (z2, z1)
        };
        Self { first, second }
    }
}

type NatsMap = HashMap<NatsKey, TestNat>;

#[derive(Copy, Clone)]
enum SettledZones {
    Unique,
    AllTheSame,
}

impl SettledZones {
    fn from_bool(b: bool) -> SettledZones {
        if b {
            SettledZones::Unique
        } else {
            SettledZones::AllTheSame
        }
    }
}

// TestNetworkSwitchBoard implements the multizone setup for testing
// the spanning ring.
#[derive(Clone)]
struct TestNetworkSwitchBoard {
    zones: Arc<RwLock<ZoneMap>>,
    servers: Arc<RwLock<Vec<TestServer>>>,
    addresses: Arc<Mutex<Addresses>>,
    swim_channel_map: Arc<RwLock<ChannelMap>>,
    gossip_channel_map: Arc<RwLock<ChannelMap>>,
    nats: Arc<RwLock<NatsMap>>,
}

impl TestNetworkSwitchBoard {
    pub fn new() -> Self {
        Self {
            zones: Arc::new(RwLock::new(ZoneMap(HashMap::new()))),
            servers: Arc::new(RwLock::new(Vec::new())),
            addresses: Arc::new(Mutex::new(Addresses::new())),
            swim_channel_map: Arc::new(RwLock::new(HashMap::new())),
            gossip_channel_map: Arc::new(RwLock::new(HashMap::new())),
            nats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn setup_nat(&self, parent_id: ZoneID, child_id: ZoneID) -> TestNat {
        {
            let mut zones = self.write_zones();
            zones.setup_zone_relationship(parent_id, child_id);
        }
        let nat = TestNat::new(parent_id, child_id, Arc::clone(&self.addresses));
        let nats_key = NatsKey::new(child_id, parent_id);
        {
            let mut nats = self.write_nats();
            assert!(
                !nats.contains_key(&nats_key),
                "nat between zone {} and zone {} was already registered",
                child_id,
                parent_id,
            );
            nats.insert(nats_key, nat.clone());
        }
        nat
    }

    pub fn start_server_in_zone(&self, zone_id: ZoneID) -> TestServer {
        self.start_server_in_zone_with_additional_addresses(zone_id, HashMap::new())
    }

    pub fn start_server_in_zone_with_additional_addresses(
        &self,
        zone_id: ZoneID,
        tagged_additional_addresses: TaggedAddressesFromAddress<TestAddr>,
    ) -> TestServer {
        let addr = {
            let mut addresses = self.get_addresses_guard();

            addresses.generate_address_for_server(zone_id)
        };
        self.start_server(addr, tagged_additional_addresses)
    }

    /*
    pub fn start_public_server_in_zone(&self, zone_id: ZoneID) -> TestServer {
        self.start_public_server_in_zone_with_additional_addresses(zone_id, HashMap::new())
    }
    */

    /*
    pub fn start_public_server_in_zone_with_additional_addresses(
        &self,
        zone_id: ZoneID,
        tagged_additional_addresses: TaggedAddressesFromAddress<TestAddr>,
    ) -> TestServer {
        let addr = {
            let mut addresses = self.get_addresses_guard();
            addresses.generate_public_address_for_server(zone_id)
        };
        self.start_server(addr, tagged_additional_addresses)
    }
    */

    pub fn wait_for_health_of_all(&self, health: Health) -> bool {
        let servers = self.read_servers().clone();
        let rounds = self.gossip_rounds_in(4);
        let servers_refs = servers.iter().collect::<Vec<_>>();

        self.wait_for_health_of(health, &servers_refs, &rounds)
    }

    pub fn wait_for_health_of_those<'a, T>(&self, health: Health, servers: T) -> bool
    where
        T: AsRef<[&'a TestServer]>,
    {
        let rounds = self.gossip_rounds_in(4);

        self.wait_for_health_of(health, servers.as_ref(), &rounds)
    }

    pub fn wait_for_same_settled_zone(&self, servers: &[&TestServer]) -> bool {
        self.wait_for_disjoint_settled_zones(&[servers])
    }

    pub fn wait_for_splitted_same_zone<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
    ) -> bool {
        self.wait_for_settled_zones(disjoint_servers, false)
    }

    pub fn wait_for_disjoint_settled_zones<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
    ) -> bool {
        self.wait_for_settled_zones(disjoint_servers, true)
    }

    fn wait_for_settled_zones<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
        full: bool,
    ) -> bool {
        self.dsz_assert_assumptions(disjoint_servers, SettledZones::from_bool(full));
        let rounds = self.gossip_rounds_in(4);
        let all_servers = {
            let mut all_servers = Vec::new();

            for servers in disjoint_servers {
                for server in servers.as_ref() {
                    all_servers.push(*server);
                }
            }

            all_servers
        };

        loop {
            if self.check_for_disjoint_settled_zones(disjoint_servers, full) {
                return true;
            }

            if self.reached_max_rounds(&all_servers, &rounds) {
                return false;
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn check_for_disjoint_settled_zones<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
        with_relationship_checks: bool,
    ) -> bool {
        if !self.dsz_check_equal_zones(disjoint_servers) {
            return false;
        }
        if !self.dsz_check_unique_zone_count(disjoint_servers) {
            return false;
        }
        if with_relationship_checks && !self.dsz_check_relationships(disjoint_servers) {
            return false;
        }

        true
    }

    fn dsz_assert_assumptions<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
        settled_zones: SettledZones,
    ) {
        let mut ids = Vec::new();
        let mut indices = Vec::new();

        assert!(!disjoint_servers.is_empty());
        for servers_to_ref in disjoint_servers {
            let servers = servers_to_ref.as_ref();

            assert!(!servers.is_empty());

            let zone_id = servers[0].zone_id;

            match settled_zones {
                SettledZones::Unique => {
                    assert!(!ids.contains(&zone_id));
                    ids.push(zone_id);
                }
                SettledZones::AllTheSame => {
                    if ids.is_empty() {
                        ids.push(zone_id);
                    } else {
                        assert!(ids[0] == zone_id);
                    }
                }
            }

            assert!(!indices.contains(&servers[0].idx));
            indices.push(servers[0].idx);
            for server in servers.iter().skip(1) {
                assert!(server.zone_id == zone_id);
                assert!(!indices.contains(&server.idx));
                indices.push(server.idx);
            }
        }
    }

    fn dsz_check_equal_zones<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
    ) -> bool {
        for servers in disjoint_servers {
            for pair in servers.as_ref().windows(2) {
                let s0 = &pair[0];
                let s1 = &pair[1];

                assert!(s0.zone_id == s1.zone_id);
                if !s0.butterfly.is_zone_settled() {
                    return false;
                }
                if !s1.butterfly.is_zone_settled() {
                    return false;
                }
                if s0.butterfly.get_settled_zone_id() != s1.butterfly.get_settled_zone_id() {
                    return false;
                }
            }
        }

        true
    }

    fn dsz_check_unique_zone_count<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
    ) -> bool {
        let mut zone_uuids = disjoint_servers
            .iter()
            .filter_map(|v| {
                v.as_ref()
                    .first()
                    .map(|s| s.butterfly.get_settled_zone_id())
            })
            .collect::<Vec<_>>();

        zone_uuids.sort_unstable();
        zone_uuids.dedup();
        zone_uuids.len() == disjoint_servers.len()
    }

    fn dsz_check_relationships<'a, T: AsRef<[&'a TestServer]>>(
        &self,
        disjoint_servers: &[T],
    ) -> bool {
        let all_test_and_real_zone_id_pairs = disjoint_servers
            .iter()
            .map(|servers_to_ref| {
                let servers = servers_to_ref.as_ref();

                (
                    servers[0].zone_id,
                    servers[0].butterfly.get_settled_zone_id(),
                )
            })
            .collect::<Vec<_>>();
        let mut dbg = DbgVec::new("relationship check".to_string());
        dbg.push(format!(
            "test zones and real zones: {:#?}",
            all_test_and_real_zone_id_pairs
        ));

        for servers in disjoint_servers {
            let this_test_zone_id = servers.as_ref()[0].zone_id;
            let (maybe_real_parent_zone_id, real_my_own_zone_id, real_child_zone_ids) = self
                .dsz_get_related_real_zone_ids(this_test_zone_id, &all_test_and_real_zone_id_pairs);
            let set = real_child_zone_ids.iter().cloned().collect::<HashSet<_>>();

            dbg.push(format!(
                "real ids for test id {}, parent: {:#?}, my own: {:#?}, children: {:#?}",
                this_test_zone_id,
                maybe_real_parent_zone_id,
                real_my_own_zone_id,
                real_child_zone_ids
            ));

            for server in servers.as_ref() {
                let zone_list = server.butterfly.read_zone_list();
                let zone = match zone_list.zones.get(&real_my_own_zone_id) {
                    Some(zone) => zone,
                    None => {
                        dbg.push(format!("no info about own zone?"));
                        return false;
                    }
                };
                dbg.push(format!(
                    "zone for {}({}): {:#?}",
                    server.idx,
                    server.butterfly.member_id(),
                    zone
                ));

                match (&maybe_real_parent_zone_id, zone.parent_zone_id) {
                    (&Some(ref id), Some(ref parent_id)) => {
                        if *id != *parent_id {
                            dbg.push(format!(
                                "expected parent zone id to be {}, got {}",
                                id, parent_id
                            ));
                            if let Some(parent_zone) = zone_list.zones.get(id) {
                                dbg.push(format!("expected parent zone: {:#?}", parent_zone));
                            } else {
                                dbg.push("no info about expected parent zone");
                            }
                            if let Some(parent_zone) = zone_list.zones.get(parent_id) {
                                dbg.push(format!("actual parent zone: {:#?}", parent_zone));
                            } else {
                                dbg.push("no info about actual parent zone");
                            }
                            return false;
                        }
                    }
                    (None, None) => {}
                    (_, _) => {
                        dbg.push(format!(
                            "mismatch between having and not having a parent ({:?} vs {:?})",
                            maybe_real_parent_zone_id, zone.parent_zone_id
                        ));
                        return false;
                    }
                }

                let zone_set = zone.child_zone_ids.iter().cloned().collect::<HashSet<_>>();

                if set.symmetric_difference(&zone_set).next().is_some() {
                    dbg.push("child zones of the server:");
                    for zone_id in zone_set {
                        if let Some(child_zone) = zone_list.zones.get(&zone_id) {
                            dbg.push(format!("{:#?}", child_zone));
                        } else {
                            dbg.push(format!("no info about child zone {}", zone_id));
                        }
                    }

                    dbg.push("expected child zones:");
                    for zone_id in set {
                        if let Some(child_zone) = zone_list.zones.get(&zone_id) {
                            dbg.push(format!("{:#?}", child_zone));
                        } else {
                            dbg.push(format!("no info about child zone {}", zone_id));
                        }
                    }
                    return false;
                }
            }
        }

        true
    }

    fn dsz_get_related_real_zone_ids(
        &self,
        this_test_zone_id: ZoneID,
        all_test_and_real_zone_id_pairs: &[(ZoneID, BfUuid)],
    ) -> (Option<BfUuid>, BfUuid, Vec<BfUuid>) {
        let zone_map = self.read_zones();
        let mut maybe_parent = None;
        let mut my_own = BfUuid::nil();
        let mut children = Vec::new();

        for pair in all_test_and_real_zone_id_pairs {
            if this_test_zone_id == pair.0 {
                assert!(my_own.is_nil());
                my_own = pair.1;
                continue;
            }
            if zone_map.is_zone_child_of(this_test_zone_id, pair.0) {
                assert!(maybe_parent.is_none());
                maybe_parent = Some(pair.1);
                continue;
            }
            if zone_map.is_zone_child_of(pair.0, this_test_zone_id) {
                children.push(pair.1);
            }
        }

        (maybe_parent, my_own, children)
    }

    fn start_server(
        &self,
        addr: TestAddrAndPort,
        tagged_additional_addresses: TaggedAddressesFromAddress<TestAddr>,
    ) -> TestServer {
        let network = self.create_test_network(addr);
        let mut servers = self.write_servers();
        let idx = servers.len();
        let server = self.create_test_server(network, idx, tagged_additional_addresses);
        servers.push(server.clone());
        server
    }

    fn create_test_network(&self, addr: TestAddrAndPort) -> TestNetwork {
        let (swim_in, swim_out) = self.start_routing_thread(addr, ChannelType::SWIM);
        let (gossip_in, gossip_out) = self.start_routing_thread(addr, ChannelType::Gossip);
        TestNetwork::new(addr, swim_in, swim_out, gossip_in, gossip_out, None)
    }

    fn create_test_server(
        &self,
        network: TestNetwork,
        idx: usize,
        tagged_additional_addresses: TaggedAddressesFromAddress<TestAddr>,
    ) -> TestServer {
        let addr = network.get_addr();
        let member = create_member_from_addr(addr);
        let trace = Trace::default();
        let ring_key = None;
        let name = None;
        let data_path = None::<PathBuf>;
        let suitability = Box::new(TestSuitability(idx as u64));
        let host_address = network
            .get_host_address()
            .expect("failed to get host address");
        let mut butterfly = Server::new(
            network,
            host_address,
            member,
            trace,
            ring_key,
            name,
            data_path,
            suitability,
        );
        let timing = Timing::default();
        let zone_id = addr.get_zone_id();

        butterfly.merge_additional_addresses(tagged_additional_addresses);
        butterfly.start(timing).expect("failed to start server");

        TestServer {
            butterfly,
            addr,
            idx,
            zone_id,
        }
    }

    fn wait_for_health_of(
        &self,
        health: Health,
        servers: &[&TestServer],
        rounds: &Vec<isize>,
    ) -> bool {
        for lserver in servers {
            for rserver in servers {
                if lserver.idx == rserver.idx {
                    continue;
                }
                if !self.wait_for_health_of_pair(health, lserver, rserver, &rounds) {
                    return false;
                }
                if !self.wait_for_health_of_pair(health, rserver, lserver, &rounds) {
                    return false;
                }
            }
        }
        true
    }

    fn wait_for_health_of_pair(
        &self,
        health: Health,
        from: &TestServer,
        to: &TestServer,
        rounds: &[isize],
    ) -> bool {
        loop {
            if let Some(member_health) = self.health_of_pair(from, to) {
                if member_health == health {
                    return true;
                }
            }
            if self.reached_max_rounds(&vec![from, to], rounds) {
                return false;
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

    pub fn reached_max_rounds(&self, servers: &[&TestServer], rounds: &[isize]) -> bool {
        for server in servers {
            if server.butterfly.paused() || server.butterfly.swim_rounds() > rounds[server.idx] {
                continue;
            }
            return false;
        }
        true
    }

    fn health_of_pair(&self, from: &TestServer, to: &TestServer) -> Option<Health> {
        let maybe_health = from
            .butterfly
            .member_list
            .health_of(&to.butterfly.read_member());

        debug!("{} sees {} as {:?}", from.addr, to.addr, maybe_health,);

        maybe_health
    }

    fn gossip_rounds_in(&self, count: isize) -> Vec<isize> {
        self.gossip_rounds().iter().map(|r| r + count).collect()
    }

    fn gossip_rounds(&self) -> Vec<isize> {
        let servers = self.read_servers();

        servers
            .iter()
            .map(|s| s.butterfly.gossip_rounds())
            .collect()
    }

    fn start_routing_thread(
        &self,
        addr: TestAddrAndPort,
        channel_type: ChannelType,
    ) -> (Sender<TestMessage>, Receiver<TestMessage>) {
        let (msg_in, msg_mid_out) = mpsc::channel::<TestMessage>();
        let (msg_mid_in, msg_out) = mpsc::channel::<TestMessage>();
        {
            let mut channel_map = self.write_channel_map(channel_type);
            channel_map.insert(addr, LockedSender::new(msg_mid_in));
        }
        let self_for_thread = self.clone();
        thread::spawn(move || loop {
            match msg_mid_out.recv() {
                Ok(msg) => self_for_thread.process_msg(msg, channel_type),
                Err(_) => break,
            }
        });

        (msg_in, msg_out)
    }

    fn process_msg(&self, mut msg: TestMessage, channel_type: ChannelType) {
        let src = msg.source;
        let tgt = msg.target;
        let source_zone_id = match &msg.source.addr {
            //&TestAddr::Public(ref pip) => pip.get_zone_id(),
            &TestAddr::Local(ref lip) => lip.get_zone_id(),
            _ => {
                unreachable!(
                    "expected source address to be either local or public, but it is {:?}",
                    msg.source,
                );
            }
        };

        let can_route_across_zones = {
            /*
            if let &TestAddr::Public(_) = &msg.target.addr {
                true
            } else
            */
            {
                let target_zone_id = msg.target.addr.get_zone_id();
                let zone_map = self.read_zones();
                let nats = self.read_nats();
                if let Some(route) = zone_map.get_route(source_zone_id, target_zone_id) {
                    let mut can_route = true;
                    for traversal_info in route {
                        let nats_key = NatsKey::new(traversal_info.from, traversal_info.to);
                        if let Some(nat) = nats.get(&nats_key) {
                            if !nat.can_route(&mut msg, &traversal_info) {
                                can_route = false;
                                debug!("ROUTING: can't route {:#?} from {} to {}, there is a route, but it's unreachable", msg, src, tgt);
                                break;
                            }
                        } else {
                            can_route = false;
                            debug!("ROUTING: can't route {:#?} from {} to {}, there is a route, but no nat registered", msg, src, tgt);
                            break;
                        }
                    }
                    can_route
                } else {
                    debug!(
                        "ROUTING: can't route {:#?} from {} to {}, no route",
                        msg, src, tgt
                    );
                    false
                }
            }
        };
        let routed = if can_route_across_zones {
            let mut routed = true;
            loop {
                let nats = self.read_nats();
                match msg.target.addr {
                    TestAddr::PersistentMapping(m) => {
                        let parent_id = m.get_parent_zone_id();
                        let child_id = m.get_child_zone_id();
                        let nats_key = NatsKey::new(parent_id, child_id);
                        if let Some(nat) = nats.get(&nats_key) {
                            routed = nat.route(&mut msg);
                            if !routed {
                                debug!(
                                    "ROUTING: can't {:#?} route from {} to {}, no mapping",
                                    msg, src, tgt
                                );
                                break;
                            }
                        } else {
                            debug!(
                                "ROUTING: can't route {:#?} from {} to {}, no registered nat",
                                msg, src, tgt
                            );
                            routed = false;
                        }
                    }
                    /*
                    TestAddr::TemporaryMapping(_) => {
                        unreachable!("not implemented yet");
                    }
                    */
                    _ => break,
                }
            }
            routed
        } else {
            false
        };
        if routed {
            let maybe_out = {
                let map = self.read_channel_map(channel_type);
                map.get(&msg.target).map(|l| l.cloned_sender())
            };
            if let Some(out) = maybe_out {
                let target = msg.target;
                if out.send(msg).is_err() {
                    let mut map = self.write_channel_map(channel_type);
                    map.remove(&target);
                }
            } else {
                debug!(
                    "ROUTING: couldn't send a message {:#?} from {} to {}, no out",
                    msg, src, tgt
                );
            }
        }
    }

    fn read_zones(&self) -> RwLockReadGuard<ZoneMap> {
        self.zones.read().expect("Zone map lock is poisoned")
    }

    fn write_zones(&self) -> RwLockWriteGuard<ZoneMap> {
        self.zones.write().expect("Zone map lock is poisoned")
    }

    fn get_addresses_guard(&self) -> MutexGuard<Addresses> {
        self.addresses.lock().expect("Addresses lock is poisoned")
    }

    fn read_servers(&self) -> RwLockReadGuard<Vec<TestServer>> {
        self.servers.read().expect("Servers lock is poisoned")
    }

    fn write_servers(&self) -> RwLockWriteGuard<Vec<TestServer>> {
        self.servers.write().expect("Servers lock is poisoned")
    }

    fn read_channel_map(&self, channel_type: ChannelType) -> RwLockReadGuard<ChannelMap> {
        self.get_channel_map_lock(channel_type)
            .read()
            .expect("Channel map lock is poisoned")
    }

    fn write_channel_map(&self, channel_type: ChannelType) -> RwLockWriteGuard<ChannelMap> {
        self.get_channel_map_lock(channel_type)
            .write()
            .expect("Channel map lock is poisoned")
    }

    fn get_channel_map_lock(&self, channel_type: ChannelType) -> &RwLock<ChannelMap> {
        match channel_type {
            ChannelType::SWIM => &self.swim_channel_map,
            ChannelType::Gossip => &self.gossip_channel_map,
        }
    }

    fn read_nats(&self) -> RwLockReadGuard<NatsMap> {
        self.nats.read().expect("Nats lock is poisoned")
    }

    fn write_nats(&self) -> RwLockWriteGuard<NatsMap> {
        self.nats.write().expect("Nats lock is poisoned")
    }
}

// TestSwimSender is an implementation of a SwimSender trait based on
// channels.
#[derive(Debug)]
struct TestSwimSender {
    addr: TestAddrAndPort,
    sender: LockedSender<TestMessage>,
    dbg_key: Option<SymKey>,
}

impl SwimSender<TestAddrAndPort> for TestSwimSender {
    fn send(&self, buf: &[u8], addr: TestAddrAndPort) -> Result<usize> {
        let msg = TestMessage {
            source: self.addr,
            target: addr,
            bytes: buf.to_owned(),
            dbg_key: self.dbg_key.clone(),
            channel_type: ChannelType::SWIM,
        };
        self.sender.send(msg).map_err(|_| {
            Error::SwimSendError("Receiver part of the channel is disconnected".to_owned())
        })?;
        Ok(buf.len())
    }
}

// TestSwimReceiver is an implementation of a SwimReceiver trait based
// on channels.
struct TestSwimReceiver(Receiver<TestMessage>);

impl SwimReceiver<TestAddrAndPort> for TestSwimReceiver {
    fn receive(&self, buf: &mut [u8]) -> Result<(usize, TestAddrAndPort)> {
        let msg = self.0.recv().map_err(|_| {
            Error::SwimReceiveError("Sender part of the channel is disconnected".to_owned())
        })?;
        if buf.len() < msg.bytes.len() {
            panic!(
                "allowed buffer has length {}, but message from {} to {} is larger ({})",
                buf.len(),
                msg.source,
                msg.target,
                msg.bytes.len(),
            );
        }
        buf[..msg.bytes.len()].copy_from_slice(&msg.bytes);
        Ok((msg.bytes.len(), msg.source))
    }
}

// TestGossipSender is an implementation of a GossipSender trait based
// on channels.
struct TestGossipSender {
    source: TestAddrAndPort,
    target: TestAddrAndPort,
    sender: Sender<TestMessage>,
    dbg_key: Option<SymKey>,
}

impl GossipSender for TestGossipSender {
    fn send(&self, buf: &[u8]) -> Result<()> {
        let msg = TestMessage {
            source: self.source,
            target: self.target,
            bytes: buf.to_vec(),
            dbg_key: self.dbg_key.clone(),
            channel_type: ChannelType::Gossip,
        };
        self.sender.send(msg).map_err(|_| {
            Error::GossipSendError("Receiver part of the channel is disconnected".to_owned())
        })?;
        Ok(())
    }
}

// TestGossipReceiver is an implementation of a GossipReceiver trait
// based on channels.
struct TestGossipReceiver(Receiver<TestMessage>);

impl GossipReceiver for TestGossipReceiver {
    fn receive(&self) -> Result<Vec<u8>> {
        let msg = self.0.recv().map_err(|_| {
            Error::GossipReceiveError("Sender part of the channel is disconnected".to_owned())
        })?;
        return Ok(msg.bytes);
    }
}

// TestNetwork is an implementation of a Network trait. It provides
// channel-based senders and receivers.
#[derive(Debug)]
struct TestNetwork {
    addr: TestAddrAndPort,
    swim_in: LockedSender<TestMessage>,
    swim_out: Mutex<Option<Receiver<TestMessage>>>,
    gossip_in: LockedSender<TestMessage>,
    gossip_out: Mutex<Option<Receiver<TestMessage>>>,
    dbg_key: Option<SymKey>,
}

impl TestNetwork {
    pub fn new(
        addr: TestAddrAndPort,
        swim_in: Sender<TestMessage>,
        swim_out: Receiver<TestMessage>,
        gossip_in: Sender<TestMessage>,
        gossip_out: Receiver<TestMessage>,
        dbg_key: Option<SymKey>,
    ) -> Self {
        Self {
            addr: addr,
            swim_in: LockedSender::new(swim_in),
            swim_out: Mutex::new(Some(swim_out)),
            gossip_in: LockedSender::new(gossip_in),
            gossip_out: Mutex::new(Some(gossip_out)),
            dbg_key: dbg_key,
        }
    }

    pub fn get_addr(&self) -> TestAddrAndPort {
        self.addr
    }
}

impl Network for TestNetwork {
    type AddressAndPort = TestAddrAndPort;
    type SwimSender = TestSwimSender;
    type SwimReceiver = TestSwimReceiver;
    type GossipSender = TestGossipSender;
    type GossipReceiver = TestGossipReceiver;

    fn get_host_address(&self) -> Result<TestAddr> {
        Ok(self.addr.get_address())
    }

    fn get_swim_addr(&self) -> TestAddrAndPort {
        self.addr
    }

    fn create_swim_sender(&self) -> Result<Self::SwimSender> {
        Ok(Self::SwimSender {
            addr: self.addr,
            sender: LockedSender::new(self.swim_in.cloned_sender()),
            dbg_key: self.dbg_key.clone(),
        })
    }

    fn create_swim_receiver(&self) -> Result<Self::SwimReceiver> {
        match self
            .swim_out
            .lock()
            .expect("SWIM receiver lock is poisoned")
            .take()
        {
            Some(receiver) => Ok(TestSwimReceiver(receiver)),
            None => Err(Error::SwimChannelSetupError(format!(
                "no test swim receiver, should not happen"
            ))),
        }
    }

    fn get_gossip_addr(&self) -> TestAddrAndPort {
        self.addr
    }

    fn create_gossip_sender(&self, addr: TestAddrAndPort) -> Result<Self::GossipSender> {
        Ok(Self::GossipSender {
            source: self.addr,
            target: addr,
            sender: self.gossip_in.cloned_sender(),
            dbg_key: self.dbg_key.clone(),
        })
    }

    fn create_gossip_receiver(&self) -> Result<Self::GossipReceiver> {
        match self
            .gossip_out
            .lock()
            .expect("Gossip receiver lock is poisoned")
            .take()
        {
            Some(receiver) => Ok(TestGossipReceiver(receiver)),
            None => Err(Error::SwimChannelSetupError(format!(
                "no test gossip receiver, should not happen"
            ))),
        }
    }
}
