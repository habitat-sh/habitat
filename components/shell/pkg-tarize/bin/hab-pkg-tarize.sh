#!/bin/bash
#
# # Usage
#
# ```
# $ hab-pkg-tarize [PKG ...]
# ```
#
# # Synopsis
#
# Create an application archive from a set of Habitat packages.
#
# # License and Copyright
#
# ```
# Copyright: Copyright (c) 2016-2017 Chef Software, Inc.
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

# defaults for the application
: ${PKG:="unknown"}

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

# ## Help

# **Internal** Prints help
print_help() {
  printf -- "$program $version

$author

Habitat Package Tarize - Create an application archive from a set of Habitat packages

USAGE:
  $program [FLAGS] <PKG_IDENT>

FLAGS:
    --help           Prints help information

ARGS:
    <PKG_IDENT>      Habitat package identifier (ex: acme/redis)
"
}

# **Internal** Exit the program with an error message and a status code.
#
# ```sh
# exit_with "Something bad went down" 55
# ```
exit_with() {
  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    printf -- "ERROR: $1\n"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[1;31mERROR: \033[1;37m$1\033[0m\n"
        ;;
      *)
        printf -- "ERROR: $1\n"
        ;;
    esac
  fi
  exit $2
}

find_system_commands() {
  if $(mktemp --version 2>&1 | grep -q 'GNU coreutils'); then
    _mktemp_cmd=$(command -v mktemp)
  else
    if $(/bin/mktemp --version 2>&1 | grep -q 'GNU coreutils'); then
      _mktemp_cmd=/bin/mktemp
    else
      exit_with "We require GNU mktemp to build applications archives; aborting" 1
    fi
  fi
}

# parse the CLI flags and options
parse_options() {
  for i in "$@"
  do
    case $i in
      --help)
        print_help
        exit
        ;;
      *)
        PKG=${i}
        ;;
    esac
  done
  if [ "$PKG" == "unknown" ]; then
    print_help
    exit_with "You must specify one or more Habitat packages to archive." 1
  fi
}

# Create a hab studio baseimage and populate it with the application
build_tarball_image() {
  TARBALL_CONTEXT="$($_mktemp_cmd -t -d "${program}-XXXX")"
  pushd $TARBALL_CONTEXT > /dev/null
  env PKGS="$PKG" NO_MOUNT=1 hab studio -r $TARBALL_CONTEXT -t bare new
  echo $PKG > $TARBALL_CONTEXT/.hab_pkg
  popd > /dev/null
  tar -cpzf $(package_name_with_version $PKG).tar.gz -C $TARBALL_CONTEXT ./hab/pkgs ./hab/bin
}

package_name_with_version() {
  local ident_file=$(find $TARBALL_CONTEXT/$HAB_ROOT_PATH/pkgs/$PKG -name IDENT)
  cat $ident_file | awk 'BEGIN { FS = "/" }; { print $1 "-" $2 "-" $3 "-" $4 }'
}

# The root of the filesystem. If the program is running on a separate
# filesystem or chroot environment, this environment variable may need to be
# set.
: ${FS_ROOT:=}
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: ${HAB_ROOT_PATH:=$FS_ROOT/hab}

# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$(basename $0)

find_system_commands

parse_options $@
build_tarball_image
rm -rf $TARBALL_CONTEXT
