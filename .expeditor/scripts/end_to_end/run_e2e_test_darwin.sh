#!/bin/bash

# macOS (aarch64-darwin) entry point for end-to-end tests.
# This is the Darwin-specific counterpart of run_e2e_test.sh.

set -euo pipefail

channel=${1:?You must specify a channel value}
test_name=${2:-}

source .expeditor/scripts/end_to_end/setup_environment_darwin.sh "$channel"

# Clean up the writable APFS volume mounted at /hab when done
trap 'sudo -E bash -c "source .expeditor/scripts/shared.sh && HAB_VOLUME_DEVICE=$HAB_VOLUME_DEVICE teardown_hab_root_macos_pipeline"' EXIT

if [ -n "$test_name" ]; then
    sudo -E pwsh .expeditor/scripts/end_to_end/run_e2e_test_core_darwin.ps1 "$test_name"
else
    bash
fi
