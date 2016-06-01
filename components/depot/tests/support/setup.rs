// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

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
