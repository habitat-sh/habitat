#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_hab_root
}

teardown() {
    stop_supervisor
}

# TODO (CM): Need to come up with a dedicated, stable small composite
# to test with than part of Builder.
composite_ident="core/builder-tiny/1.0.0/20170930190003"
composite_hart=fixtures/core-builder-tiny-1.0.0-20170930190003-x86_64-linux.hart
composite_short_ident="core/builder-tiny"
composite_name="builder-tiny"

@test "hab pkg install: local hart file (composite services)" {
    skip "Composites will be going away soon"
    run ${hab} pkg install "${composite_hart}"
    assert_success
    assert_composite_and_services_are_installed "${composite_ident}"
}

@test "hab pkg install: trying to binlink with a composite doesn't blow up" {
    skip "Composites will be going away soon"
    run ${hab} pkg install "${composite_hart}"
    assert_success
    assert_composite_and_services_are_installed "${composite_ident}"
}

@test "hab svc load: local hart file (composite services)" {
    skip "Composites will be going away soon"
    ${hab} svc load "${composite_hart}"
    assert_success

    assert_composite_and_services_are_installed "${composite_ident}"

    assert_composite_spec "${composite_ident}"
    for service in $(services_for_composite "${composite_ident}"); do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"

        assert_spec_value "${service_name}" ident "${service}"
        assert_spec_value "${service_name}" group default
        assert_spec_value "${service_name}" composite "${composite_name}"
        assert_spec_value "${service_name}" channel stable
        assert_spec_value "${service_name}" topology standalone
        assert_spec_value "${service_name}" update_strategy none
        assert_spec_value "${service_name}" desired_state up
        assert_spec_value "${service_name}" bldr_url "https://bldr.habitat.sh"
    done
}

@test "hab svc load: application/environment apply to all composite services" {
    skip "Composites will be going away soon"
    run ${hab} svc load --application=skunkworks --environment=dev "${composite_hart}"
    assert_success

    for service in $(services_for_composite "${composite_ident}"); do
        service_name=$(name_from_ident "${service}")
        assert_spec_value "${service_name}" application_environment "skunkworks.dev"
    done
}

@test "hab svc load: reload a composite using --force, without changing the composite ident" {
    skip "Composites will be going away soon"
    run ${hab} svc load --channel=unstable "${composite_hart}"
    assert_success

    assert_composite_and_services_are_installed "${composite_ident}"

    assert_composite_spec "${composite_ident}"
    for service in $(services_for_composite "${composite_ident}"); do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"

        assert_spec_value "${service_name}" ident "${service}"
        assert_spec_value "${service_name}" group default
        assert_spec_value "${service_name}" composite "${composite_name}"
        assert_spec_value "${service_name}" channel unstable
        assert_spec_value "${service_name}" topology standalone
        assert_spec_value "${service_name}" update_strategy none
        assert_spec_value "${service_name}" desired_state up
        assert_spec_value "${service_name}" bldr_url "https://bldr.habitat.sh"
    done

    # Note that we're reloading *by ident* a composite we loaded from
    # a .hart and it's working; we shouldn't need to go out to Builder
    # just to change specs.
    run ${hab} svc load --force --group=zzzz "${composite_ident}"

    assert_composite_spec "${composite_ident}" # <-- should be same
    for service in $(services_for_composite "${composite_ident}"); do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"

        assert_spec_value "${service_name}" ident "${service}"
        assert_spec_value "${service_name}" group zzzz # <-- all should have switched
        assert_spec_value "${service_name}" composite "${composite_name}"
        assert_spec_value "${service_name}" channel unstable
        assert_spec_value "${service_name}" topology standalone
        assert_spec_value "${service_name}" update_strategy none
        assert_spec_value "${service_name}" desired_state up
        assert_spec_value "${service_name}" bldr_url "https://bldr.habitat.sh"
    done
}

@test "hab svc load: reload a composite using --force, without changing binds or composite ident, should preserve existing binds, including extra-composite binds" {
    skip "Composites will be going away soon"
    background ${hab} run

    run ${hab} pkg install core/runit --binlink
    assert_success

    # This is the version of router that was current when the test
    # composite was built.
    run ${hab} svc load --group=outside core/builder-router/5131/20170923114145
    assert_success

    wait_for_service_to_run builder-router

    # Now that the router is present, let's load the API-only
    # composite. Inside the composite, one service will bind to the
    # other service, but the other service itself needs to bind to the
    # router, which is outside the composite.
    run ${hab} svc load \
        --bind=builder-api:router:builder-router.outside \
        fixtures/core-builder-api-only-1.0.0-20171001023721-x86_64-linux.hart
    assert_success

    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy

    assert_spec_value builder-api channel "stable"
    assert_spec_value builder-api binds '["router:builder-router.outside"]'
    assert_spec_value builder-api-proxy channel "stable"
    assert_spec_value builder-api-proxy binds '["http:builder-api.default"]'

    # OK, here's where the actual test begins (whew!)
    #
    # We've got a composite, and it's got extra binds for one of the
    # services. If we do a force load of the composite to, say, change
    # the update strategy, then the binds should remain in place (just
    # as they would if this were a standalone service we were
    # force-loading without changing any binds).

    # We need to use the the exact ident, since we installed from a
    # hart :/
    run ${hab} svc load --channel=unstable --force core/builder-api-only/1.0.0/20171001023721
    assert_success

    assert_spec_value builder-api channel "unstable"
    assert_spec_value builder-api binds '["router:builder-router.outside"]' # <-- if this isn't here, the test failed
    assert_spec_value builder-api-proxy channel "unstable"
    assert_spec_value builder-api-proxy binds '["http:builder-api.default"]'
}

@test "hab svc load: reload a composite using --force, changing the ident, can change which services are running" {
    skip "Composites will be going away soon"
    # v1 contains the router, api, and api-proxy services
    # v2 contains the router, admin, and admin-proxy services
    #
    # Thus, doing a force-load from v1 to v2 should remove api and
    # api-proxy services, while adding admin and admin-proxy services.
    v1_hart="fixtures/core-builder-tiny-1.0.0-20171001014549-x86_64-linux.hart"
    v1_ident="core/builder-tiny/1.0.0/20171001014549"
    v2_hart="fixtures/core-builder-tiny-2.0.0-20171001014611-x86_64-linux.hart"
    v2_ident="core/builder-tiny/2.0.0/20171001014611"

    run ${hab} svc load --channel=unstable "${v1_hart}"
    assert_success

    assert_composite_and_services_are_installed "${v1_ident}"
    assert_composite_spec "${v1_ident}"

    assert_spec_exists_for "builder-api"
    assert_spec_exists_for "builder-api-proxy"
    assert_spec_exists_for "builder-router"
    assert_spec_not_exists_for "builder-admin"
    assert_spec_not_exists_for "builder-admin-proxy"

    for service in $(services_for_composite "${v1_ident}"); do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"

        assert_spec_value "${service_name}" ident "${service}"
        assert_spec_value "${service_name}" group default
        assert_spec_value "${service_name}" composite "${composite_name}"
        assert_spec_value "${service_name}" channel unstable
        assert_spec_value "${service_name}" topology standalone
        assert_spec_value "${service_name}" update_strategy none
        assert_spec_value "${service_name}" desired_state up
        assert_spec_value "${service_name}" bldr_url "https://bldr.habitat.sh"
    done

    # HERE'S WHERE THE MAGIC HAPPENS
    run ${hab} svc load --force --group=zzzz "${v2_hart}"
    assert_success

    assert_composite_spec "${v2_ident}"
    for service in $(services_for_composite "${composite_ident}"); do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"

        assert_spec_value "${service_name}" ident "${service}"
        assert_spec_value "${service_name}" group zzzz # <-- all should have switched
        assert_spec_value "${service_name}" composite "${composite_name}"
        assert_spec_value "${service_name}" channel unstable
        assert_spec_value "${service_name}" topology standalone
        assert_spec_value "${service_name}" update_strategy none
        assert_spec_value "${service_name}" desired_state up
        assert_spec_value "${service_name}" bldr_url "https://bldr.habitat.sh"
    done

    # Assert that the old services have no spec, but the new ones are up!
    assert_spec_not_exists_for "builder-api"
    assert_spec_not_exists_for "builder-api-proxy"
    assert_spec_exists_for "builder-router"
    assert_spec_exists_for "builder-admin"
    assert_spec_exists_for "builder-admin-proxy"
}

@test "hab svc unload: composite services" {
    skip "Composites will be going away soon"
    # Load a composite and two other standalone services and verify
    # all specs are in place
    ########################################################################
    run ${hab} svc load "${composite_hart}"
    assert_success

    run ${hab} svc load core/redis
    assert_success

    run ${hab} svc load core/nginx
    assert_success

    all_composite_services=($(services_for_composite "${composite_ident}"))

    # Verify all the specs are there
    assert_composite_spec "${composite_ident}"
    for service in "${all_composite_services[@]}"; do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"
    done

    # Redis and Nginx are there, too!
    assert_spec_exists_for redis
    assert_spec_exists_for nginx

    # Unload nginx now; everything else should remain
    ########################################################################
    run ${hab} svc unload core/nginx
    assert_success
    assert_file_not_exist $(spec_file_for nginx)

    # All the composite services are still there
    assert_composite_spec "${composite_ident}"
    for service in "${all_composite_services[@]}"; do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"
    done

    # Redis, too!
    assert_spec_exists_for redis

    # Now, unload the composite
    ########################################################################
    run ${hab} svc unload "${composite_short_ident}"
    assert_success

    # Show that all the specs are gone
    assert_file_not_exist $(composite_spec_file_for builder)
    for service in "${all_composite_services[@]}"; do
        service_name=$(name_from_ident "${service}")
        assert_file_not_exist $(spec_file_for "${service_name}")
    done

    # Redis is still there, though!
    assert_spec_exists_for redis
}

@test "hab svc stop: composite services" {
    skip "Composites will be going away soon"
    # Load a composite and two other standalone services and verify
    # all specs are in place
    ########################################################################
    run ${hab} svc load "${composite_hart}"
    assert_success

    run ${hab} svc load core/redis
    assert_success

    run ${hab} svc load core/nginx
    assert_success

    all_composite_services=($(services_for_composite "${composite_ident}"))
    # Verify all the specs are there, and that their desired state is "up"
    assert_composite_spec "${composite_ident}"
    for service in "${all_composite_services[@]}"; do
        service_name=$(name_from_ident "${service}")
        assert_spec_exists_for "${service_name}"
        assert_spec_value "${service_name}" desired_state up
    done

    # Redis and Nginx are there, too!
    assert_spec_exists_for redis
    assert_spec_value redis desired_state up
    assert_spec_exists_for nginx
    assert_spec_value nginx desired_state up

    # Stop nginx; show that it's down and everything else remains up
    ########################################################################
    run ${hab} svc stop core/nginx
    assert_success
    assert_spec_exists_for nginx
    assert_spec_value nginx desired_state down

    # Composite services are still up
    for service in "${all_composite_services[@]}"; do
        service_name=$(name_from_ident "${service}")
        assert_spec_value "${service_name}" desired_state up
    done

    # So is redis
    assert_spec_value redis desired_state up

    # Stop the composite; redis should stay up
    ########################################################################

    run ${hab} svc stop "${composite_short_ident}"
    assert_success

    # Composite services are DOWN
    for service in "${all_composite_services[@]}"; do
        service_name=$(name_from_ident "${service}")
        assert_spec_value "${service_name}" desired_state down
    done

    # Redis is still up!
    assert_spec_value redis desired_state up

    # (Just for kicks, nginx should still be down)
    assert_spec_value nginx desired_state down
}

@test "hab svc start: composite services" {
    skip "Composites will be going away soon"
    # Need to install runit (for chpst) to get builder-tiny working!
    # (a packaged supervisor would have this already)
    ${hab} pkg install core/runit --binlink

    background ${hab} svc start "${composite_hart}"

    # TODO (CM): Need to pull these services from the actual list from
    # the composite. Perhaps a test helper that waits until the
    # SERVICES file exists, then polls until all are up?
    wait_for_service_to_run builder-router
    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy

    # Could also wait for the composite spec to be present for asserting
    # that everything got installed
    assert_composite_and_services_are_installed "${composite_ident}"

    assert_composite_spec "${composite_ident}"
}

@test "hab svc start: restart stopped composite services" {
    skip "Composites will be going away soon"
    ${hab} pkg install core/runit --binlink
    background ${hab} run

    run ${hab} svc load "${composite_hart}"
    assert_success

    wait_for_service_to_run builder-router
    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy

    run ${hab} svc stop "${composite_short_ident}"
    assert_success

    wait_for_service_to_die builder-router
    wait_for_service_to_die builder-api
    wait_for_service_to_die builder-api-proxy

    run ${hab} svc start "${composite_short_ident}"
    assert_success

    wait_for_service_to_run builder-router
    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy
}

@test "hab svc load: composite binds for just service groups are generated and valid" {
    skip "Composites will be going away soon"
    run ${hab} pkg install core/runit --binlink
    background ${hab} run

    run ${hab} svc load "${composite_hart}"
    assert_success

    wait_for_service_to_run builder-router
    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy

    assert_spec_value builder-router binds "[]"
    assert_spec_value builder-api binds '["router:builder-router.default"]'
    assert_spec_value builder-api-proxy binds '["http:builder-api.default"]'
}

@test "hab svc load: composite binds for service group + app/env are generated and valid" {
    skip "Composites will be going away soon"
    run ${hab} pkg install core/runit --binlink
    background ${hab} run

    run ${hab} svc load --group=default --application=finn --environment=candykingdom "${composite_hart}"
    assert_success

    wait_for_service_to_run builder-router
    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy

    assert_spec_value builder-router binds "[]"
    assert_spec_value builder-api binds '["router:finn.candykingdom#builder-router.default"]'
    assert_spec_value builder-api-proxy binds '["http:finn.candykingdom#builder-api.default"]'
}

@test "hab svc load: composite with additional extra-composite binds" {
    skip "Composites will be going away soon"
    background ${hab} run

    run ${hab} pkg install core/runit --binlink
    assert_success

    # This is the version of router that was current when the test
    # composite was built.
    run ${hab} svc load --group=outside core/builder-router/5131/20170923114145
    assert_success

    wait_for_service_to_run builder-router

    # Now that the router is present, let's load the API-only
    # composite. Inside the composite, one service will bind to the
    # other service, but the other service itself needs to bind to the
    # router, which is outside the composite.
    run ${hab} svc load \
        --bind=builder-api:router:builder-router.outside \
        fixtures/core-builder-api-only-1.0.0-20171001023721-x86_64-linux.hart
    assert_success

    wait_for_service_to_run builder-api
    wait_for_service_to_run builder-api-proxy

    assert_spec_value builder-api binds '["router:builder-router.outside"]'
    assert_spec_value builder-api-proxy binds '["http:builder-api.default"]'
}

@test "hab svc load: two-part binds on the CLI are not accepted for composites" {
    skip "Composites will be going away soon"
    # This is the version of router that was current when the test
    # composite was built.
    run ${hab} svc load --group=outside core/builder-router/5131/20170923114145
    assert_success

    run ${hab} svc load \
        --bind=router:builder-router.outside \
        fixtures/core-builder-api-only-1.0.0-20171001023721-x86_64-linux.hart
    assert_failure
    # You gotta use 3-part binds for composites!
    assert_line --partial 'Invalid binding "router:builder-router.outside"'
}

# This is tangentially related to composites, in that the notion of
# 3-part binds came in with composites.
@test "hab svc load: three-part binds on the CLI are not accepted for standalone services" {
    skip "Composites will be going away soon"
    # This particular version of builder-api has a single bind: "router"
    run ${hab} svc load \
        --bind=builder-api:router:builder-router.default \
        core/builder-api/5326/20170930215921
    assert_failure
    assert_line --partial 'Invalid binding "builder-api:router:builder-router.default"'
}
