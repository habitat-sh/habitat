#!/bin/bash

set -eou pipefail

# Download public and private keys for the "core" origin from Builder.
#
# Currently relies on a global variable `hab_binary` being set, since
# in the Linux build process, we need to switch binaries mid-way
# through the pipeline. As we bring more platforms into play, this may
# change. FYI.
import_keys() {
    echo "--- :key: Downloading 'core' public keys from Builder"
    ${hab_binary:?} origin key download core
    echo "--- :closed_lock_with_key: Downloading latest 'core' secret key from Builder"
    ${hab_binary:?} origin key download \
        --auth="${HAB_AUTH_TOKEN}" \
        --secret \
        core
    # TODO (CM): delete the secret key later?
}

# Given a platform target, channel, package name, and optional
# version, return the fully-qualified identifier of the latest such
# `core` package in Builder.
#
# Examples:
#
#  latest_from_builder x86_64-linux stable hab 0.58.0
#  # => core/hab/0.58.0/20180629144346
#
#  latest_from_builder x86_64-windows stable hab-launcher
#  # => core/hab-launcher/7241/20180321094917
latest_from_builder() {
    target="${1}"
    channel="${2}"
    package_name="${3}"
    version="${4:-}"

    if [ -z "${version}" ]; then
        url="https://bldr.habitat.sh/v1/depot/channels/core/${channel}/pkgs/${package_name}/latest?target=${target}"
    else
        url="https://bldr.habitat.sh/v1/depot/channels/core/${channel}/pkgs/${package_name}/${version}/latest?target=${target}"
    fi

    ident=$(curl -s "${url}" | jq -r '.ident | .origin + "/" + .name + "/" + .version + "/" + .release')
    echo "${ident}"
}

# Abstracts the logic (such as it is) for whether we're doing a "fake"
# release or not.

set_fake_release() {
    local release=${1}
    buildkite-agent meta-data set fake-release "${release}"
}

is_fake_release() {
    buildkite-agent meta-data exists fake-release
}

get_fake_release() {
    buildkite-agent meta-data get fake-release
}

# Until we have built both a new core/hab _and_ a new core/hab-studio
# package, we should continue to use the `hab` binary provided on our
# Buildkite builders (managed by Release Engineering) (these should be
# the latest stable release, btw).
#
# Once we have bootstrapped ourselves enough, however, we should
# switch subsequent builds to use the new hab, which in turn uses the
# new studio.
set_hab_binary() {
    echo "--- :thinking_face: Determining which 'hab' binary to use"
    # This ensure the hab cli we use going forward has the correct
    # ActiveTarget. Otherwise, if we were to attempt to install an
    # `x86_64-linux-kernel2` package with the `hab` on our path, it
    # would result in an error and fail the build.
    # 
    # TODO (SM): We also need to set the pkg_target so that we pull 
    # the correct meta-data from BK for hab-version and hab-studio
    # It might be better to expect BUILD_PKG_TARGET to always be 
    # explicitly set. 
    local pkg_target
    case "${BUILD_PKG_TARGET}" in
        x86_64-linux)
            pkg_target="x86_64-linux"
            install_hab_binary "$pkg_target" "/hab/bin"
            hab_binary="/hab/bin/hab"
            ;;
        x86_64-linux-kernel2)
            pkg_target="x86_64-linux-kernel2"
            install_hab_binary "$pkg_target" "/hab/bin"
            hab_binary="/hab/bin/hab"
            ;;
        x86_64-windows)
            # We're going to use the existing hab binary here.
            # This is hella abusing this.
            hab_binary="$(command -v hab)"
            pkg_target="x86_64-windows"
            ;;
        *) 
            echo "--- :no_entry_sign: Unknown PackageTarget: ${BUILD_PKG_TARGET}"
            exit 1
            ;;
    esac
        
    if has_hab_ident "${pkg_target}" && has_studio_ident "${pkg_target}"; then
        echo "Buildkite metadata found; installing new versions of 'core/hab' and 'core/hab-studio'"
        # By definition, these will be fully-qualified identifiers,
        # and thus do not require a `--channel` option. However, they
        # should be coming from the release channel, and should be the
        # same packages built previously in this same release pipeline.
        hab_ident=$(get_hab_ident "${pkg_target}")

        # Note that we are explicitly not binlinking here; this is to
        # prevent accidentally polluting the builder for any future
        # runs that may take place on it.
        sudo env HAB_LICENSE="${HAB_LICENSE}" "${hab_binary:?}" pkg install "${hab_ident}"
        sudo env HAB_LICENSE="${HAB_LICENSE}" "${hab_binary:?}" pkg install "$(get_studio_ident $pkg_target)"
        hab_binary="/hab/pkgs/${hab_ident}/bin/hab"
        declare -g new_studio=1
    else
        echo "Buildkite metadata NOT found; using previously-installed hab binary: $hab_binary"
    fi
    declare -g hab_binary
    echo "--- :habicat: Using $(${hab_binary} --version)"
}


# Use the install.sh script which lives in this repository to download the latest version of Habitat
install_hab_binary() {
    local target install_path
    
    target="$1"
    install_path="$2"

    sudo env HAB_LICENSE="${HAB_LICENSE}" ./components/hab/install.sh -t "$target"
    sudo env HAB_LICENSE="${HAB_LICENSE}" /bin/hab pkg binlink core/hab hab -d $install_path 
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
    buildkite-agent meta-data get "release-channel"
}

set_release_channel() {
    local channel=$1
    buildkite-agent meta-data set "release-channel" "${channel}"
}

get_version() {
    buildkite-agent meta-data get "version"
}

set_version() {
    local version=$1
    buildkite-agent meta-data set "version" "${version}"
}
