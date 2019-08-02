#!/bin/bash

set -euo pipefail 
 
# shellcheck source=./support/ci/shared.sh 
source ./support/ci/shared.sh 

branch="ci/cargo-update-$(date +"%Y%m%d")"
git checkout -b "$branch"

toolchain="$(get_toolchain)"

install_rustup
install_rust_toolchain "$toolchain"

echo "--- :box: Cargo Update"
cargo +"$toolchain" update
echo "--- :box: Cargo Check"
cargo +"$toolchain" check --quiet --all --tests

git add Cargo.lock

if [ "$BUILDKITE" != "true" ]; then 
  echo "Not running in Buildkite. Please verify cargo updated correctly before opening a Pull Request" 
  exit 0
fi

git commit -s -m "Update Cargo.lock"

open_pull_request

git checkout master 
git branch -D "$branch"
