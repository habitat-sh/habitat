#!/bin/bash

hab pkg exec core/hab-backline mkdir -m 1777 -p /tmp

source /etc/habitat-studio/import_keys.sh
source /etc/habitat-studio/environment.sh

case "$1" in
  enter)
     # shellcheck disable=SC2046
     hab pkg exec core/hab-backline env STUDIO_ENTER=true $(load_secrets) bash --login +h;;
  build)
    shift
    hab pkg exec core/hab-plan-build hab-plan-build "$@";;
  run)
    shift
    hab pkg exec core/hab-backline bash --login -c "$@";;
  *) echo "Unknown Studio Command" && exit 1;;
esac
