#!/bin/bash

: "${HAB_BLDR_CHANNEL:=stable}"
: "${IMAGE_NAME:=habitat/default-studio}"
: "${STUDIO_TYPE:=default}"

if [[ -z $1 ]]; then
  echo "Habitat version required" && exit 1
fi

export VERSION=$1

shift 1
