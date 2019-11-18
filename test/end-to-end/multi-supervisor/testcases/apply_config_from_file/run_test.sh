#!/bin/bash

set -xeuo pipefail

hab pkg install core/redis
hab pkg exec core/redis redis-cli --version

hab svc load core/redis --remote-sup=alpha.habitat.dev
hab svc load core/redis --remote-sup=beta.habitat.dev

# TODO Wait until redis is available
sleep 15

new_port=2112

echo -e "port = ${new_port}\nprotected-mode = \"no\"" > redis_config.toml
hab config apply \
    redis.default \
    "$(date +%s)" \
    redis_config.toml \
    --remote-sup=bastion.habitat.dev

sleep 5

for server in alpha beta; do
    hab pkg exec core/redis redis-cli \
        -h "${server}.habitat.dev" \
        -p "${new_port}" \
        SET from_stdin_port ${new_port}
done
for server in alpha beta; do
    hab pkg exec core/redis redis-cli \
        -h "${server}.habitat.dev" \
        -p "${new_port}" \
        GET from_stdin_port
done
