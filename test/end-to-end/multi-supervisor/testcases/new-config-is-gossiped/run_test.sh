#!/bin/bash

set -xeuo pipefail

hab pkg install core/jq-static -b -f

hab svc load core/redis --remote-sup=alpha.habitat.dev
sleep 15
hab svc load christophermaier/test-probe --bind=thing_with_a_port:redis.default --remote-sup=beta.habitat.dev
sleep 15

current_port=$(curl -q beta.habitat.dev:8000/context | jq '.bind.thing_with_a_port.first.cfg.port')

if [[ "${current_port}" != "6379" ]]; then
    echo "Before config apply: expected Redis on port 6379; got ${current_port} instead!"
    exit 1
fi

new_port=1234

echo -e "port = ${new_port}\nprotected-mode = \"no\"" > redis_config.toml
hab config apply \
    redis.default \
    "$(date +%s)" \
    redis_config.toml \
    --remote-sup=bastion.habitat.dev

# The service is restarting??? Why?
sleep 15

current_port=$(curl -q beta.habitat.dev:8000/context | jq '.bind.thing_with_a_port.first.cfg.port')

if [[ "${current_port}" != "${new_port}" ]]; then
    echo "After config apply: expected Redis on port ${new_port}; got ${current_port} instead!"
    exit 1
fi
