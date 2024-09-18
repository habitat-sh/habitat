#!/bin/bash

set -ex
set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

declare -g hab_binary
curlbash_hab "${BUILD_PKG_TARGET}"

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway
# let's make a selfcontained tempdir for this job
export JOB_TEMP_ROOT
JOB_TEMP_ROOT=$(mktemp -d /tmp/job-root-XXXXXX)
export HAB_CACHE_KEY_PATH
HAB_CACHE_KEY_PATH="$JOB_TEMP_ROOT/keys"

echo "--- :key: Generating fake origin key"
${hab_binary} origin key generate
# Install the hab-studio from the 'aarch64-linux' channel.
# Once hab is released in the LTS channel, we can use either HAB_BLDR_CHANNEL or HAB_REFRESH_CHANNEL to install the studio.
if [ "$BUILD_PKG_TARGET" = "aarch64-linux" ]; then
  ${hab_binary} pkg install core/hab-studio/1.6.706 --channel aarch64-linux
fi
echo "--- :hab: Running hab pkg build for $package_path"
${hab_binary} pkg build "$package_path"
