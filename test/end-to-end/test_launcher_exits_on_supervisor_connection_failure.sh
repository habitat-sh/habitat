#!/bin/bash

# A simple test that the launcher doesn't hang if the IPC connection to the
# supervisor doesn't complete in a timely manner. To override and test
# locally-built code, set overrides in the environment of the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

set -eou pipefail

if pgrep hab-launch &>/dev/null; then
	echo "Error: launcher process is already running"
	exit 1
fi

export HAB_LAUNCH_SUP_CONNECT_TIMEOUT_SECS=2
export HAB_FEAT_BOOT_FAIL=1
export HAB_LAUNCH_NO_SUP_VERSION_CHECK="true"
sup_log=$(mktemp)

# Preinstall these packages. If we don't, then we spend the bulk of
# our time in the following `while` loop downloading them, rather than
# actually exercising the functionality we're after. That leads to
# spurious failures, depending on how long the downloading takes.
#
# Doing things this way, we eliminate that concern.
hab pkg install core/hab-sup --channel="${HAB_BLDR_CHANNEL}"
hab pkg install core/hab-launcher --channel="${HAB_BLDR_CHANNEL}"

echo -n "Starting launcher (logging to $sup_log)..."
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
        contents=$(cat "$sup_log")
        echo "--- FAILURE LOG: ${contents}"
        exit 2
    else
        sleep 1
    fi
done

echo

if wait "$launcher_pid"; then
    echo "Failure! Launcher exited success; error expected"
else
    expected_error_string="Unable to accept connection from Supervisor"
    contents=$(cat "$sup_log")
    if [[ "${contents}" =~ ${expected_error_string} ]]; then
        echo "Success! Launcher exited with expected error: ${expected_error_string}"
    else
        echo "--- FAILURE! Launcher exited with an error, but not the expected one!"
        echo "Did not find:"
        echo "    ${expected_error_string}"
        echo "in the output (see full output below)!"
        echo
        echo "${contents}"
        exit 3
    fi
fi
