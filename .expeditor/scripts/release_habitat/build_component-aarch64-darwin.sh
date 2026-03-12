#!/bin/bash

set -eou pipefail

source .expeditor/scripts/release_habitat/shared.sh

package_path=${1?package_path argument required}

setup_hab_root_macos_pipeline

# Functions can error out too.
set -E

trap macos_teardown_exit ERR

export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"

# Following env variable is required to run MacOS Native Studio
export HAB_FEAT_MACOS_NATIVE_SUPPORT=1

# Temporary workaround to make sure CI builds pass
export CI_INTERNAL_MAC_NATIVE_SUPPORT=1

channel=$(get_release_channel)

# This will always install the hab binary from the 'aarch64-darwin' channel
# through internal call to 'curlbash_hab' that in turn installs the binary
# through '-b' parameter.
hab_binary=
install_release_channel_hab_binary "${BUILD_PKG_TARGET}"

echo "--- :key: Importing keys"
import_keys

# Install the 'hab-studio' from the aarch64-darwin channel.
# TODO: Move this to acceptance once we publish.
# so this may need updating to support other channels.
${hab_binary} pkg install chef/hab-studio -c aarch64-darwin

export HAB_STUDIO_SECRET_HAB_BLDR_CHANNEL="aarch64-darwin"

echo "--- :hab: Running hab pkg build for $package_path"
sudo -E "${hab_binary}" pkg build "$package_path"

source results/last_build.env

if [ "${pkg_target}" != "${BUILD_PKG_TARGET}" ]; then
    echo "--- :face_with_symbols_on_mouth: Expected to build for target ${BUILD_PKG_TARGET}, but built ${pkg_target} instead!"
    exit 1
fi

echo "--- :habicat: Uploading ${pkg_ident:?} (${pkg_target}) to ${HAB_BLDR_URL} in the '${channel}' channel"
${hab_binary} pkg upload \
              --channel="${channel}" \
              --auth="${HAB_AUTH_TOKEN}" \
              "results/${pkg_artifact:?}"

echo "<br>* ${pkg_ident} (${pkg_target})" | buildkite-agent annotate --append --context "release-manifest"

set_target_metadata "${pkg_ident}" "${pkg_target}"

teardown_hab_root_macos_pipeline
