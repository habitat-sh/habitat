#!/bin/bash

# We need to upload (but not publish) artifacts to Bintray right now.

set -euo pipefail

source .buildkite/scripts/shared.sh

# TODO: bintray user = chef-releng-ops!

if is_fake_release; then
    bintray_repository=unstable
else
    bintray_repository=stable
fi
echo "--- Preparing to push artifacts to the ${bintray_repository} Bintray repository"

channel=$(buildkite-agent meta-data get "release-channel")

# TODO (CM): extract set_hab_binary function to a common library and
# use it here

echo "--- :habicat: Installing core/hab-bintray-publish from '${channel}' channel"
sudo hab pkg install \
     --channel="${channel}" \
     core/hab-bintray-publish

# TODO (CM): determine artifact name for given hab identifier
#            could save this as metadata, or just save the artifact in
#            BK directly

echo "--- :habicat: Uploading core/hab to Bintray"

# TODO (CM): Continue with this approach, or just grab the artifact
# that we built out of BK?
#
# If we use `hab pkg install` we know we'll get the artifact for our
# platform.
#
# If we use Buildkite, we can potentially upload many different
# platform artifacts to Bintray from a single platform (e.g., upload
# Windows artifacts from Linux machines.)
sudo hab pkg install core/hab --channel="${channel}"

hab_artifact=$(buildkite-agent meta-data get "hab-artifact")

# We upload to the stable channel, but we don't *publish* until
# later.
#
# -s = skip publishing
# -r = the repository to upload to
sudo HAB_BLDR_CHANNEL="${channel}" \
     BINTRAY_USER="${BINTRAY_USER}" \
     BINTRAY_KEY="${BINTRAY_KEY}" \
     BINTRAY_PASSPHRASE="${BINTRAY_PASSPHRASE}" \
     hab pkg exec core/hab-bintray-publish \
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

echo "--- :habicat: Uploading core/hab-studio to Bintray"

# Set the IMAGE_NAME environment variable which influences the name of
# the image that gets generated. Anything we make in the course of
# doing a "fake release" goes into the dev repository so we don't
# pollute the stable repository.
if is_fake_release; then
    image_name="habitat-docker-registry.bintray.io/studio-dev"
else
    image_name="habitat-docker-registry.bintray.io/studio"
fi

# again, override just for backline
sudo HAB_BLDR_CHANNEL="${channel}" \
     CI_OVERRIDE_CHANNEL="${channel}" \
     BINTRAY_USER="${BINTRAY_USER}" \
     BINTRAY_KEY="${BINTRAY_KEY}" \
     BINTRAY_PASSPHRASE="${BINTRAY_PASSPHRASE}" \
     IMAGE_NAME="${image_name}" \
     hab pkg exec core/hab-bintray-publish \
         publish-studio

# The logic for the creation of this image is spread out over soooo
# many places :/
source results/last_image.env
cat << EOF | buildkite-agent annotate --style=success --context=docker-studio
<h3>Docker Studio Image (Linux)</h3>
<ul>
  <li><code>${docker_image:?}:${docker_image_version:?}</code></li>
  <li><code>${docker_image:?}:${docker_image_short_version:?}</code></li>
  <li><code>${docker_image:?}:latest</code></li>
</ul>
EOF
