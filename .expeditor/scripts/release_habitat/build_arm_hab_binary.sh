#!/bin/bash

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

# This is required to enable non-interactive installation of timezone data
export DEBIAN_FRONTEND=noninteractive

# Ensure all build and test dependencies are installed
apt-get update && apt-get install -y ca-certificates sudo gcc libc6-dev wget openssl make pkg-config libzmq3-dev curl cmake protobuf-compiler

# Build all binaries
cargo build --release

cd target/release

# Strip all debug symbols from our binaries
strip --strip-debug hab-sup
strip --strip-debug hab-launch
strip --strip-debug hab

# Put all the binaries in an archive
tar -zcf hab-aarch64-linux.tar.gz hab hab-sup hab-launch --transform 's,^,bin/,'