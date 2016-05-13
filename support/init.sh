#!/bin/sh
set -e

if [ ! -f /hab/cache/keys/core-20160423193745.pub ]; then
  cp -v /tmp/core-20160423193745.pub /hab/cache/keys
fi

exec "$@"
