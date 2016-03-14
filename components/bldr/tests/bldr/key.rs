// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate uuid;
use setup;
use util::{self, command, docker};
use uuid::Uuid;
use std::env;

/// kt_ = "key test"

/// run all gpg tests in their own cache so they
/// don't leave test certs behind etc
fn gen_test_gpg_cache() -> String {
    format!("/tmp/{}", Uuid::new_v4().to_simple_string())
}

#[ignore]
#[test] // TODO
fn kt_upload_a_key_and_install_it() {
    setup::gpg_import();
    setup::simple_service();
    let d = docker::depot("test/simple_service");
    let ipaddress = d.ipaddress();
    let mut upload = command::bldr(&["upload-repo-key",
                                     &util::path::fixture_as_string("chef-public.gpg"),
                                     "-u",
                                     &format!("http://{}:9632", ipaddress)])
                         .unwrap();
    upload.wait_with_output();
    assert_cmd_exit_code!(upload, [0]);
    assert_regex!(upload.stdout(), r"Upload Bldr key (.+)");
    let mut install = command::bldr(&["import-key",
                                      "--infile",
                                      "chef-public.gpg",
                                      "-u",
                                      &format!("http://{}:9632", ipaddress)])
                          .unwrap();
    install.wait_with_output();
    assert_cmd_exit_code!(install, [0]);
}


fn gpg_test_setup() -> String {
    let gpg_cache = gen_test_gpg_cache();
    setup::gpg_import_with_gpg_cache(&gpg_cache);
    // leaving this in as it may be useful in future testing
    // env::set_var("BLDR_GPG_CACHE", &gpg_cache);
    gpg_cache
}

#[test]
fn kt_generate_service_key() {
    // also tests list-keys
    let gpg_cache = gpg_test_setup();

    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr_with_test_gpg_cache(&["generate-service-key", &test_uuid],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}


#[test]
fn kt_generate_service_key_with_bldr_prefix() {
    // also tests list-keys
    let gpg_cache = gpg_test_setup();

    let test_uuid = Uuid::new_v4().to_simple_string();
    let test_uuid_with_prefix = "bldr_".to_string() + &test_uuid;

    let mut generate = command::bldr_with_test_gpg_cache(&["generate-service-key", &test_uuid],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);

    {
        let re = format!(".*{}.*", test_uuid_with_prefix);
        println!("Looking for \n{}\n", re);
        assert_regex!(generate.stdout(), &re);
    }

    {
        let re = format!(".*Successfully generated service key.*");
        println!("Looking for \n{}\n", re);
        assert_regex!(generate.stdout(), &re);
    }



    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}



#[test]
fn kt_generate_service_key_with_group() {
    // pass in --group as "foobar123"
    // also tests list-keys
    let gpg_cache = gpg_test_setup();

    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr_with_test_gpg_cache(&["generate-service-key",
                                                           &test_uuid,
                                                           "--group=foobar123"],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();
    let re_string = format!(".*{}\\.foobar123.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}


#[test]
fn kt_generate_service_key_with_expiration() {
    // also tests list-keys
    let gpg_cache = gpg_test_setup();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr_with_test_gpg_cache(&["generate-service-key",
                                                           &test_uuid,
                                                           "--expire-days=10"],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}

#[test]
fn kt_generate_service_key_with_expiration_and_group() {
    // also tests list-keys
    let gpg_cache = gpg_test_setup();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr_with_test_gpg_cache(&["generate-service-key",
                                                           &test_uuid,
                                                           "--expire-days=10",
                                                           "--group=foobar123"],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}\\.foobar123.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}



#[test]
fn kt_generate_user_key() {
    // also tests list-keys
    let gpg_cache = gpg_test_setup();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr_with_test_gpg_cache(&["generate-user-key",
                                                           "--user",
                                                           &test_uuid,
                                                           "--password",
                                                           "password",
                                                           "--email",
                                                           "email@bldrtest"],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}

#[test]
fn kt_generate_user_key_with_expiration() {
    // also tests list-keys
    let gpg_cache = gpg_test_setup();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let mut generate = command::bldr_with_test_gpg_cache(&["generate-user-key",
                                                           "--user",
                                                           &test_uuid,
                                                           "--password",
                                                           "password",
                                                           "--email",
                                                           "email@bldrtest",
                                                           "--expire-days=10"],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    assert_regex!(generate.stdout(), r".*Fingerprint.*");

    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}

#[test]
fn kt_generate_user_key_with_bldr_prefix() {
    // also tests list-keys
    let gpg_cache = gpg_test_setup();
    let test_uuid = Uuid::new_v4().to_simple_string();
    let test_uuid_with_prefix = "bldr_".to_string() + &test_uuid;
    let mut generate = command::bldr_with_test_gpg_cache(&["generate-user-key",
                                                           "--user",
                                                           &test_uuid_with_prefix,
                                                           "--password",
                                                           "password",
                                                           "--email",
                                                           "email@bldrtest",
                                                           "--expire-days=10"],
                                                         &gpg_cache)
                           .unwrap();

    generate.wait_with_output();
    assert_cmd_exit_code!(generate, [0]);
    {
        let re = format!(".*{}.*", test_uuid_with_prefix);
        println!("Looking for \n{}\n", re);
        assert_regex!(generate.stdout(), &re);
    }

    {
        let re = format!(".*Successfully generated user key.*");
        println!("Looking for \n{}\n", re);
        assert_regex!(generate.stdout(), &re);
    }


    // check to see if the key is in the output
    let mut listkeys = command::bldr_with_test_gpg_cache(&["list-keys"], &gpg_cache).unwrap();
    listkeys.wait_with_output();

    let re_string = format!(".*{}.*", test_uuid);
    assert_cmd_exit_code!(listkeys, [0]);
    assert_regex!(listkeys.stdout(), &re_string);
}


fn mk_tmp_filename() -> String {
    format!("/tmp/{}", Uuid::new_v4().to_simple_string())
}



#[test]
fn kt_find_key() {
    use bldr_lib::util::gpg;
    let cache_dir = gen_test_gpg_cache();
    // let cache_dir = "/opt/bldr/cache/gpg/";
    {
        // generate a test user
        let mut generate_user = command::bldr_with_test_gpg_cache(&["generate-user-key",
                                                                    "--user",
                                                                    "a",
                                                                    "--password",
                                                                    "password",
                                                                    "--email",
                                                                    "email@bldrtest",
                                                                    "--expire-days=10"],
                                                                  &cache_dir)
                                    .unwrap();
        generate_user.wait_with_output();
        println!("{}", generate_user.stdout());
        assert_cmd_exit_code!(generate_user, [0]);
        assert_regex!(generate_user.stdout(), r".*Fingerprint.*");
    }

    {
        // generate a test user
        let mut generate_user = command::bldr_with_test_gpg_cache(&["generate-user-key",
                                                                    "--user",
                                                                    "aa",
                                                                    "--password",
                                                                    "password",
                                                                    "--email",
                                                                    "email@bldrtest",
                                                                    "--expire-days=10"],
                                                                  &cache_dir)
                                    .unwrap();
        generate_user.wait_with_output();
        println!("{}", generate_user.stdout());
        assert_cmd_exit_code!(generate_user, [0]);
        assert_regex!(generate_user.stdout(), r".*Fingerprint.*");
    }

    env::set_var("BLDR_GPG_CACHE", cache_dir);
    match gpg::find_key("bldr_a") {
        Ok(result) => {
            let r = result.unwrap();
            assert_eq!(1, r.user_ids().count());
            let (_, user) = r.user_ids().enumerate().next().unwrap();
            let name = user.name().unwrap();
            assert_eq!("bldr_a", name);
        }
        Err(e) => panic!("FAIL {:?}", e),
    };


    match gpg::find_key("bldr_aa") {
        Ok(result) => {
            let r = result.unwrap();
            assert_eq!(1, r.user_ids().count());
            let (_, user) = r.user_ids().enumerate().next().unwrap();
            let name = user.name().unwrap();
            assert_eq!("bldr_aa", name);
        }
        Err(e) => panic!("FAIL {:?}", e),
    };

    match gpg::find_key("aaa") {
        Ok(Some(_)) => assert!(false),
        Ok(None) => assert!(true),
        Err(_) => assert!(false),
    };
    env::remove_var("BLDR_GPG_CACHE");
}

#[test]
fn kt_end_to_end() {
    // test all encryption functions with the default group
    test_gpg(None);
}

#[test]
fn kt_end_to_end_with_group() {
    // test all encryption functions with a group
    test_gpg(Some("testgroup"));
}

fn test_gpg(group: Option<&str>) {
    use std::io::prelude::*;
    use std::fs::File;
    use key_utils::*;
    let file_to_encrypt = mk_tmp_filename();
    let encrypted_file = mk_tmp_filename();
    let decrypted_file = mk_tmp_filename();

    let exported_user_key = mk_tmp_filename();
    let exported_service_key = mk_tmp_filename();

    println!("Plaintext file to encrypt: {}", file_to_encrypt);
    println!("Encrypted output: {}", encrypted_file);
    println!("Decrypted output: {}", decrypted_file);
    println!("Exported user key: {}", exported_user_key);
    println!("Exported service key: {}", exported_service_key);

    let mut f = File::create(file_to_encrypt.clone()).unwrap();
    let _res = f.write_all(b"Top secret data!\nTop secret data!!\n");

    let gpg_cache_a = gen_test_gpg_cache();
    let gpg_cache_b = gen_test_gpg_cache();
    println!("GPG CACHE A = {}", gpg_cache_a);
    println!("GPG CACHE B = {}", gpg_cache_b);

    // make a user and a service in the default group
    let (ua, sa) = make_user_and_service(&gpg_cache_a, group);
    let (_ub, sb) = make_user_and_service(&gpg_cache_b, group);

    // encrypt something that only GPG Cache A can decrypt
    encrypt(&ua,
            &sa,
            &file_to_encrypt,
            &encrypted_file,
            &gpg_cache_a,
            group);

    // we can decrypt a message that we own all keys for
    decrypt(&encrypted_file, &decrypted_file, &gpg_cache_a, 0);


    // Can gpg_cache_b read the message, even though it's not
    // the recipient? This should fail (return code 1), we
    // encrypted as user ua, but we need sa's private key to
    // decrypt (or ua's public key to verify)
    decrypt(&encrypted_file, &decrypted_file, &gpg_cache_b, 1);

    // export the service key from the B cache
    export_service_key(&sb, &exported_service_key, &gpg_cache_b, group);

    // export the user key from the A cache
    export_user_key(&ua, &exported_user_key, &gpg_cache_a);

    import(&exported_user_key, &gpg_cache_b);
    let search_for_user = format!(".*{}.*", ua);
    // do a list-keys and search for the user that was imported
    list_keys(&gpg_cache_b, &search_for_user);

    import(&exported_service_key, &gpg_cache_a);
    let search_for_service = format!(".*{}.*", sb);
    // do a list-leys and search for the service that was imported
    list_keys(&gpg_cache_a, &search_for_service);

    // encrypt a new message, this time using service B's public key
    // which we have imported
    encrypt(&ua,
            &sb,
            &file_to_encrypt,
            &encrypted_file,
            &gpg_cache_a,
            group);

    // returns 0 :-)
    decrypt(&encrypted_file, &decrypted_file, &gpg_cache_b, 0);
}
