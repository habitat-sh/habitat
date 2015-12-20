//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use setup;
use util::docker;

use bldr_lib::gossip::message::Message;
use bldr_lib::gossip::server::Server;
use bldr_lib::gossip::client::Client;

#[test]
fn client_server_ping() {
    setup::gpg_import();
    setup::simple_service();

    let d = docker::run("test/simple_service");
    let ip = d.ipaddress();
    let remote = format!("{}:9634", ip);
    let mut gc = Client::new(&remote[..]).unwrap();
    gc.send_message(Message::Ping).unwrap();
    if d.wait_until(r"Ping from") {
        let output = d.logs();
        assert_regex!(&output, r"Ping From");
    }
}
