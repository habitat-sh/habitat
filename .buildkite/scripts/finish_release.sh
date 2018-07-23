#!/bin/bash

set -euo pipefail

RELEASE_ACTION=$(buildkite-agent meta-data get release-action)

case "${RELEASE_ACTION}" in
    "release")
        # finish the release
        buildkite-agent pipeline upload .buildkite/finish_release_pipeline.yaml
    ;;
    "abort")
        # ABORT ABORT
        buildkite-agent pipeline upload .buildkite/abort_release_pipeline.yaml
    ;;
    *)
        # ABORT ABORT
        exit 1
    ;;
esac
