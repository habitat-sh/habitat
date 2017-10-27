#!/bin/bash

set -eux

# Add a very uniquely named user/group pair for worker builds to run under.
sudo useradd --groups=tty --create-home krangschnak || echo "User 'krangschnak' already exists"
