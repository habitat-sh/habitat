#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

set_hab_binary

acceptance_channel() {
  if is_fake_release; then
    get_release_channel
  else
    echo "stable"
  fi
}

backline_ident="$(get_backline_ident "${BUILD_PKG_TARGET}")"
backline_artifact="$(get_backline_artifact "${BUILD_PKG_TARGET}")"

# Ensure we have the package in our local artifact cache
echo "--- :drum_with_drumsticks: Downloading ${backline_ident} locally"
sudo "${hab_binary}" pkg install "${backline_ident}"

echo "--- :drum_with_drumsticks: Uploading ${backline_ident} to acceptance"
sudo "${hab_binary}" pkg upload \
  --url https://bldr.acceptance.habitat.sh \
  --channel "$(acceptance_channel)" \
  --auth "${HAB_AUTH_TOKEN}" \
  /hab/cache/artifacts/"${backline_artifact}"
