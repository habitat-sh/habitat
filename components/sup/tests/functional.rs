// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
extern crate tempdir;
extern crate time;
extern crate hyper;
extern crate url;
extern crate habitat_sup as sup;
extern crate habitat_common as common;
extern crate habitat_core as hcore;
extern crate rustc_serialize;

// NOTE: These are temporarily disabled while we refactor the test suite around the new supervisor
// design. - Adam

// pub mod util;

// #[cfg(feature = "functional")]
// mod setup {
//     use std::sync::{Once, ONCE_INIT};
//     use std::process::Command;
//     use std::env;
//     use tempdir::TempDir;
//
//     use util;
//
//     pub fn origin_setup() {
//         env::set_var("HABITAT_KEY_CACHE", util::path::key_cache());
//     }
//
//     pub fn simple_service() {
//         static ONCE: Once = ONCE_INIT;
//         ONCE.call_once(|| {
//             let mut simple_service =
//                 match util::command::plan_build(&util::path::fixture_as_string("simple_service")) {
//                     Ok(cmd) => cmd,
//                     Err(e) => panic!("{:?}", e),
//                 };
//             simple_service.wait_with_output();
//             if !simple_service.status.unwrap().success() {
//                 panic!("Failed to build simple service");
//             }
//             util::command::dockerize("test/simple_service");
//         });
//     }
//
//     pub fn simple_service_gossip() {
//         static ONCE: Once = ONCE_INIT;
//         ONCE.call_once(|| {
//             let mut simple_service = match util::command::plan_build(&util::path::fixture_as_string("simple_service_gossip")) {
//                 Ok(cmd) => cmd,
//                 Err(e) => panic!("{:?}", e),
//             };
//             simple_service.wait_with_output();
//             if !simple_service.status.unwrap().success() {
//                 panic!("Failed to build simple service gossip");
//             }
//             util::command::dockerize("test/simple_service_gossip");
//         });
//     }
//
//     pub fn fixture_service(pkg: &str) {
//         static ONCE: Once = ONCE_INIT;
//         ONCE.call_once(|| {
//             let tempdir = TempDir::new(pkg).unwrap();
//             let mut copy_cmd = Command::new("cp")
//                 .arg("-r")
//                 .arg(util::path::fixture(pkg))
//                 .arg(tempdir.path().to_str().unwrap())
//                 .spawn()
//                 .unwrap();
//             copy_cmd.wait().unwrap();
//
//             let mut simple_service = match util::command::plan_build(tempdir.path()
//                 .join(pkg)
//                 .to_str()
//                 .unwrap()) {
//                 Ok(cmd) => cmd,
//                 Err(e) => panic!("{:?}", e),
//             };
//             simple_service.wait_with_output();
//             if !simple_service.status.unwrap().success() {
//                 panic!("Failed to build {}: stdout: {:?}\nstderr: {:?}",
//                        pkg,
//                        simple_service.stdout.unwrap(),
//                        simple_service.stderr.unwrap())
//             }
//         });
//     }
// }
//
// macro_rules! poerr {
//     ($expr:expr) => (
//         match $expr {
//             Ok(val) => val,
//             Err(e) => {
//                 panic!("{:?}", e)
//             }
//         }
//         )
// }
//
// macro_rules! poerr_ref {
//     ($expr:expr) => (
//         match $expr {
//             Ok(ref val) => val,
//             Err(ref e) => {
//                 panic!("{:?}", e)
//             }
//         }
//         )
// }
//
// /// Given a Cmd struct and a list of status codes, fails
// /// if the command didn't exit with one of the status codes.
// macro_rules! assert_cmd_exit_code {
//     ($cmd:ident, [ $( $status:expr ),+ ]) => {
//         match $cmd.status().code() {
//             Some(value) => {
//                 let codes = [$($status),+];
//                 assert!(codes.into_iter().any(|x| *x == value), "Status code {} does not match {:?}", value, codes)
//             },
//             None => {
//                 panic!("Command has not finished - cannot assert exit code")
//             }
//         }
//     }
// }
//
// /// Given a string and a regex (use the r".." syntax), assert that
// /// the string matches the regex.
// macro_rules! assert_regex {
//     ($string:expr, $regexp:expr) => {
//         {
//             use regex::Regex;
//
//             let re = Regex::new($regexp).unwrap();
//             assert!(re.is_match($string), "Regex '{}' failed to match", $regexp);
//         }
//     }
// }
//
// macro_rules! assert_docker_log {
//     ($docker:expr, $regexp:expr) => {
//         {
//             assert!($docker.wait_until($regexp), "Regex '{}' failed to match", $regexp);
//         }
//     }
// }
//
// macro_rules! assert_docker_log_count {
//     ($count:expr, $regexp:expr, [ $( $docker:expr ),+ ]) => {
//         {
//             let responses = [ $( $docker.wait_until($regexp) ),+ ];
//             let num_responses = responses.iter().fold(0, |acc, &item| { let x = if item == true { 1 } else { 0 }; acc + x });
//             assert!(num_responses == $count, "Expected {} occurances of {}; got {}", $count, $regexp, num_responses);
//         }
//     }
// }
//
// macro_rules! assert_file_exists {
//     ($string:expr) => {
//         {
//             use std::fs;
//
//             let meta = match fs::metadata($string) {
//                 Ok(meta) => meta,
//                 Err(e) => panic!("{} does not exist - {:?}", $string, e)
//             };
//             assert!(meta.is_file(), "{} exists, but is not a file", $string)
//         }
//     }
// }
//
// macro_rules! assert_file_exists_in_studio {
//     ($string:expr) => {
//         {
//             use std::fs;
//             let path = format!("/hab/studios/functional-tests{}", $string);
//             let meta = match fs::metadata(&path) {
//                 Ok(meta) => meta,
//                 Err(e) => panic!("{} does not exist - {:?}", path, e)
//             };
//             assert!(meta.is_file(), "{} exists, but is not a file", path)
//         }
//     }
// }
//
// // Include the actual test modules here!
// #[cfg(feature = "functional")]
// pub mod bldr_build;
// #[cfg(feature = "functional")]
// pub mod sup_tests;
