#!/bin/bash

set -euo pipefail

channel=${1:?You must specify a channel value}
test_name=${2:-}

source .expeditor/scripts/end_to_end/setup_environment.sh "$channel"
if [ -n "$test_name" ]; then
    if command -v pwsh &>/dev/null; then
        # Use system-installed pwsh (e.g. on macOS where core/powershell may not exist)
        sudo -E pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 "$test_name"
    else
        sudo -E hab pkg exec core/powershell pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 "$test_name"
    fi
else
    bash
fi
