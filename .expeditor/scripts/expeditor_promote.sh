#!/bin/bash

# Promotion logic tailored for use in Expeditor-triggered promotions,
# rather than promotions that take place automatically within
# Buildkite pipelines.
#
# This should basically just be a bare facade around the core
# promotion logic.

set -euo pipefail

source .expeditor/scripts/shared.sh

source_environment="${EXPEDITOR_PROMOTABLE}"
destination_channel="${EXPEDITOR_TARGET_CHANNEL}"

declare -g hab_binary
curlbash_hab "x86_64-linux"

export HAB_AUTH_TOKEN
HAB_AUTH_TOKEN=$(hab_auth_token)

export HAB_BLDR_URL="${expeditor_hab_bldr_url}"

########################################################################
# CORE LOGIC
import_gpg_keys
get_manifest_for_environment "${source_environment}"
promote_packages_to_builder_channel manifest.json "${destination_channel}"

version="$(jq -r '.version' < manifest.json)"
promote_version_in_s3 "${version}" "${destination_channel}"
########################################################################
