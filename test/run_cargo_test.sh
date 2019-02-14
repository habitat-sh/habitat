#!/bin/bash

set -eou pipefail

while [[ $# -gt 1 ]]; do
  case $1 in
    -f | --features )       shift
                            features=$1
                            ;;
    -t | --test-options )   shift
                            test_options=$1
                            ;;
    * )                     echo "FAIL SCHOONER"
                            exit 1
  esac
  shift
done

# set the features string if needed
[ -z "${features:-}" ] && features_string="" || features_string="--features ${features}"

component=${1?component argument required}
cargo_test_command="cargo test ${features_string} -- --nocapture ${test_options:-}"

# TODO: fix this upstream so it's already on the path and set up
export RUSTUP_HOME=/opt/rust
export CARGO_HOME=/home/buildkite-agent/.cargo
export PATH=/opt/rust/bin:$PATH
# TODO: fix this upstream, it looks like it's not saving correctly.
sudo chown -R buildkite-agent /home/buildkite-agent

sudo hab pkg install core/bzip2
sudo hab pkg install core/libarchive
sudo hab pkg install core/libsodium
sudo hab pkg install core/openssl
sudo hab pkg install core/xz
sudo hab pkg install core/zeromq
sudo hab pkg install core/protobuf --binlink
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
export TESTING_FS_ROOT=$(mktemp -d /tmp/testing-fs-root-XXXXXX)
echo "--- Running cargo test on $component with command: '$cargo_test_command'"
cd "components/$component"
$cargo_test_command
