#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

declare -g hab_binary
curlbash_hab "${BUILD_PKG_TARGET}" acceptance chef

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

# Install hab-studio from the chef origin via the acceptance channel.
# By default, it installs from the stable channel only,
# so this may need updating to support other channels.
export HAB_INTERNAL_BLDR_CHANNEL=acceptance
echo "--- :hab: Running hab pkg build for $package_path"
${hab_binary} pkg build "$package_path"
