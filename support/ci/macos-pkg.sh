#!/bin/bash

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

src_root=$(dirname $0)/../../

if [ "$TRAVIS_PULL_REQUEST" == "false" ]; then
  sudo -E $src_root/components/hab/mac/mac-build.sh $src_root/components/hab/mac
  mkdir -p $src_root/out/hab-x86_64-darwin
  source $src_root/results/last_build.env && cp /hab/pkgs/$pkg_ident/bin/hab $src_root/out/hab-x86_64-darwin/hab
  zip -9 -r $src_root/out/hab-x86_64-darwin.zip $src_root/out/hab-x86_64-darwin
fi
