#!/bin/bash

git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
is_tmp_commit=$?

set -eu

cd www
./bin/middleman build

# If we are not on a pull request, and are on the "auto" branch that
# homu uses when auto-merging to master, and are not on a temporary
# commit, then publish to acceptance, and then production/live.
if [ "${TRAVIS_PULL_REQUEST}" = "false" ] &&
   [ "${TRAVIS_BRANCH}" = "auto" ] &&
   [[ $is_tmp_commit = 1 ]]; then
  AWS_BUCKET=habitat-www-acceptance ./bin/middleman s3_sync
  ./bin/middleman s3_sync
  # If we have published, then we need to purge the CDN cache.
  curl -H "Fastly-Key: ${FASTLY_API_KEY}" \
       -X POST "https://api.fastly.com/service/${FASTLY_SERVICE_KEY}/purge_all"
else
  echo "Not on master; skipping website deploy"
fi
