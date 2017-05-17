#!/bin/bash
set -eux

sudo mkfs.ext4 /dev/nvme0n1
sudo mount /dev/nvme0n1 /mnt
echo '/dev/nvme0n1 /hab     ext4   defaults 0 0' | sudo tee -a /etc/fstab
sudo mkdir -p /mnt/hab
sudo ln -s /mnt/hab /hab
