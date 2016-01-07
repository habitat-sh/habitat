#!/bin/bash

etcd_ipaddress=$(docker-machine ip ${DOCKER_MACHINE_NAME:-default})
curl -X DELETE http://$etcd_ipaddress:4001/v2/keys/bldr?recursive=true
