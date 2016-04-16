// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate time;

use std::env;
use std::fs;
use std::collections::HashSet;

// call a closure in a loop until it returns Ok(keyname),
// or the 30 second timeout. Pass the return value out
// as an option
pub fn wait_until_ok<F>(some_fn: F) -> Option<String>
    where F: Fn() -> Result<String, hcore::error::Error>
{
    let wait_duration = time::Duration::seconds(30);
    let current_time = time::now_utc().to_timespec();
    let stop_time = current_time + wait_duration;
    while time::now_utc().to_timespec() < stop_time {
        if let Ok(s) = some_fn() {
            return Some(s);
        }
    }
    None
}


#[test]
fn generate_key_revisions_test() {
    let key_dir = "/tmp/habitat_test_keys";
    let _ = fs::remove_dir_all(&key_dir);
    fs::create_dir_all(&key_dir).unwrap();

    // override the location where Habitat wants to store keys
    env::set_var("HAB_CACHE_KEY_PATH", &key_dir);

    let test_key_name = "habitat123";

    // there aren't any keys, but it should crash. It should
    // return an empty Vec
    match hcore::crypto::get_key_revisions(test_key_name) {
        Ok(revs) => assert!(revs.len() == 0),
        Err(e) => panic!("Can't get key revisions {}", e),
    }

    // generate a single key
    if let Err(e) = hcore::crypto::generate_origin_sig_key(test_key_name) {
        panic!("Error generating keys {}", e)
    };

    // we should only see a single revision
    let first_rev = match hcore::crypto::get_key_revisions(test_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 1);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get key revisions {}", e),
    };

    // We can't generate more than 1 key with the same name per second,
    // otherwise, the keys would have the same revision. Call
    // generate_origin_sig_key in a loop, and wait until it returns Ok(())
    // we generate another key to see if get_key_revisions() returns 2
    if let None = wait_until_ok(|| hcore::crypto::generate_origin_sig_key(test_key_name)) {
        panic!("Failed to generate another key after 30 seconds");
    }

    let second_rev = match hcore::crypto::get_key_revisions(test_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 2);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get key revisions {}", e),
    };
    assert!(first_rev != second_rev);
}


#[test]
fn mixed_key_revisions_test() {
    // given a directory containing mixed public/secret key files
    // (some missing public keys, some missing secret keys),
    // do we reliabily pull back a list of key revisions?

    let key_dir = "/tmp/habitat_test_keys";
    let _ = fs::remove_dir_all(&key_dir);
    fs::create_dir_all(&key_dir).unwrap();

    // override the location where Habitat wants to store keys
    env::set_var("HAB_CACHE_KEY_PATH", &key_dir);
    let mut revisions = Vec::new();

    for _ in 0..3 {
        match wait_until_ok(|| {
            hcore::crypto::generate_user_box_key("dillinger", "calculating_infinity")
        }) {
            None => panic!("Can't generate box key, operation timed out"),
            Some(s) => revisions.push(s),
        };
    }

    let remove_pub = format!("{}/{}.pub", &key_dir, &revisions[1]);
    if let Err(e) = fs::remove_file(remove_pub) {
        panic!("Can't remove public key {}", e);
    }

    let remove_secret = format!("{}/{}.box.key", &key_dir, &revisions[2]);
    if let Err(e) = fs::remove_file(remove_secret) {
        panic!("Can't remove secret key {}", e);
    }

    let keyname = "calculating_infinity@dillinger";

    match hcore::crypto::get_key_revisions(keyname) {
        Ok(revs) => {
            assert!(revs.len() == 3);
            let mut s = HashSet::new();
            for r in &revs {
                s.insert(r);
            }
            // we still get 3 revisions back
            assert!(s.len() == 3);
        }
        Err(e) => panic!("Couldn't get key revisions {}", e),
    };
}

#[test]
fn generate_box_keys_test() {
    // Note, user + service keys use org, not origin
    let key_dir = "/tmp/habitat_test_keys";
    let _ = fs::remove_dir_all(&key_dir);
    fs::create_dir_all(&key_dir).unwrap();

    // override the location where Habitat wants to store keys
    env::set_var("HAB_CACHE_KEY_PATH", &key_dir);

    let test_org = "someorg";
    let test_user = "foo";
    let test_service_group = "bar.testgroup";

    // generated keys SHOULD be in the following 2 formats:
    let test_user_key_name = format!("{}@{}", test_user, test_org);
    let test_service_key_name = format!("{}@{}", test_service_group, test_org);

    if let None = wait_until_ok(|| hcore::crypto::generate_user_box_key(test_org, test_user)) {
        panic!("Can't generate a user box key");
    }

    if let None = wait_until_ok(|| {
        hcore::crypto::generate_service_box_key(test_org, test_service_group)
    }) {
        panic!("Can't generate a service box key");
    }

    // we should only see a single revision
    let first_user_rev = match hcore::crypto::get_key_revisions(&test_user_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 1);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get user key revisions {}", e),
    };

    let first_service_rev = match hcore::crypto::get_key_revisions(&test_service_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 1);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get service key revisions {}", e),
    };

    if let None = wait_until_ok(|| hcore::crypto::generate_user_box_key(test_org, test_user)) {
        panic!("Can't generate a second user box key");
    }

    if let None = wait_until_ok(|| {
        hcore::crypto::generate_service_box_key(test_org, test_service_group)
    }) {
        panic!("Can't generate a second service box key");
    }

    let second_user_rev = match hcore::crypto::get_key_revisions(&test_user_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 2);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get user key revisions {}", e),
    };
    assert!(first_user_rev != second_user_rev);


    let second_service_rev = match hcore::crypto::get_key_revisions(&test_service_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 2);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get service key revisions {}", e),
    };
    assert!(first_service_rev != second_service_rev);

}
