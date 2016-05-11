#!/bin/bash

git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
is_tmp_commit=$?

# When we're on a temporary commit, don't do anything.
if [[ $is_tmp_commit = 0 ]]; then
  exit 0
fi

set -ev
if [ "${TRAVIS_PULL_REQUEST}" = "false" ]; then
  #env IN_DOCKER=true make test refresh=true
  make unit
else
  make unit
fi
