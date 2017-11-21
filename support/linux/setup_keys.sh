#!/bin/bash
set -euo pipefail

# GitHub secret keys
if [[ -f "/src/.secrets/builder-github-app.pem" ]]; then
  for svc in sessionsrv worker api; do
    mkdir -p /hab/svc/builder-${svc}/files
    cp /src/.secrets/builder-github-app.pem /hab/svc/builder-${svc}/files/
  done
else
  echo "Please follow instruction #6 here: https://github.com/habitat-sh/habitat/blob/master/BUILDER_DEV.md#pre-reqs"
fi

# Bulder key generation
KEY_NAME=$(hab user key generate bldr | grep -Po "bldr-\d+")
for svc in api jobsrv worker ; do
  mkdir -p "/hab/svc/builder-${svc}/files"
  cp "/hab/cache/keys/${KEY_NAME}.pub" "/hab/svc/builder-${svc}/files/"
  cp "/hab/cache/keys/${KEY_NAME}.box.key" "/hab/svc/builder-${svc}/files/"
done
