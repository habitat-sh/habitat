#!/bin/bash

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

# Take advantage of the fact that we're just promoting and we can run 
# 100% on linux
declare -g hab_binary
curlbash_hab "x86_64-linux"

########################################################################

# `source_channel` should be the channel we are promoting all our
# artifacts from.
#
# e.g. `habitat-release-<build-id>`, `DEV`, `ACCEPTANCE` etc.
source_channel=${1:?You must specify a source channel value}

# `destination_channel` should be the channel we are promoting to.
#
# e.g. `DEV`, `ACCEPTANCE`, `CURRENT`, etc
destination_channel=${2:?You must specify a destination channel value}

echo "--- Promoting from $source_channel to $destination_channel"

# TODO: Everything below becomes a single API call once
# channel-to-channel promotions are implemented; see
# https://github.com/habitat-sh/builder/issues/580

channel_pkgs_json=$(curl -s "${ACCEPTANCE_HAB_BLDR_URL}/v1/depot/channels/${HAB_ORIGIN}/${source_channel}/pkgs")

mapfile -t packages_to_promote < <(echo "${channel_pkgs_json}" | \
                         jq -r \
                         '.data | 
                         map(.origin + "/" + .name + "/" + .version + "/" + .release)
                         | .[]')

targets=("x86_64-linux"
         "x86_64-linux-kernel2"
         "x86_64-windows")



for pkg in "${packages_to_promote[@]}"; do
    echo "--- :habicat: Promoting '$pkg' to '$destination_channel'"
    for pkg_target in "${targets[@]}"; do
      if ident_has_target "${pkg}" "${pkg_target}"; then
          echo "--- Promoting ${pkg} (${pkg_target}) to the '${destination_channel}' channel"
          echo "${pkg} ${pkg_target}" >> manifest_entries
          ${hab_binary} pkg promote --auth="${HAB_AUTH_TOKEN}" "${pkg}" "${destination_channel}" "${pkg_target}"
      else
          echo "--- :thumbsdown: not a match"
      fi
    done
done

# create manifest!
./create_manifest manifest_entries

# push manifest to S3
echo "--- Pushing manifest file to S3"
unstable_s3_url="s3://chef-automate-artifacts/dev/latest/habitat/manifest.json"
s3_upload_file "manifest.json" "$unstable_s3_url"