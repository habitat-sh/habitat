#!/bin/bash

set -eou pipefail

echo "--- Starting Darwin test script"

# This is required for a test in components/common that relies on FS_ROOT
# as on Mac we are not mounting "/hab" to run these tests.
export FS_ROOT
FS_ROOT=$(mktemp -d /tmp/hab-root-XXXXXX)

echo "--- Sourcing shared scripts"
source .expeditor/scripts/verify/shared.sh

if [[ ${1:-"--"} = "--" ]]; then
  scope="habitat workspace"
else
  component="$1"
  shift
  scope="$component"
fi

echo "--- Getting toolchain and component info"
echo "Testing component: $component"
echo "Testing scope: $scope"

toolchain=$(get_toolchain)
echo "Using toolchain: $toolchain"

echo "--- Setting up macOS environment"

# Install protobuf via homebrew if not available
if ! command -v protoc &> /dev/null; then
  echo "Installing protobuf via homebrew..."
  brew install protobuf
else
  echo "protobuf already available"
fi


# Set up certificate file for TLS tests using macOS approach
# Bootstrap package already provides GNU tail and tar in PATH
if [[ ! -f /etc/ssl/cert.pem ]]; then
    mode="--debug"
    echo "SSL Cert File Not Found, Running tests in *debug* mode."
else
    mode="--release"
    export SSL_CERT_FILE=/etc/ssl/cert.pem
fi

install_rustup
echo "--- Installing Rust toolchain"
install_rust_toolchain "$toolchain"
echo "--- Using Rust toolchain ${toolchain}"
rustc --version

echo "--- Setting up environment variables"
export PROTOC_NO_VENDOR=1
export PROTOC
PROTOC=$(which protoc)
echo "PROTOC set to: $PROTOC"

export RUST_BACKTRACE=1
echo "RUST_BACKTRACE set to: $RUST_BACKTRACE"

echo "--- Running cargo test with scope '$scope' and args '$*'"

if [[ -n ${component:-} ]]; then
  echo "Changing to component directory: components/$component"
  cd "components/$component"
fi

# Always add `--quiet` to avoid the noise of compilation in test output.
# The invocation to this script can add `--format pretty` to the test runner
# args (that is, after --, like --nocapture and --test-threads) if the names
# of the tests being run is desired in the output.
cargo test "$mode" --quiet "$@"
