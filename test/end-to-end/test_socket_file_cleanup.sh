#!/bin/bash

set -eou pipefail

find_socket_files() {
	find /tmp -maxdepth 1 -name "rust-ipc-socket.*"
}

socket_files_before=$(mktemp)
sup_log=$(mktemp)
find_socket_files > "$socket_files_before"

echo -n "Starting launcher (logging to $sup_log)..."
# shellcheck disable=2024
hab sup run &> "$sup_log" &
until hab sup status &> /dev/null; do
	echo -n .
	sleep 1
done
echo

hab sup term
echo "Waiting for launcher to exit..."
wait

socket_files_after=$(mktemp)
find_socket_files > "$socket_files_after"

echo "Checking for socket files left behind..."
if grep -vf "$socket_files_before" "$socket_files_after"; then
	echo "Failure! Dumping supervisor log:"
	cat "$sup_log"
	exit 1
else
	echo "Success! No socket file left behind"
fi
