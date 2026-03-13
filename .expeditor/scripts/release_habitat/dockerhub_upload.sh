#!/bin/bash

# Build the hab base image and the latest studio image for the given
# release channel.
#
# This results in something like:
#
#    habitat/default-studio:0.61.0
#
# Finally push the tagged image to dockerhub.
#
# Note that pushing these to Dockerhub is OK, even though they may
# correspond to versions of Habitat that will never be officially
# released, because the image that is ultimately used by a version of
# Habitat is directly keyed to the version of the Habitat binary being
# used.

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

# Create the token file securely
echo "$HAB_AUTH_TOKEN" > ./components/rootless_studio/hab_auth_token.txt
trap 'rm -f $HOME/.docker/config.json ./components/rootless_studio/hab_auth_token.txt' INT TERM EXIT

(
    cd ./components/rootless_studio

    # TODO (CM): I'm not entirely certain why this build is split into
    # two separate invocations, when the entire flow is essentially
    # that of a multistage Dockerfile, which the second Dockerfile
    # actually is.

    docker build \
           --secret id=hab_auth_token,src=hab_auth_token.txt \
           --build-arg PACKAGE_TARGET="${BUILD_PKG_TARGET}" \
           --tag "habitat-${BUILD_PKG_TARGET}:hab-base" .
)
