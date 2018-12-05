#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

echo "--- :drum_with_drumsticks: Uploading hab-backline to acceptance" 

acceptance_channel() {
  if is_fake_release; then
    echo "$(get_release_channel)"
  else
    echo "stable"
  fi
}

backline_ident="$(get_backline_ident x86_64-linux)"
backline_artifact="$(get_backline_artifact x86_64-linux)"

# Ensure we have the package in our local artifact cache
hab pkg install "$backline_ident"

# The above pkg install is executed in the context of the buildkite user
# so the artifact (and public signing keys) will be cached in the users
# home, rather than the system cache (/hab). If the below breaks with a:
#
# Invalid value for '<HART_FILE>...': File: 'XXXX.hart' cannot be found, 
#
# then it is likely that something has changed with how Hab caches files,
# or the user this is executed as.
hab pkg upload -u https://bldr.acceptance.habitat.sh -c "$(acceptance_channel)" \
  ~/.hab/cache/artifacts/"$backline_artifact"
