#!/bin/sh
set -eux

sudo apt-get update

# Install gcc and dev tools
sudo apt-get install -y --no-install-recommends \
  build-essential

# Install habitat
export HAB_LICENSE="accept-no-persist"
curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash
