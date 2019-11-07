#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

fastly_api_token=$(fastly_token)

target_channel="${EXPEDITOR_TARGET_CHANNEL:-dev}"

echo "Purging '${target_channel}/habitat/latest' Surrogate Key group from Fastly"
curl -X POST -H "Fastly-Key: ${fastly_api_token}" "https://api.fastly.com/service/1ga2Kt6KclvVvCeUYJ3MRp/purge/${target_channel}/habitat/latest"
