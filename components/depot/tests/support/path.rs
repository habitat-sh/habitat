// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::env;
use std::path::PathBuf;

pub fn exe_path() -> PathBuf {
    env::current_exe().unwrap()
}

pub fn root() -> PathBuf {
    exe_path().parent().unwrap().parent().unwrap().parent().unwrap().join("tests")
}

pub fn fixtures() -> PathBuf {
    root().join("fixtures")
}

pub fn fixture_as_string(name: &str) -> String {
    let fixture_string = fixtures().join(name).to_string_lossy().into_owned();
    fixture_string
}

pub fn bldr() -> String {
    root().parent().unwrap().join("target/debug/bldr").to_string_lossy().into_owned()
}
