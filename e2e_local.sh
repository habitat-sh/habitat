#!/bin/bash

set -euo pipefail

if [[ ${#} == 0 ]]; then
    cat <<EOF
    You must supply arguments! For example:

        ${0} test/end-to-end/my_test.sh

EOF
    exit 1
fi

# These are the commands that will be sequentially run in the
# container. The final one will be the test itself, which is supplied
# as the arguments of the script.
#
# Not every test requires `expect`, but it's not hurting anything to
# install it for everything.
#
# Note that the `ci-studio-common.sh` file is baked into the image we
# use.
commands=(". /opt/ci-studio-common/buildkite-agent-hooks/ci-studio-common.sh"
          ".expeditor/scripts/end_to_end/setup_environment.sh DEV"
          "hab pkg install --binlink --channel=stable core/expect"
          "${*}")

# Add a `;` after every command, for feeding into the container. This
# allows them to run in sequence.
docker run \
       --rm \
       --interactive \
       --tty \
       --privileged \
       --env-file="$(pwd)/e2e_env" \
       --volume="$(pwd):/workdir" \
       --workdir=/workdir \
       chefes/buildkite /bin/bash -e -c "${commands[*]/%/;}"
