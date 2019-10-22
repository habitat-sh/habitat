#!/bin/bash
#
# Basic set of tests for the hab pkg download command
#
# There are a number of pieces of this which are fragile, and could be
# implemented in a more clever fashion. There are many opportunites
# for cleaner code and more fine grained tests. However, they are a
# bit of a pain to program in bash. This is intended to provide
# minimal testing pending our figuring out the best approach for
# command line testing.
#

set -euo pipefail

# Test the Habitat package downloader.
#
# Uses the `HAB_INTERNAL_BLDR_CHANNEL` environment variable to control
# the base packages channel for the exporter.
HAB=${HAB_TEST_CMD:-hab}
CACHE_DIR="test-cache"
IDENT_FILE="ident_file"

echo
echo "==========Testing command ${HAB}, using cache dir ${CACHE_DIR}"
echo

before_test() {
    # Remove cache if already present
    rm -rf ${CACHE_DIR}
}

check_ident_downloaded() {
    FILE="${CACHE_DIR}/artifacts/${1}-*"
    # globbing is intentional here
    # shellcheck disable=SC2086
    if [ -f ${FILE} ]; then
	echo "--- package downloaded ${FILE}"
    else
	echo "--- package not downloaded ${FILE}"
	exit 1
    fi
}

# this is fragile, and might be vulerable to updates
check_gzip_idents() {
    check_ident_downloaded "core-gzip"
    check_ident_downloaded "core-glibc"
    check_ident_downloaded "core-gcc-libs"
    check_ident_downloaded "core-grep"
    check_ident_downloaded "core-linux-headers"
    check_ident_downloaded "core-pcre"
    check_ident_downloaded "core-less"
    check_ident_downloaded "core-ncurses"

    count=$(find "${CACHE_DIR}/artifacts" -type f | wc -l)

    echo "found $count downloads"
    if [ "$count" -eq "8" ]; then
	echo "PASS $CMD"
    else
	echo "FAIL $CMD"
    fi
}

# this is fragile, and might be vulerable to updates
check_rust_idents() {
    check_ident_downloaded "core-rust"
    check_ident_downloaded "core-visual-cpp-redist-2015"
    check_ident_downloaded "core-visual-cpp-build-tools-2015"

    count=$(find "${CACHE_DIR}/artifacts" -type f | wc -l)

    echo "found $count downloads"
    if [ "$count" -eq "3" ]; then
	echo "PASS $CMD"
    else
	echo "FAIL $CMD"
    fi
}

# desc cmd
test_expecting_fail() {
    DESC=$1
    CMD=$2

    echo
    echo "==========Testing ${DESC}"
    if ${CMD}; then
	echo "FAIL (expected error) $CMD"
	exit 1
    else
	echo "PASS (got error) $CMD"
    fi;
}

success_from_command_line() {
    before_test

    echo "==========Testing command line idents"

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/gzip"
    echo "Testing command line: ${CMD}"
    ${CMD}

    check_gzip_idents
}

success_from_file() {
    before_test

    echo "==========Testing file idents"

    echo "core/gzip" > ${IDENT_FILE}
    CMD="$HAB pkg download --download-directory=${CACHE_DIR} --file=${IDENT_FILE}"
    echo "Testing command line: ${CMD}"
    ${CMD}

    check_gzip_idents
}

success_from_file_with_comments_and_emtpy_lines() {
    before_test

    echo "==========Testing file idents when file has white spaces and comments"

    cat << IDENT_FILE > ${IDENT_FILE}
# this is a series
# of comments, followed by empty lines and whitespaces

 core/gzip 
IDENT_FILE
    CMD="$HAB pkg download --download-directory=${CACHE_DIR} --file=${IDENT_FILE}"
    echo "Testing command line: ${CMD}"
    ${CMD}

    check_gzip_idents
}

success_from_alternate_arch() {
    before_test

    echo "==========Testing command line idents"

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/rust --target=x86_64-windows"
    echo "Testing command line: ${CMD}"
    ${CMD}

    check_rust_idents
}

bad_package_as_arg() {
    before_test

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} arglebargle"
    test_expecting_fail "Bad package on command line" "$CMD"
}

no_package_from_args() {
    before_test

    CMD="$HAB pkg download --download-directory=${CACHE_DIR}"
    test_expecting_fail "No package identifers provided" "$CMD"
}

bad_package_in_file() {
    before_test

    echo "arglebargle" > ${IDENT_FILE}

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} --file=${IDENT_FILE}"
    test_expecting_fail "Bad package in provided file" "$CMD"
}

no_such_package() {
    before_test

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/half_life_4"
    test_expecting_fail "Bad package on command line" "$CMD"

}

cannot_create_dir() {
    before_test

    touch ${CACHE_DIR}

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/gzip"
    test_expecting_fail "Cannot create dir" "$CMD"
}

bad_target() {
    before_test

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/gzip --target=6502-commodore"
    test_expecting_fail "Bad target" "$CMD"
}


bad_token() {
    before_test

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/gzip --auth='asdfa'"
    test_expecting_fail "Bad token" "$CMD"
}

bad_url() {
    before_test

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/gzip --url='https://www.example.org'"
    test_expecting_fail "Bad url" "$CMD"
}

bad_channel() {
    before_test

    CMD="$HAB pkg download --download-directory=${CACHE_DIR} core/gzip --channel=number_five"
    test_expecting_fail "Bad channel" "$CMD"
}


# functional tests
success_from_command_line
success_from_file
success_from_file_with_comments_and_emtpy_lines

success_from_alternate_arch

# failure tests

bad_package_as_arg
no_package_from_args
bad_package_in_file
no_such_package
cannot_create_dir
bad_target
bad_token
bad_url
bad_channel
