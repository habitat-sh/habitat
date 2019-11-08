#!/usr/bin/env bats

load 'helpers'
load "/test/.expeditor/scripts/release_habitat/shared.sh"

setup() {
    TEST_TEMP_DIR="$(temp_make)"
}
teardown() {
    temp_del "$TEST_TEMP_DIR"
}

@test "get_release_channel incorporates BUILDKITE_BUILD_ID" {
    BUILDKITE_BUILD_ID="1234-5679-abcd"
    assert_equal $(get_release_channel) "habitat-release-1234-5679-abcd"
}

@test "get_version_from_repo for real releases" {
    assert_equal $(get_version_from_repo) $(cat "VERSION")
}

@test "extract version from a hart" {
    hart=$(download_hart core/hab/0.88.0 x86_64-linux)
    assert_equal "$(get_version_from_hart "${hart}")" "0.88.0"
}

@test "extract release from a hart" {
    hart=$(download_hart core/hab/0.88.0/20191009204957 x86_64-linux)
    assert_equal "$(get_release_from_hart "${hart}")" "20191009204957"
}
