// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::sync::{Once, ONCE_INIT};

pub fn gpg_import() {
    static ONCE: Once = ONCE_INIT;
    ONCE.call_once(|| {
        let mut gpg = match super::command::studio_run("gpg",
                                               &["--import",
                                               &super::path::fixture_as_string("chef-private.gpg")]) {
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
    static ONCE: Once = ONCE_INIT;
    ONCE.call_once(|| {
        let mut cmd = match super::command::bldr(&["key",
                                                &super::path::fixture_as_string("chef-public.asc")]) {
                                                    Ok(cmd) => cmd,
                                                    Err(e) => panic!("{:?}", e),
    };
    cmd.wait_with_output();
    });
}

fn dockerize(ident_str: &str) {
    let mut install = match super::command::studio_run("hab-bpm",
                                                       &["install", "chef/hab-pkg-dockerize"]) {
        Ok(cmd) => cmd,
        Err(e) => panic!("{:?}", e),
    };
    install.wait_with_output();
    if !install.status.unwrap().success() {
        panic!("Failed to install 'chef/hab-pkg-dockerize'");
    }
    let mut docker = match super::command::studio_run("hab-bpm",
                                                      &["exec",
                                                        "chef/hab-pkg-dockerize",
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
