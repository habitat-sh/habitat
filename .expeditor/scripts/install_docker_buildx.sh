#!/bin/bash

set -euo pipefail

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
PLUGIN_DIR="${HOME}/.docker/cli-plugins"
PLUGIN_PATH="${PLUGIN_DIR}/docker-buildx"
mkdir -p "${PLUGIN_DIR}"
curl --fail -sSL "https://github.com/docker/buildx/releases/download/${BUILDX_VERSION}/buildx-${BUILDX_VERSION}.${BUILDX_ARCH}" \
     -o "${PLUGIN_PATH}"
chmod +x "${PLUGIN_PATH}"

# # Also install system-wide for sudo access
# sudo mkdir -p /usr/local/lib/docker/cli-plugins/
# sudo cp "${PLUGIN_PATH}" /usr/local/lib/docker/cli-plugins/
