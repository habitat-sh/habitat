# shellcheck disable=2154
source ../plan.sh

pkg_name=hab-bintray-publish
pkg_origin=core
pkg_version=$(cat "$SRC_PATH/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_deps=(core/coreutils
          core/util-linux
          core/bash
          core/zip
          core/tar
          core/xz
          core/gzip
          core/jfrog-cli
          core/grep
          core/findutils
          core/gawk
          core/hab)
pkg_bin_dirs=(bin)
