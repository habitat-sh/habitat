#!/bin/bash

set -x

studio="hab-studio -r /hab/studios/travis-build"

if [ "${TRAVIS_PULL_REQUEST}" = "false" ] && [ "${TRAVIS_BRANCH}" = "master" ]; then
  components/studio/install.sh
  $studio new
  cp core-20160423193745.sig.key /hab/studios/travis-build/hab/cache/keys
  $studio build components/builder-web/plan
  $studio run "hab artifact upload /hab/cache/artifacts/core-habitat-builder-web-*"
else echo "Not on master; skipping publish"; fi
