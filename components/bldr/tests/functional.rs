// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate regex;
extern crate tempdir;
extern crate time;
extern crate hyper;
extern crate url;
extern crate bldr as bldr_lib;
extern crate bldr_core as core;
extern crate uuid;
extern crate rustc_serialize;

pub mod util;

mod setup {
    use std::sync::{Once, ONCE_INIT};
    use tempdir::TempDir;
    use std::process::Command;
    use std::collections::HashMap;

    use util;

    pub fn gpg_import() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut gpg = match util::command::studio_run("gpg",
                                                   &["--import",
                                                   &util::path::fixture_as_string("chef-private.gpg")]) {
                                                       Ok(cmd) => cmd,
                                                       Err(e) => panic!("{:?}", e),
        };
            gpg.wait_with_output();
            if !gpg.status.unwrap().success() {
                match gpg.stderr {
                    Some(stderr) => {
                        use regex::Regex;
                        let re = Regex::new("already in secret keyring").unwrap();
                        if !re.is_match(&stderr) {
                            panic!("Failed to import gpg keys");
                        }
                    }
                    None => panic!("Failed to import gpg keys")
                }
            }
        });
    }

    pub fn gpg_import_with_gpg_cache(cache_dir: &str) {
        static ONCE: Once = ONCE_INIT;

        let mut env: HashMap<&str, &str> = HashMap::new();
        env.insert("BLDR_GPG_CACHE", cache_dir);

        ONCE.call_once(|| {
            let mut gpg =
                match util::command::run_with_env("gpg",
                                                  &["--import",
                                                  &util::path::fixture_as_string("chef-private.gpg")],
                                                                                 &env) {
                    Ok(cmd) => cmd,
                    Err(e) => panic!("{:?}", e),
                };
            gpg.wait_with_output();
        });
    }


    pub fn simple_service() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut simple_service = match util::command::bldr_build(&util::path::fixture_as_string("simple_service")) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
            simple_service.wait_with_output();
            if !simple_service.status.unwrap().success() {
                panic!("Failed to build simple service");
            }
            let mut docker = match util::command::studio_run("dockerize", &["test/simple_service"]) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
            docker.wait_with_output();
            if !docker.status.unwrap().success() {
                panic!("Failed to dockerize simple service");
            }
        });
    }

    pub fn simple_service_gossip() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut simple_service = match util::command::bldr_build(&util::path::fixture_as_string("simple_service_gossip")) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
            simple_service.wait_with_output();
            if !simple_service.status.unwrap().success() {
                panic!("Failed to build simple service gossip");
            }
            let mut docker = match util::command::studio_run("dockerize", &["test/simple_service_gossip"]) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
            docker.wait_with_output();
            if !docker.status.unwrap().success() {
                panic!("Failed to dockerize simple service gossip");
            }
        });
    }

    pub fn fixture_service(pkg: &str) {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let tempdir = TempDir::new(pkg).unwrap();
            let mut copy_cmd = Command::new("cp")
                                   .arg("-r")
                                   .arg(util::path::fixture(pkg))
                                   .arg(tempdir.path().to_str().unwrap())
                                   .spawn()
                                   .unwrap();
            copy_cmd.wait().unwrap();

            let mut simple_service = match util::command::bldr_build(tempdir.path()
                                                                            .join(pkg)
                                                                            .to_str()
                                                                            .unwrap()) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
            simple_service.wait_with_output();
            if !simple_service.status.unwrap().success() {
                panic!("Failed to build {}: stdout: {:?}\nstderr: {:?}",
                       pkg,
                       simple_service.stdout.unwrap(),
                       simple_service.stderr.unwrap())
            }
        });
    }

    pub fn key_install() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut cmd = match util::command::bldr(&["key",
                                                    &util::path::fixture_as_string("chef-public.asc")]) {
                                                        Ok(cmd) => cmd,
                                                        Err(e) => panic!("{:?}", e),
        };
        cmd.wait_with_output();
        });
    }


}



macro_rules! poerr {
    ($expr:expr) => (
        match $expr {
            Ok(val) => val,
            Err(e) => {
                panic!("{:?}", e)
            }
        }
        )
}

macro_rules! poerr_ref {
    ($expr:expr) => (
        match $expr {
            Ok(ref val) => val,
            Err(ref e) => {
                panic!("{:?}", e)
            }
        }
        )
}

/// Given a Cmd struct and a list of status codes, fails
/// if the command didn't exit with one of the status codes.
macro_rules! assert_cmd_exit_code {
    ($cmd:ident, [ $( $status:expr ),+ ]) => {
        match $cmd.status().code() {
            Some(value) => {
                let codes = [$($status),+];
                assert!(codes.into_iter().any(|x| *x == value), "Status code {} does not match {:?}", value, codes)
            },
            None => {
                panic!("Command has not finished - cannot assert exit code")
            }
        }
    }
}

/// Given a string and a regex (use the r".." syntax), assert that
/// the string matches the regex.
macro_rules! assert_regex {
    ($string:expr, $regexp:expr) => {
        {
            use regex::Regex;

            let re = Regex::new($regexp).unwrap();
            assert!(re.is_match($string), "Regex '{}' failed to match", $regexp);
        }
    }
}

macro_rules! assert_docker_log {
    ($docker:expr, $regexp:expr) => {
        {
            assert!($docker.wait_until($regexp), "Regex '{}' failed to match", $regexp);
        }
    }
}

macro_rules! assert_docker_log_count {
    ($count:expr, $regexp:expr, [ $( $docker:expr ),+ ]) => {
        {
            let responses = [ $( $docker.wait_until($regexp) ),+ ];
            let num_responses = responses.iter().fold(0, |acc, &item| { let x = if item == true { 1 } else { 0 }; acc + x });
            assert!(num_responses == $count, "Expected {} occurances of {}; got {}", $count, $regexp, num_responses);
        }
    }
}

macro_rules! assert_file_exists {
    ($string:expr) => {
        {
            use std::fs;

            let meta = match fs::metadata($string) {
                Ok(meta) => meta,
                Err(e) => panic!("{} does not exist - {:?}", $string, e)
            };
            assert!(meta.is_file(), "{} exists, but is not a file", $string)
        }
    }
}

macro_rules! assert_file_exists_in_studio {
    ($string:expr) => {
        {
            use std::fs;
            let path = format!("/opt/studios/functional-tests{}", $string);
            let meta = match fs::metadata(&path) {
                Ok(meta) => meta,
                Err(e) => panic!("{} does not exist - {:?}", path, e)
            };
            assert!(meta.is_file(), "{} exists, but is not a file", path)
        }
    }
}

mod key_utils {
    use util::command;
    use uuid::Uuid;

    pub fn export_service_key(key: &str, outfile: &str, cache: &str, group: Option<&str>) {
        let mut export = match group {
            Some(g) => {
                command::bldr_with_test_gpg_cache(&["export-key",
                                                    "--service",
                                                    &key,
                                                    "--outfile",
                                                    &outfile,
                                                    "--group",
                                                    &g],
                                                  &cache)
                    .unwrap()
            }
            None => {
                command::bldr_with_test_gpg_cache(&["export-key",
                                                    "--service",
                                                    &key,
                                                    "--outfile",
                                                    &outfile],
                                                  &cache)
                    .unwrap()
            }

        };
        export.wait_with_output();
        assert_cmd_exit_code!(export, [0]);
        println!("{}", export.stdout());
    }

    pub fn export_user_key(key: &str, outfile: &str, cache: &str) {
        let mut export = command::bldr_with_test_gpg_cache(&["export-key",
                                                             "--user",
                                                             &key,
                                                             "--outfile",
                                                             &outfile],
                                                           &cache)
                             .unwrap();
        export.wait_with_output();
        assert_cmd_exit_code!(export, [0]);
        println!("{}", export.stdout());
    }

    pub fn import(exported_user_key: &str, cache: &str) {
        let mut import = command::bldr_with_test_gpg_cache(&["import-key",
                                                             "--infile",
                                                             &exported_user_key],
                                                           &cache)
                             .unwrap();
        import.wait_with_output();
        assert_cmd_exit_code!(import, [0]);
        println!("{}", import.stdout());
    }


    pub fn encrypt(user: &str,
                   service: &str,
                   file_to_encrypt: &str,
                   encrypted_file: &str,
                   cache: &str,
                   group: Option<&str>) {
        let mut encrypt = match group {
            Some(g) => {
                command::bldr_with_test_gpg_cache(&["encrypt",
                                                    "--user",
                                                    &user,
                                                    "--service",
                                                    &service,
                                                    "--infile",
                                                    &file_to_encrypt,
                                                    "--outfile",
                                                    &encrypted_file,
                                                    "--password",
                                                    "password",
                                                    "--group",
                                                    g],
                                                  &cache)
                    .unwrap()
            }
            None => {
                command::bldr_with_test_gpg_cache(&["encrypt",
                                                    "--user",
                                                    &user,
                                                    "--service",
                                                    &service,
                                                    "--infile",
                                                    &file_to_encrypt,
                                                    "--outfile",
                                                    &encrypted_file,
                                                    "--password",
                                                    "password"],
                                                  &cache)
                    .unwrap()
            }

        };
        encrypt.wait_with_output();
        println!("{}", encrypt.stdout());
        assert_cmd_exit_code!(encrypt, [0]);
        assert_regex!(encrypt.stdout(), r".*Finished encrypting.*");
    }


    pub fn decrypt(encrypted_file: &str, decrypted_file: &str, cache: &str, expected_status: i32) {
        // try to decrypt a file that's not meant for me
        let mut decrypt = command::bldr_with_test_gpg_cache(&["decrypt",
                                                              "--infile",
                                                              &encrypted_file,
                                                              "--outfile",
                                                              &decrypted_file],
                                                            &cache)
                              .unwrap();
        decrypt.wait_with_output();
        println!("{}", decrypt.stdout());
        assert_cmd_exit_code!(decrypt, [expected_status]);
    }

    pub fn list_keys(cache: &str, output_search: &str) {
        let mut list_keys = command::bldr_with_test_gpg_cache(&["list-keys"], &cache).unwrap();
        list_keys.wait_with_output();
        assert_regex!(list_keys.stdout(), output_search);
        println!("{}", list_keys.stdout());
    }

    /// generate user and service keys w/ a given path
    pub fn make_user_and_service(cache_dir: &str, group: Option<&str>) -> (String, String) {
        let user_uuid = Uuid::new_v4().to_simple_string();
        let service_uuid = Uuid::new_v4().to_simple_string();

        // generate a test user
        let mut generate_user = command::bldr_with_test_gpg_cache(&["generate-user-key",
                                                                    "--user",
                                                                    &user_uuid,
                                                                    "--password",
                                                                    "password",
                                                                    "--email",
                                                                    "email@bldrtest",
                                                                    "--expire-days=10"],
                                                                  cache_dir)
                                    .unwrap();
        generate_user.wait_with_output();
        println!("{}", generate_user.stdout());
        assert_cmd_exit_code!(generate_user, [0]);
        assert_regex!(generate_user.stdout(), r".*Fingerprint.*");

        let mut generate_service = match group {
            Some(g) => {
                command::bldr_with_test_gpg_cache(&["generate-service-key",
                                                    &service_uuid,
                                                    "--group",
                                                    &g],
                                                  cache_dir)
                    .unwrap()
            }
            None => {
                command::bldr_with_test_gpg_cache(&["generate-service-key", &service_uuid],
                                                  cache_dir)
                    .unwrap()
            }
        };
        generate_service.wait_with_output();
        assert_cmd_exit_code!(generate_service, [0]);
        assert_regex!(generate_service.stdout(), r".*Fingerprint.*");
        (user_uuid, service_uuid)
    }


}


// Include the actual test modules here!
pub mod bldr_build;
pub mod bldr;
