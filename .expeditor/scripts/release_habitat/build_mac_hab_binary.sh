#!/usr/local/bin/bash

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

# Get secrets! (our auth token and aws creds should be auto-injected but there's a bug:
# https://github.com/chef/ci-studio-common/issues/200)
eval "$(vault-util fetch-secret-env)"

export HAB_AUTH_TOKEN="${PIPELINE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

macos_install_boostrap_package

declare -g hab_binary
curlbash_hab "$BUILD_PKG_TARGET"
import_keys

macos_use_cert_file_from_linux_cacerts_package
macos_sync_cache_signing_keys

install_rustup

# set the rust toolchain
rust_toolchain="$(cat rust-toolchain)"
echo "--- :rust: Using Rust toolchain ${rust_toolchain}"
rustc --version # just 'cause I'm paranoid and I want to double check

echo "--- :habicat: Building components/hab"

HAB_BLDR_CHANNEL="${channel}" macos_build components/hab
source results/last_build.env

echo "--- :habicat: Uploading ${pkg_ident:?} to ${HAB_BLDR_URL} in the '${channel}' channel"
${hab_binary} pkg upload \
              --channel="${channel}" \
              --auth="${HAB_AUTH_TOKEN}" \
              --no-build \
              "results/${pkg_artifact:?}"

echo "<br>* ${pkg_ident} (${BUILD_PKG_TARGET})" | buildkite-agent annotate --append --context "release-manifest"

set_target_metadata "${pkg_ident}" "${pkg_target}"
