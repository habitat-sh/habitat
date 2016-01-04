#!/usr/bin/env sh
#
# # Usage
#
# ```
# $ create_chroot.sh
# ```
#
# # Synopsis
#
# `create_chroot.sh` will created a self-contained, minimal environment in
# which we can develop, build, and package software that is free from any
# upstream operating system distribution. This program invokes several other
# programs in the hopes that the sub-components could be used independently.
#
# # Environment Variables
#
# There are several important enviroment variables that are required for
# this program and some that are optional:
#
# * `$CHROOT` (**Required**): The root directory of the chroot filesystem
# * `$PLANS` (**Required**): The directory containing the bldr Plan files
# * `$TOOLS` (*Optional*): The path to a `.tar.xz` file containing a minimal
#    "stage1" build toolchain which will be extracted into the chroot's
#    `/tools` directory
# * `$TOOLS_EXTRA` (*Optional*): The path to a `.tar.xz` file containing a
#    small number of ancillary programs required for the `bldr-build` program
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

# The path to the tools tarball, or empty if not set
tools="${TOOLS:-}"
# The path to the ancillary tools tarball, or empty if not set
tools_extra="${TOOLS_EXTRA:-}"

echo "==> Setting up CHROOT=$CHROOT"
echo "==> Mounting /plans from PLANS=$PLANS"
if [ -n "$tools" ]; then
  echo "==> Using /tools from $tools"
fi
if [ -n "$tools_extra" ]; then
  echo "==> Adding more content into /tools from $tools_extra"
fi

# Mount virtual filesystems and create the root directory structure
`dirname $0`/mount_filesystems.sh
`dirname $0`/create_filesystem.sh

# If `$tools` is set and the `/tools` directory doesn't appear to have been
# previously extracted, then extract! Finally, create some symlinks from
# the `/tools` programs into the root filesystem. This is done to satisfy
# tools such as `make(1)` which expect `/bin/sh` to exist--sad panda faces...
if [ -n "$tools" -a ! -x $CHROOT/tools/bin/bash ]; then
  echo "==> Extracting $tools into $CHROOT"
  xzcat $tools | tar xf - -C $CHROOT
  `dirname $0`/create_tools_symlinks.sh
fi

# If `$tools_extra` is set, then extract the tarball
if [ -n "$tools_extra" ]; then
  echo "==> Extracting $tools_extra into $CHROOT"
  xzcat $tools_extra | tar xf - -C $CHROOT
fi

# If `/plans` has not yet been mounted into the chroot filesystem, then do
# so--unless `$NO_MOUNT` has been set. The `$NO_MOUNT` variable is used
# by tooling such as `docker build` when permissions may not allow such a
# command to be executed. Note that we're bind-mounting the plans directory
# into the chroot, meaning it can be somewhere else on the filesystem.
if [ -z "${NO_MOUNT:-}" ] && ! mount | grep -q "on $CHROOT/plans type"; then
  echo "==> Mounting $PLANS as /plans"
  mkdir -pv $CHROOT/plans
  mount -v --bind -o ro $PLANS $CHROOT/plans
fi

# Copy minimal networking and DNS resolution configuration files into the
# chroot filesystem so that commands such as `wget(1)` will work
for f in /etc/hosts /etc/resolv.conf; do
  mkdir -pv `dirname $f`
  cp -v $f $CHROOT$f
done

echo "==> Finished creating CHROOT"

# Exit cleanly, thanks for playing
exit 0
