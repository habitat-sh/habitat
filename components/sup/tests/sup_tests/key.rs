// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate uuid;
use setup;
use util::{self, command, docker};
use uuid::Uuid;
use std::env;

/// kt_ = "key test"

/// run all crypto tests in their own cache so they
/// don't leave test certs behind etc
/*
fn gen_test_gpg_cache() -> String {
    format!("/tmp/{}", Uuid::new_v4().to_simple_string())
}
*/

#[ignore]
#[test] // TODO
fn kt_upload_a_key_and_install_it() {
    /*
     * DP TODO: GPG -> NaCl
    setup::origin_setup();
    setup::simple_service();
    let d = docker::depot("test/simple_service");
    let ipaddress = d.ipaddress();
    let mut upload = command::sup(&["upload-repo-key",
                                    &util::path::fixture_as_string("chef-public.gpg"),
                                    "-u",
                                    &format!("http://{}:9632", ipaddress)])
                         .unwrap();
    upload.wait_with_output();
    assert_cmd_exit_code!(upload, [0]);
    assert_regex!(upload.stdout(), r"Upload Habitat key (.+)");
    let mut install = command::sup(&["import-key",
                                     "--infile",
                                     "chef-public.gpg",
                                     "-u",
                                     &format!("http://{}:9632", ipaddress)])
                          .unwrap();
    install.wait_with_output();
    assert_cmd_exit_code!(install, [0]);
    */
}


/*
>>>>>>> wip
fn mk_tmp_filename() -> String {
    format!("/tmp/{}", Uuid::new_v4().to_simple_string())
}
*/

