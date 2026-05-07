#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

# Functions can error out too.
set -E

trap 'rm -rf /opt/hab' ERR

# Since we are using the *bootstrap* packages right now, we will need to 'install' `hab`
# CLI twice - first get the original `hab` CLI and then use that to download the
# 'bootstrap' version.
hab_binary=
curlbash_hab "${BUILD_PKG_TARGET}" acceptance


install_acceptance_bootstrap_hab_binary
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
${bootstrap_hab_binary} pkg install chef/hab-studio -c base-2025

# Required for the `hab pkg build` command to download the studio and deps when
# locally missing
export HAB_INTERNAL_BLDR_CHANNEL="base-2025"

# Required to download the deps of the package to be built
export HAB_BLDR_CHANNEL="base-2025"

echo "--- :hab: Running hab pkg build for $package_path"
sudo -E "${bootstrap_hab_binary}" pkg build "$package_path"

source results/last_build.env
# shellcheck disable=SC2154
echo "Package ${pkg_artifact} Built Successfully."
