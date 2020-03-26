#!/bin/bash

set -euo pipefail

channel=${1:?You must specify a channel value}
test_name=${2:-}

source .expeditor/scripts/end_to_end/setup_environment.sh "$channel"
if [ -n "$test_name" ]; then
    pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 "$test_name"
fi
