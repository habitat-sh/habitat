#!/bin/bash

set -euo pipefail 
 
# shellcheck source=.expeditor/scripts/shared.sh 
source .expeditor/scripts/shared.sh 

branch="ci/cargo-update-$(date +"%Y%m%d%H%M%S")"
git checkout -b "$branch"

toolchain="$(get_toolchain)"

install_rustup
rustup install "$toolchain"

install_hub

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

  # read always exits 1 if it can't find a delimeter
  # -d '' will always trigger this case as there is no delimeter to find, 
  # but that is required in order to write the entire message into a single PR 
  # preserving newlines.
  read -r -d '' pr_message <<EOM || true
Unable to update Cargo.lock!

For details on the failure, please visit ${BUILDKITE_BUILD_URL:-No Buildkite url}#${BUILDKITE_JOB_ID:-No Buildkite job id}
EOM

fi

echo "--- :git: REMOVE ME"
export HUB_VERBOSE=true
hub pr list --state "open"
git remote --verbose

hub pull-request --push --no-edit --draft --labels "$pr_labels" --file - <<EOM
"$pr_message"
EOM

exit "$update_status"
