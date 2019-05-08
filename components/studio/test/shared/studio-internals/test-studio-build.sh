#!/bin/bash 

set -euo pipefail 

export HAB_ORIGIN=ci
hab origin key generate $HAB_ORIGIN

if build test/fixtures/empty-plan; then 
  echo "[PASS] - test finished"
else 
  echo "[FAIL] - test finished"
  exit 1
fi

