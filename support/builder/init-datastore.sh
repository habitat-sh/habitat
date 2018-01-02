#!/bin/bash
set -euo pipefail

hab start core/builder-datastore &

running=0;

mkdir -p /hab/svc/builder-datastore/config/conf.d
chown hab:hab -R /hab/svc/builder-datastore/config/

echo "Waiting for builder-datastore to start"
pwfile=/hab/svc/builder-datastore/config/pwfile
while [ $running -eq 0 ]; do
  if [ -f $pwfile ]; then
    PGPASSWORD=$(cat $pwfile)
    export PGPASSWORD
    if hab pkg exec core/postgresql psql -w -lqt --host 127.0.0.1 -U hab; then
      running=1
    fi
  fi
  sleep 2
done

hab stop core/builder-datastore

# JW: This hack needs to stay until stop actually waits until the service has stopped
while [ -f /hab/sup/default/specs/builder-datastore.spec ]; do
  echo "Stopping builder-datastore"
  sleep 2
done

hab term
exit 0
