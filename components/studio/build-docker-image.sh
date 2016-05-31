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
# * `core/hab-static`
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
# ./build-docker-image.sh core/hab-static core/hab-studio
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

if ! command -v hab >/dev/null; then
  >&2 echo "   $(basename $0): WARN 'hab' command must be present on PATH, aborting"
  exit 9
fi

tmp_root="$(mktemp -d -t "$(basename $0)-XXXX")"
trap 'rm -rf $tmp_root; exit $?' INT TERM EXIT

export FS_ROOT="$tmp_root/rootfs"

default_pkgs="core/hab-static core/hab-studio"
hab pkg install ${*:-$default_pkgs}
if ! hab pkg path core/hab-static >/dev/null 2>&1; then
  >&2 echo "   $(basename $0): WARN core/hab-static must be installed, aborting"
  exit 1
fi
if ! hab pkg path core/hab-studio >/dev/null 2>&1; then
  >&2 echo "   $(basename $0): WARN core/hab-studio must be installed, aborting"
  exit 2
fi

hab pkg binlink core/hab-static hab
rm -rf $FS_ROOT/hab/cache

ident="$(hab pkg path core/hab-studio | rev | cut -d '/' -f 1-4 | rev)"
version=$(echo $ident | awk -F/ '{print $3 "-" $4}')

cat <<EOF > $tmp_root/Dockerfile
FROM busybox:latest
MAINTAINER The Habitat Maintainers <humans@habitat.sh>
ADD rootfs /
WORKDIR /src
RUN env NO_MOUNT=true hab studio new && rm -rf /hab/studios/src/hab/cache
ENTRYPOINT ["/bin/hab", "studio"]
EOF
cd $tmp_root
docker build --no-cache -t habitat/studio:$version .
docker tag habitat/studio:$version habitat/studio:latest
