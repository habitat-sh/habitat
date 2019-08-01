#!/bin/bash

# We'll destroy the build channel at the end of the pipeline, as
# we've promoted anything successful to DEV by this point

set -euo pipefail

source .expeditor/scripts/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

curlbash_hab "$BUILD_PKG_TARGET"

channel="$(get_release_channel)"
echo "--- Destroying release channel '${channel}'"

hab bldr channel destroy \
    --origin=core \
    "${channel}"
