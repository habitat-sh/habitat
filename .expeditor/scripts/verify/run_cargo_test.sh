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

toolchain=$(get_toolchain)

# TODO: fix this upstream, it looks like it's not saving correctly.
if ${BUILDKITE:-false}; then
  sudo chown -R buildkite-agent /home/buildkite-agent
fi

# TODO: these should be in a shared script?
sudo -E hab pkg install core/zeromq
sudo -E hab pkg install core/protobuf
sudo -E hab pkg install core/rust/"$toolchain"
export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
# now include zeromq and gcc so they exist in the runtime library path when cargo test is run
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(hab pkg path core/gcc-base)/lib64:$(hab pkg path core/zeromq)/lib"
old_path=$PATH
eval "$(hab pkg env core/rust/"$toolchain")"
export PATH=$PATH:$old_path

export PROTOC_NO_VENDOR=1
export PROTOC
PROTOC=$(hab pkg path core/protobuf)/bin/protoc

# Set testing filesystem root
export FS_ROOT
FS_ROOT=$(mktemp -d /tmp/testing-fs-root-XXXXXX)

export RUST_BACKTRACE=1

# Build the all the hab binaries so that we can run integration tests
if [[ "$scope" == "sup" ]]; then
  cargo build
fi

echo "--- Running cargo test with scope '$scope' and args '$*'"

if [[ -n ${component:-} ]]; then
  cd "components/$component"
fi

# Always add `--quiet` to avoid the noise of compilation in test output.
# The invocation to this script can add `--format pretty` to the test runner
# args (that is, after --, like --nocapture and --test-threads) if the names
# of the tests being run is desired in the output.
cargo test --quiet "$@"
