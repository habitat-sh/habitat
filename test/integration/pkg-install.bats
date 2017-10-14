#!/usr/bin/env bats

load 'helpers'

setup() {
    reset_hab_root
}

@test "hab pkg install: origin/name (standalone service)" {
    run ${hab} pkg install core/redis
    assert_success

    latest_redis=$(latest_from_builder core/redis stable)
    assert_package_and_deps_installed "${latest_redis}"
}

@test "hab pkg install: origin/name/version (standalone service)" {
    run ${hab} pkg install core/redis/3.2.4
    assert_success

    latest_redis=$(latest_from_builder core/redis/3.2.4 stable)
    assert_package_and_deps_installed "${latest_redis}"
}

@test "hab pkg install: origin/name/version/release (standalone service)" {
    desired_version="core/redis/3.2.3/20160920131015"

    run ${hab} pkg install "${desired_version}"
    assert_success
    assert_package_and_deps_installed "${desired_version}"
}

@test "hab pkg install: local hart file (standalone service)" {
    desired_version="core/redis/3.2.4/20170514150022"

    # First, grab a hart file! Then set it aside and clean everything
    # out of /hab. This way, we'll have a hart file, but nothing else,
    # which is exactly what we want.
    run ${hab} pkg install "${desired_version}"
    assert_success
    cp $(cached_artifact_for "${desired_version}") /tmp
    reset_hab_root

    # Now, install from just the local hart file
    run ${hab} pkg install /tmp/core-redis-3.2.4-20170514150022-x86_64-linux.hart
    assert_success
    assert_package_and_deps_installed ${desired_version}
}

@test "hab pkg install: local hart from /hab/cache/artifacts (standalone service)" {
    desired_version="core/redis/3.2.4/20170514150022"

    # First, grab a hart file!
    run ${hab} pkg install "${desired_version}"
    assert_success
    # We don't want to remove everything in /hab, because we want the
    # artifact cache to remain.
    remove_installed_packages
    empty_key_cache

    # Now install from the local hart file *from the cache*
    run ${hab} pkg install "$(cached_artifact_for ${desired_version})"
    assert_success
    assert_package_and_deps_installed "${desired_version}"
}

@test "hab pkg install: installing from a file that isn't a hart fails" {
    echo "lolwut" > /tmp/not-a.hart

    run ${hab} pkg install /tmp/not-a.hart
    assert_failure
    [[ "$output" =~ "Can't read keyname" ]]
}

@test "hab pkg install: installing from a nonexistent file that looks a hart fails" {
    run ${hab} pkg install looks-like/an-ident.hart
    assert_failure
    [[ "$output" =~ "File not found at: looks-like/an-ident.hart" ]]
}
