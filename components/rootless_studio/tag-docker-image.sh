#!/bin/bash

source ./docker-base.sh

docker pull "${IMAGE_NAME}:${VERSION}"

for arg in "$@"; do
  docker tag "${IMAGE_NAME}:${VERSION}" "${IMAGE_NAME}:${arg}"
done
