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

use std::sync::{Once, ONCE_INIT};
use std::env;

pub fn origin_setup() {
    env::set_var("HAB_CACHE_KEY_PATH", super::path::key_cache());
}

pub fn simple_service() {
    static ONCE: Once = ONCE_INIT;
    ONCE.call_once(|| {
        let mut simple_service =
            match super::command::plan_build(&super::path::fixture_as_string("simple_service")) {
                Ok(cmd) => cmd,
                Err(e) => panic!("{:?}", e),
            };
        simple_service.wait_with_output();
        if !simple_service.status.unwrap().success() {
            panic!("Failed to build simple service");
        }
        dockerize("test/simple_service");
    });
}

pub fn key_install() {
    // TODO DP: is there a relatively static pub key I can use?
}

fn dockerize(ident_str: &str) {
    let mut install = match super::command::studio_run("hab",
                                                       &["install", "core/hab-pkg-dockerize"]) {
        Ok(cmd) => cmd,
        Err(e) => panic!("{:?}", e),
    };
    install.wait_with_output();
    if !install.status.unwrap().success() {
        panic!("Failed to install 'core/hab-pkg-dockerize'");
    }
    let mut docker = match super::command::studio_run("hab",
                                                      &["exec",
                                                        "core/hab-pkg-dockerize",
                                                        "hab-pkg-dockerize",
                                                        ident_str]) {
        Ok(cmd) => cmd,
        Err(e) => panic!("{:?}", e),
    };
    docker.wait_with_output();
    if !docker.status.unwrap().success() {
        panic!("Failed to dockerize simple service");
    }
}
