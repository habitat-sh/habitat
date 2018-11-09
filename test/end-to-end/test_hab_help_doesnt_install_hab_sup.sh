#!/bin/bash

# A simple test assertion that running `hab sup --help` will not
# attempt to install `core/hab-sup` if that pkg is not present.

set -ou pipefail

export TESTING_FS_ROOT
TESTING_FS_ROOT=$(mktemp -d)
export HAB_SUP_BINARY
HAB_SUP_BINARY=''

print_help() {
  program=$(basename "$0")
  echo "$program

Test \`hab sup --help\` will not install core/hab-sup.

USAGE:
  $program <path-to-hab-binary>

ARGS:
  <path-to-hab-binary>  The name or path of the hab binary under test (eg: './target/debug/hab' or
simply 'hab'). In the latter case when just passing the binary name, the shell will search in \$PATH.
"
}

if [[ $# -eq 0 ]] ; then
  print_help
  echo
  echo "<path-to-hab-binary> must be specified"
  exit 1
else
  HAB_BINARY="$1"
fi

echo "Running \`$HAB_BINARY sup --help\` - which should NOT attempt an install of core/hab-sup"

if [ -z "$($HAB_BINARY sup --help)" ]; then
  echo
  echo "ERROR: $HAB_BINARY was not the proper executable hab binary!"
  exit 1
elif [ -d "$TESTING_FS_ROOT/hab/pkgs/core/hab-sup" ]; then
  echo
  echo "ERROR: detected an installation of core/hab-sup"
  exit 1
fi

echo "Success!"
