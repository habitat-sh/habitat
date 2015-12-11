#!/usr/bin/env sh
#
# # Usage
#
# ```
# $ mount_filesystems.sh
# ```
#
# # Synopsis
#
# `mount_filesystems.sh` mounts several filesystems into the chroot environment
#
# # Environment Variables
#
# There are several enviroment variables that are used with this program:
#
# * `$CHROOT` (**Required**): The root directory of the chroot filesystem
# * `$DEBUG` (*Optional*): If set, the program will output the shell commands
#    as they are being executed
#
# # License and Copyright
#
# ```
# Copyright: Copyright (c) 2015 Chef Software, Inc.
# License: Apache License, Version 2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ```
#
#

# # Main program

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
fi

mkdir -pv $CHROOT/dev
mkdir -pv $CHROOT/proc
mkdir -pv $CHROOT/sys
mkdir -pv $CHROOT/run

# Make  a `/dev/console` device, if it doesn't exist
if [ ! -c $CHROOT/dev/console ]; then
  mknod -m 600 $CHROOT/dev/console c 5 1
fi
# Make  a `/dev/null` device, if it doesn't exist
if [ ! -c $CHROOT/dev/null ]; then
  mknod -m 666 $CHROOT/dev/null c 1 3
fi

# Unless `$NO_MOUNT` is set, mount filesystems such as `/dev`, `/proc`, and
# company. If the mount already exists, skip it to be all idempotent and nerdy
# like that
if [ -z "${NO_MOUNT:-}" ]; then
  if ! mount | grep -q "on $CHROOT/dev type"; then
    mount -v --bind /dev $CHROOT/dev
  fi

  if ! mount | grep -q "on $CHROOT/dev/pts type"; then
    mount -vt devpts devpts $CHROOT/dev/pts -o gid=5,mode=620
  fi
  if ! mount | grep -q "on $CHROOT/proc type"; then
    mount -vt proc proc $CHROOT/proc
  fi
  if ! mount | grep -q "on $CHROOT/sys type"; then
    mount -vt sysfs sysfs $CHROOT/sys
  fi
  if ! mount | grep -q "on $CHROOT/run type"; then
    mount -vt tmpfs tmpfs $CHROOT/run
  fi

  if [ -h $CHROOT/dev/shm ]; then
    mkdir -pv $CHROOT/`readlink $CHROOT/dev/shm`
  fi
fi

# To be continued? Nope!
exit 0
