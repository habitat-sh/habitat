#!/bin/bash

set -euo pipefail

component=sup

# shellcheck source=.expeditor/scripts/release_habitat/shared.sh
source .expeditor/scripts/release_habitat/shared.sh

echo "--- :hammer_and_pick: Generating Habitat Supervisor API docs"

#  Check if hub is installed and install if not
hub_check=$(which hub)
if [ -z "$hub_check" ]; then
  install_hub
fi

hab pkg install core/node --binlink

if [ -z "$(npm list webapi-parser | grep webapi-parser)" ]; then
  echo "webapi installed"
else
  echo "webapi not installed"
  npm install webapi-parser@0.5.0
fi

if [ -z "$(npm list minimist | grep minimist)" ]; then
  echo "minimist installed"
else
  echo "minimist not installed"
  npm install minimist@1.2.5
fi

tempdir="$(mktemp --directory --tmpdir="$(pwd)" -t "docs-XXXX")"

#  Generate the api docs file. 
input_file=components/${component}/doc/api.raml
output_file=${tempdir}/${component}"-api.json"

repo_file=components/docs-chef-io/static/habitat-api-docs/${component}"-api.json"

node .expeditor/scripts/release_habitat/hab-raml-converter.js -i "${input_file}" -o "${output_file}"

#  Only proceed with pull request if it has changed.
if cmp -s "${output_file}" "${repo_file}"; then
  echo "Habitat API docs generation is unnecessary"
  echo "Removing temp directory"
  rm -rf "${tempdir}"
  exit 0
else
  echo "--- Habitat API docs generation is necessary"
fi

echo "Updating repo file ${repo_file}"
cp "${output_file}" "${repo_file}"

TIMESTAMP=$(date '+%Y%m%d%H%M%S')
readonly branch="expeditor/habitat_release_$TIMESTAMP"
git checkout -b "${branch}"

echo "--- :git: Pushing new branch ${branch}"
git add "${repo_file}"
git commit --signoff --message "Update Habitat Supervisor API Docs - ${TIMESTAMP}"
push_current_branch

echo "--- :github: Creating PR"
hub pull-request --force --no-edit --message "Update Habitat Supervisor API Docs - ${TIMESTAMP}"

echo "Removing temp directory"
rm -rf "${tempdir}"
