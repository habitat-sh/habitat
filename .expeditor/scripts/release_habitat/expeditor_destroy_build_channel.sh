#!/bin/bash

# Script that is run by Expeditor to unconditionally clean up the
# release-pipeline scoped Builder channel that contains our release
# candidates.
#
# A successful pipeline run will have promoted these artifacts into
# the `dev` pipeline.
#
# We want to delete the channel whether or not the pipeline completes
# successfully, because otherwise, our channels will build up, causing
# API slowdowns :(

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_BLDR_URL="${expeditor_hab_bldr_url}"
export HAB_AUTH_TOKEN
HAB_AUTH_TOKEN="$(hab_auth_token)"

# Since this script is being run by Expeditor in response to the end
# of a pipeline run, it will provide us the needed information via
# environment variables.
export BUILDKITE_BUILD_ID="${EXPEDITOR_BUILD_ID}"
channel="$(get_release_channel)"

hab bldr channel destroy \
    --origin=core \
    "${channel}"
