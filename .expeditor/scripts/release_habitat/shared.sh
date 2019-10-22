#!/bin/bash

source .expeditor/scripts/shared.sh

### This file should include things that are used exclusively by the release pipeline

get_release_channel() {
    echo "habitat-release-${BUILDKITE_BUILD_ID}"
}

# Read the contents of the VERSION file. This will be used to
# determine where generated artifacts go in S3.
get_version_from_repo() {
    dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
    if [[ -n "${DO_FAKE_RELEASE:-}" ]]; then
        cat "$dir/../../../VERSION_FAKE"
    else
        cat "$dir/../../../VERSION"
    fi
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
    curl -s "${HAB_BLDR_URL}/v1/depot/channels/core/$(get_release_channel)/pkgs/${pkg_name}/latest?target=${BUILD_PKG_TARGET}" \
        | jq -r '.ident | .version + "/" + .release'
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

# Until we can reliably deal with packages that have the same
# identifier, but different target, we'll track the information in
# Buildkite metadata.
#
# Each time we put a package into our release channel, we'll record
# what target it was built for.
set_target_metadata() {
    package_ident="${1}"
    target="${2}"

    echo "--- :partyparrot: Setting target metadata for '${package_ident}' (${target})"
    buildkite-agent meta-data set "${package_ident}-${target}" "true"
}

# When we do the final promotions, we need to know the target of each
# package in order to properly get the promotion done. If Buildkite metadata for
# an ident/target pair exists, then that means that's a valid
# combination, and we can use the target in the promotion call.
ident_has_target() {
    package_ident="${1}"
    target="${2}"

    echo "--- :partyparrot: Checking target metadata for '${package_ident}' (${target})"
    buildkite-agent meta-data exists "${package_ident}-${target}"
}
