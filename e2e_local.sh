#!/bin/bash

set -euo pipefail

# These are the commands that will be sequentially run in the
# container. The final one will be the test itself, which is supplied
# as the first argument to this script
#
# Not every test requires `expect`, but it's not hurting anything to
# install it for everything.
#
# Note that the `ci-studio-common.sh` file is baked into the image we
# use.
raw_commands=(". /opt/ci-studio-common/buildkite-agent-hooks/ci-studio-common.sh"
              ".expeditor/scripts/setup_environment.sh DEV"
              "hab pkg install --binlink --channel=stable core/expect"
              "${*}")

# Add a newline after every command, for feeding into the container.
commands="${raw_commands[*]/%/\\n}"

# The `${variable@E}` notation interprets the `\n` sequences as actual
# newlines.
docker run \
       --rm \
       --interactive \
       --tty \
       --privileged \
       --env-file=$(pwd)/e2e_env \
       --volume $(pwd):/workdir \
       --workdir=/workdir \
       chefes/buildkite /bin/bash -e -c "${commands@E}"
