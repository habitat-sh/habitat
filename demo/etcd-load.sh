#!/bin/bash

etcd_ipaddress=$(docker-machine ip default)
foo=$(cat $1); curl -L http://$etcd_ipaddress:4001/v2/keys/bldr/redis/default/config -XPUT -d value="${foo}"
