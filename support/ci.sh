#!/bin/bash

set -ev
if [ "${TRAVIS_PULL_REQUEST}" = "false" ]; then
  #env IN_DOCKER=true make test refresh=true
  make unit
else
  make unit
fi
