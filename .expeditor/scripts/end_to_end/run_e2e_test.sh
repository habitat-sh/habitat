#!/bin/bash

set -euo pipefail

channel=${1:?You must specify a channel value}
test_name=${2:?You must specify a test name}

source .expeditor/scripts/end_to_end/setup_environment.sh "$channel"
pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 "$test_name"
