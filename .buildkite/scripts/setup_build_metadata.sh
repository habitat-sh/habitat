#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

if [[ "${FAKE_RELEASE_TAG:-}" || "${BUILDKITE_TAG}" ]]; then
    # Our releases are currently triggered by the existence of a tag
    echo "--- :sparkles: Preparing for a release! :sparkles:"

    if [[ "${FAKE_RELEASE_TAG:-}" ]]; then
        echo "Using fake release tag '${FAKE_RELEASE_TAG}'"
        set_fake_release "${FAKE_RELEASE_TAG}"
        release="${FAKE_RELEASE_TAG}"
    elif [[ "${BUILDKITE_TAG}" ]]; then
        echo "Using release tag '${BUILDKITE_TAG}'"
        release="${BUILDKITE_TAG}"
    fi
    channel="rc-${release}"
    echo "Release channel is '${channel}'"
    echo "## Habitat Release _${release}_" | buildkite-agent annotate --context "release-manifest"

    set_release_channel "${channel}"

    # We'll record the explicit version being built for future access,
    # even though 'cat'ing a file is pretty easy; we'll be sticking
    # other things into the Buildkite metadata, so it'll all be uniform.
    version=$(cat VERSION)
    set_version "${version}"
else
    echo "--- :sparkles: Preparing for a CI run! :sparkles:"
    echo "We're just going to fail!"
    exit 1
fi
