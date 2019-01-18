// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

#![cfg(not(windows))]

/// Integration tests for exercising the hook and config recompilation
/// behavior of the Supervisor
extern crate habitat_core as hcore;

#[macro_use]
extern crate lazy_static;
use rand;

mod utils;

// The fixture location is derived from the name of this test
// suite. By convention, it is the same as the file name.
lazy_static! {
    static ref FIXTURE_ROOT: utils::FixtureRoot = utils::FixtureRoot::new("integration");
}

#[test]
fn config_only_packages_restart_on_config_application() {
    let hab_root = utils::HabRoot::new("config_only_packages_restart_on_config_application");

    let origin_name = "sup-integration-test";
    let package_name = "config-only";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = hab_root.pid_of(package_name);
    let config_before_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(r#"config_value = "something new and different""#);
    utils::sleep_seconds(2);

    let pid_after_apply = hab_root.pid_of(package_name);
    let config_after_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    assert_ne!(config_before_apply, config_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
fn hook_only_packages_restart_on_config_application() {
    let hab_root = utils::HabRoot::new("hook_only_packages_restart_on_config_application");

    let origin_name = "sup-integration-test";
    let package_name = "no-configs-only-hooks";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = hab_root.pid_of(package_name);
    let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");

    test_sup.apply_config(r#"hook_value = "something new and different""#);
    utils::sleep_seconds(2);

    let pid_after_apply = hab_root.pid_of(package_name);
    let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");

    assert_ne!(hook_before_apply, hook_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
fn config_files_change_but_hooks_do_not_still_restarts() {
    let hab_root = utils::HabRoot::new("config_files_change_but_hooks_do_not_still_restarts");

    let origin_name = "sup-integration-test";
    let package_name = "config-changes-hooks-do-not";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = hab_root.pid_of(package_name);
    let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
    let config_before_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(
        r#"
config_value = "applied"
hook_value = "default"
"#,
    );
    utils::sleep_seconds(2);

    let pid_after_apply = hab_root.pid_of(package_name);
    let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
    let config_after_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    assert_ne!(config_before_apply, config_after_apply);
    assert_eq!(hook_before_apply, hook_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
fn hooks_change_but_config_files_do_not_still_restarts() {
    let hab_root = utils::HabRoot::new("hooks_change_but_config_files_do_not_still_restarts");

    let origin_name = "sup-integration-test";
    let package_name = "hook-changes-config-does-not";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = hab_root.pid_of(package_name);
    let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
    let config_before_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(
        r#"
config_value = "default"
hook_value = "applied"
"#,
    );
    utils::sleep_seconds(2);

    let pid_after_apply = hab_root.pid_of(package_name);
    let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
    let config_after_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    assert_eq!(config_before_apply, config_after_apply);
    assert_ne!(hook_before_apply, hook_after_apply);
    assert_ne!(pid_before_apply, pid_after_apply);
}

#[test]
fn applying_identical_configuration_results_in_no_changes_and_no_restart() {
    let hab_root = utils::HabRoot::new(
        "applying_identical_configuration_results_in_no_changes_and_no_restart",
    );

    let origin_name = "sup-integration-test";
    let package_name = "no-changes-no-restart";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let pid_before_apply = hab_root.pid_of(package_name);
    let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
    let config_before_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    test_sup.apply_config(
        r#"
config_value = "default"
hook_value = "default"
"#,
    );
    utils::sleep_seconds(2);

    let pid_after_apply = hab_root.pid_of(package_name);
    let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
    let config_after_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

    assert_eq!(config_before_apply, config_after_apply);
    assert_eq!(hook_before_apply, hook_after_apply);
    assert_eq!(pid_before_apply, pid_after_apply);
}

#[test]
fn install_hook_success() {
    let hab_root = utils::HabRoot::new("install_hook_success");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-succeeds";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = hab_root.install_status_created(origin_name, package_name);

    assert_eq!(hab_root.install_status_of(origin_name, package_name), 0);
    assert!(hab_root.pid_of(package_name) > 0);

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = hab_root.install_status_created(origin_name, package_name);

    assert_eq!(status_created_before, status_created_after);
}

#[test]
fn package_with_successful_install_hook_in_dependency_is_loaded() {
    let hab_root =
        utils::HabRoot::new("package_with_successful_install_hook_in_dependency_is_loaded");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-succeeds-with-dependency";
    let dep = "install-hook-succeeds";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = hab_root.install_status_created(origin_name, dep);

    assert_eq!(hab_root.install_status_of(origin_name, dep), 0);
    assert!(hab_root.pid_of(package_name) > 0);

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = hab_root.install_status_created(origin_name, dep);

    assert_eq!(status_created_before, status_created_after);
}

#[test]
fn install_hook_fails() {
    let hab_root = utils::HabRoot::new("install_hook_fails");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-fails";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = hab_root.install_status_created(origin_name, package_name);
    let result = std::panic::catch_unwind(|| hab_root.pid_of(package_name));

    assert_eq!(hab_root.install_status_of(origin_name, package_name), 1);
    assert!(result.is_err());

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = hab_root.install_status_created(origin_name, package_name);

    assert_ne!(status_created_before, status_created_after);
}

#[test]
fn package_with_failing_install_hook_in_dependency_is_not_loaded() {
    let hab_root =
        utils::HabRoot::new("package_with_failing_install_hook_in_dependency_is_not_loaded");

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-fails-with-dependency";
    let dep = "install-hook-fails";
    let service_group = "default";

    utils::setup_package_files(
        &origin_name,
        &package_name,
        &service_group,
        &FIXTURE_ROOT,
        &hab_root,
    );

    let mut test_sup = utils::TestSup::new_with_random_ports(
        &hab_root,
        &origin_name,
        &package_name,
        &service_group,
    );

    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_before = hab_root.install_status_created(origin_name, dep);
    let result = std::panic::catch_unwind(|| hab_root.pid_of(package_name));

    assert_eq!(hab_root.install_status_of(origin_name, dep), 1);
    assert!(result.is_err());

    test_sup.stop();
    utils::sleep_seconds(3);
    test_sup.start();
    utils::sleep_seconds(3);

    let status_created_after = hab_root.install_status_created(origin_name, dep);

    assert_ne!(status_created_before, status_created_after);
}
