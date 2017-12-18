#!/bin/bash
set -euo pipefail

builder_github_app_pem="/src/.secrets/builder-github-app.pem"

# GitHub secret keys
if [[ -f $builder_github_app_pem ]]; then
  for svc in sessionsrv worker api; do
    sudo mkdir -p /hab/svc/builder-${svc}/files
    sudo cp $builder_github_app_pem /hab/svc/builder-${svc}/files/
  done
else
  echo "Please install $builder_github_app_pem. and rerun $0 $*"
  echo "See https://github.com/habitat-sh/habitat/blob/master/BUILDER_DEV.md"
  exit 1
fi

# Bulder key generation
# key generate is done as root so the results end up in /hab rather than ~/.hab
# regardless of whether this script is run as root
KEY_NAME=$(sudo hab user key generate bldr | grep -Po "bldr-\d+")
for svc in api jobsrv worker ; do
  sudo mkdir -p "/hab/svc/builder-${svc}/files"
  sudo cp "/hab/cache/keys/${KEY_NAME}.pub" "/hab/svc/builder-${svc}/files/"
  sudo cp "/hab/cache/keys/${KEY_NAME}.box.key" "/hab/svc/builder-${svc}/files/"
done
