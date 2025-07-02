#!/bin/bash

set -eou pipefail

package_path=${1?package_path argument required}

# Install hab from a temporarily uploaded aarch64 package
sudo ./components/hab/install.sh -t "$BUILD_PKG_TARGET" -c acceptance -o chef

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway
# let's make a selfcontained tempdir for this job
export JOB_TEMP_ROOT
JOB_TEMP_ROOT=$(mktemp -d /tmp/job-root-XXXXXX)
export HAB_CACHE_KEY_PATH
HAB_CACHE_KEY_PATH="$JOB_TEMP_ROOT/keys"

echo "--- :key: Generating temporary origin key"
hab origin key generate "$HAB_ORIGIN"

# Install hab-studio from the chef origin via the acceptance channel.
# By default, it installs from the stable channel only,
# so this may need updating to support other channels.
export HAB_INTERNAL_BLDR_CHANNEL=acceptance
echo "--- :hab: Running hab pkg build for $package_path"
hab pkg build "$package_path"
