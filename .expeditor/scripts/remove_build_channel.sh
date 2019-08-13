#!/bin/bash

# We'll destroy the build channel at the end of the pipeline, as
# we've promoted anything successful to DEV by this point

set -euo pipefail

source .expeditor/scripts/shared_release_habitat.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

declare -g hab_binary
curlbash_hab "$BUILD_PKG_TARGET"

channel="$(get_release_channel)"
echo "--- Destroying release channel '${channel}'"

${hab_binary} bldr channel destroy \
    --origin=core \
    "${channel}"
