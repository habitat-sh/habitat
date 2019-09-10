#!/bin/bash

# A simple test that the launcher doesn't go into a tight loop restarting the
# supervisor if the supervisor fails to start up. To override and test
# locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

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
sup_log="sup.log"

echo "Setting $TESTING_FS_ROOT to read-only to induce supervisor start failure"
chmod -R a-w "$TESTING_FS_ROOT"
echo -n "Starting launcher with root $TESTING_FS_ROOT (logging to $sup_log)..."

hab sup run &> "$sup_log" &
launcher_pid=$!
trap 'pgrep hab-launch &>/dev/null && pkill -9 hab-launch' INT TERM EXIT

retries=0
max_retries=5
while ps -p "$launcher_pid" &>/dev/null; do
	echo -n .
	if [[ $((retries++)) -gt $max_retries ]]; then
		echo
		echo "Failure! Launcher failed to exit before timeout"
		exit 2
	else
		sleep 1
	fi
done

echo
echo "Success! Launcher cleanly exited"
