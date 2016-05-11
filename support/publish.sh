#!/bin/bash

git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
is_tmp_commit=$?
studio_path=/hab/studios/travis-build
studio="hab-studio -r $studio_path"

# If we are not on a pull request, on the "auto" branch (which homu uses when
# auto-merging master), and not on a temporary commit, then run the publish
# script.
if [ "${TRAVIS_PULL_REQUEST}" = "false" ] &&
   [ "${TRAVIS_BRANCH}" = "auto" ] &&
   [[ $is_tmp_commit = 1 ]]; then
  set -x
  components/studio/install.sh
  $studio new
  cp core-20160423193745.sig.key $studio_path/hab/cache/keys
  $studio build components/builder-web/plan
  $studio run "hab artifact upload /hab/cache/artifacts/core-habitat-builder-web-*"
else echo "Not on master; skipping publish"; fi
