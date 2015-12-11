#!/usr/bin/env sh
#
# # Usage
#
# ```
# $ create_directories.sh
# ```
#
# # Synopsis
#
# `create_directories.sh` creates a base filesystem layout for the chroot
# environment.
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

# If `$CHROOT` is set, it will be prefixed before every directory path, and
# otherwise the directories will be absolute paths in the current environment
root="${CHROOT:-}"

mkdir -pv $root/bin
mkdir -pv $root/boot
mkdir -pv $root/etc/opt
mkdir -pv $root/etc/sysconfig
mkdir -pv $root/home
mkdir -pv $root/lib/firmware
mkdir -pv $root/mnt
mkdir -pv $root/opt

mkdir -pv $root/media/floppy
mkdir -pv $root/media/cdrom
mkdir -pv $root/sbin
mkdir -pv $root/srv
mkdir -pv $root/var

install -dv -m 0750 $root/root
install -dv -m 1777 $root/tmp $root/var/tmp

mkdir -pv $root/usr/bin
mkdir -pv $root/usr/include
mkdir -pv $root/usr/lib
mkdir -pv $root/usr/sbin
mkdir -pv $root/usr/src
mkdir -pv $root/usr/local/bin
mkdir -pv $root/usr/local/include
mkdir -pv $root/usr/local/lib
mkdir -pv $root/usr/local/sbin
mkdir -pv $root/usr/local/src

mkdir -pv $root/usr/share/color
mkdir -pv $root/usr/share/dict
mkdir -pv $root/usr/share/doc
mkdir -pv $root/usr/share/info
mkdir -pv $root/usr/share/locale
mkdir -pv $root/usr/share/man
mkdir -pv $root/usr/local/share/color
mkdir -pv $root/usr/local/share/dict
mkdir -pv $root/usr/local/share/doc
mkdir -pv $root/usr/local/share/info
mkdir -pv $root/usr/local/share/locale
mkdir -pv $root/usr/local/share/man

mkdir -pv $root/usr/share/misc
mkdir -pv $root/usr/share/terminfo
mkdir -pv $root/usr/share/zoneinfo
mkdir -pv $root/usr/local/share/misc
mkdir -pv $root/usr/local/share/terminfo
mkdir -pv $root/usr/local/share/zoneinfo

mkdir -pv $root/usr/libexec

mkdir -pv $root/usr/share/man/man1
mkdir -pv $root/usr/share/man/man2
mkdir -pv $root/usr/share/man/man3
mkdir -pv $root/usr/share/man/man4
mkdir -pv $root/usr/share/man/man5
mkdir -pv $root/usr/share/man/man6
mkdir -pv $root/usr/share/man/man7
mkdir -pv $root/usr/share/man/man8
mkdir -pv $root/usr/local/share/man/man1
mkdir -pv $root/usr/local/share/man/man2
mkdir -pv $root/usr/local/share/man/man3
mkdir -pv $root/usr/local/share/man/man4
mkdir -pv $root/usr/local/share/man/man5
mkdir -pv $root/usr/local/share/man/man6
mkdir -pv $root/usr/local/share/man/man7
mkdir -pv $root/usr/local/share/man/man8

# If the system is 64-bit, a few symlinks will be required
case $(uname -m) in
x86_64)
  ln -sfv lib $root/lib64
  ln -sfv lib $root/usr/lib64
  ln -sfv lib $root/usr/local/lib64
  ;;
esac

mkdir -pv $root/var/log
mkdir -pv $root/var/mail
mkdir -pv $root/var/spool

ln -sfv /run $root/var/run
ln -sfv /run/lock $root/var/lock

mkdir -pv $root/var/opt
mkdir -pv $root/var/cache
mkdir -pv $root/var/lib/color
mkdir -pv $root/var/lib/misc
mkdir -pv $root/var/lib/locate
mkdir -pv $root/var/local

# So long, thank for all the fish!
exit 0
