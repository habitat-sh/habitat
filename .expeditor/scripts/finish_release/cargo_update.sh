#!/bin/bash

set -euo pipefail 
 
# shellcheck source=.expeditor/scripts/shared.sh 
source .expeditor/scripts/shared.sh 

branch="ci/cargo-update-$(date +"%Y%m%d%H%M%S")"
git checkout -b "$branch"

toolchain="$(get_toolchain)"

install_rustup
rustup install "$toolchain"

#install_hub

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
cargo +"$toolchain" check --all --tests && update_status=$? || update_status=$?

echo "--- :git: Publishing updated Cargo.lock"
git add Cargo.lock

git commit -s -m "Update Cargo.lock"

pr_labels=""
pr_message=""
if [ "$update_status" -ne 0 ]; then 
  pr_labels="T-DO-NOT-MERGE"

  # It would be nice to use a heredoc to generate this, but JSON strings require escaped newlines.
  pr_message="Unable to update Cargo.lock!\n\n For details on the failure, please visit ${BUILDKITE_BUILD_URL:-No Buildkite url}#${BUILDKITE_JOB_ID:-No Buildkite job id}"

fi

open_pull_request "$pr_message"
#hub pull-request --push --no-edit --draft --labels "$pr_labels" --file - <<EOM
#"$pr_message"
#EOM

exit "$update_status"
