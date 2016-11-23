#!/bin/bash

set -eu

if [[ "${TRAVIS_PULL_REQUEST}" = "false" ]] && [[ "${TRAVIS_BRANCH}" = "master" ]]; then
  echo "We are not on a PR and on the master branch. Going to deploy the site."
  cd www
  make deploy
else
  echo "We are either on a PR or a branch other than master. Doing nothing."
fi
