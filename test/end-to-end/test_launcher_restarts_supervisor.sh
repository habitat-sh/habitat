#!/bin/bash

# A simple test that the launcher restarts a supervisor when it exits abruptly
# The one optional argument set the exit code of the supervisor (default: 1).
# By default this runs against the installed habitat binaries. To override and
# test locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

set -eou pipefail


find_if_exists() {
  command -v "${1}" || { log "Required utility '${1}' cannot be found!  Aborting."; exit 1; }
}

find_if_exists pgrep

wait_for_sup_to_start() {
	until pgrep hab-sup &>/dev/null; do
		echo -n .
		sleep 1
	done
	echo
}

if pgrep hab-launch &>/dev/null; then
	echo "Error: launcher process is already running"
	exit 1
fi

sup_log="sup.log"
exit_file=$(mktemp)
exit_code=${1:-1}

mkdir -p "$(dirname "$sup_log")"
echo -n "Starting launcher (logging to $sup_log)..."

env HAB_FEAT_TEST_EXIT="$exit_file" hab sup run &> "$sup_log" &
trap 'hab sup term' INT TERM EXIT

wait_for_sup_to_start
read -r launcher_pid < <(pgrep hab-launch)
read -r supervisor_pid < <(pgrep hab-sup)

echo "Launcher is process $launcher_pid, supervisor is process $supervisor_pid"
echo "Forcing supervisor to exit $exit_file..."
echo "$exit_code" >> "$exit_file"

echo -n "Waiting for old supervisor process to exit..."
while ps -p "$supervisor_pid" &>/dev/null; do
	echo -n .
	sleep 1
done
echo

if ! pgrep hab-launch &>/dev/null; then
	echo "Failure! Launcher process exited"
	exit 2
fi

echo "Waiting for new supervisor process to start..."
wait_for_sup_to_start
read -r new_launcher_pid < <(pgrep hab-launch)
read -r new_supervisor_pid < <(pgrep hab-sup)

if [[ $supervisor_pid == "$new_supervisor_pid" ]]; then
	echo "Failure! Supervisor process did not change"
	exit 3
elif [[ $launcher_pid != "$new_launcher_pid" ]]; then
	echo "Failure! Launcher process changed unexpectedly"
	exit 4
else
	echo "Success! Launcher restarted supervisor process"
fi
