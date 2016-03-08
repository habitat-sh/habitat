// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use setup;
use util::{command, docker};

#[test]
#[ignore]
fn upload_a_package_and_then_install_it() {
    setup::gpg_import();
    setup::key_install();
    setup::simple_service();
    let d = docker::depot("test/simple_service");
    let ipaddress = d.ipaddress();

    let mut upload = command::bldr(&["upload",
                                     "test/simple_service",
                                     "-u",
                                     &format!("http://{}:9632", ipaddress)])
                         .unwrap();
    upload.wait_with_output();
    assert_cmd_exit_code!(upload, [0]);
    assert_regex!(upload.stdout(), r"Upload Bldr Package (.+)");
    assert_regex!(upload.stdout(), r"Uploading from (.+)");
    assert_regex!(upload.stdout(), r"Complete");
    let mut install = command::bldr(&["install",
                                      "test/simple_service",
                                      "-u",
                                      &format!("http://{}:9632", ipaddress)])
                          .unwrap();
    install.wait_with_output();
    assert_cmd_exit_code!(install, [0]);
}
