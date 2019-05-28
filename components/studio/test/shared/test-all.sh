#!/bin/bash

set -euo pipefail

for t in test/shared/studio-internals/test-studio-*.sh; do 
  test_case="$(basename "$t")"
  echo "--- Running $test_case"
  hab studio rm 
  if ! expect test/shared/studio-enter.exp "$t"; then 
    exit 1
  fi
done
