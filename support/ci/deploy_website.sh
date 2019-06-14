#!/bin/bash

set -eu

# TRAVIS_PULL_REQUEST is set to the pull request number if the current job is a pull request build, or false if it’s not.
#  https://docs.travis-ci.com/user/environment-variables/#convenience-variables
if [[ "${TRAVIS_PULL_REQUEST}" = "false" ]];then
  pr_triggered_build=false
else
  pr_triggered_build=true
fi

if $pr_triggered_build; then
  echo "We are on a PR. Going to just build the site."
  cd www
  make build
elif [[ "${TRAVIS_BRANCH}" = "master" ]]; then
  echo "We are not on a PR and on the master branch. Going to deploy the site."
  cd www
  make deploy
elif [[ "${TRAVIS_BRANCH}" =~ ^acceptance_deploy ]]; then
  echo "We are not on a PR and on an acceptance_deploy branch. Deploying to Acceptance."
  cd www
  BUILDER_WEB_URL="https://bldr.acceptance.habitat.sh" GITHUB_APP_URL="https://github.com/apps/habitat-builder-acceptance" make acceptance
else
  echo "Not building web site."
fi
