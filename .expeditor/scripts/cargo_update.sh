#!/bin/bash

set -euo pipefail 
 
if ! "${BUILDKITE:-false}"; then 
  echo "This script does not appear to be running in Buildkite. Exiting!" 
  exit 1
fi

# shellcheck source=./support/ci/shared.sh 
source ./support/ci/shared.sh 

branch="ci/cargo-update-$(date +"%Y%m%d%H%M%S")"
git checkout -b "$branch"

toolchain="$(get_toolchain)"

install_rustup
install_rust_toolchain "$toolchain"

echo "--- :box: Cargo Update"
cargo +"$toolchain" update
echo "--- :box: Cargo Check"
cargo +"$toolchain" check --quiet --all --tests

git add Cargo.lock


git commit -s -m "Update Cargo.lock"

# https://expeditor.chef.io/docs/reference/script/#open-pull-request
open_pull_request

git checkout master 
git branch -D "$branch"
