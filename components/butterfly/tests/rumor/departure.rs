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

use crate::btest;
use habitat_butterfly::{client::Client,
                        member::Health};

#[test]
fn two_members_share_departures() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    net.add_departure(0);
    net.wait_for_gossip_rounds(1);
    assert!(net[1].departure_store
                  .contains_rumor("departure", net[0].member_id()));
}

#[test]
#[ignore]
fn departure_via_client() {
    let mut net = btest::SwimNet::new(3);
    net.mesh();

    net.wait_for_gossip_rounds(1);
    let mut client =
        Client::new(&net[0].gossip_addr().to_string(), None).expect("Cannot create Butterfly \
                                                                     Client");
    client.send_departure(&net[1].member_id())
          .expect("Cannot send the departure");
    net.wait_for_gossip_rounds(1);
    assert!(net[2].departure_store
                  .contains_rumor("departure", net[1].member_id()));
    assert_wait_for_health_of!(net, 1, Health::Departed);
}
