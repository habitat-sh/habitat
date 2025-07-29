#!/bin/bash

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_AUTH_TOKEN="${PIPELINE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"

# Before we do *ANYTHING*, we're going to just delete any prior
# version of Habitat that exists in the container.
rm -Rf /hab/pkgs/core/hab

########################################################################

# `component` should be the subdirectory name in `components` where a
# particular component code resides.
#
# e.g. `hab` for `core/hab`, `plan-build` for `core/hab-plan-build`,
# etc.
component=${1:?You must specify a component value}

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

declare -g hab_binary
install_release_channel_hab_binary "$BUILD_PKG_TARGET"
import_keys

echo "--- :zap: Cleaning up old studio, if present"
${hab_binary} studio rm

echo "--- :habicat: Building components/${component} using ${hab_binary}"

# Set the refresh channel to the release channel.
export HAB_STUDIO_SECRET_HAB_REFRESH_CHANNEL="${channel}"

HAB_BLDR_CHANNEL="${channel}" ${hab_binary} pkg build "components/${component}"
source results/last_build.env

if [ "${pkg_target}" != "${BUILD_PKG_TARGET}" ]; then
    echo "--- :face_with_symbols_on_mouth: Expected to build for target ${BUILD_PKG_TARGET}, but built ${pkg_target} instead!"
    exit 1
fi

echo "--- :habicat: Uploading ${pkg_ident:?} (${pkg_target}) to ${HAB_BLDR_URL} in the '${channel}' channel"
${hab_binary} pkg upload \
              --channel="${channel}" \
              --auth="${HAB_AUTH_TOKEN}" \
              --no-build \
              "results/${pkg_artifact:?}"

echo "<br>* ${pkg_ident} (${pkg_target})" | buildkite-agent annotate --append --context "release-manifest"

set_target_metadata "${pkg_ident}" "${pkg_target}"
