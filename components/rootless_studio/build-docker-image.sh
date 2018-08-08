#!/bin/bash

source ./docker-base.sh

docker build -t habitat:hab-base .

docker build --build-arg BLDR_CHANNEL="${HAB_BLDR_CHANNEL}" --no-cache -t "${IMAGE_NAME}:${VERSION}" "./${STUDIO_TYPE}"
