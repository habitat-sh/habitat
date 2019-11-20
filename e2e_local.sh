#!/bin/bash

set -euo pipefail

test_name=${1:?You must specify a test name}
channel=${2:-dev}

docker run \
       --rm \
       --interactive \
       --tty \
       --privileged \
       --env-file="$(pwd)/e2e_env" \
       --volume="$(pwd):/workdir" \
       --workdir=/workdir \
       chefes/buildkite bash .expeditor/scripts/end_to_end/run_e2e_test.sh "$channel" "$test_name"
