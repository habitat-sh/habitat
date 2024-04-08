#!/bin/bash

set -exou pipefail

# Download the latest Hab manifest
aws s3 cp "s3://chef-automate-artifacts/${EXPEDITOR_TARGET_CHANNEL:-stable}/latest/habitat/manifest.json" manifest.json --profile chef-cd

build_version=$(jq -r -c ".version"  manifest.json)

# Download the release-notes for our specific build
curl -o release-notes.md "https://packages.chef.io/release-notes/${EXPEDITOR_PROJECT:-habitat}/${build_version}.md"

topic_title="Chef Habitat $build_version Released!"
topic_body=$(cat <<EOH
We are delighted to announce the availability of version $build_version of Chef Habitat.
$(cat release-notes.md)

---
## Get the Build

You can download binaries from (https://www.chef.io/downloads).
EOH
)

discourse_api_token=$(vault kv get -field token account/static/discourse/chef-ci)
curl --fail -X POST https://discourse.chef.io/posts \
  -H "Content-Type: multipart/form-data" \
  -H "Api-Username: chef-ci" \
  -H "Api-Key: ${discourse_api_token}" \
  -F "category=9" \
  -F "title=${topic_title}" \
  -F "raw=${topic_body}"

# Cleanup
rm release-notes.md
rm manifest.json
