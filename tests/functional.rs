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
extern crate uuid;

pub mod util;

mod setup {
    use std::sync::{Once, ONCE_INIT};
    use tempdir::TempDir;
    use std::process::Command;

    use util;

    pub fn gpg_import() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let mut gpg = match util::command::run("gpg",
                                                   &["--import",
                                                     &util::path::fixture_as_string("chef-pri\
                                                                                     vate.gpg")]) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
            gpg.wait_with_output();
        });
    }

    pub fn simple_service() {
        static ONCE: Once = ONCE_INIT;
        ONCE.call_once(|| {
            let tempdir = TempDir::new("simple_service").unwrap();
            let mut copy_cmd = Command::new("cp")
                                   .arg("-r")
                                   .arg(util::path::fixture("simple_service"))
                                   .arg(tempdir.path().to_str().unwrap())
                                   .spawn()
                                   .unwrap();
            copy_cmd.wait().unwrap();

            let mut simple_service =
            match util::command::bldr_build(tempdir.path().join("simple_service")) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
            simple_service.wait_with_output();
            if !simple_service.status.unwrap().success() {
                panic!("Failed to build simple service: stdout: {:?}\nstderr: {:?}",
                       simple_service.stdout,
                       simple_service.stderr)
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

            let mut simple_service =
            match util::command::bldr_build(tempdir.path().join(pkg)) {
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
                                                      &util::path::fixture_as_string("chef-pu\
                                                                                      blic.as\
                                                                                      c")]) {
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

// Include the actual test modules here!
pub mod bldr_build;
pub mod bldr;
