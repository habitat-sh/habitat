#!/bin/bash

# Build the hab base image and the latest studio image for the given release channel.
# This results in something like:
# habitat/default-studio:0.61.0
# Finally push the tagged image to dockerhub

set -euo pipefail

source .buildkite/scripts/shared.sh
version=$(get_version)
channel=$(get_release_channel)
target="${BUILD_PKG_TARGET}"
image_name="habitat/default-studio-${target}"
image_name_with_tag="${image_name}:${version}"

if is_fake_release; then
  image_name_with_tag="habitat/fakey-mc-fake-face-studio"
fi

docker login \
  --username="${DOCKER_LOGIN_USER}" \
  --password="${DOCKER_LOGIN_PASSWORD}"

trap 'rm -f $HOME/.docker/config.json' INT TERM EXIT

pushd ./components/rootless_studio >/dev/null
docker build --build-arg PACKAGE_TARGET="${target}" -t "habitat-${target}:hab-base" .
docker build \
  --build-arg HAB_LICENSE="${HAB_LICENSE}" \
  --build-arg BLDR_CHANNEL="${channel}" \
  --build-arg PACKAGE_TARGET="${target}" \
  --no-cache \
  --tag "${image_name_with_tag}" \
  ./default

docker push "${image_name_with_tag}"
popd >/dev/null

cat << EOF | buildkite-agent annotate --style=success --context=docker-studio	
<h3>DockerHub Studio Image (Linux)</h3>	
<ul>	
  <li><code>${image_name_with_tag}</code></li>	
</ul>	
EOF
