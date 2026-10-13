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

# On macOS, /usr/bin is SIP-protected so we use /usr/local/bin.
HAB_BINLINK_DIR="/usr/local/bin"
hab_binary="$(command -v hab)"

# This is required till we have binaries
curlbash_hab "aarch64-darwin" acceptance


# Now we are ready to install the `hab`
# Install the package first (without binlinking)
echo "--- Installing latest chef/hab from ${HAB_BLDR_URL}, ${channel} channel"
sudo -E "$hab_binary" pkg install chef/hab \
     --channel="${channel}" \
     --url="${HAB_BLDR_URL}"

# The Anka VM may have a pre-existing /usr/local/bin/hab as a regular
# file (not a symlink). --binlink --force only overwrites symlinks,
# so remove the old file first, then binlink using the newly installed hab.
HAB_PKG_PATH=$("$hab_binary" pkg path chef/hab)
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

echo "--- Installing latest core/powershell from ${HAB_BLDR_URL}, base-2025 channel"
# Try the hab package first, fall back to Microsoft's prebuilt package.
if sudo -E "$hab_binary" pkg install core/powershell \
    --binlink \
    --binlink-dir="/usr/local/bin" \
    --force \
    --channel="base-2025" \
    --url="${HAB_BLDR_URL}"; then
    echo "--- Using core/powershell version $(pwsh --version)"
else
    echo "--- core/powershell installation failed, checking for system pwsh"
    if ! command -v pwsh &>/dev/null; then
        # Pin the .NET 8-based PowerShell release for the macOS 14 Anka VM.
        # Homebrew builds dependencies from source on this VM, exhausting the
        # E2E timeout. The Microsoft package includes the runtime.
        (
            powershell_version="7.4.20"
            # From https://github.com/PowerShell/PowerShell/releases/tag/v7.4.20
            powershell_sha256="d80572a1c25c3d463d53f68ad0aade05744e3f8b43d5ab764ca23ae2f2170526"
            powershell_tmpdir="$(mktemp -d)"
            trap 'rm -rf "$powershell_tmpdir"' EXIT
            powershell_pkg="${powershell_tmpdir}/powershell-${powershell_version}-osx-arm64.pkg"

            echo "--- Installing Microsoft PowerShell ${powershell_version} for macOS ARM64"
            curl --fail --location --retry 3 \
                "https://github.com/PowerShell/PowerShell/releases/download/v${powershell_version}/powershell-${powershell_version}-osx-arm64.pkg" \
                --output "$powershell_pkg"
            printf '%s  %s\n' "$powershell_sha256" "$powershell_pkg" | shasum -a 256 -c -
            sudo /usr/sbin/installer -pkg "$powershell_pkg" -target /
        )
        # The Microsoft installer creates /usr/local/bin/pwsh.
        export PATH="/usr/local/bin:$PATH"
    fi
    powershell_version="$(pwsh --version)"
    echo "--- Using system pwsh version ${powershell_version}"
fi

pwsh_path="$(command -v pwsh)"

echo "--- Installing latest core/pester from ${HAB_BLDR_URL}, base-2025 channel"
# Try the hab package first, fall back to PowerShell module
if ! sudo -E "$hab_binary" pkg install core/pester \
    --channel="base-2025" \
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
