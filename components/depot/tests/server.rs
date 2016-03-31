// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate regex;
extern crate time;
extern crate hyper;
extern crate url;
extern crate habitat_core as core;
extern crate uuid;
extern crate rustc_serialize;

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

mod support;

use support::{command, docker, setup};

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
