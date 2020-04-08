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

echo "--- Installing latest core/hab-pkg-export-docker from ${HAB_BLDR_URL}, ${channel} channel"
sudo -E hab pkg install core/hab-pkg-export-docker \
    --channel="${channel}" \
    --url="${HAB_BLDR_URL}"

echo "--- Installing latest core/netcat from ${HAB_BLDR_URL}, stable channel"
sudo -E hab pkg install core/netcat \
    --binlink \
    --force \
    --channel="stable" \
    --url="${HAB_BLDR_URL}"

echo "--- Installing latest core/powershell from ${HAB_BLDR_URL}, stable channel"
# Binlink to '/usr/local/bin' to ensure we do not run the system installed version. The system
# version is installed in `/usr/bin` which occurs earlier in the PATH than '/bin' (the default)
# binlink location).
sudo -E hab pkg install core/powershell \
    --binlink \
    --binlink-dir="/usr/local/bin" \
    --force \
    --channel="stable" \
    --url="${HAB_BLDR_URL}"
echo "--- Using core/powershell version $(pwsh --version)"

echo "--- Installing latest core/pester from ${HAB_BLDR_URL}, stable channel"
sudo -E hab pkg install core/pester \
    --channel="stable" \
    --url="${HAB_BLDR_URL}"

sudo useradd --system --no-create-home hab
