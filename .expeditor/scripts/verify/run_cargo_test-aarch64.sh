#!/bin/bash

set -eou pipefail

# Install hab from a temporarily uploaded aarch64 package
cat ../../../components/hab/install.sh | sudo bash -s -- -t aarch64-linux -c dev

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

sudo -E hab pkg install core/zeromq
sudo -E hab pkg install core/protobuf
sudo -E hab pkg install core/rust/"$toolchain"
sudo -E hab pkg install core/xz
sudo -E hab pkg install core/coreutils
sudo -E hab pkg install core/openssl
sudo -E hab pkg install core/perl
sudo -E hab pkg install core/make

export OPENSSL_DIR
OPENSSL_DIR="$(hab pkg path core/openssl)/bin"
export OPENSSL_LIB_DIR
OPENSSL_LIB_DIR="$(hab pkg path core/openssl)/lib"

export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
# now include zeromq and gcc so they exist in the runtime library path when cargo test is run
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(hab pkg path core/gcc-base)/lib:$(hab pkg path core/zeromq)/lib:$(hab pkg path core/xz)/lib:$(hab pkg path core/openssl)/lib"

export LIBRARY_PATH
LIBRARY_PATH="$(hab pkg path core/xz)/lib"

export PROTOC_NO_VENDOR=1
export PROTOC
PROTOC=$(hab pkg path core/protobuf)/bin/protoc

_oldPth=$PATH
_pth="$(hab pkg path core/coreutils)/bin:$(hab pkg path core/openssl)/bin:$(hab pkg path core/perl)/bin:$(hab pkg path core/make)/bin"
eval "$(hab pkg env core/rust/"$toolchain"):$PATH"
export PATH="$PATH:$_pth:$_oldPth"

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

# We do not have any packages in the stable channel for aarch64 and probably never will. 
# Set the HAB_INTERPRETER_IDENT to point to LTS-2024 to proceed with the tests.
export HAB_INTERPRETER_IDENT="core/busybox-static/1.36.1/20240805133911"

# Always add `--quiet` to avoid the noise of compilation in test output.
# The invocation to this script can add `--format pretty` to the test runner
# args (that is, after --, like --nocapture and --test-threads) if the names
# of the tests being run is desired in the output.
cargo test --quiet "$@"
