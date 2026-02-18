#!/bin/bash

set -eou pipefail

echo "--- Starting Darwin test script"

# Set up writable HAB_ROOT_PATH on macOS BEFORE sourcing shared.sh to avoid read-only filesystem issues
export HAB_ROOT_PATH
HAB_ROOT_PATH=$(mktemp -d /tmp/hab-root-XXXXXX)
echo "HAB_ROOT_PATH set to: $HAB_ROOT_PATH"
# Clean up Darwin-specific temp directory on exit
trap 'rm -rf "$HAB_ROOT_PATH"' EXIT

# Set HAB_LICENSE to skip prompts entirely
export HAB_LICENSE=accept-no-persist
echo "HAB_LICENSE set to: $HAB_LICENSE"

echo "--- Sourcing shared scripts"
# shellcheck source=.expeditor/scripts/shared.sh
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

# Install GNU tools needed for certificate extraction
if ! command -v gtail &> /dev/null; then
  echo "Installing GNU coreutils for certificate extraction..."
  brew install coreutils gnu-tar
fi

# Install hab for package operations
if ! command -v hab &> /dev/null; then
  echo "Installing hab via curlbash..."
  curlbash_hab "x86_64-darwin"
else
  echo "hab already available"
fi

# Install bootstrap package which provides GNU tools needed for certificate extraction
echo "Installing bootstrap package for GNU tools..."
macos_install_bootstrap_package

# Set up certificate file for TLS tests using macOS approach
# Bootstrap package already provides GNU tail and tar in PATH
echo "--- Setting up certificate file for TLS tests"
macos_use_cert_file_from_linux_cacerts_package

# Ensure SSL_CERT_FILE is properly exported for cargo test
echo "--- Verifying SSL_CERT_FILE export"
if [[ -z "${SSL_CERT_FILE:-}" ]]; then
  echo "ERROR: SSL_CERT_FILE not set after certificate extraction!"
  exit 1
fi
echo "SSL_CERT_FILE is set to: ${SSL_CERT_FILE}"
if [[ ! -f "${SSL_CERT_FILE}" ]]; then
  echo "ERROR: SSL_CERT_FILE points to non-existent file: ${SSL_CERT_FILE}"
  exit 1
fi
echo "Certificate file exists and is readable"
export SSL_CERT_FILE

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

# Set testing filesystem root
export FS_ROOT
FS_ROOT=$(mktemp -d /tmp/testing-fs-root-XXXXXX)
echo "FS_ROOT set to: $FS_ROOT"

export RUST_BACKTRACE=1
echo "RUST_BACKTRACE set to: $RUST_BACKTRACE"

# Build the all the hab binaries so that we can run integration tests
if [[ "$scope" == "sup" ]]; then
  echo "--- Building cargo for sup integration tests"
  cargo build
fi

echo "--- Running cargo test with scope '$scope' and args '$*'"

if [[ -n ${component:-} ]]; then
  echo "Changing to component directory: components/$component"
  cd "components/$component"
fi

# Always add `--quiet` to avoid the noise of compilation in test output.
# The invocation to this script can add `--format pretty` to the test runner
# args (that is, after --, like --nocapture and --test-threads) if the names
# of the tests being run is desired in the output.
cargo test --quiet "$@"