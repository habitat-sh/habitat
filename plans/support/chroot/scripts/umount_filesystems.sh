#!/usr/bin/env sh
#
# # Usage
#
# ```
# $ umount_filesystems.sh
# ```
#
# # Synopsis
#
# `umount_filesystems.sh` unmounts filesystems from the chroot environment
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

# Unmount filesystems that were set up in `mount_filesystems.sh`, but only
# if they are currently mounted. You know, so you can run this all day long,
# like, for fun and stuff.

if mount | grep -q "on $CHROOT/plans type"; then
  umount -v -l $CHROOT/plans
fi

if mount | grep -q "on $CHROOT/run type"; then
  umount -v $CHROOT/run
fi

if mount | grep -q "on $CHROOT/sys type"; then
  umount -v $CHROOT/sys
fi

if mount | grep -q "on $CHROOT/proc type"; then
  umount -v $CHROOT/proc
fi

if mount | grep -q "on $CHROOT/dev/pts type"; then
  umount -v $CHROOT/dev/pts
fi

if  mount | grep -q "on $CHROOT/dev type"; then
  umount -v -l $CHROOT/dev
fi

# Next time, on Batman and Friends...
exit 0
