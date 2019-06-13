#!/bin/bash

set -euo pipefail

studio_command="${1}"

for t in test/shared/studio-internals/test-studio-*.sh; do 
  test_case="$(basename "$t")"
  echo "--- Running $test_case"
  hab studio rm 
  if ! expect test/shared/studio-enter.exp "${studio_command}" "$t"; then 
    exit 1
  fi
done

for t in test/shared/studio-exit/test-studio-*.sh; do
  test_case="$(basename "$t")"
  echo "--- Running $test_case"
  $t "${studio_command}"
done
