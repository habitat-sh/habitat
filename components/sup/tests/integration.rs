#![cfg(not(windows))]
use crate::utils::FileSystemSnapshot;
use anyhow::Result;
use glob::Pattern;
use std::time::Duration;

/// Integration tests for exercising the hook and config recompilation
/// behavior of the Supervisor
extern crate habitat_core as hcore;

#[macro_use]
extern crate lazy_static;

mod utils;

// The fixture location is derived from the name of this test
// suite. By convention, it is the same as the file name.
lazy_static! {
    static ref FIXTURE_ROOT: utils::FixtureRoot = utils::FixtureRoot::new("integration");
    static ref EXCLUDED_PATHS: Vec<Pattern> = vec![Pattern::new("logs/*").unwrap()];
}

/// Tests the scenario where a applying a config parameter causes restarts
/// immediately due to an update to an application configuration file.
#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn templated_config_with_no_reconfigure_hook_restart_on_config_application() -> Result<()> {
    let hab_root = utils::HabRoot::new("templated_config_with_no_reconfigure_hook_restart_on_config_application");

    let origin_name = "sup-integration-test";
    let package_name = "config-and-hooks-no-reconfigure";
    let service_group = "default";

    let service_min_backoff_period = Duration::from_secs(10);
    let service_max_backoff_period = Duration::from_secs(30);
    let service_restart_cooldown_period = Duration::from_secs(60);

    let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
                                                             service_min_backoff_period,
                                                             service_max_backoff_period,
                                                             service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let pid =
        test_sup.wait_for_service_startup(package_name, service_group, Duration::from_secs(10))
                .await?;
    let initial_snapshot =
        FileSystemSnapshot::new(hab_root.svc_path(package_name).as_path()).await?;
    let initial_pid_snapshot = initial_snapshot.file("PID")?;
                                                                          
    test_sup.apply_config(package_name, service_group, r#"app_name = "Test App""#).await;
    let _pid = test_sup.wait_for_service_restart(pid,
                                                package_name,
                                                service_group,
                                                Duration::from_secs(30))
                      .await?;

    let final_snapshot =
        FileSystemSnapshot::new(hab_root.svc_path(package_name).as_path()).await?;
    let final_pid_snapshot = final_snapshot.file("PID")?;

    let delta = final_snapshot.modifications_since(&initial_snapshot, EXCLUDED_PATHS.to_vec());

    let duration_between_restarts = final_pid_snapshot.duration_between_modification(initial_pid_snapshot).unwrap();
    
    // Ensure the time between restarts is less than our configured minimum restart duration
    assert!(duration_between_restarts > Duration::ZERO);
    assert!(duration_between_restarts < service_min_backoff_period);
    assert_eq!(delta.updated(),
               vec!["PID",
                    "config/app-config.toml"]);
    assert_eq!(delta.added(), vec![] as Vec<&str>);
    assert_eq!(delta.removed(), vec![] as Vec<&str>);
    
    test_sup.stop().await?;
    Ok(())
}

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn templated_run_hook_causes_restart_on_config_application() {
//     let hab_root = utils::HabRoot::new("hook_only_packages_restart_on_config_application");

//     let origin_name = "sup-integration-test";
//     let package_name = "config-and-hooks-no-reconfigure";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let pid_before_apply = hab_root.pid_of(package_name);
//     let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "run");

//     test_sup.apply_config(r#"run_templated_value = "Running V1""#);
//     utils::sleep_seconds(5);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "run");

//     assert_ne!(hook_before_apply, hook_after_apply);
//     assert_ne!(pid_before_apply, pid_after_apply);

//     let pid_before_apply = pid_after_apply;
//     let hook_before_apply = hook_after_apply;

//     test_sup.apply_config(r#"run_templated_value = "Running V2""#);
//     utils::sleep_seconds(5);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "run");

//     assert_ne!(hook_before_apply, hook_after_apply);
//     assert_ne!(pid_before_apply, pid_after_apply);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn templated_post_run_hook_causes_restart_on_config_application() {
//     let hab_root = utils::HabRoot::new("hook_only_packages_restart_on_config_application");

//     let origin_name = "sup-integration-test";
//     let package_name = "config-and-hooks-no-reconfigure";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let pid_before_apply = hab_root.pid_of(package_name);
//     let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "post-run");

//     test_sup.apply_config(r#"post_run_templated_value = "Post Run V1""#);
//     utils::sleep_seconds(5);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "post-run");

//     assert_ne!(hook_before_apply, hook_after_apply);
//     assert_ne!(pid_before_apply, pid_after_apply);

//     let pid_before_apply = pid_after_apply;
//     let hook_before_apply = hook_after_apply;

//     test_sup.apply_config(r#"post_run_templated_value = "Post Run V2""#);
//     utils::sleep_seconds(5);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "post-run");

//     assert_ne!(hook_before_apply, hook_after_apply);
//     assert_ne!(pid_before_apply, pid_after_apply);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn templated_health_check_hook_does_not_cause_restart_on_config_application() {
//     let hab_root = utils::HabRoot::new("hook_only_packages_restart_on_config_application");

//     let origin_name = "sup-integration-test";
//     let package_name = "no-configs-only-hooks";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let pid_before_apply = hab_root.pid_of(package_name);
//     let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");

//     test_sup.apply_config(r#"health_check_hook_value = "something new and different""#);
//     utils::sleep_seconds(5);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");

//     assert_ne!(hook_before_apply, hook_after_apply);
//     assert_eq!(pid_before_apply, pid_after_apply);

//     let pid_before_apply = pid_after_apply;
//     let hook_before_apply = hook_after_apply;

//     test_sup.apply_config(r#"health_check_hook_value = "something even better""#);
//     utils::sleep_seconds(5);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");

//     assert_ne!(hook_before_apply, hook_after_apply);
//     assert_eq!(pid_before_apply, pid_after_apply);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn config_files_change_but_hooks_do_not_still_restarts() {
//     let hab_root = utils::HabRoot::new("config_files_change_but_hooks_do_not_still_restarts");

//     let origin_name = "sup-integration-test";
//     let package_name = "config-changes-hooks-do-not";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let pid_before_apply = hab_root.pid_of(package_name);
//     let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
//     let config_before_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

//     test_sup.apply_config(
//                           r#"
// config_value = "applied"
// hook_value = "default"
// "#,
//     );
//     utils::sleep_seconds(2);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
//     let config_after_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

//     assert_ne!(config_before_apply, config_after_apply);
//     assert_eq!(hook_before_apply, hook_after_apply);
//     assert_ne!(pid_before_apply, pid_after_apply);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn hooks_change_but_config_files_do_not_still_restarts() {
//     let hab_root = utils::HabRoot::new("hooks_change_but_config_files_do_not_still_restarts");

//     let origin_name = "sup-integration-test";
//     let package_name = "hook-changes-config-does-not";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let pid_before_apply = hab_root.pid_of(package_name);
//     let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
//     let config_before_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

//     test_sup.apply_config(
//                           r#"
// config_value = "default"
// hook_value = "applied"
// "#,
//     );
//     utils::sleep_seconds(2);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
//     let config_after_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

//     assert_eq!(config_before_apply, config_after_apply);
//     assert_ne!(hook_before_apply, hook_after_apply);
//     assert_ne!(pid_before_apply, pid_after_apply);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn applying_identical_configuration_results_in_no_changes_and_no_restart() {
//     let hab_root = utils::HabRoot::new(
//         "applying_identical_configuration_results_in_no_changes_and_no_restart",
//     );

//     let origin_name = "sup-integration-test";
//     let package_name = "no-changes-no-restart";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let pid_before_apply = hab_root.pid_of(package_name);
//     let hook_before_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
//     let config_before_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

//     test_sup.apply_config(
//                           r#"
// config_value = "default"
// hook_value = "default"
// "#,
//     );
//     utils::sleep_seconds(2);

//     let pid_after_apply = hab_root.pid_of(package_name);
//     let hook_after_apply = hab_root.compiled_hook_contents(&package_name, "health-check");
//     let config_after_apply = hab_root.compiled_config_contents(&package_name, "config.toml");

//     assert_eq!(config_before_apply, config_after_apply);
//     assert_eq!(hook_before_apply, hook_after_apply);
//     assert_eq!(pid_before_apply, pid_after_apply);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn install_hook_success() {
//     let hab_root = utils::HabRoot::new("install_hook_success");

//     let origin_name = "sup-integration-test";
//     let package_name = "install-hook-succeeds";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_before = hab_root.install_status_created(origin_name, package_name);

//     assert_eq!(hab_root.install_status_of(origin_name, package_name), 0);
//     assert!(hab_root.pid_of(package_name) > 0);

//     test_sup.stop();
//     utils::sleep_seconds(3);
//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_after = hab_root.install_status_created(origin_name, package_name);

//     assert_eq!(status_created_before, status_created_after);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn package_with_successful_install_hook_in_dependency_is_loaded() {
//     let hab_root =
//         utils::HabRoot::new("package_with_successful_install_hook_in_dependency_is_loaded");

//     let origin_name = "sup-integration-test";
//     let package_name = "install-hook-succeeds-with-dependency";
//     let dep = "install-hook-succeeds";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_before = hab_root.install_status_created(origin_name, dep);

//     assert_eq!(hab_root.install_status_of(origin_name, dep), 0);
//     assert!(hab_root.pid_of(package_name) > 0);

//     test_sup.stop();
//     utils::sleep_seconds(3);
//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_after = hab_root.install_status_created(origin_name, dep);

//     assert_eq!(status_created_before, status_created_after);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn install_hook_fails() {
//     let hab_root = utils::HabRoot::new("install_hook_fails");

//     let origin_name = "sup-integration-test";
//     let package_name = "install-hook-fails";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_before = hab_root.install_status_created(origin_name, package_name);
//     let result = std::panic::catch_unwind(|| hab_root.pid_of(package_name));

//     assert_eq!(hab_root.install_status_of(origin_name, package_name), 1);
//     assert!(result.is_err());

//     test_sup.stop();
//     utils::sleep_seconds(3);
//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_after = hab_root.install_status_created(origin_name, package_name);

//     assert_ne!(status_created_before, status_created_after);
// }

// #[test]
// #[cfg_attr(feature = "ignore_integration_tests", ignore)]
// fn package_with_failing_install_hook_in_dependency_is_not_loaded() {
//     let hab_root =
//         utils::HabRoot::new("package_with_failing_install_hook_in_dependency_is_not_loaded");

//     let origin_name = "sup-integration-test";
//     let package_name = "install-hook-fails-with-dependency";
//     let dep = "install-hook-fails";
//     let service_group = "default";

//     utils::setup_package_files(origin_name,
//                                package_name,
//                                service_group,
//                                &FIXTURE_ROOT,
//                                &hab_root);

//     let mut test_sup = utils::TestSup::new_with_random_ports(&hab_root,
//                                                              origin_name,
//                                                              package_name,
//                                                              service_group,
//                                                              10,
//                                                              30,
//                                                              60);

//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_before = hab_root.install_status_created(origin_name, dep);
//     let result = std::panic::catch_unwind(|| hab_root.pid_of(package_name));

//     assert_eq!(hab_root.install_status_of(origin_name, dep), 1);
//     assert!(result.is_err());

//     test_sup.stop();
//     utils::sleep_seconds(3);
//     test_sup.start();
//     utils::sleep_seconds(3);

//     let status_created_after = hab_root.install_status_created(origin_name, dep);

//     assert_ne!(status_created_before, status_created_after);
// }
