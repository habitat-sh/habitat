#!/bin/bash

set -eoux pipefail

CHANNEL=$1
VERSION=$2

shift 2
sudo hab pkg install -c "$CHANNEL" core/hab-backline/"$VERSION"
hab pkg upload -u https://bldr.acceptance.habitat.sh -c stable "$@" \
    /hab/cache/artifacts/core-hab-backline-"$VERSION"-*-x86_64-linux.hart
