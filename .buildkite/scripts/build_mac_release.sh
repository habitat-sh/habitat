#!/bin/bash

set -euo pipefail

source .buildkite/env
source .buildkite/scripts/shared.sh

echo "--- Installing Habitat Toolchain Omnibus package"
# This is a temporary situation until we have a pipeline building the
# toolchain packages and automatically installing them in our
# workers. Until then, we'll do it ourselves
bootstrap_package="habitat-bootstrap-1.0.0%2B20180813202835-1.pkg"
curl --remote-name \
     --location \
     "https://s3-us-west-2.amazonaws.com/shain-bk-test/${bootstrap_package}"
sudo installer \
     -verbose \
     -pkg "${bootstrap_package}" \
     -target /
rm "${bootstrap_package}"

# TODO (CM): consider getting Bash updated on builders (likely via Homebrew)
# TODO (CM): have a cleanup function
#     Clear out /hab
#     Uninstall Homebrew stuff
#     Uninstall Rustup, Rust?

echo "--- :beer: Updating Homebrew dependencies"
brew bundle install --verbose --file=.buildkite/Brewfile

echo "--- :habicat: Using $(hab --version)"

# Declaring this variable for the import_keys function only; see its
# documentation for further background.
#
# Alternatively, consider implementing set_hab_binary with platform-awareness
#
# declare -g isn't in the bash that is currently on our mac builders
hab_binary="$(which hab)"
import_keys

echo "--- :key: :arrow_right: :desktop_computer: Moving keys to system-wide location"
# TODO (CM): consider having `import_keys` install in the system directory instead
sudo mkdir -p /hab/cache/keys
sudo cp ~/.hab/cache/keys/* /hab/cache/keys

echo "--- :rust: Installing Rust"
curl https://sh.rustup.rs -sSf | sh -s -- -y
# This ensures the appropriate binaries are on our path
source "${HOME}/.cargo/env"

# NB: RUST_TOOLCHAIN is currently defined in the `.buildkite/env`, which
# we source above.
echo "--- :rust: Using Rust toolchain ${RUST_TOOLCHAIN}"
rustup default "${RUST_TOOLCHAIN}"
rustc --version # just 'cause I'm paranoid and I want to double check

echo "--- Cleanup caches"
# TODO (CM): enable control of cache clearing on a pipeline-wide basis
sudo rm -Rf /hab/cache/src
rm -Rf "${HOME}/.cargo/{git,registry}"

echo "--- :habicat: :hammer_and_wrench: Building 'hab'"

# NOTE: This does *not* need the CI_OVERRIDE_CHANNEL /
# HAB_BLDR_CHANNEL variables that builds for other supported platforms
# need, because we're not pulling anything from Builder. Once we do,
# we'll need to make sure we pull from the right channels.
sudo PATH="/opt/hab-bundle/embedded/bin:${PATH}" \
     "$(brew --prefix bash)/bin/bash" \
     components/plan-build/bin/hab-plan-build.sh \
     components/hab
source results/last_build.env

echo "--- :buildkite: Annotating build"

echo "<br>* ${pkg_ident:?} (${pkg_target:?})" | buildkite-agent annotate --append --context "release-manifest"

# Since we can't store macOS packages in Builder yet, we'll store it
# in Buildkite until we grab it later for upload to Bintray
echo "--- :buildkite: Storing ${pkg_target:?} 'hab' artifact ${pkg_artifact:?}"
set_hab_artifact "${pkg_target:?}" "${pkg_artifact:?}"
set_hab_release "${pkg_target:?}" "${pkg_release:?}"
(
    cd results
    buildkite-agent artifact upload "${pkg_artifact}"
)
