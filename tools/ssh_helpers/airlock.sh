#!/usr/bin/env bash

environment=${1}

airlock_interface="ens4"

for worker in $(grep "Host ${environment}-builder-worker" ~/.ssh/config | awk '{print $2}' | sort); do
    echo "Worker ${worker}"
    output=$(ssh ${worker} "ip address show dev ${airlock_interface} | grep 'inet '")
    if [ -z "$output" ]; then
        echo "OMG NO IP! Restarting network stack..."
        ssh ${worker} "sudo /etc/init.d/networking restart"
    else
        echo "${output}"
        echo "${worker} ${airlock_interface} => OK"
    fi
    echo "---"
done
