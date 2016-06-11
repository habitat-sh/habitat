#!/bin/bash

git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
is_tmp_commit=$?

# If we are not on a pull request, on the "auto" branch (which homu uses when
# auto-merging master), and not on a temporary commit, then run the publish
# script.
if [ "${TRAVIS_PULL_REQUEST}" = "false" ] &&
   [ "${TRAVIS_BRANCH}" = "auto" ] &&
   [[ $is_tmp_commit = 1 ]]; then
  set -eux
  sh components/hab/install.sh
  mkdir -p /hab/cache/keys
  ## Use environment variables set by Travis decrypt and import the origin key
  #openssl aes-256-cbc \
  #  -K $encrypted_c4f852370b68_key \
  #  -iv $encrypted_c4f852370b68_iv \
  #  -in core-20160423193745.sig.key.enc \
  #  -d \
  #  | hab origin key import
  #hab studio -k core build components/builder-web/habitat
else echo "Not on master; skipping publish"; fi
