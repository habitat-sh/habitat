#!/bin/bash
#
# Basic set of tests for the hab pkg bulkupload command
#
# Assumptions:
# 1. ACCEPTANCE_HAB_AUTH_TOKEN or HAB_AUTH_TOKEN Environment variables are set and valid
# 2. ${CACHE_DIR} can be set to a writable location on the filesystem
# 3. non zero exit code from each command implies success

# TODO: Future improvement would be to create a clean room Builder or origin.
# Doing so would allow for more accurate validation of the bulkupload by interrogating
# the Builder. Of course this introduces significant setup time cost..

set -euo pipefail

export HAB_NOCOLORING=true
export HAB_NONINTERACTIVE=true
export HAB_AUTH_TOKEN=${ACCEPTANCE_HAB_AUTH_TOKEN:-${HAB_AUTH_TOKEN}}
export HAB_ORIGIN="testbulkupload"
export HAB_BLDR_URL=${HAB_BLDR_URL:-https://bldr.acceptance.habitat.sh}

HAB=${HAB_TEST_CMD:-hab}
CACHE_DIR="test-cache"
FIXTURES_DIR="fixtures/bulkupload"
PKG_A_TAR="${FIXTURES_DIR}/testbulkupload-testpkg1-0.1.0-20191024190939.tar"
PKG_B_TAR="${FIXTURES_DIR}/testbulkupload-testpkg2-0.1.0-20191024191005.tar"
PKG_A_HART="${CACHE_DIR}/artifacts/testbulkupload-testpkg1-0.1.0-20191024190939-x86_64-linux.hart"
PKG_B_HART="${CACHE_DIR}/artifacts/testbulkupload-testpkg2-0.1.0-20191024191005-x86_64-linux.hart"

echo
echo "--- Testing with command ${HAB}, using cache dir ${CACHE_DIR}"
echo

before_upload() {
    echo
    echo ">>>>> before_upload() tasks"
    rm -rf ${CACHE_DIR}
    # origin create will exit 0 if the origin already exists
    ${HAB} origin create ${HAB_ORIGIN}
    ${HAB} origin key download --secret ${HAB_ORIGIN}
    mkdir -p ${CACHE_DIR}/artifacts ${CACHE_DIR}/keys
    cp -f ~/.hab/cache/keys/${HAB_ORIGIN}-*pub ${CACHE_DIR}/keys/
    ${HAB} pkg sign --origin ${HAB_ORIGIN} ${PKG_A_TAR} ${PKG_A_HART}
    ${HAB} pkg sign --origin ${HAB_ORIGIN} ${PKG_B_TAR} ${PKG_B_HART}
    echo
}

test_expecting_fail() {
    DESC=$1
    CMD=$2

    echo
    echo "--- Expected failure: Testing ${DESC}"
    if ${CMD}; then
	echo "FAIL (expected error) $CMD"
	exit 1
    else
	echo "PASS $CMD"
    fi
}

test_expecting_pass() {
    DESC=$1
    CMD=$2

    echo
    echo "--- Expected success: Testing ${DESC}"
    if ! ${CMD}; then
	echo "FAIL (expected pass) $CMD"
	exit 1
    else
	echo "PASS $CMD"
    fi
}

success_upload() {
    before_upload

    CMD="${HAB} pkg bulkupload ${CACHE_DIR}"
    echo "Testing command line: ${CMD}"

    test_expecting_pass "bulkupload with no options" "${CMD}"
}

success_upload_force() {
    before_upload

    CMD="${HAB} pkg bulkupload --force ${CACHE_DIR}"
    echo "Testing command line: ${CMD}"

    test_expecting_pass "bulkupload with force option" "${CMD}"
}

success_upload_channel_promotion() {
    before_upload

    CMD="${HAB} pkg bulkupload --channel bulkuploadtest ${CACHE_DIR}"
    echo "Testing command line: ${CMD}"

    test_expecting_pass "bulkupload with channel promotion option" "${CMD}"
}

success_upload_auto_build() {
    before_upload

    CMD="${HAB} pkg bulkupload --auto-build ${CACHE_DIR}"
    echo "Testing command line: ${CMD}"

    test_expecting_pass "bulkupload with auto-build option" "${CMD}"
}

fail_no_upload_dir() {
    CMD="${HAB} pkg bulkupload"
    echo "Testing command line: ${CMD}"

    test_expecting_fail "bulkupload without specifying upload directory argument" "${CMD}"
}

fail_upload_dir_nonexistant() {
    CMD="${HAB} pkg bulkupload doesnotexist"
    echo "Testing command line: ${CMD}"

    test_expecting_fail "bulkupload specifying nonexistant upload directory argument" "${CMD}"
}

fail_bad_url() {
    CMD="${HAB} pkg bulkupload --url asdf ${CACHE_DIR}"
    echo "Testing command line: ${CMD}"

    test_expecting_fail "bulkupload bad url" "${CMD}"
}

fail_bad_auth() {
    CMD="${HAB} pkg bulkupload --auth asdfjkl ${CACHE_DIR}"
    echo "Testing command line: ${CMD}"

    test_expecting_fail "bulkupload bad auth" "${CMD}"
}

fail_missing_channel_name() {
    CMD="${HAB} pkg bulkupload --channel ${CACHE_DIR}"
    echo "Testing command line: ${CMD}"

    test_expecting_fail "bulkupload missing channel name" "${CMD}"
}

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
