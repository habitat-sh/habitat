#!/bin/bash

set -euo pipefail

# Download public and private keys for the "core" origin from Builder.
#
# Currently relies on a global variable `hab_binary` being set, since
# in the Linux build process, we need to switch binaries mid-way
# through the pipeline. As we bring more platforms into play, this may
# change. FYI.
import_keys() {
    echo "--- :key: Downloading 'core' public keys from ${HAB_BLDR_URL:-Builder}"
    ${hab_binary:?} origin key download core
    echo "--- :closed_lock_with_key: Downloading latest 'core' secret key from ${HAB_BLDR_URL:-Builder}"
    ${hab_binary:?} origin key download \
        --auth="${HAB_AUTH_TOKEN}" \
        --secret \
        core
}

# Always install the latest hab binary appropriate for your linux platform
#
# Assumes you have `BUILD_PKG_TARGET` set
install_latest_stable_hab_binary() {
    echo "--- Installing latest hab binary for $BUILD_PKG_TARGET using curl|bash"
    curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash -s -- -t "$BUILD_PKG_TARGET"
    hab_binary="/bin/hab"
    declare -g hab_binary
    echo "--- :habicat: Installed latest stable hab: $(${hab_binary} --version)"
}

# The following get/set functions abstract the meta-data key
# names to provide consistant access, taking into account the
# target, where appropriate.

get_hab_ident() {
    local target=$1
    buildkite-agent meta-data get "hab-ident-${target}"
}

has_hab_ident() {
    local target=$1
    buildkite-agent meta-data exists "hab-ident-${target}"
}

set_hab_ident() {
    local target=$1
    local ident=$2
    buildkite-agent meta-data set "hab-ident-${target}" "${ident}"
}

get_hab_artifact() {
    local target=$1
    buildkite-agent meta-data get "hab-artifact-${target}"
}

set_hab_artifact() {
    local target=$1
    local artifact=$2
    buildkite-agent meta-data set "hab-artifact-${target}" "${artifact}"
}

get_hab_release() {
    local target=$1
    buildkite-agent meta-data get "hab-release-${target}"
}

set_hab_release() {
    local target=$1
    local release=$2
    buildkite-agent meta-data set "hab-release-${target}" "${release}"
}

get_studio_ident() {
    local target=$1
    buildkite-agent meta-data get "studio-ident-${target}"
}

has_studio_ident() {
    local target=$1
    buildkite-agent meta-data exists "studio-ident-${target}"
}

set_studio_ident() {
    local target=$1
    local ident=$2
    buildkite-agent meta-data set "studio-ident-${target}" "${ident}"
}

get_backline_ident() {
    local target=$1
    buildkite-agent meta-data get "backline-ident-${target}"
}

set_backline_ident() {
    local target=$1
    local ident=$2
    buildkite-agent meta-data set "backline-ident-${target}" "${ident}"
}

get_backline_artifact() {
    local target=$1
    buildkite-agent meta-data get "backline-artifact-${target}"
}

set_backline_artifact() {
    local target=$1
    local ident=$2
    buildkite-agent meta-data set "backline-artifact-${target}" "${ident}"
}

get_release_channel() {
    echo "habitat-release-${BUILDKITE_BUILD_ID}"
}

get_version() {
    buildkite-agent meta-data get "version"
}

set_version() {
    local version=$1
    buildkite-agent meta-data set "version" "${version}"
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