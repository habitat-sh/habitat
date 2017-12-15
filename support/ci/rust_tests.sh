#!/bin/bash

git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
is_tmp_commit=$?

# When we're on a temporary commit, don't do anything.
if [[ $is_tmp_commit = 0 ]]; then
  exit 0
fi

echo "--> Running $0"

source $(dirname $0)/rust_env.sh

set -e
make unit-${COMPONENTS:-all}
