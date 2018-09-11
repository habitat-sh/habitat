#!/bin/bash

_test_source_dir=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
_hab_plan_build_dir=$(hab pkg path core/hab-plan-build)
source "${_hab_plan_build_dir}/bin/public.bash"

# Plan directory (First argument to the script)
_plan_context=${1:-.}
# Name of the directory containing test.bats
: "${PLAN_TEST_DIR:=tests}"
# Full directory path for the tests directory
: "${TEST_CONTEXT:=$_plan_context/$PLAN_TEST_DIR}"
# Allow users to skip the build. Super useful for builds that take a long time, and just want to tweak tests
: ${SKIPBUILD:=0}
# Period of time to wait before performing tests on a service
: ${SERVICEWAIT:=3}
# BATS path
: "${BATSPATH:=$(hab pkg path core/bats)}"

# Fetch last build information if available
_source_last_build() {
  local _last_build_env="${_plan_context}/results/last_build.env"
  if [[ ! -f "${_last_build_env}" ]]; then
    exit_with "[TEST] There is no previoud build available at ${_last_build_env}. Do you need to build first?" 42
  fi
  source "${_last_build_env}"
}

_install_artifact() {
  local _artifact_file="${_plan_context}/results/${pkg_artifact}"
  if [[ ! -f "${_artifact_file}" ]]; then
    exit_with "[TEST] Unable to find artifact at ${_artifact_file}" 42
  fi
  hab pkg install --binlink --force "${_artifact_file}"
}

do_test_build_wrapper() {
  build_line "==============================================================="
  build_line "[TEST] - Starting build"
  build_line "==============================================================="
  do_test_build
}

do_test_build() {
  do_default_test_build
}

do_default_test_build() {
  build "${_plan_context}"
}

do_test_install_wrapper() {
  build_line "==============================================================="
  build_line "[TEST] - Starting install"
  build_line "==============================================================="
  do_test_install
}

do_test_install() {
  do_default_test_install
}

do_default_test_install() {
  _source_last_build
  _install_artifact
}

do_test_service_wait_wrapper() {
  build_line "==============================================================="
  build_line "[TEST] - Waiting for service"
  build_line "==============================================================="
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
  build_line "==============================================================="
  build_line "[TEST] - Starting bats tests"
  build_line "==============================================================="
  do_test_bats
}

do_test_bats() {
  do_default_test_bats
}

do_default_test_bats() {
  "${BATSPATH}/bin/bats" test.bats
}

source "${_plan_context}/plan.sh"

# Actual testing steps
do_test_build_wrapper
do_test_install_wrapper
do_test_service_wait_wrapper
# do_test_bats_wrapper

