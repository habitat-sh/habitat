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

use habitat_butterfly::member::Health;
use habitat_butterfly::rumor::election::ElectionStatus;

use crate::btest;

#[test]
fn three_members_run_election() {
    let mut net = btest::SwimNet::new(3);
    net.mesh();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");

    net.add_election(0, "witcher");
    net.add_election(1, "witcher");
    net.add_election(2, "witcher");

    assert_wait_for_election_status!(net, [0..3], "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..3, 0..3], "witcher.prod");
}

#[test]
fn three_members_run_election_from_one_starting_rumor() {
    let mut net = btest::SwimNet::new(3);
    net.mesh();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");
    net.add_election(0, "witcher");
    assert_wait_for_election_status!(net, [0..3], "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..3, 0..3], "witcher.prod");
}

#[test]
#[ignore]
fn five_members_elect_a_new_leader_when_the_old_one_dies() {
    let mut net = btest::SwimNet::new(5);
    net.mesh();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");
    net.add_service(3, "core/witcher/1.2.3/20161208121212");
    net.add_service(4, "core/witcher/1.2.3/20161208121212");
    net.add_election(0, "witcher");
    assert_wait_for_election_status!(net, [0..5], "witcher.prod", ElectionStatus::Finished);
    assert_wait_for_equal_election!(net, [0..5, 0..5], "witcher.prod");

    let mut leader_id = String::from("");
    net[0]
        .election_store
        .with_rumor("witcher.prod", "election", |e| {
            leader_id = e.member_id.to_string();
        });

    let mut paused = 0;
    for (index, server) in net.iter_mut().enumerate() {
        if server.member_id() == &leader_id[..] {
            paused = index;
        }
    }
    net[paused].pause();
    let paused_id = net[paused].member_id();
    assert_wait_for_health_of!(net, paused, Health::Confirmed);
    if paused == 0 {
        net[1].restart_elections();
    } else {
        net[0].restart_elections();
    }

    for i in 0..5 {
        if !i == paused {
            assert_wait_for_election_status!(net, i, "witcher.prod", ElectionStatus::Running);
        }
    }

    for i in 0..5 {
        if !i == paused {
            assert_wait_for_election_status!(net, i, "witcher.prod", ElectionStatus::Finished);
        }
    }

    net[if paused == 0 { 1 } else { 0 }]
        .election_store
        .assert_rumor_is("witcher.prod", "election", |e| {
            e.term == 1 && e.member_id != paused_id
        });
}
