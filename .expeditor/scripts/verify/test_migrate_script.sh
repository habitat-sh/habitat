#!/bin/sh

echo "--- Testing migrate.sh"
hab pkg install core/bats --binlink
bats components/hab/tests/test_migrate_script.bats
