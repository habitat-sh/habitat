#!/bin/bash


git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
is_tmp_commit=$?

set -eu

cd www
./bin/middleman build

# If we are not on a pull request, on the "auto" branch (which homu uses when
# auto-merging master), and not on a temporary commit, then run the publish
# script.
if [ "${TRAVIS_PULL_REQUEST}" = "false" ] &&
   [ "${TRAVIS_BRANCH}" = "auto" ] &&
   [[ $is_tmp_commit = 1 ]]; then
  ./bin/middleman s3_sync
  curl -H "Fastly-Key: ${FASTLY_API_KEY}" -X POST "https://api.fastly.com/service/${FASTLY_SERVICE_KEY}/purge_all"
else echo "Not on master; skipping website deploy"; fi
