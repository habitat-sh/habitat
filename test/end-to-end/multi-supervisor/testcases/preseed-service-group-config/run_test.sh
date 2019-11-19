#!/bin/bash

set -xeuo pipefail

# Ensures that we can `hab config apply` some configuration to a
# Habitat network before any services are running, and have those
# services pick up the configuration the first time they load.

new_port=8888

# Add some non-standard configuration to the network BEFORE we run
# anything in the targeted service group.
#
# Normally, Redis is available at port 6379, but here we're setting it
# to 8888.
echo -e "port = ${new_port}\nprotected-mode = \"no\"" |
hab config apply \
    redis.default \
    "$(date +%s)" \
    --remote-sup=bastion.habitat.dev

# Install redis locally so we have access to the redis CLI
hab pkg install core/redis
hab pkg exec core/redis redis-cli --version

# Start up a redis instance in the network and wait for it to come
# up. We expect it to pick up the configuration we injected into the
# network earlier.
hab svc load core/redis --remote-sup=alpha.habitat.dev
sleep 10

# We should be able to interact with the service at the new,
# non-standard port without a problem.
hab pkg exec core/redis redis-cli \
    -h "alpha.habitat.dev" \
    -p "${new_port}" \
    SET secret_message "Hello World"
