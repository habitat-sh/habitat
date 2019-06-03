#!/bin/bash 

set -euo pipefail 

studio_command="${1}"

sudo hab pkg install core/e2fsprogs

# Maximum directory name length is 255 characters so we need to create
# a nested set of directories to have a mount point with > 1024 characters. 
tmpdir=$(mktemp -d -p /tmp hab-studio-XXXXXXX)
directory="$(printf "a%.0s" {1..100})"
mnt_path="/mnt/$(printf "$directory/%.0s" {1..10})"

cleanup() { 
  sudo umount "$mnt_path" || true
  sudo rm -rf "$mnt_path"
  ( cd "$tmpdir"/studio && $studio_command rm )
  rm -rf "$tmpdir" 
}

trap cleanup EXIT

(
  cd "$tmpdir"

  # Create a tiny filesystem and mount it as a loopback device before we 
  # create our studio. It is important that this happens before we create the 
  # studio so that it appears first in /proc/mounts. The specific bug this is 
  # intended to detect (https://github.com/habitat-sh/habitat/issues/6591) 
  # won't be triggered if the studio mount entries are first. 
  dd if=/dev/zero of=empty-fs.img bs=10M count=1 
  hab pkg exec core/e2fsprogs mkfs.ext4 empty-fs.img
  sudo mkdir -p "$mnt_path"
  sudo mount -o loop "$tmpdir"/empty-fs.img "$mnt_path"

  mkdir studio 
  cd studio
  $studio_command new 
  # Print out the mount table using the system tools before removing it
  mount 
  $studio_command rm 
)
