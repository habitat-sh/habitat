#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

# Take advantage of the fact that we're just promoting and we can run 
# 100% on linux
curlbash_hab "x86_64-linux"

########################################################################

# `target_channel` should be channel we are promoting all our artifacts from
#
# e.g. `habitat-release-<build-id>`, `DEV`, `ACCEPTANCE` etc.
target_channel=${1:?You must specify a target channel value}

# `destination_channel` should be the channel we are promoting to
#
# e.g. `DEV`, `ACCEPTANCE`, `CURRENT`, etc
destination_channel=${2:?You must specify a destination channel value}

echo "--- Promoting from $target_channel to $destination_channel"

# TODO: Everything below becomes a single API call once
# channel-to-channel promotions are implemented; see
# https://github.com/habitat-sh/builder/issues/580

channel_pkgs_json=$(curl -s "${ACCEPTANCE_HAB_BLDR_URL}/v1/depot/channels/${HAB_ORIGIN}/${target_channel}/pkgs")

mapfile -t packages_to_promote < <(echo "${channel_pkgs_json}" | \
                         jq -r \
                         '.data | 
                         map(.origin + "/" + .name + "/" + .version + "/" + .release)
                         | .[]')

for pkg in "${packages_to_promote[@]}"; do
    # We only do this extra API call because we can't get the target
    # from the above API call. Once
    # https://github.com/habitat-sh/builder/issues/1111 is addressed,
    # we won't need this additional call (we'll have to modify the
    # above `jq` call to create `packages_to_promote`, however).
    pkg_target=$(curl -s "${ACCEPTANCE_HAB_BLDR_URL}/v1/depot/pkgs/${pkg}" | jq -r '.target')
    echo "--- Promoting ${pkg} (${pkg_target}) to the '${destination_channel}' channel"
    ${hab_binary} pkg promote --auth="${HAB_AUTH_TOKEN}" "${pkg}" "${destination_channel}" "${pkg_target}"
done
