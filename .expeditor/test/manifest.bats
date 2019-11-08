#!/usr/bin/env bats

load 'helpers'
load "/test/.expeditor/scripts/release_habitat/shared.sh"

setup() {
    # Put create_manifest.rb on the PATH
    PATH="${REPO_ROOT}/.expeditor/scripts/release_habitat:${PATH}"
    TEST_TEMP_DIR="$(temp_make)"
    cd "${TEST_TEMP_DIR}"
}
teardown() {
    temp_del "$TEST_TEMP_DIR"
}

@test "manifest generation works properly" {
    assert_file_not_exist "manifest.json"
    run create_manifest.rb \
        "${FIXTURES}/sample_manifest_input.txt" \
        "0.88.0" \
        "badbeefcafe"
    assert_success
    assert_file_exist "manifest.json"

    run cat "manifest.json"
    assert_output "$(cat "${FIXTURES}/expected_manifest.json")"
}
