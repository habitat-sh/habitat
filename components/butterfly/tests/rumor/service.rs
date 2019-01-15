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

#[test]
fn two_members_share_services() {
    let mut net = btest::SwimNet::new(2);
    net.mesh();
    net.add_service(0, "core/witcher/1.2.3/20161208121212");
    net.wait_for_rounds(2);
    net[1]
        .service_store
        .with_rumor("witcher.prod", net[0].member_id(), |u| assert!(u.is_some()));
}
