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

channel=$(get_release_channel)

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

# TODO (SM): The 0.59.0 hab cli that we rely on for x86_64-linux builds 
# doesn't emit pkg_target. Until we've sufficiently bootstrapped ourselves
# we need to set it. This can be removed when studio-ci-common pulls 0.63.0 
# or newer. This is safe to do because the x86_64-linux-kernel2 builds will
# already have this value set.
: "${pkg_target:=x86_64-linux}"


# TODO: after 0.79.0 we can reenable this. We are explicitly using curl to upload
# due to this bug: https://github.com/habitat-sh/builder/issues/940
# echo "--- :habicat: Uploading ${pkg_ident} to Builder in the '${channel}' channel"
# ${hab_binary} pkg upload \
#     --channel="${channel}" \
#     --auth="${HAB_AUTH_TOKEN}" \
#     "results/${pkg_artifact}"
#
# ${hab_binary} pkg promote \
#     --auth="${HAB_AUTH_TOKEN}" \
#     "${pkg_ident}" "${channel}" "${pkg_target}"

echo "--- :partyparrot: Manually uploading '${pkg_ident:?}' (${pkg_target}) to Builder"
curl --request POST \
     --header "Content-Type: application/octet-stream" \
     --header "Authorization: Bearer $HAB_AUTH_TOKEN" \
     --data-binary "@results/${pkg_artifact:?}" \
     --fail \
     --verbose \
    "https://bldr.habitat.sh/v1/depot/pkgs/${pkg_ident}?checksum=${pkg_blake2bsum:?}&target=${pkg_target}"

promote "${pkg_ident}" "${pkg_target}" "${channel}"
set_target_metadata "${pkg_ident}" "${pkg_target}"

echo "--- :writing_hand: Recording Build Metadata"
case "${component}" in
    "hab")
        echo "--- :buildkite: Storing artifact ${pkg_ident:?}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        set_hab_ident "${pkg_target:?}" "${pkg_ident:?}"
        set_hab_release "${pkg_target:?}" "${pkg_release:?}"
        set_hab_artifact "${pkg_target:?}" "${pkg_artifact:?}"
        ;;
    "studio")
        echo "--- :buildkite: Recording metadata for ${pkg_ident}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        set_studio_ident "${pkg_target:?}" "${pkg_ident:?}"
        ;;
    "backline")
        echo "--- :buildkite: Recording metadata for ${pkg_ident}"
        set_backline_ident "${pkg_target}" "${pkg_ident}"
        set_backline_artifact "${pkg_target}" "${pkg_artifact}"
        ;;
    *)
        ;;
esac
echo "<br>* ${pkg_ident:?} (${pkg_target:?})" | buildkite-agent annotate --append --context "release-manifest"
