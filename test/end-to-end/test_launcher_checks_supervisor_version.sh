#!/bin/bash

# A simple test that the launcher correctly checks the version of the
# supervisor binary is compatible.
#
# To override and test locally-built code, set overrides in the environment of
# the script.
# See https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

set -eou pipefail

create_sup_binary_stub() {
    version=${1?}
    binary=$(mktemp)

    cat <<EOF > "$binary"
#!/bin/bash
echo "hab-sup $version"
EOF

    chmod +x "$binary"
    echo "$binary"
}

launcher_exits_with_error() {
    version=${1?}
    sup_binary=$(create_sup_binary_stub "$version")

    sup_log=$(mktemp)

    echo -n "Starting launcher with supervisor version \"$version\" (logging to $sup_log)..."
    HAB_SUP_BINARY="$sup_binary" hab sup run &> "$sup_log" &
    launcher_pid=$!

    retries=0
    max_retries=3
    while ps -p "$launcher_pid" &>/dev/null; do
        echo -n .
        if [[ $((retries++)) -gt $max_retries ]]; then
            echo
            kill "$launcher_pid"
            return 1
        else
            sleep 1
        fi
    done
    echo

    ! wait "$launcher_pid"
}

trap 'pgrep hab-launch &>/dev/null && pkill -9 hab-launch' INT TERM EXIT

incompatible_version="0.55.0/20180321222338"
if HAB_LAUNCH_NO_SUP_VERSION_CHECK='' launcher_exits_with_error "$incompatible_version"; then
    echo "Success! Launcher exited with error"
    echo
else
    echo "Failure! Expected launcher to exit with error"
    contents=$(cat "$sup_log")
    echo "--- FAILURE LOG: ${contents}"
    exit 1
fi

if HAB_LAUNCH_NO_SUP_VERSION_CHECK=1 launcher_exits_with_error "$incompatible_version"; then
    echo "Failure! Expected launcher remain running"
    contents=$(cat "$sup_log")
    echo "--- FAILURE LOG: ${contents}"
    exit 1
else
    echo "Success! Setting HAB_LAUNCH_NO_SUP_VERSION_CHECK skips version check"
    echo
fi

compatible_version="0.56.0/20180530235935"
if HAB_LAUNCH_NO_SUP_VERSION_CHECK='' launcher_exits_with_error "$compatible_version"; then
    echo "Failure! Expected launcher remain running"
    contents=$(cat "$sup_log")
    echo "--- FAILURE LOG: ${contents}"
    exit 1
else
    echo "Success! Supervisor passed version check"
    echo
fi

dev_version="0.62.0-dev"
if HAB_LAUNCH_NO_SUP_VERSION_CHECK='' launcher_exits_with_error "$dev_version"; then
    echo "Failure! Expected launcher remain running"
    contents=$(cat "$sup_log")
    echo "--- FAILURE LOG: ${contents}"
    exit 1
else
    echo "Success! Supervisor passed version check"
    echo
fi

invalid_version="one-point-twenty-one"
if HAB_LAUNCH_NO_SUP_VERSION_CHECK='' launcher_exits_with_error "$invalid_version"; then
    echo "Success! Launcher exited with error"
    echo
else
    echo "Failure! Expected launcher to exit with error"
    contents=$(cat "$sup_log")
    echo "--- FAILURE LOG: ${contents}"
    exit 1
fi
