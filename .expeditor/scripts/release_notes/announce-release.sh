#!/bin/bash

set -exou pipefail

# Download the latest Hab manifest
aws s3 cp "s3://chef-automate-artifacts/${EXPEDITOR_TARGET_CHANNEL}/latest/habitat/manifest.json" manifest.json --profile chef-cd

build_version=$(jq -r -c ".version"  manifest.json)

# Download the release-notes for our specific build
curl -o release-notes.md "https://packages.chef.io/release-notes/${EXPEDITOR_PROJECT}/${build_version}.md"

topic_title="Chef Habitat $build_version Released!"
topic_body=$(cat <<EOH
We are delighted to announce the availability of version $build_version of Chef Habitat.
$(cat release-notes.md)

---
## Get the Build

You can download binaries directly from [chef.io/downloads](https://www.chef.io/downloads/tools/habitat?v=$EXPEDITOR_VERSION).
EOH
)

# Use Expeditor's built in Bash helper to post our message: https://git.io/JvxPm
post_discourse_release_announcement "$topic_title" "$topic_body"

# Cleanup
rm release-notes.md
rm manifest.json
