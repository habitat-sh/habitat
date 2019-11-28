#!/bin/bash

# This is a simple "happy path" test of a rolling update.
# We will load services on two nodes to achieve quorum and
# then promote an update and expect the new release to show
# up after waiting 15 seconds. Note: we set HAB_UPDATE_STRATEGY_FREQUENCY_MS
# to 3000 in the docker-compose.override.yml.

set -xeuo pipefail

hab pkg install core/jq-static -b

test_channel=rolling-$(date '+%s%3N')
test_ident=habitat-testing/nginx/1.17.4/20191115184838

hab pkg promote ${test_ident} ${test_channel}

hab svc load habitat-testing/nginx --topology leader --strategy rolling --channel ${test_channel} --remote-sup=alpha.habitat.dev
hab svc load habitat-testing/nginx --topology leader --strategy rolling --channel ${test_channel} --remote-sup=beta.habitat.dev

cleanup () {
    hab bldr channel destroy ${test_channel} --origin habitat-testing
}

sleep 15

for server in alpha beta; do
    current_ident=$(curl -s "${server}.habitat.dev:9631/services/nginx/default" | jq -r '.pkg.ident')
    if [[ "${current_ident}" != "${test_ident}" ]]; then
        echo "Expected nginx ident ${test_ident} on ${server}; got ${current_ident} instead!"
        cleanup
        exit 1
    fi
done


test_ident=habitat-testing/nginx/1.17.4/20191115185517
hab pkg promote ${test_ident} ${test_channel}

sleep 15

for server in alpha beta; do
    current_ident=$(curl -s ${server}.habitat.dev:9631/services/nginx/default | jq -r '.pkg.ident')
    if [[ "${current_ident}" != "${test_ident}" ]]; then
        echo "Expected nginx ident ${test_ident} on ${server}; got ${current_ident} instead!"
        cleanup
        exit 1
    fi
done

cleanup
