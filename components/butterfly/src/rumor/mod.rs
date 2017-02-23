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

//! Tracks rumors for distribution.
//!
//! Each rumor is represented by a `RumorKey`, which has a unique key and a "kind", which
//! represents what "kind" of rumor it is (for example, a "member").
//!
//! These keys are added to a RumorList, which tracks each rumors spread to each member it sends
//! to. Each rumor is shared with every member `RUMOR_MAX` times.
//!
//! New rumors need to implement the `From` trait for `RumorKey`, and then can track the arrival of
//! new rumors, and dispatch them according to their `kind`.

pub mod election;
pub mod service;
pub mod service_config;
pub mod service_file;

pub use self::election::Election;
pub use self::service::Service;
pub use self::service_config::ServiceConfig;
pub use self::service_file::ServiceFile;

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::default::Default;
use std::ops::Deref;
use std::result;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use message::swim::Rumor_Type;
use error::{Result, Error};

/// The description of a `RumorKey`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct RumorKey {
    pub kind: Rumor_Type,
    pub id: String,
    pub key: String,
}

impl RumorKey {
    pub fn new<A: Into<String>, B: Into<String>>(kind: Rumor_Type, id: A, key: B) -> RumorKey {
        RumorKey {
            kind: kind,
            id: id.into(),
            key: key.into(),
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
pub trait Rumor: Serialize {
    fn kind(&self) -> Rumor_Type;
    fn key(&self) -> &str;
    fn id(&self) -> &str;
    fn merge(&mut self, other: Self) -> bool;
    fn write_to_bytes(&self) -> Result<Vec<u8>>;
}

impl<'a, T: Rumor + Clone> From<&'a T> for RumorKey {
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

impl<T: Rumor + Clone> Default for RumorStore<T> {
    fn default() -> RumorStore<T> {
        RumorStore {
            list: Arc::new(RwLock::new(HashMap::new())),
            update_counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl<T: Rumor> Deref for RumorStore<T> {
    type Target = RwLock<HashMap<String, HashMap<String, T>>>;

    fn deref(&self) -> &Self::Target {
        &*self.list
    }
}

impl<T: Rumor> Serialize for RumorStore<T> {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("rumor_store", 2));
        try!(strukt.serialize_field("list", &*(self.list.read().unwrap())));
        try!(strukt.serialize_field("update_counter",
                                    &self.update_counter.load(Ordering::Relaxed)));
        strukt.end()
    }
}

impl<T: Rumor + Clone> RumorStore<T> {
    /// Create a new RumorStore for the given type. Allows you to initialize the counter to a
    /// pre-set value. Useful mainly in testing.
    pub fn new(counter: usize) -> RumorStore<T> {
        RumorStore { update_counter: Arc::new(AtomicUsize::new(counter)), ..Default::default() }
    }

    /// Increment the update counter for this store.
    ///
    /// We don't care if this repeats - it just needs to be unique for any given two states, which
    /// it will be.
    pub fn increment_update_counter(&self) {
        self.update_counter.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_update_counter(&self) -> usize {
        self.update_counter.load(Ordering::Relaxed)
    }

    pub fn len_for_key(&self, key: &str) -> usize {
        let list = self.list.read().expect("Rumor store lock poisoned");
        list.get(key).map_or(0, |r| r.len())
    }

    /// Insert a rumor into the Rumor Store. Returns true if the value didn't exist or if it was
    /// mutated; if nothing changed, returns false.
    pub fn insert(&self, rumor: T) -> bool {
        let mut list = self.list.write().expect("Rumor store lock poisoned");
        let mut rumors = list.entry(String::from(rumor.key())).or_insert(HashMap::new());
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
        list.get_mut(key).and_then(|mut r| r.remove(id));
    }

    pub fn with_keys<F>(&self, mut with_closure: F)
        where F: FnMut((&String, &HashMap<String, T>))
    {
        let list = self.list.read().expect("Rumor store lock poisoned");
        for x in list.iter() {
            with_closure(x);
        }
    }

    pub fn with_rumors<F>(&self, key: &str, mut with_closure: F)
        where F: FnMut(&T)
    {
        let list = self.list.read().expect("Rumor store lock poisoned");
        if list.contains_key(key) {
            for x in list.get(key).unwrap().values() {
                with_closure(x);
            }
        }
    }

    pub fn with_rumor<F>(&self, key: &str, member_id: &str, mut with_closure: F)
        where F: FnMut(Option<&T>)
    {
        let list = self.list.read().expect("Rumor store lock poisoned");
        with_closure(list.get(key).and_then(|r| r.get(member_id)));
    }

    pub fn write_to_bytes(&self, key: &str, member_id: &str) -> Result<Vec<u8>> {
        let list = self.list.read().expect("Rumor store lock poisoned");
        match list.get(key).and_then(|l| l.get(member_id)) {
            Some(rumor) => rumor.write_to_bytes(),
            None => Err(Error::NonExistentRumor(String::from(member_id), String::from(key))),
        }
    }

    pub fn contains_rumor(&self, key: &str, id: &str) -> bool {
        let list = self.list.read().expect("Rumor store lock poisoned");
        match list.get(key).and_then(|l| l.get(id)) {
            Some(_) => true,
            None => false,
        }
    }
}

/// The number of times a rumor will be shared before it goes cold for that member.
pub const RUMOR_MAX: usize = 2;

/// The RumorList is a map of `RumorKey` entries to member ID's, whose value is the number of times
/// we have shared this rumor with that member. The list is lazily populated when we ask for rumors
/// to share for a given member.
///
/// When a rumor changes, we re-insert it into the `RumorList` - this automatically sets all the
/// counters for every member, and starts the sharing cycle over again.
#[derive(Debug, Clone)]
pub struct RumorList {
    rumor_list: Arc<RwLock<HashMap<RumorKey, HashMap<String, usize>>>>,
}

impl Default for RumorList {
    fn default() -> RumorList {
        RumorList { rumor_list: Arc::new(RwLock::new(HashMap::new())) }
    }
}

pub type RumorVec = Vec<(RumorKey, usize)>;

impl RumorList {
    /// Add/Update a rumor to the list.
    pub fn insert<T: Into<RumorKey>>(&self, rumor: T) {
        let rk: RumorKey = rumor.into();
        let mut rumors = self.rumor_list.write().expect("Rumor Map lock poisoned");
        rumors.insert(rk, HashMap::new());
    }

    /// Return a list of rumors, along with their current heat, sorted by heat. Lowest to highest.
    /// So all the "0" rumors sort higher than the "2" rumors.
    pub fn rumors(&self, id: &str) -> RumorVec {
        let rumors = self.rumor_list.read().expect("Rumor map lock poisoned");
        let mut rumor_vec: RumorVec = rumors.iter()
            .map(|(rk, heat_map)| match heat_map.get(id) {
                Some(h) => (rk.clone(), h.clone()),
                None => (rk.clone(), 0),
            })
            .filter(|&(ref _rk, heat)| heat < RUMOR_MAX)
            .collect();
        rumor_vec.sort_by(|&(ref _a_rk, ref a_heat), &(ref _b_rk, ref b_heat)| b_heat.cmp(&a_heat));
        rumor_vec
    }

    /// Take a certain amount of rumors.
    pub fn take(&self, id: &str, amount: usize) -> RumorVec {
        self.rumors(id).into_iter().take(amount).collect()
    }

    /// Take a certain amount of rumors of a given kind.
    pub fn take_by_kind(&self, id: &str, amount: usize, kind: Rumor_Type) -> RumorVec {
        self.rumors(id)
            .into_iter()
            .filter(|&(ref rk, ref _heat)| rk.kind == kind)
            .take(amount)
            .collect()
    }

    /// Increment the heat for a given member for the list of rumors given.
    pub fn update_heat(&self, id: &str, rumors: &RumorVec) {
        if rumors.len() > 0 {
            let mut rumor_map = self.rumor_list.write().expect("Rumor map lock poisoned");
            for &(ref rk, ref _heat) in rumors {
                if rumor_map.contains_key(&rk) {
                    let mut heat_map = rumor_map.get_mut(&rk).unwrap();
                    if heat_map.contains_key(id) {
                        let mut heat = heat_map.get_mut(id).unwrap();
                        *heat += 1;
                    } else {
                        heat_map.insert(String::from(id), 1);
                    }
                } else {
                    debug!("Rumor does not exist in map; was probably deleted between retrieval \
                            and sending");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use rumor::Rumor;
    use message::swim::Rumor_Type;
    use error::Result;

    #[derive(Clone, Debug, Serialize)]
    struct FakeRumor {
        pub id: String,
        pub key: String,
    }

    impl Default for FakeRumor {
        fn default() -> FakeRumor {
            FakeRumor {
                id: format!("{}", Uuid::new_v4().simple()),
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
        fn kind(&self) -> Rumor_Type {
            Rumor_Type::Fake
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

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    impl Default for TrumpRumor {
        fn default() -> TrumpRumor {
            TrumpRumor {
                id: format!("{}", Uuid::new_v4().simple()),
                key: String::from("fakerton"),
            }
        }
    }

    impl Rumor for TrumpRumor {
        fn kind(&self) -> Rumor_Type {
            Rumor_Type::Fake2
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

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    mod rumor_store {
        use super::FakeRumor;
        use rumor::RumorStore;
        use rumor::Rumor;
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
            assert_eq!(rs.list
                           .read()
                           .unwrap()
                           .get(&key)
                           .unwrap()
                           .get(&f1_id)
                           .unwrap()
                           .id,
                       f1_id);
            assert_eq!(rs.list
                           .read()
                           .unwrap()
                           .get(&key)
                           .unwrap()
                           .get(&f2_id)
                           .unwrap()
                           .id,
                       f2_id);
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

    mod rumor_list {
        use super::{FakeRumor, TrumpRumor};
        use message::swim::Rumor_Type;
        use rumor::{RumorList, RUMOR_MAX};

        #[test]
        fn insert() {
            let rl = RumorList::default();
            let rumor = FakeRumor::default();
            rl.insert(&rumor);
        }

        #[test]
        fn rumors() {
            let rl = RumorList::default();
            for _ in 0..100 {
                let rumor = FakeRumor::default();
                rl.insert(&rumor);
            }
            let rumors = rl.rumors(&String::from("fake"));
            assert_eq!(rumors.len(), 100);
        }

        #[test]
        fn take() {
            let rl = RumorList::default();
            for _ in 0..100 {
                let rumor = FakeRumor::default();
                rl.insert(&rumor);
            }
            let rumors = rl.take(&String::from("fake"), 5);
            assert_eq!(rumors.len(), 5);
        }

        #[test]
        fn take_by_kind() {
            let rl = RumorList::default();
            for _ in 0..100 {
                let rumor = FakeRumor::default();
                rl.insert(&rumor);
            }
            for _ in 0..100 {
                let rumor = TrumpRumor::default();
                rl.insert(&rumor);
            }
            let rumors = rl.take_by_kind(&String::from("fake"), 100, Rumor_Type::Fake2);
            assert_eq!(rumors.len(), 100);
            assert_eq!(rumors.iter().all(|&(ref rk, ref _heat)| rk.kind == Rumor_Type::Fake2),
                       true);
        }

        #[test]
        fn update_heat() {
            let rl = RumorList::default();
            for _ in 0..10 {
                let rumor = FakeRumor::default();
                rl.insert(&rumor);
            }
            let rumors = rl.take(&String::from("fake"), 5);
            rl.update_heat(&String::from("fake"), &rumors);
            assert_eq!(rl.rumors(&String::from("fake"))
                           .iter()
                           .filter(|&&(ref _rk, ref heat)| *heat == 1)
                           .count(),
                       5);
            assert_eq!(rl.rumors(&String::from("fake"))
                           .iter()
                           .filter(|&&(ref _rk, ref heat)| *heat == 0)
                           .count(),
                       5);

        }

        #[test]
        fn update_heat_and_take_returns_colder_rumors() {
            let rl = RumorList::default();
            for _ in 0..10 {
                let rumor = FakeRumor::default();
                rl.insert(&rumor);
            }
            let updated_rumors = rl.take(&String::from("fake"), 5);
            rl.update_heat(&String::from("fake"), &updated_rumors);
            rl.take(&String::from("fake"), 5);
            assert_eq!(rl.rumors(&String::from("fake"))
                           .iter()
                           .filter(|&&(ref _rk, ref heat)| *heat == 0)
                           .count(),
                       5);
        }

        #[test]
        fn rumor_list_obeys_max_heat() {
            let rl = RumorList::default();
            for _ in 0..10 {
                let rumor = FakeRumor::default();
                rl.insert(&rumor);
            }
            let rumors = rl.take(&String::from("fake"), 5);
            for _x in 0..RUMOR_MAX {
                rl.update_heat(&String::from("fake"), &rumors);
            }
            assert_eq!(rl.rumors(&String::from("fake")).len(), 5);
        }

    }
}
