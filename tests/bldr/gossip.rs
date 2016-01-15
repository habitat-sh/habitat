// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

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
