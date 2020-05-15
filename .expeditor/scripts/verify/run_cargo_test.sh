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
sudo hab pkg install core/bzip2
sudo hab pkg install core/openssl
sudo hab pkg install core/xz
sudo hab pkg install core/zeromq
sudo hab pkg install core/protobuf --binlink
sudo hab pkg install core/rust --binlink
export OPENSSL_DIR # so the openssl crate knows what to build against
OPENSSL_DIR="$(hab pkg path core/openssl)"
export OPENSSL_STATIC=true # so the openssl crate builds statically
export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
# now include openssl and zeromq so thney exists in the runtime library path when cargo test is run
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(hab pkg path core/zeromq)/lib"
# include these so that the cargo tests can bind to openssl at *runtime*
export LIBRARY_PATH
LIBRARY_PATH="$(hab pkg path core/bzip2)/lib:$(hab pkg path core/openssl)/lib:$(hab pkg path core/xz)/lib"
# setup pkgconfig so the openssl crate can use pkg-config at *build* time
export PKG_CONFIG_PATH
PKG_CONFIG_PATH="$(hab pkg path core/openssl)/lib/pkgconfig"

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
