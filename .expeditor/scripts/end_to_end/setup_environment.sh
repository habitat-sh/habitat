#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

# `channel` should be channel we are pulling from
#
# e.g. `dev`, `acceptance` etc.
channel=${1:?You must specify a channel value}
OS=$(uname -s)

# On macOS, /hab is on a read-only root volume (SIP). Create a writable
# APFS volume and mount it at /hab before installing any packages.
# Note: setup_hab_root_macos_pipeline writes to /etc/synthetic.conf which
# requires root privileges, so we run the setup in a sudo bash context
# and persist the volume device for later teardown.
if [[ "${OS}" == "Darwin" ]]; then
    HAB_VOLUME_DEVICE_FILE=$(mktemp /tmp/hab-vol-device.XXXXXX)
    export HAB_VOLUME_DEVICE_FILE
    sudo -E bash -c "
        source .expeditor/scripts/shared.sh
        setup_hab_root_macos_pipeline
        echo \"\$HAB_VOLUME_DEVICE\" > \"$HAB_VOLUME_DEVICE_FILE\"
    "
    HAB_VOLUME_DEVICE=$(cat "$HAB_VOLUME_DEVICE_FILE")
    export HAB_VOLUME_DEVICE
fi

# Note: We should always have a `hab` binary installed in our CI
# builders / containers.

echo "--- Installing latest chef/hab from ${HAB_BLDR_URL}, ${channel} channel"
# On macOS, /usr/bin is SIP-protected so we use /usr/local/bin instead.
# On Linux, binlink to '/usr/bin' to ensure we do not run the Chef Workstation
# version of `hab`. (In fact, this will overwrite the link to
# Workstation's `hab`; for our purposes, that's fine.) That is because
# this occurs earlier in the PATH than '/bin' (the default) binlink
# location).
#
# (On container workers, we could use `/usr/local/bin`, like we do with
# Powershell below, but that falls apart on NON-container workloads,
# because there, `/usr/local/bin` comes *later* on the path.  See
# https://github.com/chef/release-engineering/issues/1241 for more.)
if [[ "${OS}" == "Darwin" ]]; then
    HAB_BINLINK_DIR="/usr/local/bin"
else
    HAB_BINLINK_DIR="/usr/bin"
fi
# Install the package first (without binlinking)
sudo -E hab pkg install chef/hab \
     --channel="${channel}" \
     --url="${HAB_BLDR_URL}"
# On macOS, the Anka VM may have a pre-existing /usr/local/bin/hab as a
# regular file (not a symlink). --binlink --force only overwrites symlinks,
# so remove the old file first, then binlink separately.
if [[ "${OS}" == "Darwin" ]]; then
    sudo rm -f "${HAB_BINLINK_DIR}/hab" "${HAB_BINLINK_DIR}/NOTICES.txt"
fi
sudo -E hab pkg binlink chef/hab \
     --dest="${HAB_BINLINK_DIR}" \
     --force
echo "--- Using chef/hab version $("${hab_binary}" --version)"

echo "--- Installing latest chef/hab-pkg-export-container from ${HAB_BLDR_URL}, ${channel} channel"
sudo -E hab pkg install chef/hab-pkg-export-container \
    --channel="${channel}" \
    --url="${HAB_BLDR_URL}"

# macOS ships with netcat at /usr/bin/nc; no need to install the Habitat package
if [[ "${OS}" != "Darwin" ]]; then
    echo "--- Installing latest core/netcat from ${HAB_BLDR_URL}, base-2025 channel"
    sudo -E hab pkg install core/netcat \
        --binlink \
        --force \
        --channel="base" \
        --url="${HAB_BLDR_URL}"
fi

echo "--- Installing latest core/powershell from ${HAB_BLDR_URL}, stable channel"
# Binlink to '/usr/local/bin' to ensure we do not run the system installed version. The system
# version is installed in `/usr/bin` which occurs earlier in the PATH than '/bin' (the default)
# binlink location).
if sudo -E hab pkg install core/powershell \
    --binlink \
    --binlink-dir="/usr/local/bin" \
    --force \
    --channel="base" \
    --url="${HAB_BLDR_URL}" 2>/dev/null; then
    echo "--- Using core/powershell version $(pwsh --version)"
elif [[ "${OS}" == "Darwin" ]]; then
    echo "--- core/powershell not available for this platform, installing via Homebrew"
    if ! command -v pwsh &>/dev/null; then
        brew install --cask powershell
    fi
    echo "--- Using system pwsh version $(pwsh --version)"
else
    echo "--- Failed to install core/powershell" >&2
    exit 1
fi

echo "--- Installing latest core/pester from ${HAB_BLDR_URL}, stable channel"
sudo -E hab pkg install core/pester \
    --channel="stable" \
    --url="${HAB_BLDR_URL}"

if [[ "${OS}" == "Darwin" ]]; then
    # On macOS, create the 'hab' system user using dscl if it does not already exist
    if ! id hab &>/dev/null; then
        NEXT_UID=$(dscl . -list /Users UniqueID | awk '{print $2}' | sort -n | tail -1 | xargs -I{} expr {} + 1)
        sudo dscl . -create /Users/hab
        sudo dscl . -create /Users/hab UniqueID "$NEXT_UID"
        sudo dscl . -create /Users/hab PrimaryGroupID 20
        sudo dscl . -create /Users/hab UserShell /usr/bin/false
        sudo dscl . -create /Users/hab NFSHomeDirectory /var/empty
        sudo dscl . -create /Users/hab RealName "Habitat"
    fi
else
    sudo useradd --system --no-create-home hab
fi
