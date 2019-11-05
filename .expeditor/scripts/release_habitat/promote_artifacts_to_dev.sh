#!/bin/bash

# Promote all artifacts into the dev channel. This includes:
#
# * Generating a manifest.json enumerating all the Habitat packages
#   stored in Builder.
# * Promoting all the Habitat packages from the current
#   pipeline-scoped channel to "dev" in Builder.
# * Promoting (copying) previously-generated `hab` binary distribution
#   artifacts stored in S3 to the "dev" environment/channel hierarchy
#   in S3.
#
# Any subsequent promotions will be driven by the contents of the
# manifest.json file. This script just handles the creation of that
# file and the *initial* promotion.
set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"
export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"

# Take advantage of the fact that we're just promoting and we can run
# 100% on linux
declare -g hab_binary
curlbash_hab "x86_64-linux"

########################################################################

# `source_channel` should be the channel we are promoting all our
# artifacts from.
#
# e.g. `habitat-release-<build-id>`.
source_channel=${1:?You must specify a source channel value}

echo "--- Generating manifest input from $source_channel"

# Note that because we don't yet have a good API from which to query
# both ident and package target information, we abuse Buildkite
# metadata to store that information locally. We then use this to
# determine the exact information we put into the manifest.json file.
#
# As such, this script can basically be thought of as a way to extract
# this Buildkite metadata and capture it in a durable,
# Buildkite-independent form (namely, `manifest.json`) that we can use
# in downstream applications.
channel_pkgs_json=$(curl -s "${HAB_BLDR_URL}/v1/depot/channels/${HAB_ORIGIN}/${source_channel}/pkgs")
mapfile -t packages_to_promote < <(echo "${channel_pkgs_json}" | \
                         jq -r \
                         '.data |
                         map(.origin + "/" + .name + "/" + .version + "/" + .release)
                         | .[]')

# Generate the input file
manifest_input_file="manifest_input.txt"
targets=("x86_64-linux"
         "x86_64-linux-kernel2"
         "x86_64-windows"
         "x86_64-darwin")

for pkg in "${packages_to_promote[@]}"; do
    for pkg_target in "${targets[@]}"; do
        # Note that we must check all targets, and not short-circuit
        # after the first match, because it is currently possible for
        # two packages to have the same identifier, but different
        # targets.
        if ident_has_target "${pkg}" "${pkg_target}"; then
          echo ":thumbsup: Adding ${pkg} (${pkg_target}) to the '${manifest_input_file}' file"
          echo "${pkg} ${pkg_target}" >> "${manifest_input_file}"
      else
          echo ":thumbsdown: ${pkg} (${pkg_target}) was not a valid combination"
      fi
    done
done

echo "--- Generating manifest.json file"
version=$(get_version_from_repo)
sha="${BUILDKITE_COMMIT}"
.expeditor/scripts/release_habitat/create_manifest.rb "${manifest_input_file}" "${version}" "${sha}"
# Note that the "manifest.json" filename is determined by the
# `create_manifest.rb` script.

########################################################################

import_gpg_keys

echo "--- Pushing manifest file to S3"
store_in_s3 "${version}" manifest.json

# The remaining logic is essentially the same as in
# `buildkite_promote.sh` and `expeditor_promote.sh`, but
# done separately since we just stored the manifest in the "files"
# hierarchy, and not in the "channels" hierarchy; *this* code is what
# ultimately gets it into said hierarchy.

promote_packages_to_builder_channel manifest.json dev
promote_version_in_s3 "${version}" dev
