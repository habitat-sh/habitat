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
use habitat_butterfly::client::Client;
use habitat_core::service::ServiceGroup;

#[test]
fn two_members_share_service_config() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    net.add_service_config(0, "witcher", "tcp-backlog = 128");
    net.wait_for_gossip_rounds(1);
    assert!(net[1].service_config_store
                  .contains_rumor("witcher.prod", "service_config"));
}

#[test]
fn service_config_via_client() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();

    net.wait_for_gossip_rounds(1);
    let mut client =
        Client::new(&net[0].gossip_addr().to_string(), None).expect("Cannot create Butterfly \
                                                                     Client");
    let payload = b"I want to get lost in you, tokyo";
    client.send_service_config(ServiceGroup::new(None, "witcher", "prod", None).unwrap(),
                               0,
                               payload,
                               false)
          .expect("Cannot send the service configuration");
    net.wait_for_gossip_rounds(1);
    assert!(net[1].service_config_store
                  .contains_rumor("witcher.prod", "service_config"));
}
