#!/bin/bash

set -eou pipefail

source ./support/ci/shared.sh

component=${1?component argument required}
shift
cd "components/$component"

if [[ ${1:-} == --nightly ]]; then
  shift
  toolchain=$(get_nightly_toolchain)
else
  toolchain=stable
fi

# All the remaining args are passed to cargo test +"$toolchain"

install_rustup
install_rust_toolchain "$toolchain"

# TODO: fix this upstream, it looks like it's not saving correctly.
sudo chown -R buildkite-agent /home/buildkite-agent

# TODO: these should be in a shared script?
sudo hab pkg install core/bzip2
sudo hab pkg install core/libarchive
sudo hab pkg install core/libsodium
sudo hab pkg install core/openssl
sudo hab pkg install core/xz
sudo hab pkg install core/zeromq
sudo hab pkg install core/protobuf --binlink
sudo hab pkg install core/rust --binlink
export SODIUM_STATIC=true # so the libarchive crate links to sodium statically
export LIBARCHIVE_STATIC=true # so the libarchive crate *builds* statically
export OPENSSL_DIR # so the openssl crate knows what to build against
OPENSSL_DIR="$(hab pkg path core/openssl)"
export OPENSSL_STATIC=true # so the openssl crate builds statically
export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
# now include openssl and zeromq so thney exists in the runtime library path when cargo test is run
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(hab pkg path core/libsodium)/lib:$(hab pkg path core/zeromq)/lib"
# include these so that the cargo tests can bind to libarchive (which dynamically binds to xz, bzip, etc), openssl, and sodium at *runtime*
export LIBRARY_PATH
LIBRARY_PATH="$(hab pkg path core/bzip2)/lib:$(hab pkg path core/libsodium)/lib:$(hab pkg path core/openssl)/lib:$(hab pkg path core/xz)/lib"
# setup pkgconfig so the libarchive crate can use pkg-config to fine bzip2 and xz at *build* time
export PKG_CONFIG_PATH
PKG_CONFIG_PATH="$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"

# Set testing filesystem root
export TESTING_FS_ROOT
TESTING_FS_ROOT=$(mktemp -d /tmp/testing-fs-root-XXXXXX)

export RUST_BACKTRACE=1

echo "--- Running cargo test on $component with $*"
cargo +"$toolchain" test "$@"
