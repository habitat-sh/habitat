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

# Just use the hab that's on the box
declare -g hab_binary
hab_binary="hab"

export HAB_AUTH_TOKEN
HAB_AUTH_TOKEN=$(hab_auth_token)

export HAB_BLDR_URL="${temporary_hab_bldr_url}"

########################################################################
# CORE LOGIC
get_manifest_for_environment "${source_environment}"
promote_packages_to_builder_channel manifest.json "${destination_channel}"

version="$(jq -r '.version' < manifest.json)"
promote_version_in_s3 "${version}" "${destination_channel}"
########################################################################
