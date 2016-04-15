pkg_name=hab-plan-build
pkg_origin=chef
pkg_version=0.4.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_build_deps=()
pkg_bin_dirs=(bin)

pkg_deps=(
  chef/bash
  chef/binutils
  chef/bzip2
  chef/coreutils
  chef/file
  chef/findutils
  chef/gawk
  chef/grep
  chef/gzip
  chef/hab-static
  chef/sed
  chef/tar
  chef/wget
  chef/xz
)

program=$pkg_name

do_build() {
  cp -v $PLAN_CONTEXT/bin/${program}.sh $program

  # Use the bash from our dependency list as the shebang. Also, embed the
  # release version of the program.
  sed \
    -e "s,#!/bin/bash$,#!$(pkg_path_for bash)/bin/bash," \
    -e "s,^HAB_PLAN_BUILD=.*$,HAB_PLAN_BUILD=$pkg_version/$pkg_rel," \
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
