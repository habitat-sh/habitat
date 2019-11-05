#!/bin/bash

# This deploys the web site

set -euo pipefail

# check to see if any of the files in the www directory were modified and bail out if not
if ! git log --oneline --name-only master..HEAD | grep "www/"; then
  echo "No files in the www directory were modified. Exiting."
  exit 0
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
aws s3 sync . "s3://$AWS_BUCKET"
make purge_cache
