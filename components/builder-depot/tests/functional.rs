// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate regex;
extern crate time;
extern crate hyper;
extern crate url;
extern crate habitat_core as core;
extern crate uuid;

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
                assert!(codes.into_iter().any(|x| *x == value),
                        "Status code {} does not match {:?}", value, codes)
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
            let num_responses = responses.iter().fold(0, |acc, &item| {
                let x = if item == true { 1 } else { 0 }; acc + x
            });
            assert!(num_responses == $count,
                    "Expected {} occurrences of {}; got {}", $count, $regexp, num_responses);
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
            let path = format!("/hab/studios/functional-tests{}", $string);
            let meta = match fs::metadata(&path) {
                Ok(meta) => meta,
                Err(e) => panic!("{} does not exist - {:?}", path, e)
            };
            assert!(meta.is_file(), "{} exists, but is not a file", path)
        }
    }
}

#[cfg(feature = "functional")]
mod support;

#[cfg(feature = "functional")]
pub mod depot_tests {
    use support::{command, docker, setup};

    #[test]
    #[ignore]
    fn upload_a_package_and_then_install_it() {
        setup::origin_setup();
        setup::key_install();
        setup::simple_service();
        let d = docker::depot("test/simple_service");
        let ipaddress = d.ipaddress();

        let mut upload = command::sup(
            &[
                "upload",
                "test/simple_service",
                "-u",
                &format!("http://{}:9632", ipaddress),
            ],
        ).unwrap();
        upload.wait_with_output();
        assert_cmd_exit_code!(upload, [0]);
        assert_regex!(upload.stdout(), r"Upload Bldr Package (.+)");
        assert_regex!(upload.stdout(), r"Uploading from (.+)");
        assert_regex!(upload.stdout(), r"Complete");
        let mut install = command::sup(
            &[
                "install",
                "test/simple_service",
                "-u",
                &format!("http://{}:9632", ipaddress),
            ],
        ).unwrap();
        install.wait_with_output();
        assert_cmd_exit_code!(install, [0]);
    }
}
