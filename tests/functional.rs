//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

extern crate regex;
extern crate tempdir;
extern crate time;
extern crate hyper;
extern crate url;

pub mod util;

use std::collections::HashMap;

use regex::Regex;

mod setup {
    use std::sync::{Once, ONCE_INIT};
    use tempdir::TempDir;
    use std::process::Command;

    use util;

    pub fn gpg_import() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut gpg = match util::command::run("gpg", &["--import", &util::path::fixture_as_string("chef-private.gpg")]) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e)
            };
            gpg.wait_with_output();
        });
    }

    pub fn bldr_release() {
        // static ONCE: Once = ONCE_INIT;
        // ONCE.call_once(|| {
        //     let mut bldr = match util::command::bldr_build(util::path::bldr_package()) {
        //         Ok(cmd) => cmd,
        //         Err(e) => panic!("{:?}", e)
        //     };
        //     bldr.wait_with_output();
        // });
    }

    pub fn simple_service() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let tempdir = TempDir::new("simple_service").unwrap();
            let mut copy_cmd = Command::new("cp")
                .arg("-r")
                .arg(util::path::fixture("simple_service"))
                .arg(tempdir.path().to_str().unwrap())
                .spawn().unwrap();
            copy_cmd.wait().unwrap();

            bldr_release();

            let mut simple_service = match util::command::bldr_build(tempdir.path().join("simple_service")) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e)
            };
            simple_service.wait_with_output();
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

// Include the actual test modules here!
pub mod bldr_build;
pub mod bldr;
