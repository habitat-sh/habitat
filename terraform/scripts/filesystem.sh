#!/bin/bash
set -eux

sudo mkfs.ext4 /dev/xvdf
sudo mount /dev/xvdf /mnt
echo '/dev/xvdf /hab     ext4   defaults 0 0' | sudo tee -a /etc/fstab
sudo mkdir -p /mnt/hab
sudo ln -s /mnt/hab /hab
