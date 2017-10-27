#!/bin/bash

set -eux

# Add a very uniquely named user/group pair for worker builds to run under.
sudo adduser --group sparkleparty || echo "Group 'sparkleparty' already exists"
sudo useradd -g sparkleparty --groups=tty --create-home krangschnak || echo "User 'krangschnak' already exists"
