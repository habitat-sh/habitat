#!/bin/sh
set -eux

# Install Docker to run bats tests
# From: https://docs.docker.com/install/linux/docker-ce/ubuntu/#install-using-the-repository
#
sudo apt-get install -y --no-install-recommends \
    apt-transport-https \
    ca-certificates \
    curl \
    software-properties-common

curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository \
  "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) \
  stable"

sudo -E apt-get install -y --no-install-recommends \
  docker-ce

# if running on a Vagrantbox, add the vagrant user
# as well as the current user
if getent passwd vagrant > /dev/null 2>&1; then
  sudo -E usermod -aG docker vagrant
fi
sudo -E usermod -aG docker "${USER}"
