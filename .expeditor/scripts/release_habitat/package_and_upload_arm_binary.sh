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

# Import GPG keys for s3 upload
pushd /tmp 
import_gpg_keys 
popd

# Upload to S3
pushd target/release 
store_in_s3 "$(get_version_from_repo)" "hab-aarch64-linux.tar.gz"