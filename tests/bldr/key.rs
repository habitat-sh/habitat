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
    let mut upload = command::bldr(&["upload-key",
                                     &util::path::fixture_as_string("chef-public.gpg"),
                                     "-u",
                                     &format!("http://{}:9632", ipaddress)])
                         .unwrap();
    upload.wait_with_output();
    assert_cmd_exit_code!(upload, [0]);
    assert_regex!(upload.stdout(), r"Upload Bldr key (.+)");
    let mut install = command::bldr(&["install-key",
                                      "chef-public.gpg",
                                      "-u",
                                      &format!("http://{}:9632", ipaddress)])
                          .unwrap();
    install.wait_with_output();
    assert_cmd_exit_code!(install, [0]);
}


#[test]
fn generate_service_key() {
    // also tests list-keys
    extern crate uuid;
    use uuid::Uuid;

    setup::gpg_import();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr(&["generate-service-key", &test_uuid]).unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr(&["list-keys"]).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);

}

#[test]
fn generate_service_key_with_expiration() {
    // also tests list-keys
    extern crate uuid;
    use uuid::Uuid;

    setup::gpg_import();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr(&["generate-service-key",
                                       &test_uuid,
                                       "--expire-days=10"]).unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr(&["list-keys"]).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}



#[test]
fn generate_user_key() {
    // also tests list-keys
    extern crate uuid;
    use uuid::Uuid;

    setup::gpg_import();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr(&["generate-user-key",
                                       &test_uuid,
                                       "password",
                                       "email@bldrtest"])
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr(&["list-keys"]).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}

#[test]
fn generate_user_key_with_expiration() {
    // also tests list-keys
    extern crate uuid;
    use uuid::Uuid;

    setup::gpg_import();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr(&["generate-user-key",
                                       &test_uuid,
                                       "password",
                                       "email@bldrtest",
                                       "--expire-days=10"])
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr(&["list-keys"]).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}
