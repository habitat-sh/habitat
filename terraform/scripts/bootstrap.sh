#!/bin/bash
set -ex

# Create an hab user
adduser --user-group hab || echo "User 'hab' already exists"

# Install hab
yum install -y wget
bash /home/ec2-user/hab-install.sh

# Install the SystemD unit
cp /home/ec2-user/sh.habitat.habitat-builder-web.service /etc/systemd/system
systemctl daemon-reload
systemctl start sh.habitat.habitat-builder-web
systemctl enable sh.habitat.habitat-builder-web

# Apply the configuration
hab config apply habitat-builder-web.default 1 /home/ec2-user/habitat-builder-web.toml
