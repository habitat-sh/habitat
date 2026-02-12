#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

# Not sure whether this is required but exporting it regardless just to be safe
# export HAB_AUTH_TOKEN
# HAB_AUTH_TOKEN=$(hab_auth_token)

setup_hab_root_macos_pipeline

# Since we are using the *bootstrap* packages right now, we will need to 'install' `hab`
# CLI twice - first get the original `hab` CLI and then use that to download the
# 'bootstrap' version.
declare -g hab_binary
curlbash_hab "${BUILD_PKG_TARGET}" acceptance chef


${hab_binary} pkg install chef/hab -c aarch64-darwin --binlink -f
bootstrap_hab_binary=$(command -v hab)
echo "Bootstrap Package Version is : $($bootstrap_hab_binary -V)."

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway

echo "--- :key: Generating fake origin key"
sudo -E "${bootstrap_hab_binary}" origin key generate

# Install hab-studio from the chef origin via the acceptance channel.
# By default, it installs from the stable channel only,
# so this may need updating to support other channels.
${bootstrap_hab_binary} pkg install chef/hab-studio -c aarch64-darwin-test

export HAB_STUDIO_SECRET_HAB_BLDR_CHANNEL="aarch64-darwin"

echo "--- :hab: Running hab pkg build for $package_path"
sudo -E "${bootstrap_hab_binary}" pkg build "$package_path" || macos_teardown_exit

teardown_hab_root_macos_pipeline

