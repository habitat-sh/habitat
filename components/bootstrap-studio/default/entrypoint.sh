#!/bin/bash

mkdir -m 1777 -p /tmp
mkdir -m 0750 -p /root
mkdir -m 0755 -p /usr/bin

source /etc/habitat-studio/import_keys.sh
source /etc/habitat-studio/environment.sh

declare -a secrets
readarray -t secrets < <(load_secrets)

case "$1" in
  enter)
    env -i STUDIO_ENTER=true "${secrets[@]}" bash --login +h;;
  build)
    shift
    env "${secrets[@]}" /bin/build "$@";;
  run)
    shift
    env "${secrets[@]}" bash --login -c "$@";;
  *)
    echo "Unknown Studio Command" && exit 1;;
esac
