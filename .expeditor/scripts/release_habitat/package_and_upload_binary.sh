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
tmp_root="$(mktemp --directory --tmpdir="$(pwd)" -t "repackage-XXXX")"
cd "${tmp_root}"

echo "--- Downloading core/hab for $BUILD_PKG_TARGET from ${channel} channel"

# Currently, we must explicitly specify `--url`, despite the presence
# of `HAB_BLDR_URL` due to a bug in `hab pkg download`.
#
# Explicitness never hurt, though :)
${hab_binary} pkg download core/hab \
              --target="${BUILD_PKG_TARGET}" \
              --url="${HAB_BLDR_URL}" \
              --channel="${channel}" \
              --download-directory="."
hart="$(find . -type f -name 'core-hab-*-'"${BUILD_PKG_TARGET}"'.hart')"

echo "--- Generating standalone archive from ${hart}"

if [[ $(head -n 1 "${hart}") != HART-1 ]]; then
  echo "Hart file does not match expected format, exiting."
  exit 1
fi
pkg_artifact="$(create_archive_from_hart "${hart}" "${BUILD_PKG_TARGET}")"

echo "--- Uploading to S3"
store_in_s3 "$(get_version_from_repo)" "${pkg_artifact}"
