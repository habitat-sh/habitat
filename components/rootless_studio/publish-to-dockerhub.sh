#!/bin/bash

set -eou pipefail

source ./docker-base.sh

if [[ -z $DOCKER_LOGIN_USER ]]; then
  echo "DockerHub user required" && exit 1
fi

if [[ -z $DOCKER_LOGIN_PASSWORD ]]; then
  echo "DockerHub password required" && exit 1
fi

docker login -u="$DOCKER_LOGIN_USER" -p="$DOCKER_LOGIN_PASSWORD"
trap 'rm -f $HOME/.docker/config.json' INT TERM EXIT

# Always push the version
docker push "${IMAGE_NAME}:${VERSION}"

# Push additional tags
for arg in "$@"; do
  docker push "${IMAGE_NAME}:${arg}"
done
