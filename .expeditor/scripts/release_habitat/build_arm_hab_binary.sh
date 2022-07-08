#!/bin/bash

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

# This is required to enable non-interactive installation of timezone data
export DEBIAN_FRONTEND=noninteractive

# Ensure all build and test dependencies are installed
apt-get update && apt-get install -y ca-certificates sudo gcc libc6-dev wget openssl make pkg-config libzmq3-dev curl cmake

# Build all binaries
cargo build --release

cd target/release

# Strip all debug symbols from our binaries
strip --strip-debug hab-sup
strip --strip-debug hab-launch
strip --strip-debug hab

# Put all the binaries in an archive
tar -zcf hab-aarch64-linux.tar.gz hab hab-sup hab-launch

HAB_VERSION=$(get_version_from_repo)

# Copy archive to temp directory to ensure we have read/write permissions
WORKDIR=$(mktemp -d -t hab-build-XXXXXX)
cp hab-aarch64-linux.tar.gz "$WORKDIR"

# Import GPG keys for s3 upload
cd "$WORKDIR"
import_gpg_keys

# Upload to S3
store_in_s3 "$HAB_VERSION" "hab-aarch64-linux.tar.gz"