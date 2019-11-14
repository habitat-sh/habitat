#!/bin/bash

set -euo pipefail

. .expeditor/scripts/shared.sh

echo "--- :lock: Importing GPG keys"
import_gpg_keys

echo "--- :ruby: Installing 'hub' gem"
gem install hub

echo "--- :thinking_face: Determining sha and version to release"
get_manifest_for_environment "stable"
read -r version gitsha <<< "$(jq -r '.version + " " + .sha' manifest.json)"

echo "--- :github: Creating release"
maybe_run hub release create --message "$version" --commitish "$gitsha" "$version"
