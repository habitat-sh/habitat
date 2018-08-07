#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

# Until we have built both a new core/hab _and_ a new core/hab-studio
# package, we should continue to use the `hab` binary provided on our
# Buildkite builders (managed by Release Engineering) (these should be
# the latest stable release, btw).
#
# Once we have bootstrapped ourselves enough, however, we should
# switch subsequent builds to use the new hab, which in turn uses the
# new studio.
set_hab_binary() {
    echo "--- :thinking_face: Determining which 'hab' binary to use"

    if buildkite-agent meta-data exists hab-version &&
            buildkite-agent meta-data exists studio-version; then
        echo "Buildkite metadata found; installing new versions of 'core/hab' and 'core/hab-studio'"
        # By definition, these will be fully-qualified identifiers,
        # and thus do not require a `--channel` option. However, they
        # should be coming from the release channel, and should be the
        # same packages built previously in this same release pipeline.
        hab_ident=$(buildkite-agent meta-data get hab-version)

        # Note that we are explicitly not binlinking here; this is to
        # prevent accidentally polluting the builder for any future
        # runs that may take place on it.
        sudo hab pkg install "${hab_ident}"
        sudo hab pkg install "$(buildkite-agent meta-data get studio-version)"
        declare -g hab_binary="/hab/pkgs/${hab_ident}/bin/hab"
        declare -g new_studio="true"
    else
        echo "Buildkite metadata NOT found; using previously-installed hab binary"
        hab_binary="$(which hab)"
        declare -g hab_binary
        declare -g new_studio="false"
    fi
    echo "--- :habicat: Using $(${hab_binary} --version)"
}

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
if [[ "${new_studio}" == "true" ]]; then
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
        buildkite-agent meta-data set "hab-release-linux" "${pkg_release:?}"
        buildkite-agent meta-data set "hab-artifact" "${pkg_artifact:?}"
        ;;
    "studio")
        echo "--- :buildkite: Recording metadata for ${pkg_ident}"
        # buildkite-agent artifact upload "results/${pkg_artifact}"
        buildkite-agent meta-data set "studio-version" "${pkg_ident}"
        ;;
    *)
        ;;
esac

# TODO (CM): Replace "Linux" below with ${pkg_target:?} once that's in
# hab-plan-build (see https://github.com/habitat-sh/habitat/pull/5373)
echo "<br>* ${pkg_ident} (Linux)" | buildkite-agent annotate --append --context "release-manifest"

echo "--- :habicat: Uploading ${pkg_ident} to Builder in the '${channel}' channel"
${hab_binary} pkg upload \
    --channel="${channel}" \
    --auth="${HAB_TEAM_AUTH_TOKEN}" \
    "results/${pkg_artifact}"
