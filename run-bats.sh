#!/usr/bin/env bash

# Builds a "cleanroom" Docker container to run BATS tests in, and then
# executes the tests in that container, mounting the tests and Habitat
# binaries as needed.

if [ $# -eq 0 ] ; then
    TESTS="."
else
    TESTS="$*"
fi

docker build -t hab-bats-cleanroom "$(pwd)"/test/integration

docker run -it --rm \
       --mount type=bind,source="$(pwd)/test/integration",target=/test \
       --mount type=bind,source="$(pwd)/target/debug/hab-launch",target=/bin/hab-launch \
       --mount type=bind,source="$(pwd)/target/debug/hab-sup",target=/bin/hab-sup \
       --mount type=bind,source="$(pwd)/target/debug/hab",target=/bin/hab \
       --env HAB_BIN_DIR=/bin \
       --workdir=/test \
       --name hab-bats \
       hab-bats-cleanroom \
       bats "${TESTS}"