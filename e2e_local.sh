#!/bin/bash

set -euo pipefail

channel=${1:?You must specify a channel value}
# If no `$test_name` is specified, after the test setup you will be dropped into an interactive bash
# prompt. From there you can run `pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 $test_name`
# to quickly iterate on tests.
test_name=${2:-}

echo ".expeditor/scripts/end_to_end/run_e2e_test.sh '$channel' '$test_name'"

# Note: the Docker socket is added just for testing docker export
# functionality locally.

if [ -n "$test_name" ]; then
    docker run \
           --rm \
           --interactive \
           --tty \
           --privileged \
           --env-file="$(pwd)/e2e_env" \
           --volume="$(pwd):/workdir" \
           --workdir=/workdir \
           -v /var/run/docker.sock:/var/run/docker.sock \
           chefes/buildkite bash -c ".expeditor/scripts/end_to_end/run_e2e_test.sh $channel $test_name"
else
    docker run \
           --rm \
           --interactive \
           --tty \
           --privileged \
           --env-file="$(pwd)/e2e_env" \
           --volume="$(pwd):/workdir" \
           --workdir=/workdir \
           -v /var/run/docker.sock:/var/run/docker.sock \
           chefes/buildkite bash -c ".expeditor/scripts/end_to_end/run_e2e_test.sh $channel"
fi
