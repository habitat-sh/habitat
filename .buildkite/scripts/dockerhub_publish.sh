#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh
version=$(buildkite-agent meta-data get "version")
channel=$(buildkite-agent meta-data get release-channel)

if is_fake_release; then
  # This overrides IMAGE_NAME in docker-base.sh
  export IMAGE_NAME="habitat/fakey-mc-fake-face-studio"
fi

pushd ./components/rootless_studio >/dev/null
export HAB_BLDR_CHANNEL="${channel}"
./tag-docker-image.sh "${version}" "latest"
./publish-to-dockerhub.sh "${version}" "latest"
popd >/dev/null
