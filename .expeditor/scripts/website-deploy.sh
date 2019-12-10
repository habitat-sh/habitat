#!/bin/bash

# This deploys the web site

set -euo pipefail

# Explicitly get the current directory rather than assuming this is being run
# from the root of the habitat project, because this script is called from
# multiple places.
mydir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

echo "mydir = $mydir"

# shellcheck source=/dev/null
source "$mydir/shared.sh"

env="${1:-}"

if [[ "$env" != "acceptance" && "$env" != "live"  ]]; then
  echo "The argument to this script should be either 'acceptance' or 'live'"
  exit 1
fi

export AWS_BUCKET="habitat-www-$env"

# verify that all the environment variables are properly set
vars=(AWS_BUCKET AWS_DEFAULT_REGION FASTLY_SERVICE_KEY)
for var in "${vars[@]}"
do
  if [ -z "${!var:-}" ]; then
    echo "Please ensure that $var is exported in the environment."
    exit 1
  fi
done

(
cd "$mydir/../../www"
make build
cd build
s3_sync "." "s3://$AWS_BUCKET"
purge_fastly_cache "$FASTLY_SERVICE_KEY"
)
