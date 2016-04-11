pkg_name=hab-pkg-dockerize
pkg_origin=chef
pkg_version=0.1.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_deps=(chef/coreutils chef/findutils chef/gawk chef/grep chef/bash chef/docker chef/hab-studio)
pkg_build_deps=()
pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B

program=$pkg_name

do_build() {
  cp -v $PLAN_CONTEXT/bin/${program}.sh ${program}

  # Use the bash from our dependency list as the shebang. Also, embed the
  # release version of the program.
  sed \
    -e "s,#!/bin/bash$,#!$(pkg_path_for bash)/bin/bash," \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_rel,g" \
    -i $program
}

do_install() {
  install -v -D $program $pkg_prefix/bin/$program
}

# Turn the remaining default phases into no-ops

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  return 0
}
