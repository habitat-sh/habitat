#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

target_channel="${EXPEDITOR_TARGET_CHANNEL:-dev}"

echo "Purging '${target_channel}/habitat/latest' Surrogate Key group from Fastly"
purge_fastly_cache "1ga2Kt6KclvVvCeUYJ3MRp" "$target_channel"
