#!/bin/bash
#
# # Usage
#
# ```
# $ hab-pkg-aci [PKG ...]
# ```
#
# # Synopsis
#
# Create a App Container Image from a set of Habitat packages.
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

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

ACBUILD="acbuild"
ACI_SIGN=""

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
  ACBUILD="acbuild --debug"
fi

# If the variable `$SIGN` is set, then we'll attempt to sign the ACI as we write it.
# The user can set additional arguments to gpg using the `$SIGN_ARGS` variable
if [ -n "${SIGN:-}" ]; then
  ACI_SIGN="--sign"
  if [ -n "${SIGN_ARGS:-}" ]; then
    ACI_SIGN="${ACI_SIGN} -- ${SIGN_ARGS}"
  fi
fi

# ## Help

# **Internal** Prints help
print_help() {
  echo -- "$program $version

$author

Habitat Package ACI - Create an App Container Image from a set of Habitat packages

USAGE:
  $program [PKG ..]
"
}

# **Internal** Exit the program with an error message and a status code.
#
# ```sh
# exit_with "Something bad went down" 55
# ```
exit_with() {
  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    echo -- "ERROR: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[1;31mERROR: \033[1;37m%s\033[0m\n" "$1"
        ;;
      *)
        printf -- "ERROR: %s\n" "$1"
        ;;
    esac
  fi
  exit "$2"
}

find_system_commands() {
  if mktemp --version 2>&1 | grep -q 'GNU coreutils'; then
    _mktemp_cmd=$(command -v mktemp)
  else
    if /bin/mktemp --version 2>&1 | grep -q 'GNU coreutils'; then
      _mktemp_cmd=/bin/mktemp
    else
      exit_with "We require GNU mktemp to build docker images; aborting" 1
    fi
  fi
}

package_name_for() {
  local pkg="$1"
  echo "$pkg" | cut -d "/" -f 2
}

package_exposes() {
  local pkg="$1"
  local expose_file
  expose_file=$(find "$ACI_CONTEXT"/rootfs/"$HAB_ROOT_PATH"/pkgs/"$pkg" -name EXPOSES)
  if [ -f "$expose_file" ]; then
    cat "$expose_file"
  fi
}

package_version_tag() {
  local pkg="$1"
  local ident_file
  ident_file=$(find "$ACI_CONTEXT"/rootfs/"$HAB_ROOT_PATH"/pkgs/"$pkg" -name IDENT)
  awk 'BEGIN { FS = "/" }; { print $1 "-" $2 "-" $3 "-" $4 }' < "$ident_file"
}

build_aci() {
  ACI_CONTEXT="$($_mktemp_cmd -t -d "${program}-XXXX")"
  BUILD_DIR="$($_mktemp_cmd -t -d "${program}-XXXX")"
  pushd "$BUILD_DIR" > /dev/null
  env PKGS="$*" NO_MOUNT=1 hab-studio -r "$ACI_CONTEXT"/rootfs -t baseimage new
  local pkg_name
  pkg_name=$(package_name_for "$1")
  local version_tag
  version_tag=$(package_version_tag "$1")
  echo "$1" > "$ACI_CONTEXT"/rootfs/.hab_pkg
  $ACBUILD begin "$ACI_CONTEXT"/rootfs
  $ACBUILD set-name "$1"
  # expose the habitat status endpoint by default
  $ACBUILD port add habitat tcp 9631
  while read -r expose; do
    $ACBUILD port add "${pkg_name}"-"${expose}" tcp "${expose}"
  done < <(package_exposes "$1")
  # mount habitat's data and config
  $ACBUILD mount add hab-data "$HAB_ROOT_PATH"/svc/"${pkg_name}"/data
  $ACBUILD mount add hab-config "$HAB_ROOT_PATH"/svc/"${pkg_name}"/config
  $ACBUILD environment add "$(grep PATH "$ACI_CONTEXT"/rootfs/init.sh)"
  $ACBUILD set-exec "/init.sh" start "$1"
  $ACBUILD write "$version_tag.aci" "${ACI_SIGN}"
  $ACBUILD end
  cp -a "$version_tag.aci" "$HAB_RESULTS_DIR"
  [ -f "$version_tag.aci.asc" ] && cp -a "$version_tag.aci.asc" "$HAB_RESULTS_DIR"
  popd > /dev/null
  rm -rf "$ACI_CONTEXT" "$BUILD_DIR"
}

# The root of the filesystem. If the program is running on a separate
# filesystem or chroot environment, this environment variable may need to be
# set.
: "${FS_ROOT:=}"
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: "${HAB_ROOT_PATH:=$FS_ROOT/hab}"
# Directory to write ACIs to. If $HAB_RESULTS_DIR environment variable
# is set, this value is overridden
: "${HAB_RESULTS_DIR:=/src/results}"

# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$(basename "$0")

find_system_commands

if [ -z "$*" ]; then
  print_help
  exit_with "You must specify one or more Habitat packages to create an ACI from." 1
elif [ "$*" == "--help" ]; then
  print_help
else
  build_aci "$@"
fi
