#!/bin/bash

set -euo pipefail

. .expeditor/scripts/shared.sh

echo "--- :lock: Importing GPG keys"
import_gpg_keys

echo "--- :ruby: Installing 'hub' gem"
gem install hub

echo "--- :thinking_face: Determining sha and version to release"
get_manifest_for_environment "dev"
release_info=$(jq -r '.version + " " + .sha' < manifest.json)
version=$(cut -d' ' -f1 <<< $release_info)
gitsha=$(cut -d' ' -f2 <<< $release_info)

echo "--- :github: Creating release"
if is_real_release; then
  hub release create --message "$version" --commitish "$gitsha" "$version"
else 
  echo "--- NOT CREATING RELEASE: Build triggered by ${BUILDKITE_BUILD_CREATOR}"
  echo "hub release create --message \"$version\" --commitish \"$gitsha\" \"$version\""
fi
