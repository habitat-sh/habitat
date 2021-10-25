#!/usr/local/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

package_path=${1?package_path argument required}

# Since we are only verifying we don't have build failures, make everything
# temp!
export HAB_ORIGIN
HAB_ORIGIN=throwaway

curlbash_hab "x86_64-darwin"

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

rust_toolchain="$(cat rust-toolchain)"
echo "--- :rust: Using Rust toolchain ${rust_toolchain}"
rustc --version # just 'cause I'm paranoid and I want to double check

macos_build "$package_path"

# Uncomment the below if you want the verify pipeline to render the built
# package and key. This is convenient for testing until we get bldr
# to store aarch-darwin packages.
# source results/last_build.env
# rm -f results/"$pkg_artifact"

# tar -cf temp.tar $HAB_ROOT_PATH/pkgs --transform="s,"${HAB_ROOT_PATH:1}",hab," --transform="s,tmp,hab,"
# xz --compress -6 --threads=0 temp.tar
# hab pkg sign --origin $HAB_ORIGIN temp.tar.xz results/"$pkg_artifact"

# buildkite-agent artifact upload ~/.hab/cache/keys/throwaway-*.pub
# buildkite-agent artifact upload "results/*.hart"
