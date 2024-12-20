#!/bin/bash

set -euo pipefail

docker-compose --version

# TODO (CM): Pass the name of a single test case to run

channel=${1:-stable}
image_name="supervisor-testing-${channel}"

output_dir="habitat_integration_output"
rm -Rf "${output_dir}"
mkdir "${output_dir}"

# Ensure the requisite images are present.
make habitat_integration_base CHANNEL="${channel}"
make supervisor_image CHANNEL="${channel}" IMAGE_NAME="${image_name}"

# Assume success until told otherwise; the first failure will set this
# to non-zero.
exit_code=0

for testcase in testcases/*; do
    # We just want the test case name here, not the path to it.
    # (the `testcases/*` glob above gets us things like
    # "testcases/foo", and we just want "foo")
    testcase="$(basename "${testcase}")"
    printf "Running: %s..." "${testcase}"
    # TODO (CM): pass output directory to run_test_case.sh and have it
    # split the output into several files as appropriate
    if SUPERVISOR_IMAGE="${image_name}" ./run_test_case.sh "${testcase}" &> "${output_dir}/${testcase}_output.txt"; then
        printf " PASS\n"
    else
        printf " FAIL\n"
        exit_code=1
    fi
    # TODO (CM): possible to hijack something like TAP to run these instead?
done

echo "Done; output can be found in ${output_dir}"
exit ${exit_code}
