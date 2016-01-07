#!/usr/bin/env sh
#
# # Usage
#
# ```
# $ create_tools_symlinks.sh
# ```
#
# # Synopsis
#
# `create_tools_symlinks.sh` creates symlinks from the minimal toolchain
# installed under `/tools` into the root of the chroot environment. This is
# done to satisfy tools such as `make(1)` which expect `/bin/sh` to exist.
#
# # Environment Variables
#
# There are several enviroment variables that are used with this program:
#
# * `$CHROOT` (*Optional*): The root directory of the chroot filesystem. If
#    you are running this program outside of a chrooted environment, you
#    must provide this variable.
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

# If `$CHROOT` is set, it will be prefixed before every path, and otherwise
# the paths will be absolute paths in the current environment
root="${CHROOT:-}"

# If the `/tools` software isn't present, there is no need continuing, so
# we'll bail instead!
if [ ! -d "$root/tools" ]; then
  echo ">>> No directory $root/tools present, quitting"
  exit 1
fi

ln -sfv /tools/bin/bash $root/bin
ln -sfv /tools/bin/cat $root/bin
ln -sfv /tools/bin/echo $root/bin
ln -sfv /tools/bin/pwd $root/bin
ln -sfv /tools/bin/stty $root/bin

ln -sfv /tools/bin/perl $root/usr/bin
ln -sfv /tools/lib/libgcc_s.so $root/usr/lib
ln -sfv /tools/lib/libgcc_s.so.1 $root/usr/lib
ln -sfv /tools/lib/libstdc++.so $root/usr/lib
ln -sfv /tools/lib/libstdc++.so.6 $root/usr/lib
sed 's/tools/usr/' $root/tools/lib/libstdc++.la > $root/usr/lib/libstdc++.la
ln -sfv bash $root/bin/sh

# That's all folks!
exit 0
