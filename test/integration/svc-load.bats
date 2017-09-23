#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_hab_root
}

@test "load a service" {
    run ${hab} svc load core/redis
    assert_success

    latest_redis=$(latest_from_builder core/redis stable)
    assert_package_and_deps_installed "${latest_redis}"

    # TODO: Should we test that the service is running? If so, need to sleep
    assert_spec_exists_for redis
}

@test "load a service with version" {
    run ${hab} svc load core/redis/3.2.4
    assert_success

    latest_redis=$(latest_from_builder core/redis stable)
    assert_package_and_deps_installed "${latest_redis}"
    assert_spec_exists_for redis
}

@test "load a service from a fully-qualified identifier" {
    desired_version="core/redis/3.2.3/20160920131015"
    run ${hab} svc load "${desired_version}"
    assert_success

    assert_package_and_deps_installed "${desired_version}"
    assert_spec_exists_for redis
}

@test "CANNOT load a service from hart file" {
    # First, grab a hart file!
    desired_version="core/redis/3.2.4/20170514150022"
    hart_path=$(download_hart_for "${desired_version}")
    reset_hab_root

    run ${hab} svc load "${hart_path}"
    assert_failure

    [[ "$output" =~ "Installing ${hart_path} from channel 'stable'" ]]
    [[ "$output" =~ "Package not found" ]]
}
