#!/bin/bash

set -xeuo pipefail

hab pkg install core/redis
hab pkg exec core/redis redis-cli --version

hab svc load core/redis --remote-sup=alpha.habitat.dev
hab svc load core/redis --remote-sup=beta.habitat.dev

# TODO Wait until redis is available
sleep 15

echo "Hello from Habitat!" > message.txt
hab file upload \
    redis.default \
    "$(date +%s)" \
    message.txt \
    --remote-sup=bastion.habitat.dev

# TODO give the file time to get out
sleep 5

for service in alpha beta; do
    # TODO (CM): abstract this pattern a bit better
    docker exec "${COMPOSE_PROJECT_NAME}_${service}_1" cat /hab/svc/redis/files/message.txt
done
