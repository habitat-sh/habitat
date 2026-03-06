#!/usr/bin/env bash

# Builds a "cleanroom" Docker container to run BATS tests in, and then
# executes the tests in that container, mounting the tests and Habitat
# binaries as needed.

set -euo pipefail

# Handy escape hatch for running a single file for quicker local
# development, e.g.:
#
# cd <ROOT>/.expeditor
# run-bats.sh manifest.bats
#
if [ $# -eq 1 ] ; then
    TESTS=".expeditor/test/$1"
else
    TESTS=".expeditor/test"
fi

image="hab-bats-cleanroom"
container_name="expeditor-ci-bats-${BUILDKITE_BUILD_ID:-local}"

echo "$HAB_AUTH_TOKEN" > hab_auth_token.txt
trap 'rm -f hab_auth_token.txt' INT TERM EXIT

docker build --secret id=hab_auth_token,src=hab_auth_token.txt --tag "${image}" --file test/Dockerfile ..

# Mount the whole repository at /test, because various `source` calls
# assume that's where you are.
docker run -it --rm \
       --mount type=bind,source="$(pwd)/..",target=/test \
       --workdir=/test \
       --name "$container_name" \
       --env HAB_AUTH_TOKEN \
       "${image}" \
       bats "${TESTS}"
