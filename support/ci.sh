#!/bin/bash

set -ev
if [ "${TRAVIS_PULL_REQUEST}" = "false" ]; then
  env IN_DOCKER=true make test refresh=true
else
  env IN_DOCKER=true make unit refresh=true
fi

# Someday, figure out deployment - sight
