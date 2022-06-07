#!/bin/bash

set -eou pipefail

git clone https://x-access-token:${GITHUB_TOKEN}@github.com/habitat-sh/habitat.wiki.git

# Download the latest Hab manifest
aws s3 cp "s3://chef-automate-artifacts/${EXPEDITOR_TARGET_CHANNEL}/latest/habitat/manifest.json" manifest.json --profile chef-cd

build_version=$(jq -r -c ".version"  manifest.json)

pushd ./chef-server.wiki
  # Publish release notes to S3
  aws s3 cp Pending-Release-Notes.md "s3://chef-automate-artifacts/release-notes/${EXPEDITOR_PROJECT}/${build_version}.md" --acl public-read --content-type "text/plain" --profile chef-cd

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
rm -rf chef-server.wiki
rm manifest.json
