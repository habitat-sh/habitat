#!/bin/bash
#
# Basic set of tests for the hab pkg bulkupload command
#
# Assumptions:
# 1. PIPELINE_HAB_AUTH_TOKEN and PIPELINE_HAB_BLDR_URL environment variables are set and valid
# 2. ${CACHE_DIR} can be set to a writable location on the filesystem
# 3. non zero exit code from each command implies success

# TODO: Future improvement would be to create a clean room Builder or origin.
# Doing so would allow for more accurate validation of the bulkupload by interrogating
# the Builder. Of course this introduces significant setup time
# cost... For now, we'll want to point to the Acceptance Builder, not Production.

set -euo pipefail

export HAB_NOCOLORING=true
export HAB_NONINTERACTIVE=true
export HAB_ORIGIN="habitat-testing"
export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"
export HAB_AUTH_TOKEN="${ACCEPTANCE_PIPELINE_HAB_AUTH_TOKEN}"
export BUILD_PKG_TARGET=x86_64-linux
unset HAB_BLDR_CHANNEL

HAB=${HAB_TEST_CMD:-hab}
CACHE_DIR="test-cache"
export HAB_CACHE_KEY_PATH="${CACHE_DIR}"/keys
FIXTURES_DIR="test/fixtures"
TESTPKG1_DIR="${FIXTURES_DIR}/testpkg1"
TESTPKG2_DIR="${FIXTURES_DIR}/testpkg2"
TESTPKG1_IDENT="${HAB_ORIGIN}/testpkg1"
TESTPKG2_IDENT="${HAB_ORIGIN}/testpkg2"

echo
echo "--- Testing with command ${HAB}, using cache dir ${CACHE_DIR}"
echo

setup_tasks() {
    echo
    echo ">>>>> setup_tasks"
    # origin create will exit 0 if the origin already exists
    ${HAB} origin create --url "${HAB_BLDR_URL}" ${HAB_ORIGIN}
    rm -rf "${CACHE_DIR}"
    mkdir -p ${CACHE_DIR}/artifacts ${CACHE_DIR}/keys
    # We always attempt to re-use same the package versions so we are not cluttering up Builder needlessly.
    # The packages may not exist yet in Builder so we allow for failure on the download.
    ${HAB} pkg download --url "${HAB_BLDR_URL}" --download-directory ${CACHE_DIR} --channel unstable ${TESTPKG1_IDENT} ${TESTPKG2_IDENT} || true
    if [ -z "$(ls -A ${CACHE_DIR}/artifacts)" ]; then
        echo "--- INFO: Packages were not found for download, proceeding with a new build."
        ${HAB} origin key download --secret --url "${HAB_BLDR_URL}" --cache-key-path ${CACHE_DIR}/keys ${HAB_ORIGIN}
        ${HAB} origin key download --url "${HAB_BLDR_URL}" --cache-key-path ${CACHE_DIR}/keys ${HAB_ORIGIN}
        for dir in ${TESTPKG1_DIR} ${TESTPKG2_DIR}; do
            ${HAB} pkg build ${dir}
            # shellcheck disable=SC1091
            source results/last_build.env
            # shellcheck disable=SC2154
            cp -f results/"$pkg_artifact" ${CACHE_DIR}/artifacts/
        done
    fi
}

test_expecting_fail() {
    DESC=$1
    CMD=$2

    echo
    echo "--- Expected failure: Testing ${DESC}"
    echo "Testing command line: ${CMD}"
    if ${CMD}; then
	echo "FAIL (expected error) ${CMD}"
	exit 1
    else
	echo "PASS ${CMD}"
    fi
}

test_expecting_pass() {
    DESC=$1
    CMD=$2

    echo
    echo "--- Expected success: Testing ${DESC}"
    echo "Testing command line: ${CMD}"
    if ! ${CMD}; then
	echo "FAIL (expected pass) ${CMD}"
	exit 1
    else
	echo "PASS ${CMD}"
    fi
}

success_upload() {
    CMD="${HAB} pkg bulkupload ${CACHE_DIR}"
    test_expecting_pass "bulkupload with no options" "${CMD}"
}

success_upload_force() {
    CMD="${HAB} pkg bulkupload --force ${CACHE_DIR}"
    test_expecting_pass "bulkupload with force option" "${CMD}"
}

success_upload_channel_promotion() {
    CMD="${HAB} pkg bulkupload --channel bulkuploadtest ${CACHE_DIR}"
    test_expecting_pass "bulkupload with channel promotion option" "${CMD}"
}

success_upload_auto_build() {
    CMD="${HAB} pkg bulkupload --auto-build ${CACHE_DIR}"
    test_expecting_pass "bulkupload with auto-build option" "${CMD}"
}

fail_no_upload_dir() {
    CMD="${HAB} pkg bulkupload"
    test_expecting_fail "bulkupload without specifying upload directory argument" "${CMD}"
}

fail_upload_dir_nonexistant() {
    CMD="${HAB} pkg bulkupload doesnotexist"
    test_expecting_fail "bulkupload specifying nonexistant upload directory argument" "${CMD}"
}

fail_bad_url() {
    CMD="${HAB} pkg bulkupload --url asdf ${CACHE_DIR}"
    test_expecting_fail "bulkupload bad url" "${CMD}"
}

fail_bad_auth() {
    CMD="${HAB} pkg bulkupload --auth asdfjkl ${CACHE_DIR}"
    test_expecting_fail "bulkupload bad auth" "${CMD}"
}

fail_missing_channel_name() {
    CMD="${HAB} pkg bulkupload --channel ${CACHE_DIR}"
    test_expecting_fail "bulkupload missing channel name" "${CMD}"
}

setup_tasks

# Expecting PASS
success_upload
success_upload_force
success_upload_channel_promotion
success_upload_auto_build

# Expecting FAIL
fail_no_upload_dir
fail_upload_dir_nonexistant
fail_bad_url
fail_bad_auth
fail_missing_channel_name
