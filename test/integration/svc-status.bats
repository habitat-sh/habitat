#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_hab_root
    start_supervisor
}

teardown() {
    stop_supervisor
}

@test "hab svc status: when no supervisor is running" {
    stop_supervisor

    run ${hab} svc status
    assert_failure 1
    assert_output --partial "Unable to contact the Supervisor."
}

@test "hab svc status: when no services are running" {
    run ${hab} svc status
    assert_success
    assert_output --partial "No services loaded"
}

@test "hab svc status: for a single running service" {
    run ${hab} svc load core/redis

    wait_for_service_to_run redis

    sleep 3 # give the services.dat file time to be
    # written... otherwise the state can show as down

    run ${hab} svc status core/redis
    assert_success

    # OUTPUT:
    # package                           type        desired  state  elapsed (s)  pid   group
    # core/redis/4.0.10/20180801003001  standalone  up       up     3            1016  redis.default
    assert_line --regexp "core/redis/.*/[0-9]{14}\s+standalone\s+up\s+up\s+.*redis.default"
}

@test "hab svc status: for a single service that is not loaded" {
    run ${hab} svc load core/redis

    wait_for_service_to_run redis

    sleep 3 # give the services.dat file time to be
            # written... otherwise the state can show as down

    run ${hab} svc status core/nginx # nginx != redis
    assert_failure 1
    assert_output --partial "Service not loaded, core/nginx"
}

@test "hab svc status: for all running services" {
    run ${hab} svc load core/redis
    assert_success

    ${hab} pkg install core/runit --binlink # whyyyyy
    run ${hab} svc load core/nginx
    assert_success

    wait_for_service_to_run redis
    wait_for_service_to_run nginx

    sleep 3 # let services.dat get written

    run ${hab} svc status
    assert_success

    assert_line --regexp "core/redis/.*/[0-9]{14}\s+standalone\s+up\s+up\s+.*redis.default"
    assert_line --regexp "core/nginx/.*/[0-9]{14}\s+standalone\s+up\s+up\s+.*nginx.default"
}
