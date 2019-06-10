//! In Butterfly, as in life, new rumors are "hot", but they get less
//! exciting the more you hear them. For a given rumor, we keep track
//! of how many times we've sent it to each member. Once we've sent
//! that member the rumor a maximum number of times, the rumor has
//! "cooled off". At that point we'll stop sending that rumor to the
//! member; by now they will have heard it!
//!
//! Note that the "heat" of a rumor is tracked *per member*, and is
//! not global.

use crate::rumor::{RumorKey,
                   RumorType};
use std::{collections::HashMap,
          sync::{Arc,
                 RwLock}};

// TODO (CM): Can we key by member instead? What do we do more frequently?
// TODO (CM): Might want to type the member ID explicitly
// TODO (CM): what do we do with rumors that have officially
// "cooled off"? Can we just remove them?

habitat_core::env_config_int!(/// The number of times that a rumor will be shared with a given
                              /// member before we stop sending it to that same member.
                              ///
                              /// This is roughly analogous to the parameter `k` (used as a
                              /// blind counter) in the paper _Epidemic Algorithms for
                              /// Replicated Database Maintenance_ by Demers, et al., Section
                              /// 1.4 "Complex Epidemics", subsection "Variations".
                              ///
                              /// (To correspond more closely with the paper, this should be used
                              /// on a per rumor basis, though,
                              /// instead of a per rumor/member basis. As
                              /// it is, this Supervisor will share each rumor `RumorShareLimit*n`
                              /// times, where `n` is the number of Supervisors in the network.)
                              #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
                              RumorShareLimit,
                              usize,
                              HAB_RUMOR_SHARE_LIMIT,
                              2);

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
        let mut rumor_heat: Vec<(RumorKey, usize)> =
            self.0
                .read()
                .expect("RumorHeat lock poisoned")
                .iter()
                .map(|(k, heat_map)| (k.clone(), *heat_map.get(id).unwrap_or(&0)))
                .filter(|&(_, heat)| heat < RumorShareLimit::configured_value().0)
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
        if !rumors.is_empty() {
            let mut rumor_map = self.0.write().expect("RumorHeat lock poisoned");
            for rk in rumors {
                if rumor_map.contains_key(&rk) {
                    let heat_map = rumor_map.get_mut(&rk).unwrap();
                    if heat_map.contains_key(id) {
                        let heat = heat_map.get_mut(id).unwrap();
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

    /// When a member is considered "gone" (e.g., once it is
    /// considered Departed), we can get rid of all the "cooling"
    /// information, since we're not going to be sending anything
    /// their way again.
    ///
    /// Without this, we would continue carrying around this
    /// information for Supervisors that we're never going to see
    /// again. The larger the network of Supervisors is, the more
    /// memory this consumes. If that member should ever come back
    /// again, all rumors would be considered "hot" for them, so they
    /// will get a bit more network traffic initially.
    pub fn purge(&self, id: &str) {
        let mut heat_map = self.0.write().expect("RumorHeat lock poisoned");

        // Remove any information about Service rumors for this
        // particular member... it's leaving, so none of its services
        // will be around either.
        let count_before = heat_map.len();
        heat_map.retain(|k, _| !(k.kind == RumorType::Service && k.id == id));
        let count_after = heat_map.len();
        debug!("Purged {} service rumor mappings for {:?}",
               count_before - count_after,
               id);

        // Remove any "cooling" information for this member, across
        // all types of rumors.
        let mut count = 0;
        for heat in heat_map.values_mut() {
            if heat.remove(id).is_some() {
                count += 1;
            }
        }
        debug!("Purged {} heat count entries for {:?}", count, id);
    }
}

impl Default for RumorHeat {
    fn default() -> RumorHeat { RumorHeat(Arc::new(RwLock::new(HashMap::new()))) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::Result,
                protocol::{self,
                           newscast},
                rumor::{Rumor,
                        RumorKey,
                        RumorType}};
    use habitat_common::locked_env_var;
    use uuid::Uuid;

    use crate::{member::Member,
                rumor::service::{Service,
                                 SysInfo}};
    use habitat_core::{package::PackageIdent,
                       service::ServiceGroup};

    // TODO (CM): This FakeRumor implementation is copied from
    // rumor.rs; factor this helper code better.

    #[derive(Clone, Debug, Serialize)]
    struct FakeRumor {
        pub id:  String,
        pub key: String,
    }

    impl Default for FakeRumor {
        fn default() -> FakeRumor {
            FakeRumor { id:  format!("{}", Uuid::new_v4().to_simple_ref()),
                        key: String::from("fakerton"), }
        }
    }

    impl Rumor for FakeRumor {
        fn kind(&self) -> RumorType { RumorType::Fake }

        fn key(&self) -> &str { &self.key }

        fn id(&self) -> &str { &self.id }

        fn merge(&mut self, mut _other: FakeRumor) -> bool { false }
    }

    impl protocol::FromProto<newscast::Rumor> for FakeRumor {
        fn from_proto(_other: newscast::Rumor) -> Result<Self> { Ok(FakeRumor::default()) }
    }

    impl From<FakeRumor> for newscast::Rumor {
        fn from(_other: FakeRumor) -> newscast::Rumor { newscast::Rumor::default() }
    }

    impl protocol::Message<newscast::Rumor> for FakeRumor {
        fn from_bytes(_bytes: &[u8]) -> Result<Self> { Ok(FakeRumor::default()) }

        fn write_to_bytes(&self) -> Result<Vec<u8>> {
            Ok(Vec::from(format!("{}-{}", self.id, self.key).as_bytes()))
        }
    }

    locked_env_var!(HAB_RUMOR_SHARE_LIMIT, lock_rumor_limit);

    /// Helper function that tests that a given rumor is currently
    /// considered "hot" for the given member.
    fn assert_rumor_is_hot<T>(heat: &RumorHeat, member_id: &str, rumor: T)
        where T: Into<RumorKey>
    {
        let key = rumor.into();
        let hot_rumors = heat.currently_hot_rumors(&member_id);
        assert!(hot_rumors.contains(&key));
    }

    /// Helper function that tests that a given rumor is currently
    /// NOT considered "hot" for the given member.
    fn assert_rumor_is_cold<T>(heat: &RumorHeat, member_id: &str, rumor: T)
        where T: Into<RumorKey>
    {
        let key = rumor.into();
        let hot_rumors = heat.currently_hot_rumors(&member_id);
        assert!(!hot_rumors.contains(&key));
    }

    /// Helper function that takes a rumor that has already been
    /// introduced into the `RumorHeat` and cools it enough to no
    /// longer be considered "hot".
    fn cool_rumor_completely<T>(heat: &RumorHeat, member_id: &str, rumor: T)
        where T: Into<RumorKey>
    {
        let rumor_keys = &[rumor.into()];
        for _ in 0..RumorShareLimit::default().0 {
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
        let l = lock_rumor_limit();
        l.unset();

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
        let l = lock_rumor_limit();
        l.unset();

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
        for _ in 0..RumorShareLimit::default().0 {
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
        let l = lock_rumor_limit();
        l.unset();

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
        let l = lock_rumor_limit();
        l.unset();

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
        let l = lock_rumor_limit();
        l.unset();

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

    fn test_service(member_id: &str) -> Service {
        let package: PackageIdent = "core/foo/1.0.0/20180701125610".parse().unwrap();
        let sg = ServiceGroup::new(None, "foo", "default", None).unwrap();
        Service::new(member_id, &package, sg, SysInfo::default(), None)
    }

    fn test_member(member_id: &str) -> Member {
        let mut m = Member::default();
        m.id = member_id.to_string();
        m
    }

    #[test]
    fn purging_removes_heat_information_for_a_given_member() {
        let l = lock_rumor_limit();
        l.unset();

        // Here's our world... we've got 3 members. We'll have a
        // Service rumor and a Member rumor for each of them. Then,
        // we'll ensure that all rumors have cooled for all members,
        // which will totally fill the RumorHeat structure.
        //
        // Then we'll purge member 2.
        //
        // We'll expect the entry for rumor for the service running on
        // member 2 to be completely gone, while only the "heat"
        // information for member 2 is removed from all other entries.

        let heat = RumorHeat::default();

        let member_1_id = "test_member_1";
        let member_2_id = "test_member_2";
        let member_3_id = "test_member_3";

        let member_1 = test_member(&member_1_id);
        let member_2 = test_member(&member_2_id);
        let member_3 = test_member(&member_3_id);

        let service_1 = test_service(&member_1_id);
        let service_2 = test_service(&member_2_id);
        let service_3 = test_service(&member_3_id);

        // We're going to add a bunch of rumors, and then ensure
        // they're completely "cooled" for every member. This should
        // approximate a long-standing, stable network, where all
        // rumors have been disseminated to all members.
        heat.start_hot_rumor(&member_1);
        heat.start_hot_rumor(&member_2);
        heat.start_hot_rumor(&member_3);
        heat.start_hot_rumor(&service_1);
        heat.start_hot_rumor(&service_2);
        heat.start_hot_rumor(&service_3);

        for m in &[member_1_id, member_2_id, member_3_id] {
            cool_rumor_completely(&heat, m, &service_1);
            cool_rumor_completely(&heat, m, &service_2);
            cool_rumor_completely(&heat, m, &service_3);
            cool_rumor_completely(&heat, m, &member_1);
            cool_rumor_completely(&heat, m, &member_2);
            cool_rumor_completely(&heat, m, &member_3);
        }

        // Peek at the internals; the purge method is basically about
        // reclaiming memory.
        //
        // This just asserts our baseline.
        {
            let inner = heat.0.read().unwrap();
            assert_eq!(inner.len(), 6);

            // Check the Member rumors
            for m in &[&member_1, &member_2, &member_3] {
                let heat_map = inner.get(&RumorKey::from(*m))
                                    .expect("Should have had a member rumor present");
                for m in &[member_1_id, member_2_id, member_3_id] {
                    assert_eq!(heat_map.get(*m)
                                       .expect("Should have had an entry for the member"),
                               &RumorShareLimit::default().0);
                }
            }

            // Check the Service rumors
            for s in &[&service_1, &service_2, &service_3] {
                let heat_map = inner.get(&RumorKey::from(*s))
                                    .expect("Should have had a service rumor present");
                for m in &[member_1_id, member_2_id, member_3_id] {
                    assert_eq!(heat_map.get(*m)
                                       .expect("Should have had an entry for the member"),
                               &RumorShareLimit::default().0);
                }
            }
        }

        // This is the meat of the test
        heat.purge(&member_2_id);

        // Now we peek at the internals again, verifying that only the
        // information pertaining to member 2 is gone.
        {
            let inner = heat.0.read().unwrap();
            assert_eq!(inner.len(), 5);

            // Check the Member rumors... all these should be present
            for m in &[&member_1, &member_2, &member_3] {
                let heat_map = inner.get(&RumorKey::from(*m))
                                    .expect("Should have had a member rumor present");
                assert_eq!(heat_map.get(member_1_id).expect("lulz"),
                           &RumorShareLimit::default().0);
                assert!(heat_map.get(member_2_id).is_none(),
                        "Heat information for a purged member should be removed");
                assert_eq!(heat_map.get(member_3_id).expect("lulz"),
                           &RumorShareLimit::default().0);
            }

            // Check the Service rumors
            assert!(inner.get(&RumorKey::from(&service_2)).is_none(),
                    "Service keys from the purged member should be removed");
            for s in &[&service_1, &service_3] {
                let heat_map = inner.get(&RumorKey::from(*s))
                                    .expect("Should have had a service rumor present");
                assert_eq!(heat_map.get(member_1_id).expect("lulz"),
                           &RumorShareLimit::default().0);
                assert!(heat_map.get(member_2_id).is_none(),
                        "Heat information for a purged member should be removed");
                assert_eq!(heat_map.get(member_3_id).expect("lulz"),
                           &RumorShareLimit::default().0);
            }
        }
    }
}
