#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

# `channel` should be channel we are pulling from
#
# e.g. `DEV`, `ACCEPTANCE` etc.
channel=${1:?You must specify a channel value}

curlbash_hab "$BUILD_PKG_TARGET"

echo "--- Installing latest core/hab from ${HAB_BLDR_URL}, ${channel} channel"
hab pkg install core/hab \
    --binlink \
    --force \
    --channel "${channel}" \
    --url="${HAB_BLDR_URL}"
echo "--- Using $(hab --version)"
