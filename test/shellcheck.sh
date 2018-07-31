#!/bin/bash

set -euo pipefail

shellcheck --version

# Run shellcheck against any files that appear to be shell script based on
# filename or `file` output
#
# Exclude the handlebars template files since their syntax confuses shellcheck
# There's not much bash in them anyway.
#
# Exclude *.sample files because they are automatically created by git
#
# Exclude *.ps1 files because shellcheck doesn't support them
#
# Exclude the bats submodules since we don't own that code.
#
# Exclude the following shellcheck issues since they're pervasive and innocuous:
# https://github.com/koalaman/shellcheck/wiki/SC1090
# https://github.com/koalaman/shellcheck/wiki/SC1091
# https://github.com/koalaman/shellcheck/wiki/SC1117
# https://github.com/koalaman/shellcheck/wiki/SC2148
# https://github.com/koalaman/shellcheck/wiki/SC2034
find . -type f \
  -and \( -name "*.*sh" \
      -or -exec sh -c 'file -b "$1" | grep -q "shell script"' {} \; \) \
  -and \! -path "*_template_plan.sh" \
  -and \! -path "*.sample" \
  -and \! -path "*.ps1" \
  -and \! -path "./test/integration/helpers.bash" \
  -and \! -path "./test/integration/test_helper/bats-assert/*" \
  -and \! -path "./test/integration/test_helper/bats-file/*" \
  -and \! -path "./test/integration/test_helper/bats-support/*" \
  -print \
  | xargs shellcheck --external-sources --exclude=1090,1091,1117,2148,2034

# This is a BATS file, so we need to override the interpreter
# See: https://github.com/koalaman/shellcheck/issues/709
shellcheck --shell=bash --exclude=1008 test/integration/helpers.bash

echo "shellcheck found no errors"
