#!/bin/bash

: ${HAB_BLDR_CHANNEL:="stable"}
: ${IMAGE_NAME:="habitat/default-studio"}
: ${STUDIO_TYPE:="default"}

if [[ -z $1 ]]; then
  echo "Habitat version required" && exit 1
fi

docker build -t habitat:hab-base .

docker build --build-arg BLDR_CHANNEL="${HAB_BLDR_CHANNEL}" --no-cache -t "${IMAGE_NAME}:${1}" ./${STUDIO_TYPE}

docker tag "${IMAGE_NAME}:${1}" "${IMAGE_NAME}:latest"
