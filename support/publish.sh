#!/bin/bash

studio_path=/hab/studios/travis-build
studio="hab-studio -r $studio_path"

if [ "${TRAVIS_PULL_REQUEST}" = "false" ] && [ "${TRAVIS_BRANCH}" = "auto" ]; then
  set -x
  components/studio/install.sh
  $studio new
  cp core-20160423193745.sig.key $studio_path/hab/cache/keys
  $studio build components/builder-web/plan
  $studio run "hab artifact upload /hab/cache/artifacts/core-habitat-builder-web-*"
else echo "Not on master; skipping publish"; fi
