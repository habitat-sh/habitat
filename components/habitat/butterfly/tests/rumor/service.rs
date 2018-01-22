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

use btest;
use habitat_butterfly::member::Health;

#[test]
fn two_members_share_services() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.wait_for_rounds(2);
    net[1].service_store.with_rumor(
        "witcher.prod",
        net[0].member_id(),
        |u| assert!(u.is_some()),
    );
}

#[test]
fn six_members_unmeshed_with_same_service_forces_departure_on_new_members() {
    let mut net = btest::SwimNet::new(6);
    net.connect(0, 1);
    net.connect(1, 2);
    net.connect(2, 3);
    net.connect(3, 4);
    net.connect(4, 5);
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.add_service(1, "core/witcher/1.2.3/20161208121212");
    net.add_service(2, "core/witcher/1.2.3/20161208121212");
    net.add_service(3, "core/witcher/1.2.3/20161208121212");
    net.add_service(4, "core/witcher/1.2.3/20161208121212");
    net.add_service(5, "core/witcher/1.2.3/20161208121212");
    assert_wait_for_health_of!(net, [0..6, 0..6], Health::Alive);
    trace_it!(TEST: &net[0], "Paused");
    net[0].pause();
    assert_wait_for_health_of!(net, 0, Health::Confirmed);

    net.add_member();
    net.add_service(6, "core/witcher/1.2.3/20161208121212");
    net.mesh();
    assert_wait_for_health_of!(net, 6, Health::Alive);
    assert_wait_for_health_of!(net, 0, Health::Departed);
}
