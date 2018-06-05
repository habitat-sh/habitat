#!/bin/bash

: ${IMAGE_NAME:="habitat/default-studio"}

if [[ -z $DOCKERHUB_USER ]]; then
  echo "DockerHub user required" && exit 1
fi

if [[ -z $DOCKERHUB_PASSWORD ]]; then
  echo "DockerHub password required" && exit 1
fi

if [[ -z $1 ]]; then
  echo "Habitat version required" && exit 1
fi

docker login -u="$DOCKERHUB_USER" -p="$DOCKERHUB_PASSWORD"
trap 'rm -f $HOME/.docker/config.json; exit $?' INT TERM EXIT

docker push "${IMAGE_NAME}:$1"
docker push "${IMAGE_NAME}:latest"
