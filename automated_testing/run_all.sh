#!/bin/bash

set -euo pipefail

# TODO (CM): Pass the name of a single test case to run

channel=${1:-stable}
# The ,, is Bash 4 for "make this whole string lowercase"... Docker
# image names must be lowercase
image_name="supervisor-testing-${channel,,}"

output_dir="habitat_integration_output"
rm -Rf "${output_dir}"
mkdir "${output_dir}"

# Ensure the requisite images are present.
make habitat_integration_base
make supervisor_image CHANNEL=${channel} IMAGE_NAME=${image_name}

# Assume success until told otherwise; the first failure will set this
# to non-zero.
exit_code=0

for testcase in $(ls testcases); do
    printf "Running: ${testcase}..."
    # TODO (CM): pass output directory to run_test_case.sh and have it
    # split the output into several files as appropriate
    if SUPERVISOR_IMAGE=${image_name} ./run_test_case.sh "${testcase}" &> "${output_dir}/${testcase}_output.txt"; then
        printf " PASS\n"
    else
        printf " FAIL\n"
        exit_code=1
    fi
    # TODO (CM): possible to hijack something like TAP to run these instead?
done

echo "Done; output can be found in ${output_dir}"
exit ${exit_code}
