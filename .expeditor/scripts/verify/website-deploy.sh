#!/bin/bash

# This deploys the web site

set -euo pipefail

source .expeditor/scripts/shared.sh

env="${1:-}"

if [[ "$env" != "acceptance" && "$env" != "production"  ]]; then
  echo "The argument to this script should be either 'acceptance' or 'production'"
  exit 1
fi

if [ "$env" == "acceptance" ]; then
  export AWS_BUCKET="habitat-www-acceptance"
elif [ "$env" == "production" ]; then
  export AWS_BUCKET="habitat-www-live"
fi

# verify that all the environment variables are properly set
vars=(AWS_BUCKET AWS_DEFAULT_REGION AWS_ACCESS_KEY_ID AWS_SECRET_ACCESS_KEY FASTLY_API_KEY FASTLY_SERVICE_KEY)
for var in "${vars[@]}"
do
  if [ -z "${!var:-}" ]; then
    echo "Please ensure that $var is exported in the environment."
    exit 1
  fi
done

cd www
make build
cd build
s3_sync "." "s3://$AWS_BUCKET"
purge_fastly_cache "$FASTLY_SERVICE_KEY"
