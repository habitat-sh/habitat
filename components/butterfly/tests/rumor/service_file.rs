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


use btest;

#[test]
fn two_members_share_service_files() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    net.add_service_file(0,
                         "witcher",
                         "yeppers",
                         "I like to have contents in my file");
    net.wait_for_gossip_rounds(1);
    net[1].service_file_store.with_rumor("witcher.prod", "service_file", |u| assert!(u.is_some()));
}
