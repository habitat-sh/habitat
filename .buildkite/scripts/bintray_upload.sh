#!/bin/bash

# We need to upload (but not publish) artifacts to Bintray right now.

set -euo pipefail

source .buildkite/scripts/shared.sh

set_hab_binary

# TODO: bintray user = chef-releng-ops!

if is_fake_release; then
    bintray_repository=unstable
else
    bintray_repository=stable
fi
echo "--- Preparing to push artifacts to the ${bintray_repository} Bintray repository"

channel=$(get_release_channel)

# TODO (CM): extract set_hab_binary function to a common library and
# use it here

echo "--- :habicat: Installing core/hab-bintray-publish from '${channel}' channel"
sudo "${hab_binary:?}" pkg install \
     --channel="${channel}" \
     core/hab-bintray-publish

# TODO (CM): determine artifact name for given hab identifier
#            could save this as metadata, or just save the artifact in
#            BK directly

echo "--- :habicat: Uploading core/hab to Bintray"

hab_artifact=$(get_hab_artifact "${BUILD_PKG_TARGET}")

# For this section we will manually pull down the windows hart from bldr
# rather than rewrite the bintray publish plugin for windows. The existing
# implementation doesn't support the upload, so we'll stage the windows
# file in the artifact cache on linux and utilize the existing code.
if [ "$BUILD_PKG_TARGET" = "x86_64-windows" ]; then
    version=$(get_version)
    windows_ident=$(latest_from_builder x86_64-windows "${channel}" hab "${version}")
    echo "--- Downloading Windows version directly from bldr: $windows_ident"
    sudo curl "https://bldr.habitat.sh/v1/depot/pkgs/$windows_ident/download?target=$BUILD_PKG_TARGET" -o "/hab/cache/artifacts/$hab_artifact"
else
    sudo "${hab_binary:?}" pkg install core/hab --channel="${channel}"
fi

# We upload to the stable channel, but we don't *publish* until
# later.
#
# -s = skip publishing
# -r = the repository to upload to
sudo HAB_BLDR_CHANNEL="${channel}" \
     BINTRAY_USER="${BINTRAY_USER}" \
     BINTRAY_KEY="${BINTRAY_KEY}" \
     BINTRAY_PASSPHRASE="${BINTRAY_PASSPHRASE}" \
     "${hab_binary:?}" pkg exec core/hab-bintray-publish \
         publish-hab \
         -s \
         -r "${bintray_repository}" \
         "/hab/cache/artifacts/${hab_artifact}"

source results/last_build.env
shasum=$(awk '{print $1}' "results/${pkg_artifact:?}.sha256sum")
cat << EOF | buildkite-agent annotate --style=success --context=bintray-hab
<h3>Habitat Bintray Binary (${pkg_target:?})</h3>
Artifact: <code>${pkg_artifact}</code>
<br/>
SHA256: <code>${shasum}</code>
EOF
