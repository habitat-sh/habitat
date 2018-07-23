#!/bin/bash
#
# # Usage
#
# ```sh
# $ publish-studio
# ```
#
# See the `print_help()` function below for complete usage instructions.
#
# # License and Copyright
#
# ```
# Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
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

# # Internals

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi


# ## Help/Usage functions

# **Internal** Prints help and usage information. Straight forward, no?
print_help() {
  echo -- "$program $version

$author

Habitat Studio Docker Image Bintray Publisher

USAGE:
        $program [FLAGS] [OPTIONS]

COMMON FLAGS:
    -h  Prints this message
    -o  Specify the Bintray organization to publish to (default: habitat)
    -r  Specify the Bintray repo to publish to (default: stable)
    -V  Prints version information
"
}


# **Internal** Check that the command exists, 0 if it does, 1 if it does not.
#
# ```sh
# exists gsha256sum
# ```
#
# Would return 0 if gsha256sum exists, 1 if it does not.
exists() {
  if command -v "$1" >/dev/null 2>&1
  then
    return 0
  else
    return 1
  fi
}

# **Internal** Exit the program with an error message and a status code.
#
# ```
# exit_with "Something bad went down" 55
# ```
exit_with() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      echo -e "\033[1;31mERROR: \033[1;37m$1\033[0m"
      ;;
    *)
      echo "ERROR: $1"
      ;;
  esac
  exit "$2"
}

# **Internal** Ensures that the correct versions of key system commands are
# able to be used by this program. If we cannot find suitable versions, we will
# abort early.
#
# The following variables are set which contain an absolute path to the desired
# command:
#
# * `$_docker_cmd` (docker on system)
#
# Note that all of the commands noted above are considered internal
# implementation details and are subject to change with little to no notice,
# which means the variables such as `$_tar_cmd` should **not** be used directly
# by Plan authors. The leading underscore denotes an internal/private variable
# or function.

# If the commands are not found, `exit_with` is called and the program is
# terminated.
_find_system_commands() {
  if exists docker; then
    _docker_cmd=$(command -v docker)
  else
    exit_with "We require docker to push the image; aborting" 1
  fi
}

# **Internal** Print a line of output. Takes the rest of the line as its only
# argument.
#
# ```sh
# info "Running command"
# ```
info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;36m%s: \033[1;37m%s\033[0m\n" "${program:-unknown}" "${1:-}"
      ;;
    *)
      printf -- "   %s: %s\n" "${program:-unknown}" "${1:-}"
      ;;
  esac
  return 0
}

cleanup() {
    original_exit="${?}"
    rm -f "${HOME}/.docker/config.json"
    exit "${original_exit}"
}

# **Internal** Main program.
# shellcheck disable=2120,2154
_main() {
  build-docker-image
  if [[ ! -f ./results/last_image.env ]]; then
    exit_with "Image build report ./results/last_image.env missing, aborting" 5
  fi
  source ./results/last_image.env

  info "Logging in to Bintray Docker repo"
  docker login -u="$BINTRAY_USER" -p="$BINTRAY_KEY" habitat-docker-registry.bintray.io
  trap cleanup INT TERM EXIT

  info "Pushing ${docker_image}:$docker_image_version"
  docker push "${docker_image}:$docker_image_version"
  info "Pushing ${docker_image}:$docker_image_short_version tag for $docker_image_version"
  docker push "${docker_image}:$docker_image_short_version"
  info "Pushing latest tag for $docker_image_version"
  docker push "${docker_image}:latest"

  info
  info "Docker Image: docker pull ${docker_image}"
  info
}


# # Main Flow

# ## Default variables

BINTRAY_ORG=habitat
BINTRAY_REPO=stable
# shellcheck disable=2123
PATH=@path@

# The current version of this program
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$(basename "$0")

# ## CLI Argument Parsing

# Parse command line flags and options.
while getopts "o:r:Vh" opt; do
  case $opt in
    o)
      BINTRAY_ORG=$OPTARG
      ;;
    r)
      BINTRAY_REPO=$OPTARG
      ;;
    V)
      echo "$program $version"
      exit 0
      ;;
    h)
      print_help
      exit 0
      ;;
    \?)
      print_help
      exit_with "Invalid option: -$OPTARG" 1
      ;;
  esac
done
# Shift off all parsed token in `$*` so that the subcommand is now `$1`.
shift "$((OPTIND - 1))"

if [[ -z "${BINTRAY_USER:-}" ]]; then
  print_help
  exit_with "Required environment variable: BINTRAY_USER" 2
fi
if [[ -z "${BINTRAY_KEY:-}" ]]; then
  print_help
  exit_with "Required environment variable: BINTRAY_KEY" 2
fi

_find_system_commands
# shellcheck disable=2119
_main
