#!/bin/bash
#
# # Usage
#
# ```
# $ hab-pkg-dockerize [PKG ...]
# ```
#
# # Synopsis
#
# Create a Docker container from a set of Habitat packages.
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

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

# ## Help

# **Internal** Prints help
print_help() {
  echo -- "$program $version

$author

Habitat Package Dockerize - Create a Docker container from a set of Habitat packages

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
    printf -- "ERROR: %s\n" "$1"
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

# Wraps `dockerfile` to ensure that a Docker image build is being executed in a
# clean directory with native filesystem permissions which is outside the
# source code tree.
build_docker_image() {
  DOCKER_CONTEXT="$($_mktemp_cmd -t -d "${program}-XXXX")"
  pushd "$DOCKER_CONTEXT" > /dev/null
  docker_image "$@"
  popd > /dev/null
  rm -rf "$DOCKER_CONTEXT"
}

package_name_for() {
  local pkg="$1"
  echo "$pkg" | cut -d "/" -f 2
}

package_exposes() {
  local pkg="$1"
  local expose_file
  expose_file=$(find "$DOCKER_CONTEXT"/rootfs/"$HAB_ROOT_PATH"/pkgs/"$pkg" -name EXPOSES)
  if [ -f "$expose_file" ]; then
    cat "$expose_file"
  fi
}

package_version_tag() {
  local pkg="$1"
  local ident_file
  ident_file=$(find "$DOCKER_CONTEXT"/rootfs/"$HAB_ROOT_PATH"/pkgs/"$pkg" -name IDENT)
  awk 'BEGIN { FS = "/" }; { print $1 "/" $2 ":" $3 "-" $4 }' < "$ident_file"
}

package_latest_tag() {
  local pkg="$1"
  local ident_file
  ident_file=$(find "$DOCKER_CONTEXT"/rootfs/"$HAB_ROOT_PATH"/pkgs/"$pkg" -name IDENT)
  awk 'BEGIN { FS = "/" }; { print $1 "/" $2 ":latest" }' < "$ident_file"
}

package_latest_path() {
  local pkg="$1"
  local ident_file
  ident_file=$(find "$DOCKER_CONTEXT"/rootfs/"$HAB_ROOT_PATH"/pkgs/"$pkg" -name IDENT)
  echo "$HAB_ROOT_PATH"/pkgs/"$(cat "$ident_file")"
}

docker_image() {
  env PKGS="$*" NO_MOUNT=1 hab-studio -r "$DOCKER_CONTEXT"/rootfs -t baseimage new
  local pkg_name
  pkg_name=$(package_name_for "$1")
  local version_tag
  version_tag=$(package_version_tag "$1")
  local latest_tag
  latest_tag=$(package_latest_tag "$1")
  echo "$1" > "$DOCKER_CONTEXT"/rootfs/.hab_pkg
  cat <<EOT > "$DOCKER_CONTEXT"/Dockerfile
FROM scratch
ENV $(grep PATH= "$DOCKER_CONTEXT"/rootfs/init.sh | cut -d' ' -f2-)
WORKDIR /
ADD rootfs /
VOLUME $HAB_ROOT_PATH/svc/${pkg_name}/data $HAB_ROOT_PATH/svc/${pkg_name}/config
EXPOSE 9631 $(package_exposes "$1")
RUN ["ln", "-s", "$(package_latest_path core/cacerts)/ssl", "/etc/"]
ENTRYPOINT ["/init.sh"]
CMD ["start", "$1"]
EOT
  # Docker tags downcased via ${string,,}
  # https://www.gnu.org/software/bash/manual/bashref.html#Shell-Parameter-Expansion
  docker build --force-rm --no-cache -t "${version_tag,,}" .
  docker tag "${version_tag,,}" "${latest_tag,,}"
}

# The root of the filesystem. If the program is running on a separate
# filesystem or chroot environment, this environment variable may need to be
# set.
: "${FS_ROOT:=}"
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: "${HAB_ROOT_PATH:=$FS_ROOT/hab}"

# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$(basename "$0")

find_system_commands

if [ -z "$*" ]; then
  print_help
  exit_with "You must specify one or more Habitat packages to Dockerize." 1
elif [ "$*" == "--help" ]; then
  print_help
else
  build_docker_image "$@"
fi
