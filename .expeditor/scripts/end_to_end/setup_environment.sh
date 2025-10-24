#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

# `channel` should be channel we are pulling from
#
# e.g. `dev`, `acceptance` etc.
channel=${1:?You must specify a channel value}

# Note: We should always have a `hab` binary installed in our CI
# builders / containers.

echo "--- Installing latest chef/hab from ${HAB_BLDR_URL}, ${channel} channel"
# Binlink to '/usr/bin' to ensure we do not run the Chef Workstation
# version of `hab`. (In fact, this will overwrite the link to
# Workstation's `hab`; for our purposes, that's fine.) That is because
# this occurs earlier in the PATH than '/bin' (the default) binlink
# location).
#
# (On container workers, we could use `/usr/local/bin`, like we do with
# Powershell below, but that falls apart on NON-container workloads,
# because there, `/usr/local/bin` comes *later* on the path.  See
# https://github.com/chef/release-engineering/issues/1241 for more.)
sudo -E hab pkg install chef/hab \
     --binlink \
     --binlink-dir=/usr/bin \
     --force \
     --channel="${channel}" \
     --url="${HAB_BLDR_URL}"
echo "--- Using chef/hab version $("${hab_binary}" --version)"

echo "--- Installing latest chef/hab-pkg-export-container from ${HAB_BLDR_URL}, ${channel} channel"
sudo -E hab pkg install chef/hab-pkg-export-container \
    --channel="${channel}" \
    --url="${HAB_BLDR_URL}"

echo "--- Installing latest core/netcat from ${HAB_BLDR_URL}, base-2025 channel"
sudo -E hab pkg install core/netcat \
    --binlink \
    --force \
    --channel="base" \
    --url="${HAB_BLDR_URL}"

echo "--- Installing latest core/powershell from ${HAB_BLDR_URL}, stable channel"
# Binlink to '/usr/local/bin' to ensure we do not run the system installed version. The system
# version is installed in `/usr/bin` which occurs earlier in the PATH than '/bin' (the default)
# binlink location).
sudo -E hab pkg install core/powershell \
    --binlink \
    --binlink-dir="/usr/local/bin" \
    --force \
    --channel="unstable" \
    --url="${HAB_BLDR_URL}"
echo "--- Using core/powershell version $(pwsh --version)"

echo "--- Installing latest core/pester from ${HAB_BLDR_URL}, stable channel"
sudo -E hab pkg install core/pester \
    --channel="stable" \
    --url="${HAB_BLDR_URL}"

sudo useradd --system --no-create-home hab
