#!/usr/local/bin/bash

set -euo pipefail

source .expeditor/scripts/shared_release_habitat.sh

# Get secrets! (our auth token and aws creds should be auto-injected but there's a bug:
# https://github.com/chef/ci-studio-common/issues/200)
eval "$(vault-util fetch-secret-env)"

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

echo "--- Installing buildkite agent"
brew tap buildkite/buildkite
brew install --token="$BUILDKITE_AGENT_ACCESS_TOKEN" buildkite-agent

echo "--- Installing mac bootstrap package"
# TODO: subscribe to releases: https://github.com/habitat-sh/release-engineering/issues/84
bootstrap_package_version="$(cat MAC_BOOTSTRAPPER_VERSION)"
bootstrap_package_name="mac-bootstrapper-${bootstrap_package_version}-1"
curl "https://packages.chef.io/files/current/mac-bootstrapper/${bootstrap_package_version}/mac_os_x/10.12/${bootstrap_package_name}.dmg" -o "${bootstrap_package_name}.dmg"
sudo hdiutil attach "${bootstrap_package_name}.dmg"
sudo installer -verbose -pkg "/Volumes/Habitat macOS Bootstrapper/${bootstrap_package_name}.pkg" -target /
sudo hdiutil detach "/Volumes/Habitat macOS Bootstrapper"
brew install wget
export PATH=/opt/mac-bootstrapper/embedded/bin:/usr/local/bin:$PATH

declare -g hab_binary
curlbash_hab "$BUILD_PKG_TARGET"
import_keys

# Set SSL cert location
export SSL_CERT_FILE=/usr/local/etc/openssl/cert.pem

# We invoke hab-plan-build.sh directly via sudo, so we don't get the key management that studio provides. 
# Copy keys from the user account Habitat cache to the system Habitat cache so that they are present for root.
sudo mkdir -p /hab/cache/keys
sudo cp -r ~/.hab/cache/keys/* /hab/cache/keys/

echo "--- :rust: Installing Rust"
curl https://sh.rustup.rs -sSf | sh -s -- -y
# This ensures the appropriate binaries are on our path
source "${HOME}/.cargo/env"

# set the rust toolchain
rust_toolchain="$(cat rust-toolchain)"
echo "--- :rust: Using Rust toolchain ${rust_toolchain}"
rustup default "${rust_toolchain}"
rustc --version # just 'cause I'm paranoid and I want to double check

echo "--- :habicat: Building components/hab"

HAB_BLDR_CHANNEL="${channel}" sudo -E bash \
        components/plan-build/bin/hab-plan-build.sh \
        components/hab
source results/last_build.env

echo "--- :habicat: Uploading ${pkg_ident:?} to ${HAB_BLDR_URL} in the '${channel}' channel"
${hab_binary} pkg upload \
              --channel="${channel}" \
              --auth="${HAB_AUTH_TOKEN}" \
              "results/${pkg_artifact:?}"

${hab_binary} pkg promote \
              --auth="${HAB_AUTH_TOKEN}" \
              "${pkg_ident:?}" "${channel}" "${BUILD_PKG_TARGET}"

set_target_metadata "${pkg_ident:?}" "${BUILD_PKG_TARGET}"

echo "--- :buildkite: Storing artifact ${pkg_ident:?}"
buildkite-agent artifact upload "results/${pkg_artifact}"
set_hab_ident "${BUILD_PKG_TARGET:?}" "${pkg_ident:?}"
set_hab_release "${BUILD_PKG_TARGET:?}" "${pkg_release:?}"
set_hab_artifact "${BUILD_PKG_TARGET:?}" "${pkg_artifact:?}"

echo "<br>* ${pkg_ident:?} (${BUILD_PKG_TARGET:?})" | buildkite-agent annotate --append --context "release-manifest"
