#!/bin/bash

set -euo pipefail 
 
# shellcheck source=.expeditor/scripts/shared.sh 
source .expeditor/scripts/shared.sh 

branch="ci/cargo-update-$(date +"%Y%m%d%H%M%S")"
git checkout -b "$branch"

toolchain="$(get_toolchain)"

install_rustup
rustup install "$toolchain"

echo "--- :ruby: Install hub"
gem install hub

echo "--- :habicat: Installing and configuring build dependencies"
hab pkg install core/libsodium core/libarchive core/openssl core/zeromq

PKG_CONFIG_PATH="$(< "$(hab pkg path core/libarchive)"/PKG_CONFIG_PATH)"
PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$(< "$(hab pkg path core/libsodium)"/PKG_CONFIG_PATH)"
PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$(< "$(hab pkg path core/openssl)"/PKG_CONFIG_PATH)"
PKG_CONFIG_PATH="$PKG_CONFIG_PATH:$(< "$(hab pkg path core/zeromq)"/PKG_CONFIG_PATH)"
export PKG_CONFIG_PATH 

# The library detection for the zeromq crate needs this additional hint. 
LD_RUN_PATH="$(hab pkg path core/zeromq)/lib"
export LD_RUN_PATH

echo "--- :rust: Cargo Update"
cargo clean
cargo +"$toolchain" update

echo "--- :rust: Cargo Check"
cargo +"$toolchain" check --all --tests && update_status="success" || update_status="failure"

git add Cargo.lock

git commit -s -m "Update Cargo.lock"

pr_message=""
pr_labels=""
if [ "$update_status" == "failure" ]; then 
  reason="Unable to update Cargo.lock!"
  details="For details on the failure, please visit ${BUILDKITE_BUILD_URL:-No Buildkite url}#${BUILDKITE_JOB_ID:-No Buildkite job id}"

  pr_message="--message \"$reason\" --message \"$details\""
  pr_labels="T-DO-NOT-MERGE"
fi
echo "$pr_message"
echo "--- :github: Open Pull Request"
hub pull-request --no-edit --draft "$pr_message" --labels "$pr_labels"


