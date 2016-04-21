extern crate habitat_core as hcore;
extern crate time;

use std::collections::HashSet;
use std::fs;
use util::wait::wait_until_ok;

fn setup_key_env(test_name: &str) -> (hcore::crypto::Context, String) {
    let key_dir = &format!("/tmp/habitat_test_keys_{}", test_name);
    // don't unwrap this, the directory might not be present
    let _ = fs::remove_dir_all(&key_dir);
    fs::create_dir_all(&key_dir).unwrap();
    (hcore::crypto::Context::new(&key_dir), key_dir.to_string())
}

#[test]
fn crypto_generate_key_revisions_test() {
    let (crypto_ctx, _) = setup_key_env("key_revisions");
    let test_key_name = "habitat123";
    // there aren't any keys, but it should crash. It should
    // return an empty Vec
    match crypto_ctx.get_key_revisions(test_key_name) {
        Ok(revs) => assert!(revs.len() == 0),
        Err(e) => panic!("Can't get key revisions {}", e),
    }

    // generate a single key
    if let Err(e) = crypto_ctx.generate_origin_sig_key(test_key_name) {
        panic!("Error generating keys {}", e)
    };

    // we should only see a single revision
    let first_rev = match crypto_ctx.get_key_revisions(test_key_name) {
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
    if let None = wait_until_ok(|| crypto_ctx.generate_origin_sig_key(test_key_name)) {
        panic!("Failed to generate another key after 30 seconds");
    }

    let second_rev = match crypto_ctx.get_key_revisions(test_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 2);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get key revisions {}", e),
    };
    assert!(first_rev != second_rev);
}


#[test]
fn crypto_generate_box_keys_test() {
    let (crypto_ctx, _) = setup_key_env("generate_box_keys");
    let test_org = "someorg";
    let test_user = "foo";
    let test_service_group = "bar.testgroup";

    // generated keys SHOULD be in the following 2 formats:
    let test_user_key_name = format!("{}", test_user);
    let test_service_key_name = format!("{}@{}", test_service_group, test_org);

    if let None = wait_until_ok(|| crypto_ctx.generate_user_box_key(test_user)) {
        panic!("Can't generate a user box key");
    }

    if let None = wait_until_ok(|| {
        crypto_ctx.generate_service_box_key(test_org, test_service_group)
    }) {
        panic!("Can't generate a service box key");
    }

    // we should only see a single revision
    let first_user_rev = match crypto_ctx.get_key_revisions(&test_user_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 1);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get user key revisions {}", e),
    };

    let first_service_rev = match crypto_ctx.get_key_revisions(&test_service_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 1);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get service key revisions {}", e),
    };

    if let None = wait_until_ok(|| crypto_ctx.generate_user_box_key(test_user)) {
        panic!("Can't generate a second user box key");
    }

    if let None = wait_until_ok(|| {
        crypto_ctx.generate_service_box_key(test_org, test_service_group)
    }) {
        panic!("Can't generate a second service box key");
    }

    let second_user_rev = match crypto_ctx.get_key_revisions(&test_user_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 2);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get user key revisions {}", e),
    };
    assert!(first_user_rev != second_user_rev);


    let second_service_rev = match crypto_ctx.get_key_revisions(&test_service_key_name) {
        Ok(revs) => {
            assert!(revs.len() == 2);
            revs.first().unwrap().clone()
        }
        Err(e) => panic!("Can't get service key revisions {}", e),
    };
    assert!(first_service_rev != second_service_rev);

}


#[test]
fn mixed_key_revisions_test() {
    // given a directory containing mixed public/secret key files
    // (some missing public keys, some missing secret keys),
    // do we reliabily pull back a list of key revisions?

    let (crypto_ctx, key_dir) = setup_key_env("mixed_key_revision_test");
    let mut revisions = Vec::new();

    for _ in 0..3 {
        match wait_until_ok(|| crypto_ctx.generate_user_box_key("calculating_infinity")) {
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

    let keyname = "calculating_infinity";

    match crypto_ctx.get_key_revisions(keyname) {
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
fn crypto_encrypt_decrypt_test() {
    // This test generates some keys, encrypts and decrypts a string
    // to make sure everything is working in between.
    // Next, remove public and then secret keys and ensure
    // that we can't read the message. As the key *names*
    // are embedded in the encrypted payload, the decrypt code
    // tries to load the keys and if they aren't available,
    // it fails.
    let (crypto_ctx, key_dir) = setup_key_env("encrypt_decrypt");
    let test_org = "someorg";
    let test_user1 = "foo1";
    let test_service_group1 = "service1.testgroup";

    if let None = wait_until_ok(|| crypto_ctx.generate_user_box_key(test_user1)) {
        panic!("Can't generate a user box key");
    }

    if let None = wait_until_ok(|| {
        crypto_ctx.generate_service_box_key(test_org, test_service_group1)
    }) {
        panic!("Can't generate a service box key");
    }

    let user1_keys = crypto_ctx.read_box_keys("foo1").unwrap();
    let service1_keys = crypto_ctx.read_box_keys("service1.testgroup@someorg").unwrap();

    let user1_kp = &user1_keys[0];
    let service1_kp = &service1_keys[0];

    // - service is recipient via public key
    // - user is signer via secret key
    let secret = "this is secret data";
    let data = secret.as_bytes();
    let mut payload = crypto_ctx.encrypt(data,
                                         &service1_kp.name_with_rev,
                                         &service1_kp.public.as_ref().unwrap(),
                                         &user1_kp.name_with_rev,
                                         &user1_kp.secret.as_ref().unwrap())
                                .unwrap();

    let result = crypto_ctx.decrypt(&mut payload).unwrap();
    // we encrypted a value, and got the same value back upon decryption
    assert!(result == data);

    let user_public_keyfile = format!("{}/{}.pub", key_dir, &user1_kp.name_with_rev);
    let user_public_keyfile_backup = format!("{}/{}.bak", key_dir, &user1_kp.name_with_rev);

    let service_secret_keyfile = format!("{}/{}.box.key", key_dir, &service1_kp.name_with_rev);
    let service_secret_keyfile_backup = format!("{}/{}.bak", key_dir, &service1_kp.name_with_rev);

    // try to decrypt without the user public key
    if let Err(x) = fs::rename(&user_public_keyfile, &user_public_keyfile_backup) {
        panic!("Can't rename public key {}", x);
    };

    if crypto_ctx.decrypt(&mut payload).is_ok() {
        panic!("Shouldn't be able to decrypt without user public key");
    }

    if let Err(x) = fs::rename(&user_public_keyfile_backup, &user_public_keyfile) {
        panic!("Can't rename public key {}", x);
    };

    // try to decrypt without the service secret key
    if let Err(x) = fs::rename(&service_secret_keyfile, &service_secret_keyfile_backup) {
        panic!("Can't rename secret key {}", x);
    };
    if crypto_ctx.decrypt(&mut payload).is_ok() {
        panic!("Shouldn't be able to decrypt without service secret key");
    }

}
