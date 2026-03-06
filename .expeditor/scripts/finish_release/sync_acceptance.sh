#!/bin/bash

set -eou pipefail

tempdir="$(mktemp --directory --tmpdir="$(pwd)" -t "downloads-XXXX")"

trap 'rm -rf $tempdir' INT TERM EXIT

hab pkg download --file .expeditor/scripts/finish_release/sync_acceptance.toml --download-directory "${tempdir}"
hab pkg bulkupload \
        --url "https://bldr.acceptance.habitat.sh" \
        --auth "${HAB_AUTH_TOKEN}" \
        --channel 'base-2025' \
        "${tempdir}"
