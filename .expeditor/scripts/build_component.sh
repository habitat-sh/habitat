#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

########################################################################

# `component` should be the subdirectory name in `components` where a
# particular component code resides.
#
# e.g. `hab` for `core/hab`, `plan-build` for `core/hab-plan-build`,
# etc.
component=${1:?You must specify a component value}

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

install_latest_hab_binary "$BUILD_PKG_TARGET"
import_keys

echo "--- :zap: Cleaning up old studio, if present"
${hab_binary} studio rm

echo "--- :habicat: Building components/${component}"

# This is a temporary measure so we can run fake releases
export HAB_STUDIO_SECRET_DO_FAKE_RELEASE=$DO_FAKE_RELEASE

HAB_BLDR_CHANNEL="${channel}" ${hab_binary} pkg build "components/${component}"
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

echo "--- :writing_hand: Recording Build Metadata"
case "${component}" in
    "hab")
        echo "--- :buildkite: Storing artifact ${pkg_ident:?}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        set_hab_ident "${BUILD_PKG_TARGET:?}" "${pkg_ident:?}"
        set_hab_release "${BUILD_PKG_TARGET:?}" "${pkg_release:?}"
        set_hab_artifact "${BUILD_PKG_TARGET:?}" "${pkg_artifact:?}"
        ;;
    "studio")
        echo "--- :buildkite: Recording metadata for ${pkg_ident:?}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        set_studio_ident "${BUILD_PKG_TARGET:?}" "${pkg_ident:?}"
        ;;
    "backline")
        echo "--- :buildkite: Recording metadata for ${pkg_ident:?}"
        set_backline_ident "${BUILD_PKG_TARGET}" "${pkg_ident:?}"
        set_backline_artifact "${BUILD_PKG_TARGET}" "${pkg_artifact:?}"
        ;;
    *)
        ;;
esac
echo "<br>* ${pkg_ident:?} (${BUILD_PKG_TARGET:?})" | buildkite-agent annotate --append --context "release-manifest"
