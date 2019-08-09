#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

# `channel` should be channel we are pulling from
#
# e.g. `DEV`, `ACCEPTANCE` etc.
channel=${1:?You must specify a channel value}

# This currently overrides some functions from the pure buildkite
# shared.sh file above. As we migrate, more things will be added to
# this file.
# source .expeditor/scripts/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"
export HAB_BLDR_CHANNEL="${channel}"

declare -g hab_binary
curlbash_hab "$BUILD_PKG_TARGET"

echo "--- Installing latest core/hab from ${channel}"
${hab_binary} pkg install --binlink --force --channel "${channel}" core/hab

echo "--- $(hab --version)"
