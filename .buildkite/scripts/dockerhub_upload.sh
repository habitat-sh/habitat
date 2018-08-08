#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh
version=$(buildkite-agent meta-data get "version")

if [[ "${FAKE_RELEASE_TAG}" ]]; then
  # This overrides IMAGE_NAME in docker-base.sh
  export IMAGE_NAME="habitat/fakey-mc-fake-face-studio"
fi

pushd ./components/rootless_studio >/dev/null
./build-docker-image.sh "${version}"
./tag-docker-image.sh "${version}" "rc"
./publish-to-dockerhub.sh "${version}" "rc"
popd >/dev/null
