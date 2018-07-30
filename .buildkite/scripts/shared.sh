#!/bin/bash

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
        --auth="${HAB_TEAM_AUTH_TOKEN}" \
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
