#!/bin/bash

load_package() {
  hab pkg upload --url http://localhost:9636/v1 --auth "${HAB_AUTH_TOKEN}" "$@" --channel stable
}

load_packages() {
  if [[ -d /src/pkgs ]]; then
    for pkg in /src/pkgs/core*.hart ; do
      load_package "${pkg}"
    done
  fi
}

load_bootstrap_packages() {
  load_package /src/pkgs/core-hab-backline-0.39.0-dev-20171101044221-x86_64-linux.hart
  load_package /src/pkgs/core-hab-launcher-6083-20171101045646-x86_64-linux.hart
  load_package pkgs/core-hab-sup-0.34.0-dev-20170929202328-x86_64-linux.hart
  load_package pkgs/core-docker-17.09.0-20171001205930-x86_64-linux.hart
  load_package pkgs/core-elasticsearch-5.6.1-20171015201557-x86_64-linux.hart
  hab pkg install core/docker
  hab pkg install core/hab-pkg-export-docker
}

origin() {
  curl localhost:9636/v1/depot/origins \
    -d '{"name":"core"}' \
    -H "Authorization:Bearer:${HAB_AUTH_TOKEN}"
}

keys() {
  if [ -f ~/.hab/cache/keys/core-20160810182414.pub ]; then
    cat ~/.hab/cache/keys/core-20160810182414.pub | hab origin key import
  fi

  if [ -f ~/.hab/cache/keys/core-20160810182414.sig.key ]; then
    cat ~/.hab/cache/keys/core-20160810182414.sig.key | hab origin key import
  fi

  cat /hab/cache/keys/core-20160810182414.pub | \
  curl http://localhost:9636/v1/depot/origins/core/keys/20160810182414 \
    --data-binary @- \
    -H "Authorization:Bearer:${HAB_AUTH_TOKEN}"

  cat /hab/cache/keys/core-20160810182414.sig.key | \
  curl http://localhost:9636/v1/depot/origins/core/secret_keys/20160810182414 \
    --data-binary @- \
    -H "Authorization:Bearer:${HAB_AUTH_TOKEN}"
}

function psql() {
  hab pkg exec core/postgresql env PGPASSWORD=$(cat /hab/svc/builder-datastore/config/pwfile) psql -U hab -h 127.0.0.1 "$@"
}

export -f load_packages
export -f load_bootstrap_packages
export -f origin
export -f keys
export -f psql
