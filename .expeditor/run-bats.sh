#!/usr/bin/env bash

# Runs BATS tests directly in the current environment.
# This script assumes it's running in an appropriate test environment
# (such as a CI container that already has the necessary tools installed).

set -euo pipefail

# Handy escape hatch for running a single file for quicker local
# development, e.g.:
#
# cd <ROOT>/.expeditor
# run-bats.sh manifest.bats
#
if [ $# -eq 1 ] ; then
    TESTS=".expeditor/test/$1"
else
    TESTS=".expeditor/test"
fi

# Ensure we're in the repository root for consistent behavior
cd "$(dirname "$0")/.."

# Run BATS tests directly
bats "${TESTS}"
