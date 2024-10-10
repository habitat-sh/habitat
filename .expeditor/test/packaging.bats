#!/usr/bin/env bats

load 'helpers'
load "/test/.expeditor/scripts/release_habitat/shared.sh"

setup() {
    TEST_TEMP_DIR="$(temp_make)"

    # Since we're creating lots of files in these tests, it's
    # convenient to just change into the temp directory and run things
    # from there. Anything we do will just end up getting cleaned up
    # automatically.
    cd "${TEST_TEMP_DIR}"

    # This directory name will be in the fixtures listing the expected
    # archive contents.
    scratch_dir="stuff"
    mkdir "${scratch_dir}"
}

teardown() {
    temp_del "$TEST_TEMP_DIR"
}

# Assert that the given file exists and is, in fact, gzipped.
assert_targz() {
    local input_file="${1}"
    assert_file_exist "${input_file}"

    run file "${input_file}"
    assert_output --partial "gzip compressed data"
}

# Assert that the given file exists and is, in fact, zipped.
assert_zip() {
    local input_file="${1}"
    assert_file_exist "${input_file}"

    run file "${input_file}"
    assert_output --partial "Zip archive data"
}

# Returns a list of the contents of a .tar.gz file, sorted for
# convenience.
sorted_targz_contents() {
    local input_file="${1}"
    tar -z --list -f "${input_file}" | sort
}

# Returns a list of the contents of a .zip file, sorted for
# convenience.
sorted_zip_contents() {
    local input_file="${1}"
    zipinfo -1 "${input_file}" | sort
}

@test "internal archive directory name is correct" {
    hart=$(download_hart core/hab/0.88.0/20191009204957 x86_64-linux)
    name=$(internal_archive_dir_name "${hart}" x86_64-linux)

    assert_equal "${name}" \
                 hab-0.88.0-20191009204957-x86_64-linux
}

@test "extract Linux hab binary from hart" {
    hart=$(download_hart core/hab/0.88.0/20191009204957 x86_64-linux)

    run extract_hab_binaries_from_hart "${hart}" "${scratch_dir}"
    assert_success

    run ls -1 "${scratch_dir}"
    assert_output "hab"
}

@test "extract windows hab binary and DLLs from hart" {
    hart=$(download_hart core/hab/0.88.0 x86_64-windows)

    run extract_hab_binaries_from_hart "${hart}" "${scratch_dir}"
    assert_success

    run ls -1 "${scratch_dir}"
    assert_output "$(cat ${FIXTURES}/expected_windows_hab_hart_contents.txt)"
}

@test ".tar.gz for Linux is absolutely correct" {
    expected_archive_name="hab-x86_64-linux.tar.gz"

    hart=$(download_hart core/hab/0.88.0/20191009204957 x86_64-linux)

    assert_file_not_exist "${expected_archive_name}"

    run create_archive_from_hart "${hart}" x86_64-linux
    assert_success
    assert_output "${expected_archive_name}"

    assert_targz "${expected_archive_name}"
    run sorted_targz_contents "${expected_archive_name}"
    assert_output "$(cat ${FIXTURES}/expected_hab_0.88.0_linux_archive_contents.txt)"
}

@test ".tar.gz for Linux kernel2 is absolutely correct" {
    expected_archive_name="hab-x86_64-linux-kernel2.tar.gz"

    hart=$(download_hart core/hab/0.88.0/20191009205023 x86_64-linux-kernel2)

    assert_file_not_exist "${expected_archive_name}"

    run create_archive_from_hart "${hart}" x86_64-linux-kernel2
    assert_success
    assert_output "${expected_archive_name}"

    assert_targz "${expected_archive_name}"
    run sorted_targz_contents "${expected_archive_name}"
    assert_output "$(cat ${FIXTURES}/expected_hab_0.88.0_linux_kernel2_archive_contents.txt)"
}

@test ".zip for macOS is absolutely correct" {
    expected_archive_name="hab-x86_64-darwin.zip"

    hart=$(download_hart core/hab/0.88.0/20191009205151 x86_64-darwin)

    assert_file_not_exist "${expected_archive_name}"

    run create_archive_from_hart "${hart}" x86_64-darwin
    assert_success
    assert_output "${expected_archive_name}"

    assert_zip "${expected_archive_name}"
    run sorted_zip_contents "${expected_archive_name}"
    assert_output "$(cat ${FIXTURES}/expected_hab_0.88.0_macos_archive_contents.txt)"
}

@test ".zip for Windows is absolutely correct" {
    expected_archive_name="hab-x86_64-windows.zip"

    hart=$(download_hart core/hab/0.88.0/20191009205851 x86_64-windows)
    assert_file_not_exist "${expected_archive_name}"

    run create_archive_from_hart "${hart}" x86_64-windows
    assert_success
    assert_output "${expected_archive_name}"

    assert_zip "${expected_archive_name}"
    run sorted_zip_contents "${expected_archive_name}"
    assert_output "$(cat ${FIXTURES}/expected_hab_0.88.0_windows_archive_contents.txt)"
}
