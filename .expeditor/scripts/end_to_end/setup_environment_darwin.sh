#!/bin/bash

# macOS (aarch64-darwin) environment setup for end-to-end tests.
# This is the Darwin-specific counterpart of setup_environment.sh and avoids
# conditional logic that could accidentally affect Linux CI jobs.

set -euo pipefail

source .expeditor/scripts/shared.sh

# `channel` should be channel we are pulling from
#
# e.g. `dev`, `acceptance` etc.
channel=${1:?You must specify a channel value}

# On macOS, /hab is on a read-only root volume (SIP). Create a writable
# APFS volume and mount it at /hab before installing any packages.
# setup_hab_root_macos_pipeline writes to /etc/synthetic.conf which
# requires root privileges, so we run the setup in a sudo bash context
# and persist the volume device for later teardown.
HAB_VOLUME_DEVICE_FILE=$(mktemp /tmp/hab-vol-device.XXXXXX)
export HAB_VOLUME_DEVICE_FILE
sudo -E bash -c "
    source .expeditor/scripts/shared.sh
    setup_hab_root_macos_pipeline
    echo \"\$HAB_VOLUME_DEVICE\" > \"$HAB_VOLUME_DEVICE_FILE\"
"
HAB_VOLUME_DEVICE=$(cat "$HAB_VOLUME_DEVICE_FILE")
export HAB_VOLUME_DEVICE

# Note: We should always have a `hab` binary installed in our CI
# Anka VMs.

echo "--- Installing latest chef/hab from ${HAB_BLDR_URL}, ${channel} channel"
# On macOS, /usr/bin is SIP-protected so we use /usr/local/bin.
HAB_BINLINK_DIR="/usr/local/bin"

# Install the package first (without binlinking)
sudo -E hab pkg install chef/hab \
     --channel="${channel}" \
     --url="${HAB_BLDR_URL}"

# The Anka VM may have a pre-existing /usr/local/bin/hab as a regular
# file (not a symlink). --binlink --force only overwrites symlinks,
# so remove the old file first, then binlink using the newly installed hab.
HAB_PKG_PATH=$(hab pkg path chef/hab)
sudo rm -f "${HAB_BINLINK_DIR}/hab" "${HAB_BINLINK_DIR}/NOTICES.txt"
sudo -E "${HAB_PKG_PATH}/bin/hab" pkg binlink chef/hab \
     --dest="${HAB_BINLINK_DIR}" \
     --force

hab_binary="${HAB_BINLINK_DIR}/hab"
echo "--- Using chef/hab version $("${hab_binary}" --version)"

# hab-pkg-export-container is not available for aarch64-darwin — skip it

# macOS ships with netcat at /usr/bin/nc — no need to install the Habitat package

# Supervisor tests are skipped on macOS (supervisor support is not yet
# mature on aarch64-darwin), so we do not install hab-sup / hab-launcher.

echo "--- Installing latest core/powershell from ${HAB_BLDR_URL}, stable channel"
# Try the hab package first, fall back to Homebrew
if sudo -E hab pkg install core/powershell \
    --binlink \
    --binlink-dir="/usr/local/bin" \
    --force \
    --channel="base" \
    --url="${HAB_BLDR_URL}" 2>/dev/null; then
    echo "--- Using core/powershell version $(pwsh --version)"
else
    echo "--- core/powershell not available for this platform, installing via Homebrew"
    if ! command -v pwsh &>/dev/null; then
        brew update
        brew install powershell
    fi
    echo "--- Using system pwsh version $(pwsh --version)"
fi

echo "--- Installing latest core/pester from ${HAB_BLDR_URL}, stable channel"
# Try the hab package first, fall back to PowerShell module
if ! sudo -E hab pkg install core/pester \
    --channel="stable" \
    --url="${HAB_BLDR_URL}" 2>/dev/null; then
    echo "--- core/pester not available for this platform, installing via PowerShell module"
    # Pin to Pester 4.x to match core/pester on Linux. All existing test
    # files use Pester 4 syntax and Pester 5's legacy adapter is unreliable.
    sudo pwsh -Command "Install-Module -Name Pester -MaximumVersion '4.99.99' -Force -SkipPublisherCheck"
    echo "--- Pester version: $(sudo pwsh -Command '(Get-Module -ListAvailable Pester).Version')"
fi

# Create the 'hab' system user using dscl if it does not already exist
if ! id hab &>/dev/null; then
    NEXT_UID=$(dscl . -list /Users UniqueID | awk '{print $2}' | sort -n | tail -1 | xargs -I{} expr {} + 1)
    sudo dscl . -create /Users/hab
    sudo dscl . -create /Users/hab UniqueID "$NEXT_UID"
    sudo dscl . -create /Users/hab PrimaryGroupID 20
    sudo dscl . -create /Users/hab UserShell /usr/bin/false
    sudo dscl . -create /Users/hab NFSHomeDirectory /var/empty
    sudo dscl . -create /Users/hab RealName "Habitat"
fi
