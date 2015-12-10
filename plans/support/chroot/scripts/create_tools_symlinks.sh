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

ln -sfv /proc/self/mounts $root/etc/mtab

touch $root/var/log/btmp
touch $root/var/log/lastlog
touch $root/var/log/wtmp
chgrp -v 13 $root/var/log/lastlog
chmod -v 664 $root/var/log/lastlog
chmod -v 600 $root/var/log/btmp

# If `/etc/passwd` is not present, create a minimal version to satisfy
# some software when being built
if [ ! -f "$root/etc/passwd" ]; then
  echo "> Creating minimal /etc/passwd"
  cat > $root/etc/passwd << "EOF"
root:x:0:0:root:/root:/bin/bash
bin:x:1:1:bin:/dev/null:/bin/false
daemon:x:6:6:Daemon User:/dev/null:/bin/false
messagebus:x:18:18:D-Bus Message Daemon User:/var/run/dbus:/bin/false
nobody:x:99:99:Unprivileged User:/dev/null:/bin/false
EOF
fi

# If `/etc/group` is not present, create a minimal version to satisfy
# some software when being built
if [ ! -f "$root/etc/group" ]; then
  echo "> Creating minimal /etc/group"
  cat > $root/etc/group << "EOF"
root:x:0:
bin:x:1:daemon
sys:x:2:
kmem:x:3:
tape:x:4:
tty:x:5:
daemon:x:6:
floppy:x:7:
disk:x:8:
lp:x:9:
dialout:x:10:
audio:x:11:
video:x:12:
utmp:x:13:
usb:x:14:
cdrom:x:15:
adm:x:16:
messagebus:x:18:
systemd-journal:x:23:
input:x:24:
mail:x:34:
nogroup:x:99:
users:x:999:
EOF
fi

# That's all folks!
exit 0
