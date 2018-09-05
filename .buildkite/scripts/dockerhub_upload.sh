#!/bin/bash

# Build the hab base image and the latest studio image for the given release channel.
# This results in something like:
# habitat/default-studio:0.61.0
# Finally push the tagged image to dockerhub

set -euo pipefail

source .buildkite/scripts/shared.sh
version=$(buildkite-agent meta-data get "version")
channel=$(buildkite-agent meta-data get "release-channel")
image_name="habitat/default-studio"

if is_fake_release; then
  image_name="habitat/fakey-mc-fake-face-studio"
fi

docker login \
  --username="${DOCKER_LOGIN_USER}" \
  --password="${DOCKER_LOGIN_PASSWORD}"

trap 'rm -f $HOME/.docker/config.json' INT TERM EXIT

pushd ./components/rootless_studio >/dev/null
docker build -t habitat:hab-base .
docker build --build-arg BLDR_CHANNEL="${channel}" --no-cache --tag "${image_name}:${version}" ./default
docker push "${image_name}:${version}"
popd >/dev/null

cat << EOF | buildkite-agent annotate --style=success --context=docker-studio	
<h3>DockerHub Studio Image (Linux)</h3>	
<ul>	
  <li><code>${image_name:?}:${version:?}</code></li>	
</ul>	
EOF
