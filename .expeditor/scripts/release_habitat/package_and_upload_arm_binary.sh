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

cd target/release

# Import GPG keys for s3 upload
import_gpg_keys
# Upload to S3
store_in_s3 "$(get_version_from_repo)" "hab-aarch64-linux.tar.gz"