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

docker version

# Determine architecture based on BUILD_PKG_TARGET
case "${BUILD_PKG_TARGET}" in
"x86_64-linux")
       BUILDX_ARCH="linux-amd64"
       ;;
"aarch64-linux")
       BUILDX_ARCH="linux-arm64"
       ;;
*)
       echo "Unsupported BUILD_PKG_TARGET: ${BUILD_PKG_TARGET}"
       echo "Defaulting to linux-amd64"
       BUILDX_ARCH="linux-amd64"
       ;;
esac

echo "Installing buildx manually..."
BUILDX_VERSION="v0.32.1"  # Use a stable version
mkdir -p ~/.docker/cli-plugins/
curl -sSL "https://github.com/docker/buildx/releases/download/${BUILDX_VERSION}/buildx-${BUILDX_VERSION}.${BUILDX_ARCH}" \
-o ~/.docker/cli-plugins/docker-buildx
chmod +x ~/.docker/cli-plugins/docker-buildx

# Also install system-wide for sudo access
sudo mkdir -p /usr/local/lib/docker/cli-plugins/
sudo cp ~/.docker/cli-plugins/docker-buildx /usr/local/lib/docker/cli-plugins/

# Verify installation
docker buildx version

# source .expeditor/scripts/release_habitat/shared.sh


# NOTE: This operation currently uses the `chefdelivery` service
# account; these credentials are automatically injected by the
# elastic-ci-secrets plugin.
# docker login \
#   --username="${DOCKER_LOGIN_USER}" \
#   --password="${DOCKER_LOGIN_PASSWORD}"

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

#     docker build \
#            --secret id=hab_auth_token,src=hab_auth_token.txt \
#            --build-arg HAB_LICENSE="accept-no-persist" \
#            --build-arg BLDR_URL="${bldr_url}" \
#            --build-arg BLDR_CHANNEL="${channel}" \
#            --build-arg PACKAGE_TARGET="${target}" \
#            --no-cache \
#            --tag "${image_name_with_tag}" \
#            ./default

#     docker push "${image_name_with_tag}"
)
