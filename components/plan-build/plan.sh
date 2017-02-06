pkg_name=hab-plan-build
pkg_origin=core
pkg_version=$(cat "$PLAN_CONTEXT/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_source=nosuchfile.tar.gz
pkg_build_deps=()
pkg_bin_dirs=(bin)

pkg_deps=(
  core/bash
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
  core/xz
)

program=$pkg_name

do_build() {
  cp -v $PLAN_CONTEXT/bin/${program}.sh $program

  # Use the bash from our dependency list as the shebang. Also, embed the
  # release version of the program.
  sed \
    -e "s,#!/bin/bash$,#!$(pkg_path_for bash)/bin/bash," \
    -e "s,^HAB_PLAN_BUILD=.*$,HAB_PLAN_BUILD=$pkg_version/$pkg_release," \
    -i $program
}

do_install() {
  install -D $program $pkg_prefix/bin/$program
}

# Turn the remaining default phases into no-ops

do_prepare() {
  return 0
}

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}
