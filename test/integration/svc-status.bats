#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_hab_root
}

teardown() {
    stop_supervisor
}

@test "hab svc status: when no supervisor is running" {
    run ${hab} svc status
    assert_failure 3
    assert_output --partial "The Supervisor is not running"
}

@test "hab svc status: when no services are running" {
    background ${hab} run
    sleep 2 # give it a sec to come up
    run ${hab} svc status
    assert_success
    assert_output --partial "No services loaded"
}

@test "hab svc status: for a single running service" {
    background ${hab} svc start core/redis

    wait_for_service_to_run redis

    sleep 3 # give the services.dat file time to be
    # written... otherwise the state can show as down

    run ${hab} svc status core/redis
    assert_success
    assert_output --regexp "core/redis/.*/[0-9]{14} \(standalone\), state:up, time:.*, group:redis\.default, style:transient"
}

@test "hab svc status: for a single service that is not loaded" {
    background ${hab} svc start core/redis

    wait_for_service_to_run redis

    sleep 3 # give the services.dat file time to be
    # written... otherwise the state can show as down

    run ${hab} svc status core/nginx # nginx != redis
    assert_failure 2
    assert_output --partial "core/nginx is not currently loaded"
}

@test "hab svc status: for all running services" {
    background ${hab} run

    run ${hab} svc start core/redis
    assert_success

    ${hab} pkg install core/runit --binlink # whyyyyy
    run ${hab} svc start core/nginx
    assert_success

    wait_for_service_to_run redis
    wait_for_service_to_run nginx

    sleep 3 # let services.dat get written

    run ${hab} svc status
    assert_success

    assert_line --regexp "core/redis/.*/[0-9]{14} \(standalone\), state:up, time:.*, group:redis\.default, style:transient"
    assert_line --regexp "core/nginx/.*/[0-9]{14} \(standalone\), state:up, time:.*, group:nginx\.default, style:transient"
}

@test "hab svc status: shows which composite a service is in" {
    background ${hab} run

    run ${hab} svc start core/redis
    assert_success

    ${hab} pkg install core/runit --binlink # whyyyyy
    run ${hab} svc start core/nginx
    assert_success

    wait_for_service_to_run redis
    wait_for_service_to_run nginx


    # load the composite
    local composite_hart=fixtures/core-builder-tiny-1.0.0-20170930190003-x86_64-linux.hart

    run ${hab} svc load --group=comp "${composite_hart}"
    assert_success

    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy
    wait_for_service_to_run builder-router

    sleep 3 # let services.dat get written

    run ${hab} svc status
    assert_success

    assert_line --regexp "core/redis/.*/[0-9]{14} \(standalone\), state:up, time:.*, group:redis\.default, style:transient"
    assert_line --regexp "core/nginx/.*/[0-9]{14} \(standalone\), state:up, time:.*, group:nginx\.default, style:transient"
    assert_line --regexp "core/builder-router/.*/[0-9]{14} \(builder-tiny\), state:up, time:.*, group:builder-router\.comp, style:persistent"
    assert_line --regexp "core/builder-api/.*/[0-9]{14} \(builder-tiny\), state:up, time:.*, group:builder-api\.comp, style:persistent"
    assert_line --regexp "core/builder-api-proxy/.*/[0-9]{14} \(builder-tiny\), state:up, time:.*, group:builder-api-proxy\.comp, style:persistent"
}

@test "hab svc status: asking for the status of a composite" {
    background ${hab} run

    run ${hab} svc start core/redis
    assert_success

    wait_for_service_to_run redis

    # load the composite
    local composite_hart=fixtures/core-builder-tiny-1.0.0-20170930190003-x86_64-linux.hart
    ${hab} pkg install core/runit --binlink # whyyyyy
    run ${hab} svc load --group=comp "${composite_hart}"
    assert_success

    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy
    wait_for_service_to_run builder-router

    sleep 3 # let services.dat get written

    run ${hab} svc status core/builder-tiny
    assert_success

    # We ONLY show the composite services!
    refute_line --regexp "core/redis/.*/[0-9]{14} \(standalone\), state:up, time:.*, group:redis\.default, style:transient"

    assert_line --regexp "core/builder-router/.*/[0-9]{14} \(builder-tiny\), state:up, time:.*, group:builder-router\.comp, style:persistent"
    assert_line --regexp "core/builder-api/.*/[0-9]{14} \(builder-tiny\), state:up, time:.*, group:builder-api\.comp, style:persistent"
    assert_line --regexp "core/builder-api-proxy/.*/[0-9]{14} \(builder-tiny\), state:up, time:.*, group:builder-api-proxy\.comp, style:persistent"
}
