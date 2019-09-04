#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

### This file should include things that are used exclusively by the release pipeline

get_release_channel() {
    echo "habitat-release-${BUILDKITE_BUILD_ID}"
}

# Download public and private keys for the "core" origin from Builder.
#
# Currently relies on a global variable `hab_binary` being set, since
# in the Linux build process, we need to switch binaries mid-way
# through the pipeline. As we bring more platforms into play, this may
# change. FYI.
import_keys() {
    echo "--- :key: Downloading 'core' public keys from ${HAB_BLDR_URL}"
    ${hab_binary:?} origin key download core
    echo "--- :closed_lock_with_key: Downloading latest 'core' secret key from ${HAB_BLDR_URL}"
    ${hab_binary:?} origin key download \
        --auth="${HAB_AUTH_TOKEN}" \
        --secret \
        core
}

# Returns the full "release" version in the form of X.Y.Z/DATESTAMP
get_latest_pkg_release_version_in_release_channel() {
    local pkg_name="${1:?}"
    version=$(curl -s "${HAB_BLDR_URL}/v1/depot/channels/core/$(get_release_channel)/pkgs/${pkg_name}/latest?target=${BUILD_PKG_TARGET}" \
        | jq -r '.ident | .version + "/" + .release')
    echo "${version}"
}

# Returns the semver version in the form of X.Y.Z
get_latest_pkg_version_in_release_channel() {
    local pkg_name="${1:?}"
    local release
    release=$(get_latest_pkg_release_version_in_release_channel "$pkg_name")
    echo "$release" | cut -f1 -d"/"
}

# Install the latest binary from the release channel and set the `hab_binary` variable
#
# Requires the `hab_binary` global variable is already set!
#
# Accepts a pkg target argument if you need to override it, otherwise
# will default to the value of `BUILD_PKG_TARGET`
install_release_channel_hab_binary() {
    local pkg_target="${1:-$BUILD_PKG_TARGET}"
    curlbash_hab "${pkg_target}"

    # workaround for https://github.com/habitat-sh/habitat/issues/6771	
    ${hab_binary} pkg install core/hab-studio

    echo "--- :habicat: Installed latest stable hab: $(${hab_binary} --version)"
    # now install the latest hab available in our channel, if it and the studio exist yet
    hab_version=$(get_latest_pkg_version_in_release_channel "hab")
    studio_version=$(get_latest_pkg_version_in_release_channel "hab-studio")

    if [[ -n $hab_version && -n $studio_version && $hab_version == "$studio_version" ]]; then
        echo "-- Hab and studio versions match! Found hab: ${hab_version:-null} - studio: ${studio_version:-null}. Upgrading :awesome:"
        channel=$(get_release_channel)
        ${hab_binary:?} pkg install --binlink --force --channel "${channel}" core/hab
        ${hab_binary:?} pkg install --binlink --force --channel "${channel}" core/hab-studio
        hab_binary="$(hab pkg path core/hab)/bin/hab"
        echo "--- :habicat: Installed latest build hab: $(${hab_binary} --version) at $hab_binary"
    else
        echo "--- Hab and studio versions did not match. hab: ${hab_version:-null} - studio: ${studio_version:-null}"
    fi
}
