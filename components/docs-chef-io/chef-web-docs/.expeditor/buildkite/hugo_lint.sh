#!/bin/bash

set -euo pipefail

LINT_STATUS="$(grep -r -I --color=auto -o --with-filename -n -P '[^\x00-\x7F]' ./content | grep -v '[✓]'  &> /dev/null ; echo $?)"

if [ "$LINT_STATUS" == 1 ]; then
  echo "Success!"
  exit 0
else
  echo "Failure!"
  grep -r -I --color=auto -o --with-filename -n -P '[^\x00-\x7F]' ./content | grep  -v '[✓]'
  if [ "$LINT_STATUS" == 0 ]; then
    exit 1
  else
    exit "$LINT_STATUS"
  fi
fi

