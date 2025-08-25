#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway

curlbash_hab "x86_64-darwin"

brew install protobuf

macos_install_bootstrap_package
macos_use_cert_file_from_linux_cacerts_package

echo "--- :key: Generating fake origin key"
hab origin key generate

# the macos 11 anka image does not allow us to create a /hab
# directory so we mount off /tmp
export HAB_ROOT_PATH
HAB_ROOT_PATH=$(mktemp -d /tmp/fs-root-XXXXXX)

macos_sync_cache_signing_keys

# set the rust toolchain
install_rustup

if [ "$BUILD_PKG_TARGET" == "aarch64-darwin" ]; then
    rustup target add aarch64-apple-darwin
fi

rust_toolchain="$(tail -n 1 rust-toolchain  | cut -d'"' -f 2)"
echo "--- :rust: Using Rust toolchain ${rust_toolchain}"
rustc --version # just 'cause I'm paranoid and I want to double check

macos_build "$package_path"
