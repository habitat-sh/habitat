#!/bin/bash

# Unpack the hart file from our channel, repack it, and upload it to
# package-router

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

declare -g hab_binary
curlbash_hab "x86_64-linux"

import_gpg_keys

# Unsure we *really* need to bother with this tmp_root business, but
# it does help to contain things a bit.
tmp_root="$(mktemp --directory --tmpdir="$(pwd)" -t "repackage-XXXX")"
cd "${tmp_root}"

# We need to do special things for aarch64-darwin till we have a new builder
# release. TODO: Remove this and JOB_BLDR_URL setting when we release builder
if [[ "${BUILD_PKG_TARGET}" == "aarch64-darwin" ]]; then
    export HAB_BLDR_URL="${JOB_HAB_BLDR_URL}"
    export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
fi

echo "--- Downloading chef/hab for $BUILD_PKG_TARGET from ${channel} channel"

# Currently, we must explicitly specify `--url`, despite the presence
# of `HAB_BLDR_URL` due to a bug in `hab pkg download`.
#
# Explicitness never hurt, though :)
${hab_binary} pkg download chef/hab \
              --target="${BUILD_PKG_TARGET}" \
              --url="${HAB_BLDR_URL}" \
              --channel="${channel}" \
              --download-directory="."
hart="$(find . -type f -name 'chef-hab-*-'"${BUILD_PKG_TARGET}"'.hart')"

echo "--- Generating standalone archive from ${hart}"

if [[ $(head -n 1 "${hart}") != HART-1 ]]; then
  echo "Hart file does not match expected format, exiting."
  exit 1
fi
pkg_artifact="$(create_archive_from_hart "${hart}" "${BUILD_PKG_TARGET}")"

echo "--- Uploading to S3"
store_in_s3 "$(get_version_from_repo)" "${pkg_artifact}"
