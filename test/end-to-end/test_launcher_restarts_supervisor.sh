#!/bin/bash

set -eou pipefail

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

TESTING_FS_ROOT=$(mktemp -d)
export TESTING_FS_ROOT
sup_log="$TESTING_FS_ROOT/hab/sup/default/sup.log"
exit_file=$(mktemp)
exit_code=${1:-1}

mkdir -p "$(dirname "$sup_log")"
echo -n "Starting launcher with root $TESTING_FS_ROOT (logging to $sup_log)..."
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
