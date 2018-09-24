#!/bin/bash

hab pkg exec core/hab-backline mkdir -m 1777 -p /tmp
hab pkg exec core/hab-backline mkdir -m 0750 -p /root
hab pkg exec core/hab-backline mkdir -m 0755 -p /usr/bin

source /etc/habitat-studio/import_keys.sh
source /etc/habitat-studio/environment.sh

declare -a secrets
readarray -t secrets < <(load_secrets)

case "$1" in
  enter)
    hab pkg exec core/hab-backline env STUDIO_ENTER=true "${secrets[@]}" bash --login +h;;
  build)
    shift
    hab pkg exec core/hab-backline env "${secrets[@]}" /bin/build "$@";;
  run)
    shift
    hab pkg exec core/hab-backline env "${secrets[@]}" bash --login -c "$@";;
  *)
    echo "Unknown Studio Command" && exit 1;;
esac
