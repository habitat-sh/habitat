#!/bin/bash

set -eou pipefail

# Set up writable HAB_ROOT_PATH on macOS BEFORE sourcing shared.sh to avoid read-only filesystem issues
if [[ "$OSTYPE" == "darwin"* ]]; then
  export HAB_ROOT_PATH
  HAB_ROOT_PATH=$(mktemp -d /tmp/hab-root-XXXXXX)
  # Clean up Darwin-specific temp directory on exit
  trap 'rm -rf "$HAB_ROOT_PATH"' EXIT
  
  # Set HAB_LICENSE to skip prompts entirely
  export HAB_LICENSE=accept-no-persist
fi

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
# Only do buildkite-agent chown on Linux systems
if ${BUILDKITE:-false} && [[ "$OSTYPE" != "darwin"* ]]; then
  sudo chown -R buildkite-agent /home/buildkite-agent
fi

# Platform-specific dependency setup
if [[ "$OSTYPE" == "darwin"* ]]; then
  echo "--- Setting up macOS environment"
  
  # Install protobuf via homebrew if not available
  if ! command -v protoc &> /dev/null; then
    echo "Installing protobuf via homebrew..."
    brew install protobuf
  fi
  
  # Install GNU tools needed for certificate extraction
  if ! command -v gtail &> /dev/null; then
    echo "Installing GNU coreutils for certificate extraction..."
    brew install coreutils gnu-tar
  fi
  
  # Install hab and bootstrap package for certificate extraction
  if ! command -v hab &> /dev/null; then
    echo "Installing hab via curlbash..."
    curlbash_hab "x86_64-darwin"
  fi
  
  echo "Installing bootstrap package for GNU tools..."
  macos_install_bootstrap_package
  
  # Accept habitat license after hab is installed
  accept_hab_license
  
  # Set up certificate file for TLS tests using macOS approach
  # Bootstrap package already provides GNU tail and tar in PATH
  macos_use_cert_file_from_linux_cacerts_package
  
  install_rustup
  install_rust_toolchain "$toolchain"
  echo "--- :rust: Using Rust toolchain ${toolchain}"
  rustc --version
  
  export PROTOC_NO_VENDOR=1
  export PROTOC
  PROTOC=$(which protoc)
else
  echo "--- Setting up Linux environment"
  
  # Accept habitat license
  accept_hab_license
  
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
fi

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
