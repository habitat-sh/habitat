#!/bin/bash
set -euo pipefail

sudo hab start core/builder-datastore &

running=0;

sudo mkdir -p /hab/svc/builder-datastore/config/conf.d
sudo chown hab:hab -R /hab/svc/builder-datastore/config/

echo "Waiting for builder-datastore to start"
pwfile=/hab/svc/builder-datastore/config/pwfile
while [ $running -eq 0 ]; do
  if sudo [ -f $pwfile ]; then
    PGPASSWORD=$(sudo cat $pwfile)
    export PGPASSWORD
    if hab pkg exec core/postgresql psql -w -lqt --host 127.0.0.1 -U hab; then
      running=1
    fi
  fi
  sleep 2
done

if [ -z "${NO_DB_SETUP+x}" ]; then
  for dbname in builder_sessionsrv builder_jobsrv builder_originsrv; do
    if sudo -E TERM=vt100 hab pkg exec core/postgresql psql -lqt --host 127.0.0.1 -U hab | cut -d \| -f 1 | grep -qw $dbname; then
      echo "Database $dbname exists"
    else
      echo "Creating database $dbname"
      sudo -u hab -E TERM=vt100 hab pkg exec core/postgresql createdb -O hab -h 127.0.0.1 $dbname
    fi
  done
fi

sudo hab stop core/builder-datastore

# JW: This hack needs to stay until stop actually waits until the service has stopped
while [ -f /hab/sup/default/specs/builder-datastore.spec ]; do
  echo "Stopping builder-datastore"
  sleep 2
done

sudo hab term
exit 0
