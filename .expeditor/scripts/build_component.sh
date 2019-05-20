#!/bin/bash

set -euo pipefail

# source .buildkite/scripts/shared.sh

source .expeditor/scripts/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

########################################################################

# `component` should be the subdirectory name in `components` where a
# particular component code resides.
#
# e.g. `hab` for `core/hab`, `plan-build` for `core/hab-plan-build`,
# etc.
component=${1}

channel=$(get_release_channel)

install_latest_stable_hab_binary
import_keys

echo "--- :zap: Cleaning up old studio, if present"
${hab_binary} studio rm

echo "--- :habicat: Building components/${component}"

# CI_OVERRIDE_CHANNEL is basically used to tell the studio which
# hab/backline to grab
if [[ "${new_studio:-}" ]]; then
    CI_OVERRIDE_CHANNEL="${channel}" HAB_BLDR_CHANNEL="${channel}" ${hab_binary} pkg build "components/${component}"
else
    HAB_BLDR_CHANNEL="${channel}" ${hab_binary} pkg build "components/${component}"
fi
source results/last_build.env

echo "--- :habicat: Uploading ${pkg_ident} to ${HAB_BLDR_URL:-Builder} in the '${channel}' channel"
${hab_binary} pkg upload \
              --channel="${channel}" \
              --auth="${HAB_AUTH_TOKEN}" \
              "results/${pkg_artifact}"

${hab_binary} pkg promote \
              --auth="${HAB_AUTH_TOKEN}" \
              "${pkg_ident}" "${channel}" "${BUILD_PKG_TARGET}"

set_target_metadata "${pkg_ident}" "${BUILD_PKG_TARGET}"

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
        echo "--- :buildkite: Recording metadata for ${pkg_ident}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        set_studio_ident "${BUILD_PKG_TARGET:?}" "${pkg_ident:?}"
        ;;
    "backline")
        echo "--- :buildkite: Recording metadata for ${pkg_ident}"
        set_backline_ident "${BUILD_PKG_TARGET}" "${pkg_ident}"
        set_backline_artifact "${BUILD_PKG_TARGET}" "${pkg_artifact}"
        ;;
    *)
        ;;
esac
echo "<br>* ${pkg_ident:?} (${BUILD_PKG_TARGET:?})" | buildkite-agent annotate --append --context "release-manifest"
