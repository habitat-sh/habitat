#!/bin/bash
#
# # Usage
#
# ```sh
# $ build-docker-image.sh [ARTIFACT_OR_PKG_IDENT ...]
# ```
#
# # Synopsis
#
# This program will build a `habitat/studio` Docker image using one or more
# local Habitat artifacts and/or package identifiers as arguments. Two
# packages must be installed or the program will terminate early:
#
# * `core/hab`
# * `core/hab-studio`
#
# A default usage installs both of the above packages from the Depot:
#
# ```sh
# ./build-docker-image.sh
# ```
#
# However, offline/local Habitat artifact files can be used instead, for
# example:
#
# ```sh
# ./build-docker-image.sh core/hab core/hab-studio
# ./build-docker-image.sh ./results/core-hab-{static,studio}-*.hart
# ```

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;36m$(basename $0): \033[1;37m${1:-}\033[0m\n"
      ;;
    *)
      printf -- "   $(basename $0): ${1:-}\n"
      ;;
  esac
  return 0
}

if ! command -v hab >/dev/null; then
  >&2 echo "   $(basename $0): WARN 'hab' command must be present on PATH, aborting"
  exit 9
fi

IMAGE_NAME=habitat-docker-registry.bintray.io/studio

start_dir="$(pwd)"
tmp_root="$(mktemp -d -t "$(basename $0)-XXXX")"
trap 'rm -rf $tmp_root; exit $?' INT TERM EXIT

export FS_ROOT="$tmp_root/rootfs"

info "Installing and extracting initial Habitat packages"
default_pkgs="core/hab core/hab-studio"
hab pkg install ${*:-$default_pkgs}
if ! hab pkg path core/hab >/dev/null 2>&1; then
  >&2 echo "   $(basename $0): WARN core/hab must be installed, aborting"
  exit 1
fi
if ! hab pkg path core/hab-studio >/dev/null 2>&1; then
  >&2 echo "   $(basename $0): WARN core/hab-studio must be installed, aborting"
  exit 2
fi

info "Putting \`hab' in container PATH"
hab pkg binlink core/hab hab
info "Purging container hab cache"
rm -rf $FS_ROOT/hab/cache

ident="$(hab pkg path core/hab-studio | rev | cut -d '/' -f 1-4 | rev)"
short_version=$(echo $ident | awk -F/ '{print $3}')
version=$(echo $ident | awk -F/ '{print $3 "-" $4}')

cat <<EOF > $tmp_root/Dockerfile
FROM busybox:latest
MAINTAINER The Habitat Maintainers <humans@habitat.sh>
ADD rootfs /
WORKDIR /src
RUN env NO_MOUNT=true hab studio new \
  && rm -rf /hab/studios/src/hab/cache/artifacts
ENTRYPOINT ["/bin/hab", "studio"]
EOF
cd $tmp_root

info "Building Docker image \`${IMAGE_NAME}:$version'"
docker build --no-cache -t ${IMAGE_NAME}:$version .

info "Tagging latest image to ${IMAGE_NAME}:$version"
docker tag ${IMAGE_NAME}:$version ${IMAGE_NAME}:latest

info "Tagging latest image to ${IMAGE_NAME}:$short_version"
docker tag ${IMAGE_NAME}:$version ${IMAGE_NAME}:$short_version

cat <<-EOF > "$start_dir/results/last_image.env"
docker_image=$IMAGE_NAME
docker_image_version=$version
docker_image_short_version=$short_version
EOF

info
info "Docker Image: ${IMAGE_NAME}:$version"
info "Build Report: $start_dir/results/last_image.env"
info
