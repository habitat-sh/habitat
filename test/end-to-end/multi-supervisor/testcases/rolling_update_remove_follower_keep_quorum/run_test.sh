#!/bin/bash

# This tests that removing a follower from a functioning leader topology
# service group will continue to perform a succesful roaming update
# We will load services on three nodes and then stop the supervisor on one
# of the follower nodes. Next we perform an update and expect the remaining
# two nodes to update. Prior to https://github.com/habitat-sh/habitat/pull/7167
# a rolling update after a member death would cause the leader to wait for dead
# members to update themselves which of course will never happen. So we 
# perform another update which should succeed if the leader is ignoring dead
# members as it should.

set -xeuo pipefail

readonly test_channel=rolling-$(date '+%s%3N')
readonly test_ident_v1="habitat-testing/nginx/1.17.4/20191115184838"
readonly test_ident_v2="habitat-testing/nginx/1.17.4/20191115185517"
readonly test_ident_v3="habitat-testing/nginx/1.17.4/20191115185900"

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

# find the name of the leader and choose the first node that is
# not the leader as the follower to kill. Send it a hab sup term
body=$(curl -s "bastion.habitat.dev:9631/census")
leader_id=$(echo "${body}" | jq -r ".census_groups.\"nginx.default\".leader_id")
leader_name=$(echo "${body}" | jq -r ".census_groups.\"nginx.default\".population.\"${leader_id}\".sys.hostname")
follower_name=""
for server in alpha beta gamma; do
    if [[ "${server}" != "${leader_name}" ]]; then
        follower_name=${server}
        break
    fi
done
docker exec "${COMPOSE_PROJECT_NAME}_${follower_name}_1" hab sup term
sleep 5

# perform an update
hab pkg promote ${test_ident_v2} "${test_channel}"
sleep 15

# we expect everyone to be updated now but prior to
# https://github.com/habitat-sh/habitat/pull/7167 the leader will
# indefinitely wait for the dead followers to update
for server in alpha beta gamma; do
    if [[ "${server}" != "${follower_name}" ]]; then
        current_ident=$(curl -s ${server}.habitat.dev:9631/services/nginx/default | jq -r '.pkg.ident')
        if [[ "${current_ident}" != "${test_ident_v2}" ]]; then
            echo "First promotion failed. Expected nginx ident ${test_ident_v2} on ${server}; got ${current_ident} instead!"
            exit 1
        fi
    fi
done

# update again
hab pkg promote ${test_ident_v3} "${test_channel}"
sleep 15

# if the leader is not stuck waiting for dead members for the previous update,
# this update should succeed
for server in alpha beta gamma; do
    if [[ "${server}" != "${follower_name}" ]]; then
        current_ident=$(curl -s ${server}.habitat.dev:9631/services/nginx/default | jq -r '.pkg.ident')
        if [[ "${current_ident}" != "${test_ident_v3}" ]]; then
            echo "Second promotion failed. Expected nginx ident ${test_ident_v3} on ${server}; got ${current_ident} instead!"
            exit 1
        fi
    fi
done
