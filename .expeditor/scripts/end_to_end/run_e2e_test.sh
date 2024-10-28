#!/bin/bash

set -euo pipefail

channel=${1:?You must specify a channel value}
test_name=${2:-}

ls -la /tmp

source .expeditor/scripts/end_to_end/setup_environment.sh "$channel"

ls -la /tmp

if [ -n "$test_name" ]; then
    sudo -E hab pkg exec core/powershell pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 "$test_name"
else
    bash
fi
