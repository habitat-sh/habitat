HAB_SUP_TEST_NETWORK_PEER_NAME := peer_1

up: down build CTL_SECRET
	docker build -t hab-sup-network .
	docker-compose up -d

down:
	docker-compose down
	rm -f CTL_SECRET

start:
	docker-compose start

stop:
	docker-compose stop

logs:
	docker-compose logs -f ${HAB_SUP_TEST_NETWORK_PEER_NAME}

console:
	docker exec -it $(shell docker ps -aqf name=${HAB_SUP_TEST_NETWORK_PEER_NAME}) /bin/bash

# Kill the specified container to simulate a departure
kill:
	docker stop $(shell docker ps -aqf name=${HAB_SUP_TEST_NETWORK_PEER_NAME})

# Disconnect the specified container to simulate a netsplit
disconnect:
	docker network disconnect $(shell docker network ls -qf name=sup-network) $(shell docker ps -aqf name=${HAB_SUP_TEST_NETWORK_PEER_NAME})

# Reconnect the specified container to simulate healing a netsplit
reconnect:
	docker network connect $(shell docker network ls -qf name=sup-network) $(shell docker ps -aqf name=${HAB_SUP_TEST_NETWORK_PEER_NAME})

# This gets the IP address of one of the containers that you specify. This is
# useful if you need to e.g. hab svc load --remote-sup=X.X.X.X core/redis
ip:
	docker network inspect $(shell docker network ls -qf name=sup-network) | jq ".[].Containers | to_entries | map(select(.value.Name == \"sup-network_${HAB_SUP_TEST_NETWORK_PEER_NAME}_1\")) | .[0].value.IPv4Address" | tr -d '"' | cut -d '/' -f 1

CTL_SECRET:
	hab sup secret generate > CTL_SECRET

build:
	cargo build -p habitat-launcher
	cargo build -p habitat_sup
	cargo build -p hab
	cargo build -p habitat-rst-reader
