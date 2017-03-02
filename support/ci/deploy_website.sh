#!/bin/bash

set -eu

if [[ "${TRAVIS_PULL_REQUEST}" = "false" ]] && [[ "${TRAVIS_BRANCH}" = "master" ]]; then
  echo "We are not on a PR and on the master branch. Going to deploy the site."
  cd www
  make deploy
else
  if ! [[ "${TRAVIS_PULL_REQUEST}" = "false" ]] && [[ "${TRAVIS_BRANCH}" = "master" ]]; then
    echo "We are on a PR or against the master branch. Deploying to Acceptance."
    cd www
    make build
    sed -i '/^Disallow:/ s/$/ \//' build/robots.txt
    zip -r website.zip build

    curl -H "Content-Type: application/zip" \
      -H "Authorization: Bearer $NETLIFYKEY" \
      --data-binary "@website.zip" \
      --url https://api.netlify.com/api/v1/sites/habitat-acceptance.netlify.com/deploys
  fi
fi
