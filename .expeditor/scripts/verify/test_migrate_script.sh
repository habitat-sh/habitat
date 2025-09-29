#!/bin/sh

echo "--- Testing migrate.sh"
sudo -E hab pkg install core/bats --binlink
bats components/hab/tests/test_migrate_script.bats
