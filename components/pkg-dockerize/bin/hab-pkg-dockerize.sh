#!/bin/bash
#
# # Usage
#
# ```
# $ hab-pkg-dockerize [--repo <repo>] [--push] <PKG_IDENT>
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

# parse the CLI flags and options
parse_options() {

  if [[ -z "$@" ]]; then
    # no args given, bail out
    print_help
    exit_with "You must specify one or more Habitat packages to Dockerize." 1
  fi

  while test $# -gt 0; do
    case "$1" in
      -h|--help)
        print_help
        exit
        ;;
      --push)
        DOCKER_PUSH="1"
        shift
        ;;
      --repo*)
        DOCKER_REGISTRY_URL="${1#*=}"
        if [[ "$DOCKER_REGISTRY_URL" == "--repo" ]]; then
          shift
          DOCKER_REGISTRY_URL="$1"
        fi
        shift
        ;;
      *)
        PKG="$1"
        break
        ;;
    esac
  done

  if [ "$PKG" == "unknown" ]; then
    print_help
    exit_with "You must specify one or more Habitat packages to Dockerize." 1
  fi
}

# ## Help

# **Internal** Prints help
print_help() {
  printf -- "$program $version

$author

Habitat Package Dockerize - Create a Docker container from a set of Habitat packages

USAGE:
  $program [--repo <repo>] [--push] <PKG_IDENT>

FLAGS:
    --help           Prints help information

OPTIONS:
    --repo=URL       If given, prefix the tag with the URL
    --push           Push the built images to the configured repository

ARGS:
    <PKG_IDENT>      Habitat package identifier (ex: acme/redis)

EXAMPLE:
    $program --repo docker.private.com:443 --push core/nginx

    Would create and push the following images/tags:

    docker.private.com:443/core/nginx                     1.10.1-20161207041036   380c892f61ab        2 minutes ago       183.7 MB
    docker.private.com:443/core/nginx                     latest                  380c892f61ab        2 minutes ago       183.7 MB
    docker.private.com:443/core/habitat_export_base       43691665acf57304        e965f9e803c4        13 hours ago        181.9 MB
    docker.private.com:443/core/nginx_base                43691665acf57304        e965f9e803c4        13 hours ago        181.9 MB

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
      exit_with "We require GNU mktemp to build docker images; aborting" 1
    fi
  fi
}

# Add a trailing slash to the first argument ($1)
add_trailing_slash() {
  STR="$1"
  length=${#STR}
  last_char=${STR:length-1:1}
  [[ $last_char != "/" ]] && STR="$STR/"; :
  echo $STR
}

# Wraps `dockerfile` to ensure that a Docker image build is being executed in a
# clean directory with native filesystem permissions which is outside the
# source code tree.
build_docker_image() {
  local ident_file="$(hab pkg path $PKG)/IDENT"
  if [[ ! -f "$ident_file" ]]; then
    hab pkg install $PKG # try to install it
    ident_file="$(hab pkg path $PKG)/IDENT"
  fi

  if [[ -n "$DOCKER_REGISTRY_URL" ]]; then
    DOCKER_REGISTRY_URL=$(add_trailing_slash "$DOCKER_REGISTRY_URL")
  fi

  pkg_name=$(package_name_for $PKG)
  pkg_origin=$(package_origin_for $ident_file)
  pkg_ident=$(package_ident_for $ident_file)
  pkg_version=$(version_num_for $ident_file)

  BASE_PKGS=$(base_pkgs $PKG)
  DOCKER_BASE_TAG="${DOCKER_REGISTRY_URL}${pkg_ident}_base:$(base_pkg_hash $BASE_PKGS)"
  DOCKER_BASE_TAG_ALT="${DOCKER_REGISTRY_URL}${pkg_origin}/habitat_export_base:$(base_pkg_hash $BASE_PKGS)"

  # create base layer image
  DOCKER_CONTEXT="$($_mktemp_cmd -t -d "${program}-XXXX")"
  pushd $DOCKER_CONTEXT > /dev/null
  docker_base_image $PKG
  popd > /dev/null
  rm -rf "$DOCKER_CONTEXT"


  DOCKER_RUN_TAG="${DOCKER_REGISTRY_URL}${pkg_ident}"

  # build runtime image
  DOCKER_CONTEXT="$($_mktemp_cmd -t -d "${program}-XXXX")"
  pushd $DOCKER_CONTEXT > /dev/null
  docker_image $PKG
  popd > /dev/null
  rm -rf "$DOCKER_CONTEXT"
}

package_origin_for() {
  local ident_file="$1"
  cat $ident_file | awk 'BEGIN { FS = "/" }; { print $1 }'
}

package_ident_for() {
  local ident_file="$1"
  cat $ident_file | awk 'BEGIN { FS = "/" }; { print $1 "/" $2 }'
}

package_name_for() {
  local pkg="$1"
  echo $(echo $pkg | cut -d "/" -f 2)
}

package_exposes() {
  local pkg="$1"
  local expose_file="$(hab pkg path $pkg)/EXPOSES"
  if [ -f "$expose_file" ]; then
    cat $expose_file
  fi
}

version_num_for() {
  local ident_file="$1"
  cat $ident_file | awk 'BEGIN { FS = "/" }; { print $3 "-" $4 }'
}

# Collect all dependencies for the requested package
base_pkgs() {
  local BUILD_PKGS="$@"
  for p in $BUILD_PKGS; do
    hab pkg install $p >/dev/null
    cat $(hab pkg path $p)/DEPS >> /tmp/_all_deps
  done
  (cat /tmp/_all_deps | sort | uniq) && rm -f /tmp/_all_deps
}

base_pkg_hash() {
  echo "$@" | sha256sum | cut -b1-16
}

docker_base_image() {

  if [[ -n "$(docker images -q $DOCKER_BASE_TAG 2> /dev/null)" ]]; then
    # image already exists
    echo ">> base docker image: $DOCKER_BASE_TAG already built; skipping rebuild"
    return 0;
  fi

  if [[ -n "$(docker images -q $DOCKER_BASE_TAG_ALT 2> /dev/null)" ]]; then
    # image already exists
    echo ">> base docker image: $DOCKER_BASE_TAG_ALT already built; skipping rebuild"
    # create a tag alias for our package
    docker tag $DOCKER_BASE_TAG_ALT $DOCKER_BASE_TAG
    DOCKER_BASE_TAG="$DOCKER_BASE_TAG_ALT"
    return 0;
  fi

  echo ">> base docker image: building..."

  env PKGS="$BASE_PKGS" NO_MOUNT=1 hab-studio -r $DOCKER_CONTEXT/rootfs -t baseimage new
  echo "$1" > $DOCKER_CONTEXT/rootfs/.hab_pkg

  # create base image Dockerfile
  cat <<EOT > $DOCKER_CONTEXT/Dockerfile
FROM scratch
ENV $(cat $DOCKER_CONTEXT/rootfs/init.sh | grep PATH= | cut -d' ' -f2-)
WORKDIR /
ADD rootfs /
EOT

  docker build --force-rm --no-cache -t $DOCKER_BASE_TAG .
  docker tag $DOCKER_BASE_TAG $DOCKER_BASE_TAG_ALT

  echo ">> base docker image: built $DOCKER_BASE_TAG and $DOCKER_BASE_TAG_ALT"
}

docker_image() {
  echo ">> docker image: building..."

  local pkg_file=$(ls /hab/cache/artifacts/$(cat $(hab pkg path $pkg_ident)/IDENT | tr '/' '-')-*)
  cp -a $pkg_file $DOCKER_CONTEXT/
  pkg_file=$(basename $pkg_file)

  cat <<EOT > $DOCKER_CONTEXT/Dockerfile
FROM ${DOCKER_BASE_TAG}
COPY ${pkg_file} /tmp/
RUN hab pkg install /tmp/${pkg_file} && rm -f /tmp/${pkg_file}

RUN mkdir -p $HAB_ROOT_PATH/svc/${pkg_name}/data \
             $HAB_ROOT_PATH/svc/${pkg_name}/config \
      && chown -R 42:42 $HAB_ROOT_PATH/svc/${pkg_name}
VOLUME $HAB_ROOT_PATH/svc/${pkg_name}/data $HAB_ROOT_PATH/svc/${pkg_name}/config
EXPOSE 9631 $(package_exposes $1)
ENTRYPOINT ["/init.sh"]
CMD ["start", "$1"]
EOT

  docker build --force-rm --no-cache -t "${DOCKER_RUN_TAG}:${pkg_version}" .
  docker tag "${DOCKER_RUN_TAG}:${pkg_version}" "${DOCKER_RUN_TAG}:latest"

  echo ">> docker image: built ${DOCKER_RUN_TAG}:${pkg_version} and ${DOCKER_RUN_TAG}:latest"
}

# Push the built docker images to the configured registry
push_docker_image() {
  if [[ "$DOCKER_PUSH" != "1" ]]; then
    return 0;
  fi

  local repo="$DOCKER_REGISTRY_URL"
  if [[ -z "$repo" ]]; then
    repo="public (docker.io)"
  fi
  echo ">> pushing to registry: $repo"

  # push images/tags we created to registry
  docker push $DOCKER_BASE_TAG
  docker push $DOCKER_BASE_TAG_ALT
  docker push "${DOCKER_RUN_TAG}:${pkg_version}"
  docker push "${DOCKER_RUN_TAG}:latest"
}

# The root of the filesystem. If the program is running on a separate
# filesystem or chroot environment, this environment variable may need to be
# set.
: ${FS_ROOT:=}
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: ${HAB_ROOT_PATH:=$FS_ROOT/hab}

# Set a default docker registry url (empty)
# If set, images will be tagged with this prefix
: ${DOCKER_REGISTRY_URL:=""}

# Controls whether or not we push to the configured registry
: ${DOCKER_PUSH:="0"}

# The package to dockerize
: ${PKG:="unknown"}

# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$(basename $0)

find_system_commands

parse_options $@
build_docker_image
push_docker_image
