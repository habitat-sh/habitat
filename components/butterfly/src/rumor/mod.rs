// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

//! Tracks rumors for distribution.
//!
//! Each rumor is represented by a `RumorKey`, which has a unique key and a "kind", which
//! represents what "kind" of rumor it is (for example, a "member").
//!
//! New rumors need to implement the `From` trait for `RumorKey`, and then can track the arrival of
//! new rumors, and dispatch them according to their `kind`.

pub mod dat_file;
pub mod departure;
pub mod election;
pub mod heat;
pub mod service;
pub mod service_config;
pub mod service_file;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Deref;
use std::result;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};

use bytes::BytesMut;
use prost::Message as ProstMessage;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

pub use self::departure::Departure;
pub use self::election::{Election, ElectionUpdate};
pub use self::service::Service;
pub use self::service_config::ServiceConfig;
pub use self::service_file::ServiceFile;
use error::{Error, Result};
use member::Membership;
pub use protocol::newscast::{Rumor as ProtoRumor, RumorPayload, RumorType};
use protocol::{FromProto, Message};
use zone::Zone;

#[derive(Debug, Clone, Serialize)]
pub enum RumorKind {
    Departure(Departure),
    Election(Election),
    ElectionUpdate(ElectionUpdate),
    Membership(Membership),
    Service(Service),
    ServiceConfig(ServiceConfig),
    ServiceFile(ServiceFile),
    Zone(Zone),
}

impl From<RumorKind> for RumorPayload {
    fn from(value: RumorKind) -> Self {
        match value {
            RumorKind::Departure(departure) => RumorPayload::Departure(departure.into()),
            RumorKind::Election(election) => RumorPayload::Election(election.into()),
            RumorKind::ElectionUpdate(election) => RumorPayload::Election(election.into()),
            RumorKind::Membership(membership) => RumorPayload::Member(membership.into()),
            RumorKind::Service(service) => RumorPayload::Service(service.into()),
            RumorKind::ServiceConfig(service_config) => {
                RumorPayload::ServiceConfig(service_config.into())
            }
            RumorKind::ServiceFile(service_file) => RumorPayload::ServiceFile(service_file.into()),
            RumorKind::Zone(zone) => RumorPayload::Zone(zone.into()),
        }
    }
}

/// The description of a `RumorKey`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RumorKey {
    pub kind: RumorType,
    pub id: String,
    pub key: String,
}

impl RumorKey {
    pub fn new<A, B>(kind: RumorType, id: A, key: B) -> RumorKey
    where
        A: ToString,
        B: ToString,
    {
        RumorKey {
            kind: kind,
            id: id.to_string(),
            key: key.to_string(),
        }
    }

    pub fn key(&self) -> String {
        if self.key.len() > 0 {
            format!("{}-{}", self.id, self.key)
        } else {
            format!("{}", self.id)
        }
    }
}

/// A representation of a Rumor; implemented by all the concrete types we share as rumors. The
/// exception is the Membership rumor, since it's not actually a rumor in the same vein.
pub trait Rumor: Message<ProtoRumor> + Sized {
    fn kind(&self) -> RumorType;
    fn key(&self) -> &str;
    fn id(&self) -> &str;
    fn merge(&mut self, other: Self) -> bool;
}

impl<'a, T: Rumor> From<&'a T> for RumorKey {
    fn from(rumor: &'a T) -> RumorKey {
        RumorKey::new(rumor.kind(), rumor.id(), rumor.key())
    }
}

/// Storage for Rumors. It takes a rumor and stores it according to the member that produced it,
/// and the service group it is related to.
///
/// Generic over the type of rumor it stores.
#[derive(Debug, Clone)]
pub struct RumorStore<T: Rumor> {
    pub list: Arc<RwLock<HashMap<String, HashMap<String, T>>>>,
    update_counter: Arc<AtomicUsize>,
}

impl<T> Default for RumorStore<T>
where
    T: Rumor,
{
    fn default() -> RumorStore<T> {
        RumorStore {
            list: Arc::new(RwLock::new(HashMap::new())),
            update_counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<T> Deref for RumorStore<T>
where
    T: Rumor,
{
    type Target = RwLock<HashMap<String, HashMap<String, T>>>;

    fn deref(&self) -> &Self::Target {
        &*self.list
    }
}

impl<T> Serialize for RumorStore<T>
where
    T: Rumor,
{
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut strukt = serializer.serialize_struct("rumor_store", 2)?;
        strukt.serialize_field("list", &*(self.list.read().unwrap()))?;
        strukt.serialize_field("update_counter", &self.get_update_counter())?;
        strukt.end()
    }
}

impl<T> RumorStore<T>
where
    T: Rumor,
{
    /// Create a new RumorStore for the given type. Allows you to initialize the counter to a
    /// pre-set value. Useful mainly in testing.
    pub fn new(counter: usize) -> RumorStore<T> {
        RumorStore {
            update_counter: Arc::new(AtomicUsize::new(counter)),
            ..Default::default()
        }
    }

    /// Clear all rumors and reset update counter of RumorStore.
    pub fn clear(&self) -> usize {
        let mut list = self.list.write().expect("Rumor store lock poisoned");
        list.clear();
        self.update_counter.swap(0, Ordering::Relaxed)
    }

    pub fn encode(&self, key: &str, member_id: &str) -> Result<Vec<u8>> {
        let list = self.list.read().expect("Rumor store lock poisoned");
        match list.get(key).and_then(|l| l.get(member_id)) {
            Some(rumor) => rumor.clone().write_to_bytes(),
            None => Err(Error::NonExistentRumor(
                String::from(member_id),
                String::from(key),
            )),
        }
    }

    pub fn get_update_counter(&self) -> usize {
        self.update_counter.load(Ordering::Relaxed)
    }

    /// Returns the count of all rumors in this RumorStore.
    pub fn len(&self) -> usize {
        self.list
            .read()
            .expect("Rumor store lock poisoned")
            .values()
            .map(|member| member.len())
            .sum()
    }

    /// Returns the count of all rumors in the rumor store for the given member's key.
    pub fn len_for_key(&self, key: &str) -> usize {
        let list = self.list.read().expect("Rumor store lock poisoned");
        list.get(key).map_or(0, |r| r.len())
    }

    /// Insert a rumor into the Rumor Store. Returns true if the value didn't exist or if it was
    /// mutated; if nothing changed, returns false.
    pub fn insert(&self, rumor: T) -> bool {
        let mut list = self.list.write().expect("Rumor store lock poisoned");
        let rumors = list
            .entry(String::from(rumor.key()))
            .or_insert(HashMap::new());
        // Result reveals if there was a change so we can increment the counter if needed.
        let result = match rumors.entry(rumor.id().into()) {
            Entry::Occupied(mut entry) => entry.get_mut().merge(rumor),
            Entry::Vacant(entry) => {
                entry.insert(rumor);
                true
            }
        };
        if result {
            self.increment_update_counter();
        }
        result
    }

    pub fn remove(&self, key: &str, id: &str) {
        let mut list = self.list.write().expect("Rumor store lock poisoned");
        list.get_mut(key).and_then(|r| r.remove(id));
    }

    pub fn with_keys<F>(&self, mut with_closure: F)
    where
        F: FnMut((&String, &HashMap<String, T>)),
    {
        let list = self.list.read().expect("Rumor store lock poisoned");
        for x in list.iter() {
            with_closure(x);
        }
    }

    pub fn with_rumors<F>(&self, key: &str, mut with_closure: F)
    where
        F: FnMut(&T),
    {
        let list = self.list.read().expect("Rumor store lock poisoned");
        if list.contains_key(key) {
            for x in list.get(key).unwrap().values() {
                with_closure(x);
            }
        }
    }

    pub fn with_rumor<F>(&self, key: &str, member_id: &str, mut with_closure: F)
    where
        F: FnMut(Option<&T>),
    {
        let list = self.list.read().expect("Rumor store lock poisoned");
        with_closure(list.get(key).and_then(|r| r.get(member_id)));
    }

    pub fn contains_rumor(&self, key: &str, id: &str) -> bool {
        let list = self.list.read().expect("Rumor store lock poisoned");
        match list.get(key).and_then(|l| l.get(id)) {
            Some(_) => true,
            None => false,
        }
    }

    /// Increment the update counter for this store.
    ///
    /// We don't care if this repeats - it just needs to be unique for any given two states, which
    /// it will be.
    fn increment_update_counter(&self) {
        self.update_counter.fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RumorEnvelope {
    pub type_: RumorType,
    pub from_id: String,
    pub kind: RumorKind,
}

impl RumorEnvelope {
    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let proto = ProtoRumor::decode(bytes)?;
        let type_ = RumorType::from_i32(proto.type_).ok_or(Error::ProtocolMismatch("type"))?;
        let from_id = proto
            .from_id
            .clone()
            .ok_or(Error::ProtocolMismatch("from-id"))?;
        let kind = match type_ {
            RumorType::Departure => RumorKind::Departure(Departure::from_proto(proto)?),
            RumorType::Election => RumorKind::Election(Election::from_proto(proto)?),
            RumorType::ElectionUpdate => {
                RumorKind::ElectionUpdate(ElectionUpdate::from_proto(proto)?)
            }
            RumorType::Member => RumorKind::Membership(Membership::from_proto(proto)?),
            RumorType::Service => RumorKind::Service(Service::from_proto(proto)?),
            RumorType::ServiceConfig => RumorKind::ServiceConfig(ServiceConfig::from_proto(proto)?),
            RumorType::ServiceFile => RumorKind::ServiceFile(ServiceFile::from_proto(proto)?),
            RumorType::Fake | RumorType::Fake2 => panic!("fake rumor"),
            RumorType::Zone => RumorKind::Zone(Zone::from_proto(proto)?),
        };
        Ok(RumorEnvelope {
            type_: type_,
            from_id: from_id,
            kind: kind,
        })
    }

    pub fn encode(self) -> Result<Vec<u8>> {
        let proto: ProtoRumor = self.into();
        let mut buf = BytesMut::with_capacity(proto.encoded_len());
        proto.encode(&mut buf)?;
        Ok(buf.to_vec())
    }
}

impl From<RumorEnvelope> for ProtoRumor {
    fn from(value: RumorEnvelope) -> ProtoRumor {
        ProtoRumor {
            type_: value.type_ as i32,
            tag: vec![],
            from_id: Some(value.from_id),
            payload: Some(value.kind.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use error::Result;
    use message::BfUuid;
    use protocol::{self, newscast};
    use rumor::{Rumor, RumorType};

    #[derive(Clone, Debug, Serialize)]
    struct FakeRumor {
        pub id: String,
        pub key: String,
    }

    impl Default for FakeRumor {
        fn default() -> FakeRumor {
            FakeRumor {
                id: BfUuid::generate().to_string(),
                key: String::from("fakerton"),
            }
        }
    }

    #[derive(Clone, Debug, Serialize)]
    struct TrumpRumor {
        pub id: String,
        pub key: String,
    }

    impl Rumor for FakeRumor {
        fn kind(&self) -> RumorType {
            RumorType::Fake
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn id(&self) -> &str {
            &self.id
        }

        fn merge(&mut self, mut _other: FakeRumor) -> bool {
            false
        }
    }

    impl protocol::FromProto<newscast::Rumor> for FakeRumor {
        fn from_proto(_other: newscast::Rumor) -> Result<Self> {
            Ok(FakeRumor::default())
        }
    }

    impl From<FakeRumor> for newscast::Rumor {
        fn from(_other: FakeRumor) -> newscast::Rumor {
            newscast::Rumor::default()
        }
    }

    impl protocol::Message<newscast::Rumor> for FakeRumor {
        fn from_bytes(_bytes: &[u8]) -> Result<Self> {
            Ok(FakeRumor::default())
        }

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    impl Default for TrumpRumor {
        fn default() -> TrumpRumor {
            TrumpRumor {
                id: BfUuid::generate().to_string(),
                key: String::from("fakerton"),
            }
        }
    }

    impl Rumor for TrumpRumor {
        fn kind(&self) -> RumorType {
            RumorType::Fake2
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn id(&self) -> &str {
            &self.id
        }

        fn merge(&mut self, mut _other: TrumpRumor) -> bool {
            false
        }
    }

    impl protocol::FromProto<newscast::Rumor> for TrumpRumor {
        fn from_proto(_other: newscast::Rumor) -> Result<Self> {
            Ok(TrumpRumor::default())
        }
    }

    impl From<TrumpRumor> for newscast::Rumor {
        fn from(_other: TrumpRumor) -> newscast::Rumor {
            newscast::Rumor::default()
        }
    }

    impl protocol::Message<newscast::Rumor> for TrumpRumor {
        fn from_bytes(_bytes: &[u8]) -> Result<Self> {
            Ok(TrumpRumor::default())
        }

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    mod rumor_store {
        use super::FakeRumor;
        use rumor::Rumor;
        use rumor::RumorStore;
        use std::usize;

        fn create_rumor_store() -> RumorStore<FakeRumor> {
            RumorStore::default()
        }

        #[test]
        fn update_counter() {
            let rs = create_rumor_store();
            rs.increment_update_counter();
            assert_eq!(rs.get_update_counter(), 1);
        }

        #[test]
        fn update_counter_overflows_safely() {
            let rs: RumorStore<FakeRumor> = RumorStore::new(usize::MAX);
            rs.increment_update_counter();
            assert_eq!(rs.get_update_counter(), 0);
        }

        #[test]
        fn insert_adds_rumor_when_empty() {
            let rs = create_rumor_store();
            let f = FakeRumor::default();
            assert!(rs.insert(f));
            assert_eq!(rs.get_update_counter(), 1);
        }

        #[test]
        fn insert_adds_multiple_rumors_for_same_key() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let key = String::from(f1.key());
            let f1_id = String::from(f1.id());
            let f2 = FakeRumor::default();
            let f2_id = String::from(f2.id());

            assert!(rs.insert(f1));
            assert!(rs.insert(f2));
            assert_eq!(rs.list.read().unwrap().len(), 1);
            assert_eq!(
                rs.list
                    .read()
                    .unwrap()
                    .get(&key)
                    .unwrap()
                    .get(&f1_id)
                    .unwrap()
                    .id,
                f1_id
            );
            assert_eq!(
                rs.list
                    .read()
                    .unwrap()
                    .get(&key)
                    .unwrap()
                    .get(&f2_id)
                    .unwrap()
                    .id,
                f2_id
            );
        }

        #[test]
        fn insert_adds_multiple_members() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let key = String::from(f1.key());
            let f2 = FakeRumor::default();
            assert!(rs.insert(f1));
            assert!(rs.insert(f2));
            assert_eq!(rs.list.read().unwrap().get(&key).unwrap().len(), 2);
        }

        #[test]
        fn insert_returns_false_on_no_changes() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let f2 = f1.clone();
            assert!(rs.insert(f1));
            assert_eq!(rs.insert(f2), false);
        }

        #[test]
        fn with_rumor_calls_closure_with_rumor() {
            let rs = create_rumor_store();
            let f1 = FakeRumor::default();
            let member_id = f1.id.clone();
            let key = f1.key.clone();
            rs.insert(f1);
            rs.with_rumor(&key, &member_id, |o| assert_eq!(o.unwrap().id, member_id));
        }

        #[test]
        fn with_rumor_calls_closure_with_none_if_rumor_missing() {
            let rs = create_rumor_store();
            rs.with_rumor("bar", "foo", |o| assert!(o.is_none()));
        }
    }
}
