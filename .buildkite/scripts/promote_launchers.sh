#!/bin/bash

# If the user specified a fully-qualified identifier for
# `core/hab-launcher` in the Buildkite metadata, then promote that
# release to the release channel.
#
# Otherwise, promote the latest stable release of the Launcher to the
# release channel.

set -euo pipefail
source .buildkite/scripts/shared.sh

resolve_launcher() {
    local metadata_key="${1}"
    local target="${2}"

    if buildkite-agent meta-data exists "${metadata_key}"; then
        buildkite-agent meta-data get "${metadata_key}"
    else
        latest_from_builder "${target}" stable hab-launcher
    fi
}

linux_launcher="$(resolve_launcher linux-launcher x86_64-linux)"
linux_kernel2_launcher="$(resolve_launcher linux-kernel2-launcher x86_64-linux-kernel2)"
windows_launcher="$(resolve_launcher windows-launcher x86_64-windows)"

channel=$(buildkite-agent meta-data get "release-channel")

echo "--- :linux: :habicat: Promoting ${linux_launcher} for x86_64-linux to ${channel}"
hab pkg promote --auth="${HAB_AUTH_TOKEN}" "${linux_launcher}" "${channel}"

echo "--- :linux: :two: :habicat: Promoting ${linux_kernel2_launcher} for x86_64-linux-kernel2 to ${channel}"
hab pkg promote --auth="${HAB_AUTH_TOKEN}" "${linux_kernel2_launcher}" "${channel}"

echo "--- :windows: :habicat: Promoting ${windows_launcher} for x86_64-windows to ${channel}"
hab pkg promote --auth="${HAB_AUTH_TOKEN}" "${windows_launcher}" "${channel}"
