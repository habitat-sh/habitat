#!/bin/bash

#etcd_ipaddress=$(docker inspect $(docker ps | grep etcd | cut -d " " -f 1)  | jq -r '.[0].NetworkSettings.IPAddress')
etcd_ipaddress=$(docker-machine ip default)
curl -X DELETE http://$etcd_ipaddress:4001/v2/keys/bldr/redis/default/config
