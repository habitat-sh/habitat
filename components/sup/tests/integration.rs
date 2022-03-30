#![cfg(not(windows))]
use crate::utils::FileSystemSnapshot;
use anyhow::Result;
use glob::Pattern;
use habitat_sup::manager::service::ProcessTerminationReason;
use hcore::os::process::Pid;
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
}

/// Helper to test the scenarios where a applying a config parameter causes immediate restarts
async fn test_for_restart_on_config_application(test_name: &str,
                                                package_name: &str,
                                                applied_config: &str,
                                                updated_files: Vec<&str>,
                                                termination_reason: ProcessTerminationReason)
                                                -> Result<()> {
    let origin_name = "sup-integration-test";
    let service_group = "default";

    let service_min_backoff_period = Duration::from_secs(10);
    let service_max_backoff_period = Duration::from_secs(30);
    let service_restart_cooldown_period = Duration::from_secs(60);
    let hab_root = utils::HabRoot::new(test_name);
    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let initial_service =
        test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
                .await?;
    let initial_snapshot =
        FileSystemSnapshot::new(hab_root.svc_dir_path(package_name).as_path()).await?;
    let initial_pid_snapshot = initial_snapshot.file("PID")?;

    test_sup.apply_config(package_name, service_group, applied_config)
            .await?;
    let final_service = test_sup.ensure_service_restarted(initial_service.process.pid.unwrap(),
                                                          package_name,
                                                          service_group,
                                                          Duration::from_secs(30))
                                .await?;

    let final_snapshot =
        FileSystemSnapshot::new(hab_root.svc_dir_path(package_name).as_path()).await?;
    let final_pid_snapshot = final_snapshot.file("PID")?;

    let delta =
        final_snapshot.modifications_since(&initial_snapshot,
                                           vec![Pattern::new("logs/*").unwrap()].as_slice());

    let duration_between_restarts =
        final_pid_snapshot.duration_between_modification(initial_pid_snapshot)
                          .unwrap();
    // Ensure that the restart count is 0, because it gets set to 0 for intentional restarts.
    // Templating induced changes that cause a restart are considered as intentional restarts.
    assert_eq!(final_service.restart_count, 0);
    // Ensure the restart was due to the right reason
    assert_eq!(final_service.last_process_state.unwrap().termination_reason,
               termination_reason);
    // Ensure the time between restarts is less than our configured minimum restart duration
    assert!(duration_between_restarts > Duration::ZERO);
    assert!(duration_between_restarts < service_min_backoff_period);
    assert_eq!(delta.updated(), updated_files);
    assert_eq!(delta.added(), vec![] as Vec<&str>);
    assert_eq!(delta.removed(), vec![] as Vec<&str>);

    test_sup.stop().await?;
    Ok(())
}

/// Helper to test the scenarios where a applying a config parameter do not causes restarts
async fn test_for_no_restart_on_config_application(test_name: &str,
                                                   package_name: &str,
                                                   applied_config: &str,
                                                   updated_files: Vec<&str>)
                                                   -> Result<()> {
    let origin_name = "sup-integration-test";
    let service_group = "default";

    let service_min_backoff_period = Duration::from_secs(5);
    let service_max_backoff_period = Duration::from_secs(20);
    let service_restart_cooldown_period = Duration::from_secs(60);
    let hab_root = utils::HabRoot::new(test_name);
    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let initial_service =
        test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
                .await?;
    let initial_snapshot =
        FileSystemSnapshot::new(hab_root.svc_dir_path(package_name).as_path()).await?;
    let initial_pid_snapshot = initial_snapshot.file("PID")?;

    test_sup.apply_config(package_name, service_group, applied_config)
            .await?;
    let final_service =
        test_sup.ensure_service_has_not_stopped_or_restarted(initial_service.process.pid.unwrap(),
                                                             package_name,
                                                             service_group,
                                                             Duration::from_secs(30))
                .await?;

    let final_snapshot =
        FileSystemSnapshot::new(hab_root.svc_dir_path(package_name).as_path()).await?;
    let final_pid_snapshot = final_snapshot.file("PID")?;

    let delta =
        final_snapshot.modifications_since(&initial_snapshot,
                                           vec![Pattern::new("logs/*").unwrap()].as_slice());

    let duration_between_restarts =
        final_pid_snapshot.duration_between_modification(initial_pid_snapshot)
                          .unwrap();
    // Ensure the time between restarts is 0 because the PID file was never modified
    assert_eq!(initial_service, final_service);
    assert_eq!(duration_between_restarts, Duration::ZERO);
    assert_eq!(delta.updated(), updated_files);
    assert_eq!(delta.added(), vec![] as Vec<&str>);
    assert_eq!(delta.removed(), vec![] as Vec<&str>);

    test_sup.stop().await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn restart_for_templated_config_file() -> Result<()> {
    let test_name = "restart_for_templated_config_file_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"app_name = "Test App""#;
    let updated_files = vec!["PID", "config/app-config.toml"];
    test_for_restart_on_config_application(test_name,
                                           package_name,
                                           applied_config,
                                           updated_files,
                                           ProcessTerminationReason::AppConfigUpdated).await?;

    // Templated config file changes do not cause a restart if a reconfigure hook is present
    let test_name = "restart_for_templated_config_file_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"app_name = "Test App""#;
    let updated_files = vec!["config/app-config.toml"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn restart_for_templated_run_hook() -> Result<()> {
    let test_name = "restart_for_templated_run_hook_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"run_templated_value = "Run Hook Value""#;
    // The run hook gets copied to the main service directory.
    // TODO(JJ): Find out why and mention it here?
    let updated_files = vec!["PID", "hooks/run", "run"];
    test_for_restart_on_config_application(test_name,
                                           package_name,
                                           applied_config,
                                           updated_files,
                                           ProcessTerminationReason::RunHookUpdated).await?;

    let test_name = "restart_for_templated_run_hook_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"run_templated_value = "Run Hook Value""#;
    let updated_files = vec!["PID", "hooks/run", "run"];
    test_for_restart_on_config_application(test_name,
                                           package_name,
                                           applied_config,
                                           updated_files,
                                           ProcessTerminationReason::RunHookUpdated).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn restart_for_templated_post_run_hook() -> Result<()> {
    let test_name = "restart_for_templated_post_run_hook_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"post_run_templated_value = "Post Run Hook Value""#;
    let updated_files = vec!["PID", "hooks/post-run"];
    test_for_restart_on_config_application(test_name,
                                           package_name,
                                           applied_config,
                                           updated_files,
                                           ProcessTerminationReason::PostRunHookUpdated).await?;

    let test_name = "restart_for_templated_post_run_hook_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"post_run_templated_value = "Post Run Hook Value""#;
    let updated_files = vec!["PID", "hooks/post-run"];
    test_for_restart_on_config_application(test_name,
                                           package_name,
                                           applied_config,
                                           updated_files,
                                           ProcessTerminationReason::PostRunHookUpdated).await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn no_restart_for_templated_init_hook() -> Result<()> {
    let test_name = "no_restart_for_templated_init_hook_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"init_templated_value = "Init Hook Value""#;
    let updated_files = vec!["hooks/init"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    let test_name = "no_restart_for_templated_init_hook_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"init_templated_value = "Init Hook Value""#;
    let updated_files = vec!["hooks/init"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

// TODO: Improve this test case to check for restart cooldown once
// we expose restart_attempts via the supervisor http gateway
#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn restart_backoff_for_failed_init_hook() -> Result<()> {
    let test_name = "restart_backoff_for_failed_init_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"
    init_exit_code = 1
    init_sleep = 5
    "#;
    let origin_name = "sup-integration-test";
    let service_group = "default";

    let service_min_backoff_period = Duration::from_secs(10);
    let service_max_backoff_period = Duration::from_secs(20);
    let service_restart_cooldown_period = Duration::from_secs(40);
    let hab_root = utils::HabRoot::new(test_name);
    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
            .await?;

    test_sup.service_stop(origin_name, package_name).await?;
    test_sup.ensure_service_stopped(package_name, service_group, Duration::from_secs(10))
            .await?;

    test_sup.apply_config(package_name, service_group, applied_config)
            .await?;
    // We start the service, but don't wait to ensure it is started, because it never will.
    test_sup.service_start(origin_name, package_name).await?;

    // We give the service time to start and wipe out the previous init log
    tokio::time::sleep(Duration::from_secs(5)).await;

    let initial_snapshot =
        FileSystemSnapshot::new(hab_root.svc_dir_path(package_name).as_path()).await?;
    let mut initial_init_log_snapshot = initial_snapshot.file("logs/init.stdout.log")?.clone();
    let mut attempts = 1;

    // Observe 10 restarts
    while attempts <= 10 {
        let final_init_log_snapshot =
            initial_init_log_snapshot.await_update(Duration::from_secs(60))
                                     .await?;

        let service =
            test_sup.get_service_state(package_name, service_group, Duration::from_secs(5))
                    .await?;

        let duration_between_restarts =
            final_init_log_snapshot.duration_between_modification(&initial_init_log_snapshot)
                                   .unwrap();

        // We ensure the restart was due to an init failure
        assert_eq!(service.last_process_state
                          .as_ref()
                          .unwrap()
                          .termination_reason,
                   ProcessTerminationReason::InitHookFailed);
        // Ensure the time between restarts is greater than our min backoff period
        assert!(duration_between_restarts >= service_min_backoff_period);
        // Ensure the time between restarts is less than our max backoff period with a buffer of 10
        // secs + 5 secs (init_sleep)
        assert!(duration_between_restarts <= service_max_backoff_period + Duration::from_secs(15));
        assert_ne!(final_init_log_snapshot, initial_init_log_snapshot);
        initial_init_log_snapshot = final_init_log_snapshot;
        attempts += 1;
    }

    // We sleep for half the min backoff period to ensure we stop the service mid restart.
    // This is important to test that the service desired state = "Down" is written to the gateway
    // state even if the service is not currently running and is in the middle of a restart.
    tokio::time::sleep(service_min_backoff_period.mul_f32(0.5)).await;
    // Stop the service
    test_sup.service_stop(origin_name, package_name).await?;
    test_sup.ensure_service_stopped(package_name, service_group, Duration::from_secs(10))
            .await?;
    // Make the application succeed again
    let applied_config = r#"
    init_exit_code = 0
    "#;
    test_sup.apply_config(package_name, service_group, applied_config)
            .await?;
    test_sup.service_start(origin_name, package_name).await?;

    // Ensure the service restarts after config applicaiton
    let initial_service =
        test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(30))
                .await?;

    // Ensure the service never restarts again
    let final_service =
        test_sup.ensure_service_has_not_stopped_or_restarted(initial_service.process.pid.unwrap(),
                                                             package_name,
                                                             service_group,
                                                             Duration::from_secs(60))
                .await?;

    assert_eq!(initial_service, final_service);

    test_sup.stop().await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn restart_backoff_for_failed_run_hook() -> Result<()> {
    let test_name = "restart_backoff_for_failed_run_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"
    run_exit_code = 1
    run_sleep = 5
    "#;
    let origin_name = "sup-integration-test";
    let service_group = "default";

    let service_min_backoff_period = Duration::from_secs(5);
    let service_max_backoff_period = Duration::from_secs(20);
    let service_restart_cooldown_period = Duration::from_secs(40);
    let hab_root = utils::HabRoot::new(test_name);
    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let _pid =
        test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
                .await?;
    let initial_snapshot =
        FileSystemSnapshot::new(hab_root.svc_dir_path(package_name).as_path()).await?;
    let mut initial_pid_snapshot = initial_snapshot.file("PID")?.clone();
    let mut attempts = 1;

    // This config application will cause an immediate restart as it modifies a run hook
    test_sup.apply_config(package_name, service_group, applied_config)
            .await?;

    let mut restarts_due_to_update = 0;
    // Observe 10 restarts
    while attempts <= 10 {
        // We count on the 5 secs run sleep to ensure that we don't miss the first PID.
        let final_pid_snapshot = initial_pid_snapshot.await_update(Duration::from_secs(60))
                                                     .await?;

        let service =
            test_sup.get_service_state(package_name, service_group, Duration::from_secs(5))
                    .await?;

        // Ignore the restart due to run hook updated
        if service.last_process_state
                  .as_ref()
                  .unwrap()
                  .termination_reason
           == ProcessTerminationReason::RunHookUpdated
        {
            initial_pid_snapshot = final_pid_snapshot;
            restarts_due_to_update += 1;
            continue;
        }

        let duration_between_restarts =
            final_pid_snapshot.duration_between_modification(&initial_pid_snapshot)
                              .unwrap();

        // We might potentially miss the update restart as it could happen super quick
        assert!(restarts_due_to_update <= 1);
        assert_eq!(service.last_process_state
                          .as_ref()
                          .unwrap()
                          .termination_reason,
                   ProcessTerminationReason::RunHookFailed);
        assert_eq!(service.restart_count, attempts);
        // Ensure the time between restarts is greater than our min backoff period
        assert!(duration_between_restarts >= service_min_backoff_period);
        // Ensure the time between restart is less than the max backoff period with a buffer of 15
        // secs We give 5 secs more to account for the run sleep of 5 secs
        assert!(duration_between_restarts <= service_max_backoff_period + Duration::from_secs(15));
        assert_ne!(final_pid_snapshot, initial_pid_snapshot);
        initial_pid_snapshot = final_pid_snapshot;
        attempts += 1;
    }

    // We sleep for half the min backoff period to ensure we stop the service mid restart.
    // This is important to test that the service desired state = "Down" is written to the gateway
    // state even if the service is not currently running and is in the middle of a restart.
    tokio::time::sleep(service_min_backoff_period.mul_f32(0.5)).await;

    // Make the application succeed again
    let applied_config = r#"
    run_exit_code = 0
    run_sleep = 120
    "#;
    test_sup.apply_config(package_name, service_group, applied_config)
            .await?;
    let pid = initial_pid_snapshot.current_file_content()
                                  .await?
                                  .parse::<Pid>()?;

    // Ensure the service restarts after config applicaiton
    let initial_service = test_sup.ensure_service_restarted(pid,
                                                            package_name,
                                                            service_group,
                                                            Duration::from_secs(30))
                                  .await?;

    // Ensure the service never restarts again
    let final_service =
        test_sup.ensure_service_has_not_stopped_or_restarted(initial_service.process.pid.unwrap(),
                                                             package_name,
                                                             service_group,
                                                             Duration::from_secs(60))
                .await?;

    assert_eq!(initial_service, final_service);

    test_sup.stop().await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn no_restart_for_templated_health_check_hook() -> Result<()> {
    let test_name = "no_restart_for_templated_health_check_hook_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"health_check_templated_value = "Health Check Hook Value""#;
    let updated_files = vec!["hooks/health-check"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    let test_name = "no_restart_for_templated_health_check_hook_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"health_check_templated_value = "Health Check Hook Value""#;
    let updated_files = vec!["hooks/health-check"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn no_restart_for_templated_file_updated_hook() -> Result<()> {
    let test_name = "no_restart_for_templated_file_updated_hook_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"file_updated_templated_value = "File Updated Hook Value""#;
    let updated_files = vec!["hooks/file-updated"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    let test_name = "no_restart_for_templated_file_updated_hook_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"file_updated_templated_value = "File Updated Hook Value""#;
    let updated_files = vec!["hooks/file-updated"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn no_restart_for_templated_suitability_hook() -> Result<()> {
    let test_name = "no_restart_for_templated_suitability_hook_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"suitability_templated_value = "Suitability Hook Value""#;
    let updated_files = vec!["hooks/suitability"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    let test_name = "no_restart_for_templated_suitability_hook_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"suitability_templated_value = "Suitability Hook Value""#;
    let updated_files = vec!["hooks/suitability"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn no_restart_for_templated_post_stop_hook() -> Result<()> {
    let test_name = "no_restart_for_templated_post_stop_hook_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"post_stop_templated_value = "Post Stop Hook Value""#;
    let updated_files = vec!["hooks/post-stop"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    let test_name = "no_restart_for_templated_post_stop_hook_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"post_stop_templated_value = "Post Stop Hook Value""#;
    let updated_files = vec!["hooks/post-stop"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn no_restart_for_templated_reconfigure_hook() -> Result<()> {
    let test_name = "no_restart_for_templated_reconfigure_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"reconfigure_templated_value = "Reconfigure Hook Value""#;
    let updated_files = vec!["hooks/reconfigure"];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn no_restart_if_no_change() -> Result<()> {
    let test_name = "no_restart_if_no_change_without_reconfiguration_hook";
    let package_name = "config-and-hooks-no-reconfigure";
    let applied_config = r#"random_value = "Random Value""#;
    let updated_files = vec![];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    let test_name = "no_restart_if_no_change_with_reconfiguration_hook";
    let package_name = "config-and-hooks-with-reconfigure";
    let applied_config = r#"random_value = "Random Value""#;
    let updated_files = vec![];
    test_for_no_restart_on_config_application(test_name,
                                              package_name,
                                              applied_config,
                                              updated_files).await?;

    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn install_hook_success() -> Result<()> {
    let hab_root = utils::HabRoot::new("install_hook_success");

    let service_min_backoff_period = Duration::from_secs(10);
    let service_max_backoff_period = Duration::from_secs(30);
    let service_restart_cooldown_period = Duration::from_secs(60);

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-succeeds";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    let pid = test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
                      .await?;

    let initial_snapshot =
        FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name, package_name).as_path()).await?;
    let initial_install_hook_status_file = initial_snapshot.file("INSTALL_HOOK_STATUS")?;
    let initial_install_hook_status = initial_install_hook_status_file.current_file_content()
                                                                      .await?
                                                                      .parse::<i32>()?;

    assert_eq!(initial_install_hook_status, 0);

    // Restart the supervisor
    test_sup.stop().await?;
    test_sup.start(Duration::from_secs(10)).await?;

    let restarted_pid =
        test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
                .await?;

    let final_snapshot =
        FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name, package_name).as_path()).await?;
    let final_install_hook_status_file = final_snapshot.file("INSTALL_HOOK_STATUS")?;

    // Ensure that the initial and final state of the INSTALL_HOOK_STATUS file is unchanged
    assert_eq!(initial_install_hook_status_file,
               final_install_hook_status_file);
    assert_ne!(pid, restarted_pid);

    test_sup.stop().await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn install_hook_fails() -> Result<()> {
    let hab_root = utils::HabRoot::new("install_hook_fails");

    let service_min_backoff_period = Duration::from_secs(10);
    let service_max_backoff_period = Duration::from_secs(30);
    let service_restart_cooldown_period = Duration::from_secs(60);

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-fails";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    test_sup.ensure_service_has_failed_to_start(package_name,
                                                service_group,
                                                Duration::from_secs(10))
            .await?;

    let initial_snapshot =
        FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name, package_name).as_path()).await?;
    let initial_install_hook_status_file = initial_snapshot.file("INSTALL_HOOK_STATUS")?;
    let initial_install_hook_status = initial_install_hook_status_file.current_file_content()
                                                                      .await?
                                                                      .parse::<i32>()?;

    assert_eq!(initial_install_hook_status, 1);

    // Restart the supervisor
    test_sup.stop().await?;
    test_sup.start(Duration::from_secs(10)).await?;

    test_sup.ensure_service_has_failed_to_start(package_name,
                                                service_group,
                                                Duration::from_secs(10))
            .await?;

    let final_snapshot =
        FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name, package_name).as_path()).await?;
    let final_install_hook_status_file = final_snapshot.file("INSTALL_HOOK_STATUS")?;
    let final_install_hook_status = final_install_hook_status_file.current_file_content()
                                                                  .await?
                                                                  .parse::<i32>()?;

    // Ensure that the install hook was re-run and the new failure status was written
    assert!(final_install_hook_status_file.duration_between_modification(initial_install_hook_status_file)? > Duration::ZERO);
    assert_eq!(initial_install_hook_status, final_install_hook_status);

    test_sup.stop().await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn package_with_successful_install_hook_in_dependency_is_loaded() -> Result<()> {
    let hab_root = utils::HabRoot::new("install_hook_success");

    let service_min_backoff_period = Duration::from_secs(10);
    let service_max_backoff_period = Duration::from_secs(30);
    let service_restart_cooldown_period = Duration::from_secs(60);

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-succeeds-with-dependency";
    let dependency_name = "install-hook-succeeds";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    let pid = test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
                      .await?;

    let initial_dependency_snapshot =
        FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name, dependency_name)
                                        .as_path()).await?;
    let initial_dependency_install_hook_status_file =
        initial_dependency_snapshot.file("INSTALL_HOOK_STATUS")?;
    let initial_dependency_install_hook_status =
        initial_dependency_install_hook_status_file.current_file_content()
                                                   .await?
                                                   .parse::<i32>()?;

    assert_eq!(initial_dependency_install_hook_status, 0);

    // Restart the supervisor
    test_sup.stop().await?;
    test_sup.start(Duration::from_secs(10)).await?;

    let restarted_pid =
        test_sup.ensure_service_started(package_name, service_group, Duration::from_secs(10))
                .await?;

    let final_dependency_snapshot = FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name,
                                                                                  dependency_name)
                                                                    .as_path()).await?;
    let final_dependency_install_hook_status_file =
        final_dependency_snapshot.file("INSTALL_HOOK_STATUS")?;

    // Ensure that the initial and final state of the INSTALL_HOOK_STATUS file is unchanged
    assert_eq!(initial_dependency_install_hook_status_file,
               final_dependency_install_hook_status_file);
    assert_ne!(pid, restarted_pid);

    test_sup.stop().await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(feature = "ignore_integration_tests", ignore)]
async fn package_with_failing_install_hook_in_dependency_is_not_loaded() -> Result<()> {
    let hab_root =
        utils::HabRoot::new("package_with_failing_install_hook_in_dependency_is_not_loaded");

    let service_min_backoff_period = Duration::from_secs(10);
    let service_max_backoff_period = Duration::from_secs(30);
    let service_restart_cooldown_period = Duration::from_secs(60);

    let origin_name = "sup-integration-test";
    let package_name = "install-hook-fails-with-dependency";
    let dependency_name = "install-hook-fails";
    let service_group = "default";

    utils::setup_package_files(origin_name,
                               package_name,
                               service_group,
                               &FIXTURE_ROOT,
                               &hab_root).await?;

    let mut test_sup =
        utils::TestSup::new_with_random_ports(&hab_root,
                                              service_min_backoff_period,
                                              service_max_backoff_period,
                                              service_restart_cooldown_period).await?;

    test_sup.start(Duration::from_secs(10)).await?;

    test_sup.ensure_service_has_failed_to_start(package_name,
                                                service_group,
                                                Duration::from_secs(10))
            .await?;

    let initial_dependency_snapshot =
        FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name, dependency_name)
                                        .as_path()).await?;
    let initial_dependency_install_hook_status_file =
        initial_dependency_snapshot.file("INSTALL_HOOK_STATUS")?;
    let initial_dependency_install_hook_status =
        initial_dependency_install_hook_status_file.current_file_content()
                                                   .await?
                                                   .parse::<i32>()?;

    assert_eq!(initial_dependency_install_hook_status, 1);

    // Restart the supervisor
    test_sup.stop().await?;
    test_sup.start(Duration::from_secs(10)).await?;

    test_sup.ensure_service_has_failed_to_start(package_name,
                                                service_group,
                                                Duration::from_secs(10))
            .await?;

    let final_dependency_snapshot = FileSystemSnapshot::new(hab_root.pkg_dir_path(origin_name,
                                                                                  dependency_name)
                                                                    .as_path()).await?;
    let final_dependency_install_hook_status_file =
        final_dependency_snapshot.file("INSTALL_HOOK_STATUS")?;
    let final_dependency_install_hook_status =
        final_dependency_install_hook_status_file.current_file_content()
                                                 .await?
                                                 .parse::<i32>()?;

    // Ensure that the install hook was re-run and the new failure status was written
    assert!(final_dependency_install_hook_status_file.duration_between_modification(initial_dependency_install_hook_status_file)? > Duration::ZERO);
    assert_eq!(final_dependency_install_hook_status,
               initial_dependency_install_hook_status);

    test_sup.stop().await?;
    Ok(())
}
