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

//! Tracks rumors for distribution.
//!
//! Each rumor is represented by a `RumorKey`, which has a unique key and a "kind", which
//! represents what "kind" of rumor it is (for example, a "member").
//!
//! These keys are added to a RumorList, which tracks each rumors spread to each member it sends
//! to. Each rumor is shared with every member `RUMOR_MAX` times.
//!
//! New rumors need to implement the `From` trait for `RumorKey`, and then can track the arrival of
//! new rumors, and dispatch them according to thier `kind`.

use std::collections::HashMap;
use std::default::Default;
use std::sync::{Arc, RwLock};

use member::UuidSimple;

/// The description of a `RumorKey`.
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq)]
pub struct RumorKey {
    pub kind: String,
    pub key: UuidSimple,
}

impl RumorKey {
    pub fn new<A: Into<String>, B: Into<String>>(kind: A, key: B) -> RumorKey {
        RumorKey {
            kind: kind.into(),
            key: key.into(),
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
    rumor_list: Arc<RwLock<HashMap<RumorKey, HashMap<UuidSimple, usize>>>>,
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
            .map(|(rk, heat_map)| {
                match heat_map.get(id) {
                    Some(h) => (rk.clone(), h.clone()),
                    None => (rk.clone(), 0),
                }
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
    pub fn take_by_kind(&self, id: &str, amount: usize, kind: &str) -> RumorVec {
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

    use rumor::RumorKey;

    pub struct FakeRumor {
        pub id: Uuid,
    }

    impl Default for FakeRumor {
        fn default() -> FakeRumor {
            FakeRumor { id: Uuid::new_v4() }
        }
    }

    impl<'a> From<&'a FakeRumor> for RumorKey {
        fn from(rumor: &FakeRumor) -> RumorKey {
            RumorKey {
                kind: String::from("fake"),
                key: rumor.id.simple().to_string(),
            }
        }
    }

    impl From<FakeRumor> for RumorKey {
        fn from(rumor: FakeRumor) -> RumorKey {
            RumorKey {
                kind: String::from("fake"),
                key: rumor.id.simple().to_string(),
            }
        }
    }

    pub struct TrumpRumor {
        pub id: Uuid,
    }

    impl Default for TrumpRumor {
        fn default() -> TrumpRumor {
            TrumpRumor { id: Uuid::new_v4() }
        }
    }

    impl<'a> From<&'a TrumpRumor> for RumorKey {
        fn from(rumor: &TrumpRumor) -> RumorKey {
            RumorKey {
                kind: String::from("trump"),
                key: rumor.id.simple().to_string(),
            }
        }
    }

    impl From<TrumpRumor> for RumorKey {
        fn from(rumor: TrumpRumor) -> RumorKey {
            RumorKey {
                kind: String::from("trump"),
                key: rumor.id.simple().to_string(),
            }
        }
    }

    mod rumor_list {
        use super::{FakeRumor, TrumpRumor};
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
            let rumors = rl.take_by_kind(&String::from("fake"), 100, "trump");
            assert_eq!(rumors.len(), 100);
            assert_eq!(rumors.iter().all(|&(ref rk, ref _heat)| rk.kind == "trump"),
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
