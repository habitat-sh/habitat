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

# TODO (CM): Similar to other uses at the top of the various pipeline
# YAML files, this will also need to be changed once we go to
# production; this is a token for Acceptance Builder, not Prod Builder
export HAB_AUTH_TOKEN
HAB_AUTH_TOKEN=$(vault kv get -field=scotthain-sig-key account/static/habitat/chef-ci)

########################################################################
# CORE LOGIC
get_manifest_for_environment "${source_environment}"
promote_packages_to_builder_channel manifest.json "${destination_channel}"

version="$(jq -r '.version' < manifest.json)"
promote_version_in_s3 "${version}" "${destination_channel}"
########################################################################
