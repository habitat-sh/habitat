#!/bin/bash
#
# # Usage
#
# ```
# $ hab-plan-build-test [<path>]
# ```
#
# # Synopsis
#
# `hab-plan-build-test` handles automated build and testing of Habitat packages.
#
# # tests/test.bats
#
# The automated testing assumes the conventions for Habitat plan testing are
# followed. This involves ensuring that a `tests` directory is available at
# the same location as the `plan.sh` file for the plan. Inside this directory
# at a minimum, should be `test.bats`.
#
# `test.bats` contains BATS compatible shell tests.
#
# Here is an example minimal test for [strace](https://strace.io/).
#
# ```sh
# source "${BATS_TEST_DIRNAME}/../plan.sh"
#
# @test "Version matches" {
#   result="$(strace -V | head -1 | awk '{print $4}')"
#   [ "$result" = "${pkg_version}" ]
# }
# @test "Can strace" {
#   run strace strace -h
#   [ $status -eq 0 ]
# }
# ```
#
# Its a good idea to include the plan.sh source into the test to allow tests
# to match package names and version numbers as necessary.
#
# # tests/test.sh
#
# An optional file `test.sh` in the `tests` directory can be used to override
# values and testing behaviour as necessary to suit the software being tested.
# While the testing is designed to be simple and generic, each piece of
# software has its own quirks and special requirements.
#
# ## Testing Callbacks (Test phases)
#
# `hab-plan-build-test` allows you to alter its behavior by defining callbacks
# (or test phases) within the `test.sh` file (if provided). While `strace`
# requires no custom callbacks, most software isn't quite so simple to test.
# To define a callback, simply create a shell function with its name - then
# write out your script.
#
# Most likely, due to the nature of packaging with Habitat, you will not need
# any customization. However in the case of a service that requires starting,
# or a service that has dependencies in order to run, you may need to override
# `do_test_service_wait` or others.
#
# 1. do_test_service_wait - Wait for a service to startup before testing
#
# # Environment variables
# The following can be set to override behaviour and skip major steps in a
# on-off capacity.
# `SKIPBUILD` - Skip the build step
# `SKIPINSTALL` - Skip the install step
# `SKIPSERVICE` - Skip the service install / start

_test_source_dir=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
_hab_plan_build_dir=$(hab pkg path core/hab-plan-build)
source "${_hab_plan_build_dir}/bin/public.bash"

# Plan directory (First argument to the script)
PLAN_CONTEXT=${1:-.}
# Name of the directory containing test.bats
: "${PLAN_TEST_DIR:=tests}"
# Full directory path for the tests directory
: "${TEST_CONTEXT:=$PLAN_CONTEXT/$PLAN_TEST_DIR}"
# Period of time to wait before performing tests on a service
: "${SERVICEWAIT:=3}"
# BATS path
: "${BATSPATH:=$(hab pkg path core/bats)}"
# Assume the plan has no service, until detected
: "${PLAN_HAS_SERVICE:=0}"

test_build_line() {
  local _prefix="[TEST] - "
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    build_line "${_prefix}$1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        build_line "${_prefix}\033[1;94m$1\033[0m"
        ;;
      *)
        build_line "${_prefix}$1"
        ;;
    esac
  fi
  return 0
}

test_build_separator() {
  build_line "\033[1;95m===============================================================\033[0m"
}

do_test_build_wrapper() {
  test_build_separator
  if [[ -n "${SKIPBUILD}" ]]; then
    test_build_line "Skipping Build step because SKIPBUILD=${SKIPBUILD}"
    return 0
  fi
  test_build_line "Starting build"
  do_test_build
}

do_test_build() {
  do_default_test_build
}

do_default_test_build() {
  build "${PLAN_CONTEXT}"
}

do_test_install_wrapper() {
  test_build_separator
  if [[ -n "${SKIPINSTALL}" ]]; then
    test_build_line "Skipping Install step because SKIPINSTALL=${SKIPINSTALL}"
    return 0
  fi
  test_build_line "Starting install"
  do_test_install
}

do_test_install() {
  do_default_test_install
}

do_default_test_install() {
  _source_last_build
  _install_artifact
}

# Fetch last build information if available
_source_last_build() {
  local _last_build_env="${PLAN_CONTEXT}/results/last_build.env"
  if [[ ! -f "${_last_build_env}" ]]; then
    exit_with "[TEST] There is no previoud build available at ${_last_build_env}. Do you need to build first?" 42
  fi
  source "${_last_build_env}"
}

_install_artifact() {
  # shellcheck disable=2154
  local _artifact_file="${PLAN_CONTEXT}/results/${pkg_artifact}"
  if [[ ! -f "${_artifact_file}" ]]; then
    exit_with "[TEST] Unable to find artifact at ${_artifact_file}" 42
  fi
  hab pkg install --binlink --force "${_artifact_file}"
}

do_test_service_load_wrapper() {
  _detect_plan_with_service
  if [[ "${PLAN_HAS_SERVICE}" -eq 1 ]]; then
    test_build_separator
    test_build_line "Loading service"
    do_test_service_load
  fi
}

do_test_service_load() {
  do_default_test_service_load
}

do_default_test_service_load() {
  # shellcheck disable=2154
  hab svc load "${pkg_ident}"
}

# Determine if the plan has a service
_detect_plan_with_service() {
  if [[ -f "$PLAN_CONTEXT/hooks/run" || -n "${pkg_svc_run:-}" ]]; then
    PLAN_HAS_SERVICE=1
  fi
}

do_test_service_wait_wrapper() {
  _detect_plan_with_service
  if [[ "${PLAN_HAS_SERVICE}" -eq 0 ]]; then
    return 0
  fi
  test_build_separator
  test_build_line "Waiting for service"
  do_test_service_wait
}

do_test_service_wait() {
  do_default_test_service_wait
}

do_default_test_service_wait() {
  # Give some time for the service to start up
  sleep "${SERVICEWAIT}"
}

do_test_bats_wrapper() {
  test_build_separator
  test_build_line "Starting bats tests"
  do_test_bats
}

do_test_bats() {
  do_default_test_bats
}

do_default_test_bats() {
  if [[ ! -f "${TEST_CONTEXT}/test.bats" ]]; then
    exit_with "Failed to load test.bats file for plan. Should be at ${TEST_CONTEXT}/test.bats"
  fi
  "${BATSPATH}/bin/bats" "${TEST_CONTEXT}/test.bats"
}

# Include plan variables
source "${PLAN_CONTEXT}/plan.sh"

# Include custom test.sh if available
if [[ -f "${TEST_CONTEXT}/test.sh" ]]; then
  test_build_line "Loading plan test.sh from ${TEST_CONTEXT}/test.sh"
  source "${TEST_CONTEXT}/test.sh"
fi

# Actual testing steps
do_test_build_wrapper
do_test_install_wrapper
do_test_service_load_wrapper
do_test_service_wait_wrapper
do_test_bats_wrapper
