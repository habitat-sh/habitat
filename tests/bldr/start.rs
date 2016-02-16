// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use util::{self, docker};
use setup;
use regex::Regex;
use std::thread;
use std::time::Duration;

#[test]
fn standalone_no_options() {
    setup::gpg_import();
    setup::simple_service();

    let d = docker::run("test/simple_service");
    if d.wait_until(r"Shipping out to Boston") {
        let output = d.logs();
        assert_regex!(&output, r"Starting (.+)");
    }
}

#[ignore]
#[test]
fn standalone_no_options_without_config() {
    setup::gpg_import();
    setup::fixture_service("simple_service_without_config");

    let d = docker::run("test/simple_service_without_config");
    if d.wait_until(r"Shipping out to Boston") {
        let output = d.logs();
        assert_regex!(&output, r"Starting (.+)");
    } else {
        // container didn't start in time or pkg doesn't exist
        assert!(false);
    }
}

#[ignore]
#[test]
fn standalone_with_environment_config() {
    setup::gpg_import();
    setup::simple_service();

    let d = docker::run_with_env("test/simple_service",
                                 "BLDR_simple_service=setting=\"blarg\"");
    if d.wait_until(r"End Configuration") {
        let output = d.logs();
        assert_regex!(&output, r"setting: blarg");
    }
}

#[ignore]
#[test]
fn standalone_with_discovery_config() {
    setup::gpg_import();
    setup::simple_service();

    util::discovery::clear("config");
    util::discovery::set("config", "setting = \"sepultura\"");

    let d = docker::run_with_etcd("test/simple_service");
    assert_docker_log!(d, r"setting: sepultura");
}

#[ignore]
#[test]
fn standalone_with_discovery_config_updates() {
    setup::gpg_import();
    setup::simple_service();

    util::discovery::clear("config");

    util::discovery::set("config", "setting = \"sepultura\"");
    let d = docker::run_with_etcd("test/simple_service");
    assert_docker_log!(d, r"setting: sepultura");

    util::discovery::set("config", "setting = \"against me!\"");
    assert_docker_log!(d, r"setting: against me!");
}

#[ignore]
#[test]
fn leader_with_discovery() {
    setup::gpg_import();
    setup::simple_service();

    util::discovery::clear("config");
    util::discovery::clear("topology");

    util::discovery::set("config", "setting = \"rustacean\"");
    let d1 = docker::run_with_etcd_topology("test/simple_service", "leader");
    let d2 = docker::run_with_etcd_topology("test/simple_service", "leader");
    let d3 = docker::run_with_etcd_topology("test/simple_service", "leader");

    assert_docker_log_count!(1, "Starting my term as leader", [d1, d2, d3]);
    assert_docker_log_count!(2, "Becoming a follower", [d1, d2, d3]);

    assert_docker_log!(d1, r"setting: rustacean");
    assert_docker_log!(d2, r"setting: rustacean");
    assert_docker_log!(d3, r"setting: rustacean");

    util::discovery::set("config", "setting = \"against me!\"");
    assert_docker_log!(d1, r"setting: against me!");
    assert_docker_log!(d2, r"setting: against me!");
    assert_docker_log!(d3, r"setting: against me!");

    let re = Regex::new(r"Starting my term as leader").unwrap();
    if re.is_match(&d1.logs()) {
        drop(d1);
        thread::sleep(Duration::from_millis(32000));
        assert_docker_log_count!(1, "Starting my term as leader", [d2, d3]);
    } else if re.is_match(&d2.logs()) {
        drop(d2);
        thread::sleep(Duration::from_millis(32000));
        assert_docker_log_count!(1, "Starting my term as leader", [d1, d3]);
    } else if re.is_match(&d3.logs()) {
        drop(d3);
        thread::sleep(Duration::from_millis(32000));
        assert_docker_log_count!(1, "Starting my term as leader", [d1, d2]);
    }
}
