#!/bin/bash

# Retrieves the current package manifest for a particular environment
# and promotes the packages into a designated Builder channel.

set -euo pipefail

# `source_environment` is the Expeditor environment from which to pull
# a manifest.json file from in order to drive the promotions.
#
# e.g., "dev", "acceptance", etc.
source_environment=${1:?You must provide an Expeditor environment}

# `destination_channel` should be the channel we are promoting Habitat
# packages into.
#
# e.g. `acceptance`, `current`, etc
destination_channel=${2:?You must specify a destination channel value}

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

########################################################################

source .expeditor/scripts/release_habitat/shared.sh

# Take advantage of the fact that we're just promoting and we can run
# 100% on linux
declare -g hab_binary
curlbash_hab "x86_64-linux"

echo "--- Retrieving manifest.json for ${source_environment} environment"
manifest_json="$(manifest_for_environment ${source_environment})"

# Extract the targets from the manifest
mapfile -t targets < <(echo "${manifest_json}" | jq -r ".packages | keys | .[]")

echo "--- Promoting Habitat packages to the ${destination_channel} channel of ${HAB_BLDR_URL}"
for target in "${targets[@]}"; do
    mapfile -t idents < <(echo "${manifest_json}" | jq -r ".packages.\"${target}\" | .[]")
    for ident in "${idents[@]}"; do
        echo "--- Promoting ${ident} (${target}) to '${destination_channel}'"
        ${hab_binary} pkg promote \
                      --auth="${HAB_AUTH_TOKEN}" \
                      --url="${HAB_BLDR_URL}" \
                      "${ident}" "${destination_channel}" "${target}"
    done
done
