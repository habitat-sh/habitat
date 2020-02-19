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
    make cli_docs
    make template_reference
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
