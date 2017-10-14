#!/usr/bin/env bats

load 'helpers'

setup() {
    stop_supervisor
    reset_hab_root
}

teardown() {
    stop_supervisor
}

@test "supervisor: upgrade a service" {
    # This is an old version of Redis; at the time of writing, there
    # is at least a version of 3.2.4 that is in the stable channel, so
    # we'll definitely get something new when we try to upgrade.
    old_redis="core/redis/3.2.3/20161102201135"

    # Start up an empty Supervisor in the background. The update
    # frequency is important for this test, otherwise we'll be waiting
    # too long.
    HAB_UPDATE_STRATEGY_FREQUENCY_MS=5000 ${hab} run &

    # Load up our older Redis and ensure that it's running before
    # going any further
    run ${hab} svc load ${old_redis}
    assert_success
    wait_for_service_to_run redis
    initial_pid=$(pid_of_service redis)
    initial_running_version=$(current_running_version_for redis)
    assert_equal "${initial_running_version}" "${old_redis}"
    assert_spec_exists_for redis

    assert_package_and_deps_installed "${old_redis}"

    # Since we loaded a fully-qualified service, upgrades aren't going
    # to mean anything, because there's never a newer version of a
    # fully-qualified package! We need to reload with something that
    # can be updated (and also set the upgrade strategy, too!)
    run ${hab} svc load --strategy=at-once --force core/redis
    assert_success

    # The first restart is due to the service reloading via `--force`
    wait_for_service_to_restart redis "${initial_pid}"
    reloaded_pid=$(pid_of_service redis)

    wait_for_service_to_restart redis "${reloaded_pid}"
    updated_running_version=$(current_running_version_for redis)

    # Since the latest version of Redis will change as time goes on,
    # we need to ask Builder what we should expect the upgraded
    # version to be.
    latest_redis=$(latest_from_builder core/redis stable)
    assert [ "${initial_running_version}" != "${latest_redis}" ]
    assert [ "${updated_running_version}" = "${latest_redis}" ]
    assert_package_and_deps_installed "${latest_redis}"
}
