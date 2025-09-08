#!/bin/bash

set -euo pipefail
set -x

export HAB_LICENSE=accept-no-persist
export HAB_CHANNEL=base

# shellcheck source=.expeditor/scripts/release_habitat/shared.sh
source .expeditor/scripts/release_habitat/shared.sh

echo "--- :hammer_and_pick: Generating Habitat Supervisor API docs"

#  Check if hub is installed and install if not
if ! which hub &> /dev/null; then
  install_hub
fi


hab pkg install core/node --binlink --force # v22.17.0 when this comment was written
hab pkg exec core/node npm install minimist
hab pkg exec core/node npm install oas-normalize

tempdir="$(mktemp -d)"
trap 'rm -rf "${tempdir}"' EXIT ERR

#  Generate the api docs file.
input_file=components/sup/doc/api.yaml
output_file=${tempdir}/sup-api.json
repo_file=components/docs-chef-io/static/habitat-api-docs/sup-api.json

# Verify input file exists
if [[ ! -f "$input_file" ]]; then
  echo "Error: Input file $input_file does not exist"
  exit 1
fi

echo "--- :page_facing_up: Converting OpenAPI specification"
node .expeditor/scripts/finish_release/openapi-yaml-converter.js -i "${input_file}" -o "${output_file}"

# Only proceed with pull request if it has changed
if cmp -s "${output_file}" "${repo_file}"; then
  echo "--- :white_check_mark: Habitat API docs are up to date,docs generation is unnecessary"
  exit 0
else
  echo "--- :arrow_up: Habitat API docs need updating"
fi

echo "--- :file_folder: Updating repo file ${repo_file}"
cp "${output_file}" "${repo_file}"

TIMESTAMP=$(date '+%Y%m%d%H%M%S')
readonly branch="expeditor/habitat_release_$TIMESTAMP"
git checkout -b "${branch}"

echo "--- :git: Committing changes"
git add "${repo_file}"
git commit --signoff --message "Update Habitat Supervisor API Docs - ${TIMESTAMP}"
push_current_branch

echo "--- :github: Creating PR"
hub pull-request --force --no-edit --message "Update Habitat Supervisor API Docs - ${TIMESTAMP}"

echo "--- :white_check_mark: API docs update complete"
