#!/bin/bash

# Unpack the hart file from our channel, repack it, and upload it to
# package-router

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

declare -g hab_binary
install_release_channel_hab_binary "x86_64-linux"

import_gpg_keys

# Unsure we *really* need to bother with this tmp_root business, but
# it does help to contain things a bit.
tmp_root="$(mktemp -d -t "repackage-XXXX")"
cd "${tmp_root}"

echo "--- Downloading core/hab for $BUILD_PKG_TARGET from ${channel} channel"

${hab_binary} pkg download core/hab \
              --target="${BUILD_PKG_TARGET}" \
              --channel="${channel}" \
              --download-directory="."
hart="$(find . -type f -name 'core-hab-*-'"${BUILD_PKG_TARGET}"'.hart')"

echo "--- Extracting binaries from ${hart}"

if [[ $(head -n 1 "${hart}") != HART-1 ]]; then
  echo "Hart file does not match expected format, exiting."
  exit 1
fi

# NOTE: The -r for jq is VERY IMPORTANT (otherwise you get literal '"'
# characters in the resulting string).
hab_version=$(${hab_binary} pkg info --json "${hart}" | jq -r '.version')
hab_release=$(${hab_binary} pkg info --json "${hart}" | jq -r '.release')

# We'll extract our binaries into a directory with this name. This
# will also be the sole directory within the resulting archive.
#
# e.g. "hab-0.88.0-20191009205851-x86_64-linux"
archive_dir="hab-${hab_version}-${hab_release}-${BUILD_PKG_TARGET}"
mkdir "${archive_dir}"

# This bit of magic strips off the Habitat header (first 6 lines) from
# the compressed tar file that is a core/hab .hart, and extracts the
# contents of the `bin` directory only, into the ${archive_dir}
# directory.
#
# For Linux and macOS packages, this will just include the single
# `hab` binary, but on Windows, it will include `hab.exe`, as well as
# all the DLL files needed to run it.
#
# At the end of the day, that's all we need to package up in a
# Habitat-agnostic archive.
tail --lines=+6 "${hart}" | \
    tar --extract \
        --directory="${archive_dir}" \
        --xz \
        --strip-components=7 \
        --wildcards "hab/pkgs/core/hab/*/*/bin/"

echo "--- Compressing 'hab' binary"

pkg_name="hab-${BUILD_PKG_TARGET}"

case "$BUILD_PKG_TARGET" in
    *-linux | *-linux-kernel2)
        pkg_artifact="${pkg_name}.tar.gz"
        tar --verbose \
            --create \
            "${archive_dir}" | gzip --best > "${pkg_artifact}"
        ;;
    *-darwin | *-windows)
        # TODO (CM): Why a zip for macOS?
        pkg_artifact="${pkg_name}.zip"
        zip -9 -r "${pkg_artifact}" "${archive_dir}"
        ;;
    *)
        exit_with "${hart} has unknown TARGET=$BUILD_PKG_TARGET" 3
        ;;
esac

store_in_s3 "$(get_version_from_repo)" "${pkg_artifact}"
