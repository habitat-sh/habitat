#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

# `channel` should be channel we are pulling from
#
# e.g. `dev`, `acceptance` etc.
channel=${1:?You must specify a channel value}

declare -g hab_binary
curlbash_hab "$BUILD_PKG_TARGET"

echo "--- Installing latest core/hab from ${HAB_BLDR_URL}, ${channel} channel"
sudo -E hab pkg install core/hab \
    --binlink \
    --force \
    --channel="${channel}" \
    --url="${HAB_BLDR_URL}"
echo "--- Using core/hab version $(hab --version)"

echo "--- Installing latest core/netcat from ${HAB_BLDR_URL}, stable channel"
sudo -E hab pkg install core/netcat \
    --binlink \
    --force \
    --channel="stable" \
    --url="${HAB_BLDR_URL}"

echo "--- Installing latest core/powershell from ${HAB_BLDR_URL}, stable channel"
sudo -E hab pkg install core/powershell \
    --binlink \
    --force \
    --channel="stable" \
    --url="${HAB_BLDR_URL}"
echo "--- Using core/powershell version $(pwsh --version)"

sudo useradd --system --no-create-home hab
