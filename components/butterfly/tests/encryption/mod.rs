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

use habitat_butterfly::member::Health;
use habitat_core::crypto::keys::sym_key::SymKey;

use btest;

#[test]
fn symmetric_encryption_of_wire_payloads() {
    let ring_key = SymKey::generate_in_memory("wolverine")
        .expect("Failed to generate an in memory symkey");
    let mut net = btest::SwimNet::new_ring_encryption(2, Some(ring_key));
    net.connect(0, 1);
    assert_wait_for_health_of!(net, [0..2, 0..2], Health::Alive);
    net.add_service(0, "beast");
    net.wait_for_gossip_rounds(2);
    net[1].service_store.with_rumor("beast.prod", net[0].member_id(), |u| assert!(u.is_some()));
}
