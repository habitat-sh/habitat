#!/bin/bash
set -eu
if [ -n "${DEBUG:-}" ]; then set -x; fi

sudo apt-get install curl -y
sudo adduser --group hab || echo "Group 'hab' already exists"
sudo useradd -g hab hab || echo "User 'hab' already exists"

curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
sudo hab pkg install core/hab-sup
