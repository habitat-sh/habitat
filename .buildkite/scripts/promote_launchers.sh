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
        launcher_ident=$(buildkite-agent meta-data get "${metadata_key}")
    else
        launcher_ident=$(latest_from_builder "${target}" stable hab-launcher)
    fi
    echo "${launcher_ident}"
}

linux_launcher="$(resolve_launcher linux-launcher x86_64-linux)"
windows_launcher="$(resolve_launcher windows-launcher x86_64-windows)"

channel=$(buildkite-agent meta-data get "release-channel")

echo "--- :linux: :habicat: Promoting ${linux_launcher} for Linux to ${channel}"
hab pkg promote --auth="${HAB_TEAM_AUTH_TOKEN}" "${linux_launcher}" "${channel}"

echo "--- :windows: :habicat: Promoting ${windows_launcher} for Windows to ${channel}"
hab pkg promote --auth="${HAB_TEAM_AUTH_TOKEN}" "${windows_launcher}" "${channel}"
