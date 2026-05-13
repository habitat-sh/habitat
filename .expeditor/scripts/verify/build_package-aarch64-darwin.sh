#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

# Functions can error out too.
set -E

trap 'rm -rf /opt/hab' ERR

# TODO: Right now we are doing everything from acceptance, when we release - we need not
# pass the 'acceptance' channel.

# Since we are using the *bootstrap* packages right now, we will need to 'install' `hab`
# CLI twice - first get the original `hab` CLI and then use that to download the
# 'bootstrap' version.
hab_binary=
curlbash_hab "${BUILD_PKG_TARGET}" acceptance


bootstrap_hab_binary=$(command -v hab)
echo "Bootstrap Package Version is : $($bootstrap_hab_binary -V)."

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway

echo "--- :key: Generating fake origin key"
sudo -E "${bootstrap_hab_binary}" origin key generate

# Install chef studio from the 'acceptance' channel where we downloaded 'chef/hab'
# from. Once we release we will not use 'acceptance' but the released 'hab' and
# 'hab/studio'
${bootstrap_hab_binary} pkg install chef/hab-studio -c acceptance

# This is the channel we need to download chef/hab-* packages from
export HAB_BLDR_CHANNEL="acceptance"

# This is the channel for all the 'core' build dependencies. When these packages
# are promoted to 'base' we do not need to set this.
export HAB_REFRESH_CHANNEL="base-2025"

echo "--- :hab: Running hab pkg build for $package_path"
sudo -E "${bootstrap_hab_binary}" pkg build "$package_path"

source results/last_build.env
# shellcheck disable=SC2154
echo "Package ${pkg_artifact} Built Successfully."
