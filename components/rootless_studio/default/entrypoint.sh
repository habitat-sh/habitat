#!/bin/bash

hab pkg exec chef/hab-backline mkdir -m 1777 -p /tmp
hab pkg exec chef/hab-backline mkdir -m 0750 -p /root
hab pkg exec chef/hab-backline mkdir -m 0755 -p /usr/bin

source /etc/habitat-studio/import_keys.sh
source /etc/habitat-studio/environment.sh

declare -a secrets
readarray -t secrets < <(load_secrets)

case "$1" in
  enter)
    hab pkg exec chef/hab-backline env STUDIO_ENTER=true "${secrets[@]}" bash --login +h;;
  build)
    shift
    hab pkg exec chef/hab-backline env "${secrets[@]}" /bin/build "$@";;
  run)
    shift
    hab pkg exec chef/hab-backline env "${secrets[@]}" bash --login -c "$@";;
  *)
    echo "Unknown Studio Command: ${1}" && exit 1;;
esac
