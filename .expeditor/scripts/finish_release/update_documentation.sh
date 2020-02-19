#!/bin/bash

set -euo pipefail

# shellcheck source=.expeditor/scripts/shared.sh
source .expeditor/scripts/shared.sh

echo "--- :hammer_and_pick: Installing prerequisites"
install_hub

# We want latest stable, since that's what the documentation script assumes
curlbash_hab "x86_64-linux"

hab pkg install core/hab-studio
hab pkg install core/hab-sup
hab pkg install core/hab-launcher
hab pkg install core/node --binlink

branch="expeditor/documentation-update-$(date +"%Y%m%d%H%M%S")"
git checkout -b "$branch"

echo "--- :hammer_and_pick: Generating CLI docs"
# TODO: can't currently use `hab pkg exec core/node node ...` for
# this because that blows away $PATH for the command, making it
# impossible to find `hab` :(
node .expeditor/scripts/finish_release/generate-cli-docs > www/source/docs/habitat-cli.html.md.erb

echo "--- :hammer_and_pick: Generating template reference docs"
tempdir="$(mktemp --directory --tmpdir="$(pwd)" -t "docs-XXXX")"

cp components/sup/doc/* "${tempdir}"

npm install json-schema-ref-parser@6.1.0
node .expeditor/scripts/finish_release/generate-template-reference.js \
     "${tempdir}"/render_context_schema.json > www/source/partials/docs/_reference-template-data.html.md.erb

echo "--- :git: Publishing updated documentation"
git add --update
git commit \
    --signoff \
    --message "Automated update of Habitat Documentation"
push_current_branch

echo "--- :github: Creating PR"
hub pull-request \
    --force \
    --no-edit \
    --message "Automated update of Habitat Documentation"
