#!/bin/bash

set -euo pipefail

# shellcheck source=.expeditor/scripts/release_habitat/shared.sh
source .expeditor/scripts/release_habitat/shared.sh

export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"

channel=$(get_release_channel)
version=$(get_version_from_repo)

# We want the version of Habitat we just built. This is important
# because it will be where we get the CLI docs from.
#
# At some point we can hopefully get this some other way, but for now,
# the binary tells us how to use it.
declare -g hab_binary
install_release_channel_hab_binary "$BUILD_PKG_TARGET"

# Ensure that we have this version of `hab` on the $PATH.
#
# See https://github.com/habitat-sh/habitat/pull/7835
# and https://github.com/chef/release-engineering/issues/1241 for
# further background.
${hab_binary} pkg binlink "chef/hab/${version}" --force --dest=/usr/bin

hab pkg install chef/hab-studio --channel="${channel}"
hab pkg install chef/hab-sup --channel="${channel}"
hab pkg install chef/hab-launcher --channel="${channel}"
hab pkg install core/node --binlink

echo "--- :hammer_and_pick: Generating CLI docs"

# Note: we're using this directory name to capture the generated
# artifacts by Buildkite, too, just for visibility and ease of
# troubleshooting.
#
# If we ever start doing more than generating just 2 documentation
# files this way, we may want to rethink this.
docs_dir="generated-documentation"
mkdir "${docs_dir}"

# We do not want to leak this in the documentation
unset HAB_AUTH_TOKEN

# TODO: can't currently use `hab pkg exec core/node node ...` for
# this because that blows away $PATH for the command, making it
# impossible to find `hab` :(
node .expeditor/scripts/release_habitat/generate-cli-docs.js > "${docs_dir}/habitat_cli.md"

echo "--- :hammer_and_pick: Generating template reference docs"
tempdir="$(mktemp --directory --tmpdir="$(pwd)" -t "docs-XXXX")"

cp components/sup/doc/* "${tempdir}"

npm install json-schema-ref-parser@6.1.0
node .expeditor/scripts/release_habitat/generate-template-reference.js \
     "${tempdir}"/render_context_schema.json > "${docs_dir}/service_templates.md"

echo "--- :package: Packaging generated documentation into a tarball"
docs_tarball="${docs_dir}.tar.gz"
tar --create \
    --gzip \
    --verbose \
    --file "${docs_tarball}" \
    "${docs_dir}"

echo "--- Pushing generated documentation to S3"
import_gpg_keys
store_in_s3 "${version}" "${docs_tarball}"
