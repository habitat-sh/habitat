#!/bin/bash 

# `build` is a built-in helper function that maps to `hab pkg exec core/hab-plan-build` 
# rather than `hab pkg build` to avoid 'studio-in-studio' situations. Verify that the 
# command functions. We assume that if the build succeeds (exits 0) we've passed this 
# test, and leave more detailed testing to the build output to e2e tests for hab-plan-build

set -euo pipefail 

source .expeditor/scripts/end_to_end/shared_end_to_end.sh

echo "--- Generating a signing key"
hab origin key generate "$HAB_ORIGIN"

echo "--- Test 'build' command is functional in the studio"
studio_run build test/fixtures/minimal-package
