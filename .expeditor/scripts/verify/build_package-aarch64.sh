#!/bin/bash

set -eou pipefail

package_path=${1?package_path argument required}

# Install hab from a temporarily uploaded aarch64 package
curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh | sudo bash -s -- -t "$BUILD_PKG_TARGET" -c "$HAB_FALLBACK_CHANNEL" -v 1.6.1178

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=core
# let's make a selfcontained tempdir for this job
export JOB_TEMP_ROOT
JOB_TEMP_ROOT=$(mktemp -d /tmp/job-root-XXXXXX)
export HAB_CACHE_KEY_PATH
HAB_CACHE_KEY_PATH="$JOB_TEMP_ROOT/keys"

export HAB_STUDIO_SECRET_HAB_FALLBACK_CHANNEL
HAB_STUDIO_SECRET_HAB_FALLBACK_CHANNEL="$HAB_FALLBACK_CHANNEL"
export HAB_PREFER_LOCAL_CHEF_DEPS="true"

echo "--- :key: Generating temporary origin key"
hab origin key generate "$HAB_ORIGIN"
echo "--- :hab: Running hab pkg build for $package_path"
# Install the temporarily built hab-studio.
# Once hab is released in the LTS channel, this step may no longer be required.
hab pkg install core/hab-studio -c "$HAB_FALLBACK_CHANNEL"
hab pkg build "$package_path"
