// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use setup;
use util::{self, command, docker};

#[test]
fn upload_a_key_and_install_it() {
    setup::gpg_import();
    setup::simple_service();

    let d = docker::repo("test/simple_service");
    let ipaddress = d.ipaddress();

    let mut upload = command::bldr(&["key-upload",
                                     &util::path::fixture_as_string("chef-public.gpg"),
                                     "-u",
                                     &format!("http://{}:9632", ipaddress)])
                         .unwrap();
    upload.wait_with_output();
    assert_cmd_exit_code!(upload, [0]);
    assert_regex!(upload.stdout(), r"Upload Bldr key (.+)");

    let mut install = command::bldr(&["key",
                                      "chef-public.gpg",
                                      "-u",
                                      &format!("http://{}:9632", ipaddress)])
                          .unwrap();
    install.wait_with_output();
    assert_cmd_exit_code!(install, [0]);
}
