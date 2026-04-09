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

# Note: We should always have a `hab` binary installed in our CI
# Anka VMs.

HAB_BLDR_URL="https://bldr.acceptance.habitat.sh"
HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"

# On macOS, /usr/bin is SIP-protected so we use /usr/local/bin.
HAB_BINLINK_DIR="/usr/local/bin"
hab_binary="$(command -v hab)"

install_acceptance_bootstrap_hab_binary

# Install the package first (without binlinking)
echo "--- Installing latest chef/hab from ${HAB_BLDR_URL}, ${channel} channel"
sudo -E "$hab_binary" pkg install chef/hab \
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

# Enable macOS native studio support (same flags used in release builds)
export HAB_FEAT_MACOS_NATIVE_SUPPORT=1

# Install hab-studio for native studio tests
echo "--- Installing chef/hab-studio from ${HAB_BLDR_URL}, aarch64-darwin-opt channel"
sudo -E "$hab_binary" pkg install chef/hab-studio \
     --channel="aarch64-darwin-opt" \
     --url="${HAB_BLDR_URL}"

# hab-backline is required by the studio but is only available in stable
echo "--- Installing chef/hab-backline from ${HAB_BLDR_URL}, aarch64-darwin-opt channel"
sudo -E "$hab_binary" pkg install chef/hab-backline \
     --channel="stable" \
     --url="${HAB_BLDR_URL}"
export HAB_STUDIO_BACKLINE_PKG
HAB_STUDIO_BACKLINE_PKG="$(cat "$(hab pkg path chef/hab-backline)/IDENT")"
echo "--- HAB_STUDIO_BACKLINE_PKG=${HAB_STUDIO_BACKLINE_PKG}"

# Override the interpreter identity to core/coreutils (installed via
# hab-backline) because core/busybox-static is not available for
# aarch64-darwin. This is needed for install hook execution.
export HAB_INTERPRETER_IDENT="core/coreutils"

echo "--- Installing latest core/powershell from ${HAB_BLDR_URL}, aarch64-darwin-opt channel"
# Try the hab package first, fall back to Homebrew
if sudo -E "$hab_binary" pkg install core/powershell \
    --binlink \
    --binlink-dir="/usr/local/bin" \
    --force \
    --channel="aarch64-darwin-opt" \
    --url="${HAB_BLDR_URL}"; then
    echo "--- Using core/powershell version $(pwsh --version)"
else
    echo "--- core/powershell not available for this platform, installing via Homebrew"
    if ! command -v pwsh &>/dev/null; then
        if ! command -v brew &>/dev/null; then
            echo "ERROR: Homebrew (brew) not found; cannot install PowerShell." >&2
            exit 1
        fi
        # Explicit update to refresh the stale Anka VM index; the pipeline-level
        # HOMEBREW_NO_AUTO_UPDATE=1 only suppresses the *automatic* update that
        # brew install would otherwise trigger, so there is no double-update.
        sudo -u anka brew update
        sudo -u anka brew install powershell
    fi
    echo "--- Using system pwsh version $(pwsh --version)"
fi

pwsh_path="$(command -v pwsh)"

echo "--- Installing latest core/pester from ${HAB_BLDR_URL}, aarch64-darwin-opt channel"
# Try the hab package first, fall back to PowerShell module
if ! sudo -E "$hab_binary" pkg install core/pester \
    --channel="aarch64-darwin-opt" \
    --url="${HAB_BLDR_URL}"; then
    echo "--- core/pester not available for this platform, installing via PowerShell module"
    # Pin to Pester 4.x to match core/pester on Linux. All existing test
    # files use Pester 4 syntax and Pester 5's legacy adapter is unreliable.
    sudo "$pwsh_path" -Command "Install-Module -Name Pester -MaximumVersion '4.99.99' -Force -SkipPublisherCheck"
    echo "--- Pester version: $(sudo "$pwsh_path" -Command '(Get-Module -ListAvailable Pester).Version')"
fi

# Create the 'hab' system user using sysadminctl if it does not already exist
if ! id hab &>/dev/null; then
    sudo sysadminctl -addUser hab -fullName "Habitat" -shell /bin/bash -home /var/hab -password ""
fi
