#!/bin/bash

set -euo pipefail

# shellcheck source=.expeditor/scripts/shared.sh
source .expeditor/scripts/shared.sh

install_hub
curlbash_hab "x86_64-linux"

branch="expeditor/documentation-update-$(date +"%Y%m%d%H%M%S")"
git checkout -b "$branch"

echo "--- :hammer_and_pick: Building new automated Habitat documentation"
(
    cd www

    hab pkg install core/hab-studio
    hab pkg install core/hab-sup
    hab pkg install core/hab-launcher

    hab pkg install core/node

    # Generate CLI docs
    hab pkg exec core/node node scripts/generate-cli-docs > source/docs/habitat-cli.html.md.erb

    # Generate template reference docs
    mkdir tmp
    cp ../components/sup/doc/* tmp/

    hab pkg exec core/node npm install json-schema-ref-parser@6.1.0
    hab pkg exec core/node node scripts/generate-template-reference.js \
        tmp/render_context_schema.json > ./source/partials/docs/_reference-template-data.html.md.erb
)

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
