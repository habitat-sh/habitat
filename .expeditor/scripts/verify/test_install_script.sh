#!/bin/sh

if ! command -v bats >/dev/null; then
  if [ "$(uname)" = "Darwin" ]; then
    echo "--- Installing bats"
    brew install bats-core
  fi
fi

echo "--- Testing install.sh"
# Bats in chefes/buildkite is a hab-binliked install to the default directory
# of /bin, but /bin isn't on our path.
bats components/hab/tests/test_install_script.bats
