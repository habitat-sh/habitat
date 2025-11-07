#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

git clone https://x-access-token:"${GITHUB_TOKEN:-$(chef_ci_github_token)}"@github.com/habitat-sh/habitat.wiki.git

# Download the latest Hab manifest
aws s3 cp "s3://chef-automate-artifacts/${EXPEDITOR_TARGET_CHANNEL:-stable}/latest/habitat/manifest.json" manifest.json --profile chef-cd

build_version=$(jq -r -c ".version"  manifest.json)

pushd ./habitat.wiki
  # Prepend release date to release notes
  RELEASE_DATE="Release date: $(date +"%B %d, %Y")"
  printf '%s\n\n' "$RELEASE_DATE" | cat - Pending-Release-Notes.md > temp && mv temp Pending-Release-Notes.md

  # Publish release notes to S3
  aws s3 cp Pending-Release-Notes.md "s3://chef-automate-artifacts/release-notes/${EXPEDITOR_PROJECT:-habitat}/${build_version}.md" --acl public-read --content-type "text/plain" --profile chef-cd

  # Reset "Stable Release Notes" wiki page
  cat >./Pending-Release-Notes.md <<EOH
## New Features
-
## Improvements
-
## Bug Fixes
-
## Backward Incompatibilities
-
EOH

  # Push changes back up to GitHub
  git add .
  git commit -m "Release Notes for promoted build $build_version"
  git push origin master
popd

# Cleanup
rm -rf habitat.wiki
rm manifest.json
