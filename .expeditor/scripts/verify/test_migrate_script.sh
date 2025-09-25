#!/bin/sh

echo "--- Testing migrate.sh"
# Bats in chefes/buildkite is a hab-binliked install to the default directory
# of /bin, but /bin isn't on our path. 
bats components/hab/tests/test_migrate_script.bats
