#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

########################################################################

# `component` should be the subdirectory name in `components` where a
# particular component code resides.
#
# e.g. `hab` for `core/hab`, `plan-build` for `core/hab-plan-build`,
# etc.
component=${1}

channel=$(buildkite-agent meta-data get "release-channel")

# `set_hab_binary` currently _must_ be called first!
set_hab_binary
import_keys

echo "--- :zap: Cleaning up old studio, if present"
${hab_binary} studio rm

echo "--- :habicat: Building components/${component}"

# The binlink dir is set by releng, but seems to be messing things up
# for us in the studio.
unset HAB_BINLINK_DIR
export HAB_ORIGIN=core

# Eww
#
# CI_OVERRIDE_CHANNEL is basically used to tell the studio which
# hab/backline to grab
if [[ "${new_studio:-}" ]]; then
    CI_OVERRIDE_CHANNEL="${channel}" HAB_BLDR_CHANNEL="${channel}" ${hab_binary} pkg build "components/${component}"
else
    HAB_BLDR_CHANNEL="${channel}" ${hab_binary} pkg build "components/${component}"
fi
source results/last_build.env

# TODO (CM): we'll need to scope these by architecture
case "${component}" in
    "hab")
        echo "--- :buildkite: Storing artifact ${pkg_ident:?}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        buildkite-agent meta-data set "hab-version" "${pkg_ident:?}"
        buildkite-agent meta-data set "hab-release-${pkg_target:?}" "${pkg_release:?}"
        buildkite-agent meta-data set "hab-artifact-${pkg_target:?}" "${pkg_artifact:?}"
        ;;
    "studio")
        echo "--- :buildkite: Recording metadata for ${pkg_ident}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        buildkite-agent meta-data set "studio-version" "${pkg_ident}"
        ;;
    *)
        ;;
esac

echo "<br>* ${pkg_ident:?} (${pkg_target:?})" | buildkite-agent annotate --append --context "release-manifest"

echo "--- :habicat: Uploading ${pkg_ident} to Builder in the '${channel}' channel"
${hab_binary} pkg upload \
    --channel="${channel}" \
    --auth="${HAB_AUTH_TOKEN}" \
    "results/${pkg_artifact}"
