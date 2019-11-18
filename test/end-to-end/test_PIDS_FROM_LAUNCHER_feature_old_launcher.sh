#!/bin/bash

# When using a Launcher from before the PIDS_FROM_LAUNCHER feature was
# created, we should still be using PID files for individual services,
# even if we've enabled the feature.

source test/end-to-end/shared.sh

export HAB_FEAT_PIDS_FROM_LAUNCHER=1

# This was the last stable Linux launcher prior to the
# PIDS_FROM_LAUNCHER feature.
hab pkg install core/hab-launcher/12605/20191112144831

start_supervisor

load_service "core/redis" "redis"

# This is the main assertion of this test
if [ ! -f "/hab/svc/redis/PID" ]; then
    echo "Service PID file should exist because we're using an older Launcher!"
    exit 1
fi

redis_pid="$(pgrep redis)"
sup_pid="$(pgrep hab-sup)"

restart_supervisor

new_redis_pid="$(pgrep redis)"
new_sup_pid="$(pgrep hab-sup)"

if [[ "${sup_pid}" == "${new_sup_pid}" ]]; then
    echo "Supervisor PID should have changed across restart, but it didn't!"
    exit 1
fi

if [[ "${redis_pid}" != "${new_redis_pid}" ]]; then
    echo "Service PID should have remained the same across restart, but it didn't!"
    echo "  Expected ${redis_pid}; was ${new_redis_pid}"
    exit 1
fi
