#!/bin/bash

set -eou pipefail

# shellcheck source=.expeditor/scripts/shared.sh
source .expeditor/scripts/verify/shared.sh

if [[ ${1:-"--"} == "--" ]]; then
  scope="habitat workspace"
else
  component="$1"
  shift
  scope="$component"
fi

toolchain=$(get_toolchain)
install_hab_pkg core/glibc core/gcc-base core/xz core/zeromq core/protobuf core/rust/"$toolchain"

RUSTFLAGS="-C link-arg=-Wl,--dynamic-linker=$(hab pkg path core/glibc)/lib/ld-linux-x86-64.so.2"
export RUSTFLAGS
RUSTDOCFLAGS="${RUSTFLAGS}" # because we're running tests doctests also get run
export RUSTDOCFLAGS

LD_LIBRARY_PATH="$(cat "$(hab pkg path core/gcc-base)"/LD_RUN_PATH)"
LD_LIBRARY_PATH+=":$(cat "$(hab pkg path core/zeromq)"/LD_RUN_PATH)"
LD_LIBRARY_PATH+=":$(cat "$(hab pkg path core/xz)"/LD_RUN_PATH)"
export LD_LIBRARY_PATH

# TODO: fix this upstream, it looks like it's not saving correctly.
if ${BUILDKITE:-false}; then
  sudo chown -R buildkite-agent /home/buildkite-agent
fi

# TODO: these should be in a shared script?
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
export LIBZMQ_PREFIX

old_path=$PATH
eval "$(hab pkg env core/rust/"$toolchain")"
export PATH=$PATH:$old_path

export PROTOC_NO_VENDOR=1
PROTOC=$(hab pkg path core/protobuf)/bin/protoc
export PROTOC

# Set testing filesystem root
FS_ROOT=$(mktemp -d /tmp/testing-fs-root-XXXXXX)
export FS_ROOT

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
