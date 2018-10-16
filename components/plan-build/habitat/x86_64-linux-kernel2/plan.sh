source ../plan.sh

pkg_name=hab-plan-build
pkg_origin=core
pkg_version=$(cat "$SRC_PATH/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/bash
          core/binutils
          core/bzip2
          core/coreutils
          core/file
          core/findutils
          core/gawk
          core/grep
          core/gzip
          core/hab
          core/rq
          core/sed
          core/tar
          core/unzip
          core/wget
          core/xz)
pkg_build_deps=(core/bats)
