#!/bin/bash

# These are some handy-dandy functions that can be shared across
# end-to-end tests to make it easier to write tests.

# Waits for `timeout_sec` seconds for a process named `process_name`
# to be found.
#
# Exits with code 1 if the process is not found in the allotted time.
wait_for_process() {
    local process_name="${1}"
    local timeout_sec="${2}"

    for i in $(seq "${timeout_sec}"); do
        if pgrep "${process_name}" &>/dev/null; then
            echo
            return
        else
            echo -n .
            sleep 1
        fi
    done
    echo "${process_name} did not start after ${timeout_sec} seconds!"
    exit 1
}

# Log the arguments to standard error with a helpful line header.
log() {
    echo "TEST_LOG>>> $*" >&2
}

# Helper to log the PID of a named process.
log_pid() {
    local process_name="${1}"
    local pid
    pid="$(pgrep "${process_name}")"
    log "Process '${process_name}' has PID ${pid}"
}

# Restarts the Supervisor by sending a SIGHUP, and then waits for it
# to restart.
restart_supervisor() {
    log "HUPping Supervisor"
    log_pid "hab-sup"
    pkill --signal=HUP hab-launch
    sleep 3 # wait for the signal to be processed
    wait_for_process hab-sup 5 # 5 seconds should be plenty of time
    log "New Supervisor started"
    log_pid "hab-sup"
}

# Load a given service and then wait for it to come up.
load_service() {
    local service="${1}"
    local binary="${2}"

    # We do this install first so we don't have to wait so long for
    # the service to start (we've got to download packages, and that
    # takes an indeterminate amount of time). If we had synchronous
    # loading, this wouldn't be an issue.
    hab pkg install "${service}" --channel=stable
    hab svc load "${service}"
    wait_for_process "${binary}" 5

    log_pid "redis"
}

readonly sup_log_file="sup.log"

# Starts up a Supervisor in the background and waits for it to
# start. Also wires up a trap function to shut it down at the end of
# the test.
start_supervisor() {
    hab sup run &> "${sup_log_file}" &
    trap cleanup_supervisor INT TERM EXIT
    wait_for_process "hab-sup" 10
}

# Trap function to shut down the Supervisor and also emit all the
# Supervisor's captured log output.
cleanup_supervisor() {
    hab sup term
    sed -e 's/^/TEST_SUP_LOG>>> /' "${sup_log_file}" >&2
}
