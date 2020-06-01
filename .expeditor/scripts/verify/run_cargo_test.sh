#!/bin/bash

set -eou pipefail

# shellcheck source=.expeditor/scripts/shared.sh
source .expeditor/scripts/verify/shared.sh

if [[ ${1:-"--"} = "--" ]]; then
  scope="habitat workspace"
else
  component="$1"
  shift
  scope="$component"
fi

if [[ ${1:-} == --nightly ]]; then
  shift
  toolchain=$(get_nightly_toolchain)
else
  toolchain=$(get_toolchain)
fi

# All the remaining args are passed to cargo test +"$toolchain"

install_rustup
install_rust_toolchain "$toolchain"

# TODO: fix this upstream, it looks like it's not saving correctly.
if ${BUILDKITE:-false}; then
  sudo chown -R buildkite-agent /home/buildkite-agent
fi

# TODO: these should be in a shared script?
sudo hab pkg install core/zeromq
sudo hab pkg install core/protobuf --binlink
sudo hab pkg install core/rust --binlink
export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
# now include zeromq so it exists in the runtime library path when cargo test is run
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(hab pkg path core/zeromq)/lib"

# Set testing filesystem root
export FS_ROOT
FS_ROOT=$(mktemp -d /tmp/testing-fs-root-XXXXXX)

export RUST_BACKTRACE=1

echo "--- Running cargo +$toolchain test with scope '$scope' and args '$*'"

if [[ -n ${component:-} ]]; then
  cd "components/$component"
fi

# Always add `--quiet` to avoid the noise of compilation in test output.
# The invocation to this script can add `--format pretty` to the test runner
# args (that is, after --, like --nocapture and --test-threads) if the names
# of the tests being run is desired in the output.
cargo +"$toolchain" test --quiet "$@"
