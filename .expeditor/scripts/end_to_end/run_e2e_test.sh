#!/bin/bash

set -euo pipefail

channel=${1:?You must specify a channel value}
test_name=${2:-}

# build this container on _each_ pipeline, each time
docker build --progress=plain --no-cache -t automate ./test/end-to-end/automate

source .expeditor/scripts/end_to_end/setup_environment.sh "$channel"
if [ -n "$test_name" ]; then
    sudo -E hab pkg exec core/powershell pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 "$test_name"
else
    bash
fi
