#!/bin/bash

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

docker run \
    -it \
    --rm \
    --init \
    --volume "$(pwd)":/workdir \
    --workdir /workdir \
    rust:buster \
    /bin/sh -e -c .expeditor/scripts/release_habitat/build_arm_hab_binary.sh

HAB_VERSION=$(get_version_from_repo)

# Copy archive to temp directory to ensure we have read/write permissions
WORKDIR=$(mktemp -d -t hab-build-XXXXXX)
cp target/release/hab-aarch64-linux.tar.gz "$WORKDIR"

# Import GPG keys for s3 upload
cd "$WORKDIR"
import_gpg_keys || echo "Keys imported already"

# Upload to S3
store_in_s3 "1.6.537" "hab-aarch64-linux.tar.gz"