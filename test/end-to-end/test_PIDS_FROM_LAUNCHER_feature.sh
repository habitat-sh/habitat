#!/bin/bash

# When using a current Launcher and enabling the PIDS_FROM_LAUNCHER
# feature, the Supervisor should not create PID files for the services
# it manages.

source test/end-to-end/shared.sh

export HAB_FEAT_PIDS_FROM_LAUNCHER=1

start_supervisor
wait_for_control_gateway

load_service "core/redis" "redis"

# This is the main assertion of this test
if [ -f "/hab/svc/redis/PID" ]; then
    echo "Should not have a PID file!"
    exit 1
fi

redis_pid="$(pgrep redis)"
sup_pid="$(pgrep hab-sup)"

restart_supervisor
wait_for_control_gateway

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
