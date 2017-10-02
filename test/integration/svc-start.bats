#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_hab_root
}

teardown() {
    stop_supervisor
}

@test "hab svc start: origin/name (standalone service)" {
    background ${hab} svc start core/redis
    wait_for_service_to_run redis

    latest_redis=$(latest_from_builder core/redis stable)
    assert_package_and_deps_installed "${latest_redis}"
    assert_service_running "${latest_redis}"

    assert_spec_value redis ident core/redis
    assert_spec_value redis group default
    assert_spec_value redis start_style transient
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.habitat.sh"
}

@test "hab svc start: origin/name/version (standalone service)" {
    background ${hab} svc start core/redis/3.2.4
    wait_for_service_to_run redis

    latest_redis=$(latest_from_builder core/redis/3.2.4 stable)
    assert_package_and_deps_installed "${latest_redis}"
    assert_service_running "${latest_redis}"

    assert_spec_value redis ident core/redis/3.2.4
    assert_spec_value redis group default
    assert_spec_value redis start_style transient
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.habitat.sh"
}

@test "hab svc start: origin/name/version/release (standalone service)" {
    desired_version="core/redis/3.2.3/20160920131015"
    background ${hab} svc start "${desired_version}"
    wait_for_service_to_run redis

    assert_package_and_deps_installed "${desired_version}"
    assert_service_running "${desired_version}"

    assert_spec_value redis ident "${desired_version}"
    assert_spec_value redis group default
    assert_spec_value redis start_style transient
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.habitat.sh"
}

@test "hab svc start: local hart file (standalone service)" {
    desired_version="core/redis/3.2.4/20170514150022"

    # First, grab a hart file! Then, because we're using hab to
    # download the file, reset the hab root, just to simulate the case
    # of starting with nothing but a hart file.
    hart_path=$(download_hart_for "${desired_version}")
    reset_hab_root

    background ${hab} svc start "${hart_path}"
    wait_for_service_to_run redis

    assert_package_and_deps_installed "${desired_version}"
    assert_service_running "${desired_version}"

    assert_spec_value redis ident "${desired_version}"
    assert_spec_value redis group default
    assert_spec_value redis start_style transient
    assert_spec_value redis channel stable
    assert_spec_value redis topology standalone
    assert_spec_value redis update_strategy none
    assert_spec_value redis desired_state up
    assert_spec_value redis bldr_url "https://bldr.habitat.sh"
}

@test "hab svc start: prefer local packages over pulling from Builder" {
    desired_version="core/redis/3.2.3/20160920131015"
    # Pre-install this older package. Starting the service should not cause a
    # newer package to be installed.
    run ${hab} pkg install "${desired_version}"
    background ${hab} svc start "core/redis"
    wait_for_service_to_run redis

    assert_package_and_deps_installed "${desired_version}"
    assert_service_running "${desired_version}"
}
