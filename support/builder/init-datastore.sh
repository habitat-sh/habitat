#!/bin/bash

hab start core/builder-datastore &

running=0;

mkdir -p /hab/svc/builder-datastore/config/conf.d
chown hab:hab -R /hab/svc/builder-datastore/config/*

echo "Waiting for builder-datastore to start"
while [ $running -eq 0 ]; do
  export PGPASSWORD=$(cat /hab/svc/builder-datastore/config/pwfile)
  if hab pkg exec core/postgresql psql -w -lqt --host 127.0.0.1 -U hab; then
    running=1
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
