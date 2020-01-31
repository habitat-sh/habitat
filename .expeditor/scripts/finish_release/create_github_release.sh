#!/bin/bash

set -euo pipefail

. .expeditor/scripts/shared.sh

echo "--- :lock: Importing GPG keys"
import_gpg_keys

echo "--- Installing 'hub' CLI tool"
install_hub

echo "--- :thinking_face: Determining the version to release"
get_manifest_for_environment "stable"
version="$(jq -r '.version' manifest.json)"

# It is our convention to have a tag for every version.
tag="${version}"

echo "--- :github: Creating release"
maybe_run hub release create --message "${version}" "${tag}"
