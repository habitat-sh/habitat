#!/bin/bash

# This tests that removing the leader from a functioning leader topology
# service group that has enough nodes to maintain quorum after the leader is
# lost, it will continue to perform a succesful rolling update after a new
# leader is elected.
#
# We will load services on three nodes and then stop the supervisor on
# the leader node prompting a new election where one of the two follower nodes
# becomes a leader. Next we perform an update and expect both nodes to update.
# Prior to https://github.com/habitat-sh/habitat/pull/7167, the update after a
# new leader is elected would never occur because the new leader would continue
# to behave like a follower and wait for instructions to update.

set -xeuo pipefail

readonly test_channel=rolling-$(date '+%s%3N')
readonly test_ident_v1="habitat-testing/nginx/1.17.4/20191115184838"
readonly test_ident_v2="habitat-testing/nginx/1.17.4/20191115185900"

hab pkg promote ${test_ident_v1} "${test_channel}"

for server in alpha beta gamma; do
    hab svc load habitat-testing/nginx --topology leader --strategy rolling --channel "${test_channel}" --remote-sup=${server}.habitat.dev
done

cleanup () {
    hab bldr channel destroy "${test_channel}" --origin habitat-testing
}

trap cleanup INT TERM EXIT

sleep 15

for server in alpha beta gamma; do
    current_ident=$(curl -s "${server}.habitat.dev:9631/services/nginx/default" | jq -r '.pkg.ident')
    if [[ "${current_ident}" != "${test_ident_v1}" ]]; then
        echo "Initial load failed. Expected nginx ident ${test_ident_v1} on ${server}; got ${current_ident} instead!"
        exit 1
    fi
done

body=$(curl -s "bastion.habitat.dev:9631/census")
leader_id=$(echo "${body}" | jq -r ".census_groups.\"nginx.default\".leader_id")
leader_name=$(echo "${body}" | jq -r ".census_groups.\"nginx.default\".population.\"${leader_id}\".sys.hostname")
docker exec "${COMPOSE_PROJECT_NAME}_${leader_name}_1" hab sup term
sleep 5

hab pkg promote ${test_ident_v2} "${test_channel}"
sleep 15

for server in alpha beta gamma; do
    if [[ "${server}" != "${leader_name}" ]]; then
        current_ident=$(curl -s ${server}.habitat.dev:9631/services/nginx/default | jq -r '.pkg.ident')
        if [[ "${current_ident}" != "${test_ident_v2}" ]]; then
            echo "Update failed. Expected nginx ident ${test_ident_v2} on ${server}; got ${current_ident} instead!"
            exit 1
        fi
    fi
done
