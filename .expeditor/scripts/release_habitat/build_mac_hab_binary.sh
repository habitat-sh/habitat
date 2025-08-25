#!/bin/bash

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

# Get secrets! (our auth token and aws creds should be auto-injected but there's a bug:
# https://github.com/chef/ci-studio-common/issues/200)
eval "$(vault-util fetch-secret-env)"

export HAB_AUTH_TOKEN="${PIPELINE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"


echo "--- Executing: brew install protobuf"
brew install protobuf

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

macos_install_bootstrap_package

hab_binary=""
curlbash_hab "x86_64-darwin"
import_keys

macos_use_cert_file_from_linux_cacerts_package

# the macos 11 anka image does not allow us to create a /hab
# directory so we mount off /tmp
export HAB_ROOT_PATH
HAB_ROOT_PATH=$(mktemp -d /tmp/fs-root-XXXXXX)

macos_sync_cache_signing_keys

install_rustup

if [ "$BUILD_PKG_TARGET" == "aarch64-darwin" ]; then
    rustup target add aarch64-apple-darwin
fi

# set the rust toolchain
rust_toolchain="$(tail -n 1 rust-toolchain  | cut -d'"' -f 2)"
echo "--- :rust: Using Rust toolchain ${rust_toolchain}"
rustc --version # just 'cause I'm paranoid and I want to double check

echo "--- :habicat: Building components/hab"

HAB_BLDR_CHANNEL="${channel}" macos_build components/hab
#shellcheck disable=SC1091
source results/last_build.env

# This is a hack to rebuild the hart with corrected directory structure
# changing the root from /tmp to /hab
rm -f results/"$pkg_artifact"
tar -cf temp.tar "$HAB_ROOT_PATH"/pkgs --transform="s,""${HAB_ROOT_PATH:1}"",hab," --transform="s,tmp,hab,"
xz --compress -6 --threads=0 temp.tar
hab pkg sign --origin "$HAB_ORIGIN" temp.tar.xz results/"$pkg_artifact"

echo "--- :habicat: Uploading ${pkg_ident:?} to ${HAB_BLDR_URL} in the '${channel}' channel"
${hab_binary} pkg upload \
              --channel="${channel}" \
              --auth="${HAB_AUTH_TOKEN}" \
              --no-build \
              "results/${pkg_artifact:?}"

echo "<br>* ${pkg_ident} (${BUILD_PKG_TARGET})" | buildkite-agent annotate --append --context "release-manifest"

set_target_metadata "${pkg_ident}" "${pkg_target}"
