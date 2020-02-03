#!/bin/bash

# This deploys the web site

set -euo pipefail

source .expeditor/scripts/shared.sh

env="${1:-}"

if [[ "$env" != "acceptance" && "$env" != "live"  ]]; then
  echo "The argument to this script should be either 'acceptance' or 'live'"
  exit 1
fi

export AWS_BUCKET="habitat-www-$env"
export AWS_DEFAULT_REGION=us-west-2

# verify that the expected environment variables are properly set
vars=(FASTLY_SERVICE_KEY)
for var in "${vars[@]}"
do
  if [ -z "${!var:-}" ]; then
    echo "Please ensure that $var is exported in the environment."
    exit 1
  fi
done

(
cd www
make build
cd build
s3_sync "." "s3://$AWS_BUCKET"
# This is purging the cache for either www.habitat.sh or www.acceptance.habitat.sh,
# depending on which service key was provided.
purge_fastly_cache "$FASTLY_SERVICE_KEY"
)
