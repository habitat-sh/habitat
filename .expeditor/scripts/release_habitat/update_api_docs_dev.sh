#!/bin/bash

set -euo pipefail

component=sup

# shellcheck source=.expeditor/scripts/release_habitat/shared.sh
source .expeditor/scripts/release_habitat/shared.sh

#  Check if hub is installed and install if not
hub_check=$(which hub)
if [ -z "$hub_check" ]; then
  install_hub
fi

echo "--- :hammer_and_pick: Generating Habitat Supervisor API docs"

# Check if node is installed and install if not
node_version=$(node -v)
if [ -z "$node_version" ]; then
  sudo apt-get install -y nodejs
fi

if [ -z "$(npm ls webapi-parser)" ]; then
  echo "webapi not installed"
  npm install webapi-parser@0.5.0
else
  echo "webapi installed"
fi

if [ -z "$(npm ls minimist)" ]; then
  echo "minimist not installed"
  npm install minimist@1.2.5
else
  echo "minimist installed"
fi

tempdir="$(mktemp --directory --tmpdir="$(pwd)" -t "docs-XXXX")"

cd "${tempdir}"
git clone "https://github.com/habitat-sh/habitat.git"
cd ..

#  Generate the api docs file in this repository. 
input_file=components/${component}/doc/api.raml
output_file=${tempdir}/${component}"-api.json"

repo_file=components/docs-chef-io/static/habitat-api-docs/${component}"-api.json"

node .expeditor/scripts/release_habitat/hab-raml-converter.js -i "${input_file}" -o "${output_file}"

if cmp -s "${output_file}" "${repo_file}"; then
  echo "Habitat API docs generation is unnecessary"
  echo "Removing temp directory"
  rm -rf "${tempdir}"
  exit 0
else
  echo "--- Habitat API docs generation is necessary"
fi

cd "${tempdir}/habitat"

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
hub pull-request --force --no-edit --draft --message "Update Habitat Supervisor API Docs - ${TIMESTAMP}"

echo "Removing temp directory"
rm -rf "${tempdir}"
