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
    # This ensure the hab cli we use going forward has the correct
    # ActiveTarget. Otherwise, if we were to attempt to install an
    # `x86_64-linux-kernel2` package with the `hab` on our path, it
    # would result in an error and fail the build.
    if [[ "$BUILD_PKG_TARGET" == "x86_64-linux-kernel2" ]]; do
        # This installation step is a temporary shim until we have done at
        # least one release. Once we have a release, we can update ci-studio-common
        # to fetch this binary from bintray using the install.sh script and the install
        # step is no longer needed. Until then, we need to fetch it from our 
        # bootstrap pipeline. 
        install_hab_kernel2_binary
        hab_binary="$(which hab-x86_64-linux-kernel2)"
    else 
        hab_binary="$(which hab)"
    fi 

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
        hab_binary="/hab/pkgs/${hab_ident}/bin/hab"
        declare -g new_studio=1
    else
        echo "Buildkite metadata NOT found; using previously-installed hab binary: $hab_binary"
    fi
    declare -g hab_binary
    echo "--- :habicat: Using $(${hab_binary} --version)"
}

install_hab_kernel2_binary() {
    local hab_src_url tempdir
    hab_src_url="http://habitat-boostrap-artifacts.s3.amazonaws.com/x86_64-linux-kernel2/stage2/habitat-stage2-x86_64-linux-kernel2-latest"
    tempdir=$(mktemp -d hab-kernel2-XXXX)

    pushd $tmpdir >/dev/null
    wget "$hab_src_url" hab-x86_64-linux-kernel2
    sudo mv hab-x86_64-linux-kernel2 /bin/hab-x86_64-linux-kernel2
    sudo chmod +x /bin/hab-x86_64-linux-kernel2
    popd 
    rm -rf "$tmpdir"
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

echo "<br>* ${pkg_ident:?} (${pkg_target:?})" | buildkite-agent annotate --append --context "release-manifest"

echo "--- :habicat: Uploading ${pkg_ident} to Builder in the '${channel}' channel"
${hab_binary} pkg upload \
    --channel="${channel}" \
    --auth="${HAB_AUTH_TOKEN}" \
    "results/${pkg_artifact}"
