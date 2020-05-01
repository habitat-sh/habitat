#!/usr/local/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway

# let's make a selfcontained tempdir for this job
export JOB_TEMP_ROOT
JOB_TEMP_ROOT=$(mktemp -d /tmp/job-root-XXXXXX)
export HAB_CACHE_KEY_PATH
HAB_CACHE_KEY_PATH="$JOB_TEMP_ROOT/keys"

curlbash_hab "$BUILD_PKG_TARGET"

macos_install_bootstrap_package
macos_use_cert_file_from_linux_cacerts_package

echo "--- :key: Generating fake origin key"
hab origin key generate

macos_sync_cache_signing_keys

# set the rust toolchain
install_rustup
rust_toolchain="$(cat rust-toolchain)"
echo "--- :rust: Using Rust toolchain ${rust_toolchain}"
rustc --version # just 'cause I'm paranoid and I want to double check

macos_build "$package_path"
