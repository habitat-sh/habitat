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

//! In Butterfly, as in life, new rumors are "hot", but they get less
//! exciting the more you hear them. For a given rumor, we keep track
//! of how many times we've sent it to each member. Once we've sent
//! that member the rumor a maximum number of times, the rumor has
//! "cooled off". At that point we'll stop sending that rumor to the
//! member; by now they will have heard it!
//!
//! Note that the "heat" of a rumor is tracked *per member*, and is
//! not global.

// Standard Library
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Internal Modules
use rumor::RumorKey;

// TODO (CM): Can we key by member instead? What do we do more frequently?
// TODO (CM): Might want to type the member ID explicitly
// TODO (CM): what do we do with rumors that have officially
// "cooled off"? Can we just remove them?

/// The number of times a rumor will be shared before it goes cold for
/// that member.
// NOTE: This doesn't strictly need to be public, but making it so allows it
// to be present in generated documentation (the documentation strings
// of the functions in this module make reference to it).
pub const RUMOR_COOL_DOWN_LIMIT: usize = 2;

/// Tracks the number of times a given rumor has been sent to each
/// member of the supervision ring. This models the "heat" of a
/// rumor; if a member has never heard it, it's "hot", but it "cools
/// off" with each successive hearing.
///
/// When a rumor changes, we can effectively reset things by starting
/// the rumor mill up again. This will zero out all counters for every
/// member, starting the sharing cycle over again.
#[derive(Debug, Clone)]
pub struct RumorHeat(Arc<RwLock<HashMap<RumorKey, HashMap<String, usize>>>>);

impl RumorHeat {
    /// Add a rumor to track; members will see it as "hot".
    ///
    /// If the rumor was already being tracked, we reset all
    /// previously-recorded "heat" information; the rumor is once
    /// again "hot" for _all_ members.
    pub fn start_hot_rumor<T: Into<RumorKey>>(&self, rumor: T) {
        let rk: RumorKey = rumor.into();
        let mut rumors = self.0.write().expect("RumorHeat lock poisoned");
        rumors.insert(rk, HashMap::new());
    }

    /// Return a list of currently "hot" rumors for the specified
    /// member. This will be the subset of all rumors being tracked
    /// which have not already been sent to the member more than
    /// `RUMOR_COOL_DOWN_LIMIT` times.
    ///
    /// These rumors will be sorted by their "heat"; coldest rumors
    /// first, hotter rumors later. That is, rumors that have been
    /// shared `RUMOR_COOL_DOWN_LIMIT - 1` times will come first,
    /// followed by those that have been shared `RUMOR_COOL_DOWN_LIMIT
    /// -2` times, and so on, with those that have _never_ been
    /// shared with the member coming last.
    ///
    /// **NOTE**: The ordering of rumors within each of these "heat"
    /// cohorts is currently undefined.
    pub fn currently_hot_rumors(&self, id: &str) -> Vec<RumorKey> {
        let mut rumor_heat: Vec<(RumorKey, usize)> = self.0
            .read()
            .expect("RumorHeat lock poisoned")
            .iter()
            .map(|(k, heat_map)| {
                (k.clone(), heat_map.get(id).unwrap_or(&0).clone())
            })
            .filter(|&(_, heat)| heat < RUMOR_COOL_DOWN_LIMIT)
            .collect();

        // Reverse sorting by heat; 0s come last!
        rumor_heat.sort_by(|&(_, ref h1), &(_, ref h2)| h2.cmp(h1));

        // We don't need the heat anymore, just return the rumors.
        rumor_heat.into_iter().map(|(k, _)| k).collect()
    }

    /// For each rumor given, "cool" the rumor for the given member by
    /// incrementing the count for how many times it has been sent
    /// out. As a rumor cools, it will eventually cross a threshold
    /// past which it will no longer be gossipped to the member.
    ///
    /// Call this after sending rumors out across the network.
    ///
    /// **NOTE**: "cool" in the name of the function is a *verb*; you're
    /// not going to get a list of cool rumors from this.
    pub fn cool_rumors(&self, id: &str, rumors: &[RumorKey]) {
        if rumors.len() > 0 {
            let mut rumor_map = self.0.write().expect("RumorHeat lock poisoned");
            for ref rk in rumors {
                if rumor_map.contains_key(&rk) {
                    let heat_map = rumor_map.get_mut(&rk).unwrap();
                    if heat_map.contains_key(id) {
                        let heat = heat_map.get_mut(id).unwrap();
                        *heat += 1;
                    } else {
                        heat_map.insert(String::from(id), 1);
                    }
                } else {
                    debug!(
                        "Rumor does not exist in map; was probably deleted between retrieval \
                            and sending"
                    );
                }
            }
        }
    }
}

impl Default for RumorHeat {
    fn default() -> RumorHeat {
        RumorHeat(Arc::new(RwLock::new(HashMap::new())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::Result;
    use message::swim::Rumor_Type;
    use rumor::{Rumor, RumorKey};
    use uuid::Uuid;

    // TODO (CM): This FakeRumor implementation is copied from
    // rumor.rs; factor this helper code better.

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

    impl Rumor for FakeRumor {
        fn from_bytes(_bytes: &[u8]) -> Result<Self> {
            Ok(FakeRumor::default())
        }

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

    /// Helper function that tests that a given rumor is currently
    /// considered "hot" for the given member.
    fn assert_rumor_is_hot<T>(heat: &RumorHeat, member_id: &str, rumor: T)
    where
        T: Into<RumorKey>,
    {
        let key = rumor.into();
        let hot_rumors = heat.currently_hot_rumors(&member_id);
        assert!(hot_rumors.contains(&key));
    }

    /// Helper function that tests that a given rumor is currently
    /// NOT considered "hot" for the given member.
    fn assert_rumor_is_cold<T>(heat: &RumorHeat, member_id: &str, rumor: T)
    where
        T: Into<RumorKey>,
    {
        let key = rumor.into();
        let hot_rumors = heat.currently_hot_rumors(&member_id);
        assert!(!hot_rumors.contains(&key));
    }

    /// Helper function that takes a rumor that has already been
    /// introduced into the `RumorHeat` and cools it enough to no
    /// longer be considered "hot".
    fn cool_rumor_completely<T>(heat: &RumorHeat, member_id: &str, rumor: T)
    where
        T: Into<RumorKey>,
    {
        let rumor_keys = &[rumor.into()];
        for _ in 0..RUMOR_COOL_DOWN_LIMIT {
            heat.cool_rumors(&member_id, rumor_keys);
        }
    }

    #[test]
    fn there_are_no_hot_rumors_to_begin_with() {
        let heat = RumorHeat::default();
        let member_id = "test_member";

        let hot_rumors = heat.currently_hot_rumors(&member_id);
        assert!(hot_rumors.is_empty());
    }

    #[test]
    fn a_hot_rumor_is_returned_as_such() {
        let heat = RumorHeat::default();
        let member_id = "test_member";
        let rumor = FakeRumor::default();

        heat.start_hot_rumor(&rumor);

        let hot_rumors = heat.currently_hot_rumors(&member_id);
        assert_eq!(hot_rumors.len(), 1);
        assert_eq!(hot_rumors[0], RumorKey::from(&rumor));
    }

    #[test]
    fn a_hot_rumor_eventually_cools_off() {
        let heat = RumorHeat::default();
        let member_id = "test_member";
        let rumor = FakeRumor::default();
        let rumor_key = RumorKey::from(&rumor);
        let rumor_keys = &[rumor_key.clone()];

        heat.start_hot_rumor(&rumor);

        // Simulate going through the requisite number of gossip
        // cycles to cool the rumor down
        //
        // Not using the helper function here, as this function is
        // what this test is actually testing.
        for _ in 0..RUMOR_COOL_DOWN_LIMIT {
            assert_rumor_is_hot(&heat, &member_id, &rumor);
            heat.cool_rumors(&member_id, rumor_keys);
        }

        // At this point, our member should have heard this rumor
        // enough that it's no longer hot
        let hot_rumors = heat.currently_hot_rumors(&member_id);
        assert!(!hot_rumors.contains(&rumor_key));
    }

    #[test]
    fn rumors_can_become_hot_again_by_restarting_them() {
        let heat = RumorHeat::default();
        let member_id = "test_member";
        let rumor = FakeRumor::default();

        heat.start_hot_rumor(&rumor);

        // Simulate going through the requisite number of gossip
        // cycles to cool the rumor down
        cool_rumor_completely(&heat, &member_id, &rumor);

        // At this point, our member should have heard this rumor
        // enough that it's no longer hot
        assert_rumor_is_cold(&heat, &member_id, &rumor);

        // NOW we'll start the rumor again!
        heat.start_hot_rumor(&rumor);

        // Rumors... *so hot right now*
        assert_rumor_is_hot(&heat, &member_id, &rumor);
    }

    #[test]
    fn rumor_heat_is_tracked_per_member() {
        let heat = RumorHeat::default();
        let member_one = "test_member_1";
        let member_two = "test_member_2";
        let rumor = FakeRumor::default();

        heat.start_hot_rumor(&rumor);

        // Both members should see the rumor as hot.
        assert_rumor_is_hot(&heat, &member_one, &rumor);
        assert_rumor_is_hot(&heat, &member_two, &rumor);

        // Now, let's cool the rumor for only one of the members
        cool_rumor_completely(&heat, &member_one, &rumor);

        // Now it should be cold for the one member, but still hot
        // for the other.
        assert_rumor_is_cold(&heat, &member_one, &rumor);
        assert_rumor_is_hot(&heat, &member_two, &rumor);
    }

    #[test]
    fn hot_rumors_are_sorted_colder_to_warmer() {
        let heat = RumorHeat::default();
        let member = "test_member";

        // TODO (CM): for ease of test reading (esp. with failures), I'd like fake
        // rumors that I can control the IDs
        let hot_rumor = FakeRumor::default();
        let warm_rumor = FakeRumor::default();
        let cold_rumor = FakeRumor::default();

        // Start all rumors off as hot
        heat.start_hot_rumor(&hot_rumor);
        heat.start_hot_rumor(&warm_rumor);
        heat.start_hot_rumor(&cold_rumor);

        // Cool some rumors off, to varying degrees
        let hot_key = RumorKey::from(&hot_rumor);
        let warm_key = RumorKey::from(&warm_rumor);

        // Freeze this one right out
        cool_rumor_completely(&heat, &member, &cold_rumor);

        // Cool this one off just a little bit
        heat.cool_rumors(&member, &[warm_key.clone()]);

        // cold_rumor should be completely out, and the cooler
        // rumor sorts before the hotter one.
        let rumors = heat.currently_hot_rumors(&member);
        let expected_hot_rumors = &[warm_key.clone(), hot_key.clone()];
        assert_eq!(rumors, expected_hot_rumors);
    }
}
