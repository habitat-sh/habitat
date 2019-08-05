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

# Verify we're setting the variable for package target
export HAB_PACKAGE_TARGET=$BUILD_PKG_TARGET

echo "--- Promoting from $target_channel to $destination_channel"

channel_pkgs_json=$(curl -s "${ACCEPTANCE_HAB_BLDR_URL}/v1/depot/channels/${HAB_ORIGIN}/${target_channel}/pkgs")

mapfile -t packages_to_promote < <(echo "${channel_pkgs_json}" | \
                         jq -r \
                         '.data | 
                         map(.origin + "/" + .name + "/" + .version + "/" + .release)
                         | .[]')

for pkg in "${packages_to_promote[@]}"; do
  echo "Do we promote $pkg?"
  found_pkg_target=$(curl -s "${ACCEPTANCE_HAB_BLDR_URL}/v1/depot/pkgs/${pkg}" | \
                    jq -r '.target')

  if [ "$found_pkg_target" = "$HAB_PACKAGE_TARGET" ]; then
    echo "--- Package target of ${pkg} is: ${found_pkg_target} - promoting to ${destination_channel}"
    ${hab_binary} pkg promote --auth="${HAB_AUTH_TOKEN}" "${pkg}" "${destination_channel}" "${BUILD_PKG_TARGET}"
  else
    echo "--- Package target is: ${found_pkg_target} - NOT promoting"
  fi
done
