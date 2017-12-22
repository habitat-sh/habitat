#!/bin/bash

# NOTE: This script assumes you've set up your SSH configuration
# using the scripts in the `ssh_helpers/` directory!
#
# Currently, the ens4 network interface on our workers
# appears to get lost for an as-yet-undetermined reason. The interface
# can legitimately "disappear" when it has been used in the network
# namespace of a running build, but it should be returned afterward.
#
# If you suspect the interfaces have really been lost (e.g., there
# aren't any builds running on the worker, but there's no ens4
# interface present), you can use this script to restart the network
# stack on the offending machines.
#
# To simply probe workers, but perform no restarts, do like so:
#
#     probe-workers.sh live
#
# To then restart the networking stacks of machines without an
# interface, add the -r option:
#
#     probe-workers.sh -r live
#
restart="false"
while getopts ":r" opt; do
  case ${opt} in
    r)
      restart=true
      ;;
  esac
done
shift "$((OPTIND - 1))"

environment=${1}

airlock_interface="ens4"

for worker in $(grep "Host ${environment}-builder-worker" ~/.ssh/config | awk '{print $2}' | sort); do
    echo "Worker ${worker}"
    output=$(ssh ${worker} "ip address show dev ${airlock_interface} | grep 'inet '")

    if [ -z "$output" ]; then
        echo "No ${airlock_interface} interface detected!"
        if [[ "${restart}" == "true" ]]; then
            echo "--> Restarting network stack"
            ssh ${worker} "sudo /etc/init.d/networking restart"
        fi
    else
        echo "${output}"
        echo "${worker} ${airlock_interface} => OK"
    fi
    echo "---"
done
