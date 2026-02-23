#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

setup_hab_root_macos_pipeline

# Functions can error out too.
set -E

trap macos_teardown_exit ERR

# Following env variable is required to run MacOS Native Studio
export HAB_FEAT_MACOS_NATIVE_SUPPORT=1

# Since we are using the *bootstrap* packages right now, we will need to 'install' `hab`
# CLI twice - first get the original `hab` CLI and then use that to download the
# 'bootstrap' version.
hab_binary=
curlbash_hab "${BUILD_PKG_TARGET}" acceptance

${hab_binary} pkg install chef/hab -c aarch64-darwin --binlink -f
bootstrap_hab_binary=$(command -v hab)
echo "Bootstrap Package Version is : $($bootstrap_hab_binary -V)."

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway

echo "--- :key: Generating fake origin key"
sudo -E "${bootstrap_hab_binary}" origin key generate || macos_teardown_exit

# Install hab-studio from the chef origin via the acceptance channel.
# By default, it installs from the stable channel only,
# so this may need updating to support other channels.
${bootstrap_hab_binary} pkg install chef/hab-studio -c aarch64-darwin

export HAB_STUDIO_SECRET_HAB_BLDR_CHANNEL="aarch64-darwin"

echo "--- :hab: Running hab pkg build for $package_path"
sudo -E "${bootstrap_hab_binary}" pkg build "$package_path"

teardown_hab_root_macos_pipeline
